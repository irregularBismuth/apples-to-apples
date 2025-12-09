use apples_to_apples::parsing::{
    cards::{parse_green_cards, parse_red_cards},
    config::parse_config,
};

use apples_to_apples::core::deck::Deck;

pub struct App {}

impl App {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn intilize(&mut self) -> anyhow::Result<()> {
        let path = "./Config.toml";
        let config = parse_config(path).await?;
        let gc = "./assets/original/greenApples.txt";
        let cards = parse_green_cards(gc).await?;
        let rc = "./assets/original/redApples.txt";
        let mut cards2 = parse_red_cards(rc).await?;
        cards2.shuffle();

        Ok(())
    }

    pub async fn run(self) -> anyhow::Result<()> {
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut app = App::new();
    app.intilize().await?;
    app.run().await?;
    Ok(())
}
