use clap::Parser;
use std::net::SocketAddrV4;

#[derive(Parser, Clone, Debug)]
#[command(name = "apples", about = "Apples-to-apples game")]
pub struct Cli {
    #[arg(short = 'i', long = "ip", value_name = "IP:PORT")]
    pub connect: Option<SocketAddrV4>,
    #[arg(
        short = 'b',
        long = "bind",
        default_value = "0.0.0.0:8080",
        value_name = "IP:PORT"
    )]
    pub bind: SocketAddrV4,
    #[arg(short = 'p', long = "players", default_value_t = 4,
          value_parser = clap::value_parser!(u8).range(2..=16))]
    pub max_players: u8,
}

#[derive(Debug, Clone, Copy)]
pub enum RunMode<'a> {
    Host {
        bind: SocketAddrV4,
        max_players: u8,
    },
    Client {
        connect: SocketAddrV4,
        bind_unused: &'a SocketAddrV4,
    },
}

impl Cli {
    pub fn mode(&self) -> RunMode<'_> {
        if let Some(addr) = self.connect {
            RunMode::Client {
                connect: addr,
                bind_unused: &self.bind,
            }
        } else {
            RunMode::Host {
                bind: self.bind,
                max_players: self.max_players,
            }
        }
    }
}
