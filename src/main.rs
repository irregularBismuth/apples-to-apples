use apples_to_apples::parsing::{
    cards::{parse_green_cards, parse_red_cards},
    config::parse_config,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let path = "./Config.toml";
    let config = parse_config(path).await?;
    let gc = "./assets/original/greenApples.txt";
    let cards = parse_green_cards(gc).await?;
    let rc = "./assets/original/redApples.txt";
    let cards2 = parse_red_cards(rc).await?;
    println!("{} {} {}", config.get(), cards.len(), cards2.len());
    Ok(())
}
