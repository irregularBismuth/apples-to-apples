use apples_to_apples::core::deck::Deck;
use apples_to_apples::parsing::Cli;
use apples_to_apples::parsing::{
    cards::{parse_green_cards, parse_red_cards},
    config::parse_config,
};
use apples_to_apples::setup_tracing;
use clap::Parser;
use futures::{SinkExt, StreamExt};
use std::net::SocketAddr;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::time::timeout;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
pub struct App {
    //    mode: Option<Mode>,
}

impl App {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn intilize(&mut self) -> anyhow::Result<()> {
        setup_tracing()?;
        let cli = Cli::parse();
        if let Some(c) = cli.connect {
            tracing::info!("thing");
        } else {
            tracing::info!("host");
        }
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
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut app = App::new();
    app.intilize().await?;
    app.run().await?;
    Ok(())
}
