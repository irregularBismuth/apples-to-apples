use clap::{Parser, Subcommand};

#[derive(Parser, Clone)]
#[command(name = "apples", about = "Host / client")]
pub struct Cli {
    #[command(subcommand)]
    mode: Mode,
}

#[derive(Subcommand, Clone)]
pub enum Mode {
    Host {
        #[arg(long, default_value = "0.0.0.0:8080")]
        bind: String,
        #[arg(long, default_value_t = 4)]
        max_players: u8,
    },
    Client {
        #[arg(long)]
        connect: String,
    },
}
