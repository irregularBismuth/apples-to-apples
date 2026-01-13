use crate::actors::connection::{Connection, ConnectionMsg, HostMsg};
use crate::networking::protocol::{ClientToServer, PlayerId, ServerToClient};
use ractor::Actor;
use std::collections::HashMap;
use tokio::net::TcpListener;
use tokio::sync::mpsc;

pub struct Gateway {
    bind_addr: String,
    required_connections: usize,
    next_player_id: u64,
}

impl Gateway {
    pub fn new(required_connections: usize) -> Self {
        Self {
            bind_addr: "127.0.0.1:8080".into(),
            required_connections,
            next_player_id: 1,
        }
    }

    pub fn with_addr(bind_addr: impl Into<String>, required_connections: usize) -> Self {
        Self {
            bind_addr: bind_addr.into(),
            required_connections,
            next_player_id: 1,
        }
    }

    pub async fn run(self) -> anyhow::Result<()> {
        let listener = TcpListener::bind(&self.bind_addr).await?;
        println!("listening on {}", self.bind_addr);
        let (host_tx, mut host_rx) = mpsc::channel::<HostMsg>(64);
        let mut players: HashMap<PlayerId, ractor::ActorRef<ConnectionMsg>> = HashMap::new();
        let mut game_started = false;
        let mut next_player_id = self.next_player_id;
        loop {
            tokio::select! {
                accept_result = listener.accept() => {
                    let (socket, addr) = accept_result?;
                    let tx = host_tx.clone();
                    let _ = Actor::spawn(
                        Some(format!("connection-{}", addr)),
                        Connection,
                        (socket, tx),
                    )
                    .await?;
                }
                maybe_event = host_rx.recv() => {
                    let event = match maybe_event {
                        Some(event) => event,
                        None => break,
                    };
                    match event {
                        HostMsg::NewConnection(_, addr) => {
                            println!("client connected from {addr}");
                        }
                        HostMsg::ClientEvent { connection, player_id: _, message } => {
                            match message {
                                ClientToServer::Join => {
                                    if game_started {
                                        let _ = ractor::cast!(
                                            connection,
                                            ConnectionMsg::FromHost(ServerToClient::Error {
                                                message: "game already started".into()
                                            })
                                        );
                                        let _ = ractor::cast!(connection, ConnectionMsg::Close);
                                        continue;
                                    }
                                    let pid = PlayerId(next_player_id);
                                    next_player_id = next_player_id.wrapping_add(1);
                                    players.insert(pid, connection.clone());
                                    let _ = ractor::cast!(
                                        connection,
                                        ConnectionMsg::FromHost(ServerToClient::Welcome { player_id: pid })
                                    );

                                    if self.required_connections > 0
                                        && players.len() >= self.required_connections
                                        && !game_started
                                    {
                                        game_started = true;
                                        let players_snapshot: Vec<PlayerId> = players.keys().cloned().collect();
                                        for conn in players.values() {
                                            let _ = ractor::cast!(
                                                conn,
                                                ConnectionMsg::FromHost(ServerToClient::GameStart {
                                                    players: players_snapshot.clone()
                                                })
                                            );
                                        }
                                    }
                                }
                                ClientToServer::Ping => {
                                    let _ = ractor::cast!(
                                        connection,
                                        ConnectionMsg::FromHost(ServerToClient::Pong)
                                    );
                                }
                            }
                        }
                        HostMsg::ProtocolError { connection, player_id, reason } => {
                            let _ = ractor::cast!(
                                connection,
                                ConnectionMsg::FromHost(ServerToClient::Error { message: reason })
                            );
                            if player_id != PlayerId::default() {
                                players.remove(&player_id);
                            }
                        }
                        HostMsg::Disconnected { connection: _, player_id, reason } => {
                            if player_id != PlayerId::default() {
                                players.remove(&player_id);
                            }
                            if player_id != PlayerId::default() {
                                println!("player {player_id:?} disconnected: {reason}");
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
