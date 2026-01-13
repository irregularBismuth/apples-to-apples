use crate::networking::protocol::{ClientToServer, PlayerId, ServerToClient};
use dsl_ractor::{actor, actor_handle, actor_pre_start};
use futures::{SinkExt, StreamExt};
use ractor::{ActorProcessingErr, ActorRef, cast};
use std::net::SocketAddr;
use tokio::net::TcpStream;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::mpsc;
use tokio_util::bytes::Bytes;
use tokio_util::codec::{FramedRead, FramedWrite, LengthDelimitedCodec};

#[derive(Debug)]
pub enum ConnectionMsg {
    FromHost(ServerToClient),
    FromClient(ClientToServer),
    SocketClosed(String),
    Close,
}

#[derive(Debug)]
pub enum HostMsg {
    NewConnection(ActorRef<ConnectionMsg>, SocketAddr),
    ClientEvent {
        connection: ActorRef<ConnectionMsg>,
        player_id: PlayerId,
        message: ClientToServer,
    },
    ProtocolError {
        connection: ActorRef<ConnectionMsg>,
        player_id: PlayerId,
        reason: String,
    },
    Disconnected {
        connection: ActorRef<ConnectionMsg>,
        player_id: PlayerId,
        reason: String,
    },
}

pub struct ConnectionState {
    peer: SocketAddr,
    player_id: Option<PlayerId>,
    host_tx: mpsc::Sender<HostMsg>,
    writer: FramedWrite<OwnedWriteHalf, LengthDelimitedCodec>,
}

#[actor(msg=ConnectionMsg,state=ConnectionState,args=(TcpStream, mpsc::Sender<HostMsg>))]
pub struct Connection;

impl Connection {
    actor_pre_start!({
        let (stream, host_tx) = args;
        let peer = stream.peer_addr().map_err(to_actor_err)?;
        let (read_half, write_half) = stream.into_split();
        let reader = FramedRead::new(read_half, LengthDelimitedCodec::new());
        let writer = FramedWrite::new(write_half, LengthDelimitedCodec::new());

        let reader_host_tx = host_tx.clone();
        let reader_myself = myself.clone();
        tokio::spawn(async move {
            drive_reader(reader, reader_host_tx, reader_myself).await;
        });

        let _ = host_tx
            .send(HostMsg::NewConnection(myself.clone(), peer))
            .await;

        Ok(ConnectionState {
            peer,
            player_id: None,
            host_tx,
            writer,
        })
    });

    actor_handle!({
        match msg {
            ConnectionMsg::FromHost(server_msg) => {
                if let ServerToClient::Welcome { player_id } = server_msg {
                    if state.player_id.is_none() {
                        state.player_id = Some(player_id);
                    }
                }
                let bytes = serde_json::to_vec(&server_msg).map_err(to_actor_err)?;
                state
                    .writer
                    .send(Bytes::from(bytes))
                    .await
                    .map_err(to_actor_err)?;
            }
            ConnectionMsg::FromClient(client_msg) => {
                if state.player_id.is_none() {
                    if let Some(pid) = extract_player_id(&client_msg) {
                        state.player_id = Some(pid);
                    }
                }
                let pid = state.player_id.unwrap_or_default();
                let _ = state
                    .host_tx
                    .send(HostMsg::ClientEvent {
                        connection: myself.clone(),
                        player_id: pid,
                        message: client_msg,
                    })
                    .await;
            }
            ConnectionMsg::SocketClosed(reason) => {
                let pid = state.player_id.unwrap_or_default();
                let _ = state
                    .host_tx
                    .send(HostMsg::Disconnected {
                        connection: myself.clone(),
                        player_id: pid,
                        reason,
                    })
                    .await;
                myself.stop(None);
            }
            ConnectionMsg::Close => {
                let pid = state.player_id.unwrap_or_default();
                let _ = state
                    .host_tx
                    .send(HostMsg::Disconnected {
                        connection: myself.clone(),
                        player_id: pid,
                        reason: "closed by host".into(),
                    })
                    .await;
                myself.stop(None);
            }
        }
        Ok(())
    });
}

fn extract_player_id(msg: &ClientToServer) -> Option<PlayerId> {
    match msg {
        ClientToServer::Join => None,
        ClientToServer::Ping => None,
    }
}

async fn drive_reader(
    mut reader: FramedRead<OwnedReadHalf, LengthDelimitedCodec>,
    host_tx: mpsc::Sender<HostMsg>,
    connection: ActorRef<ConnectionMsg>,
) {
    while let Some(next) = reader.next().await {
        match next {
            Ok(bytes) => match serde_json::from_slice::<ClientToServer>(&bytes) {
                Ok(msg) => {
                    let _ = cast!(connection, ConnectionMsg::FromClient(msg));
                }
                Err(e) => {
                    let _ = host_tx
                        .send(HostMsg::ProtocolError {
                            connection: connection.clone(),
                            player_id: PlayerId::default(),
                            reason: e.to_string(),
                        })
                        .await;
                }
            },
            Err(e) => {
                let _ = cast!(connection, ConnectionMsg::SocketClosed(e.to_string()));
                return;
            }
        }
    }

    let _ = cast!(connection, ConnectionMsg::SocketClosed("closed".into()));
}

fn to_actor_err<E>(err: E) -> ActorProcessingErr
where
    E: std::error::Error + Send + Sync + 'static,
{
    Box::new(err)
}
