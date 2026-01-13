use apples_to_apples::core::deck::Deck;
use apples_to_apples::networking::{
    gateway::Gateway,
    protocol::{ClientToServer, ServerToClient},
};
use apples_to_apples::parsing::Cli;
use apples_to_apples::parsing::Mode;
use apples_to_apples::parsing::{
    cards::{parse_green_cards, parse_red_cards},
    config::parse_config,
};
use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio_util::codec::{Framed, LengthDelimitedCodec};

pub struct App {
    mode: Option<Mode>,
}

impl App {
    pub fn new() -> Self {
        Self { mode: None }
    }

    pub async fn intilize(&mut self) -> anyhow::Result<()> {
        let cli: Mode = Cli::parse();
        let path = "./Config.toml";
        let _config = parse_config(path).await?;
        let gc = "./assets/original/greenApples.txt";
        let _cards = parse_green_cards(gc).await?;
        let rc = "./assets/original/redApples.txt";
        let mut cards2 = parse_red_cards(rc).await?;
        cards2.shuffle();

        Ok(())
    }

    pub async fn run(self) -> anyhow::Result<()> {
        /*match mode {
            Mode::Server { addr, required } => Gateway::with_addr(addr, required).run().await,
            Mode::Client { addr } => run_client(&addr).await,
        }*/
        Ok(())
    }
}

async fn run_client(addr: &SocketAddr) -> anyhow::Result<()> {
    let stream = TcpStream::connect(addr).await?;
    println!("connected to {addr}");
    let mut framed = Framed::new(stream, LengthDelimitedCodec::new());

    let join = serde_json::to_vec(&ClientToServer::Join)?;
    framed.send(join.into()).await?;
    println!("sent Join");

    for _ in 0..4 {
        match timeout(Duration::from_secs(300), framed.next()).await {
            Ok(Some(Ok(bytes))) => match serde_json::from_slice::<ServerToClient>(&bytes) {
                Ok(msg) => println!("server {:?}", msg),
                Err(e) => {
                    println!("failed to parse server message: {e}");
                    break;
                }
            },
            Ok(Some(Err(e))) => {
                println!("socket error: {e}");
                break;
            }
            Ok(None) => {
                println!("server closed connection");
                break;
            }
            Err(_) => {
                println!("timed out waiting for server");
                break;
            }
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut app = App::new();
    app.intilize().await?;
    app.run().await?;
    Ok(())
}
