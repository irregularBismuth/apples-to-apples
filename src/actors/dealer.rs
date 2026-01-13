use crate::core::cards::{GreenCard, RedCard};
use crate::core::deck::{GreenDeck, RedDeck};
use crate::parsing::cards::{parse_green_cards, parse_red_cards};
use core::num::NonZeroUsize;
use dsl_ractor::*;
use ractor::RpcReplyPort;
use std::path::PathBuf;

#[non_exhaustive]
struct DealerArgs {
    green: PathBuf,
    red: PathBuf,
}

#[non_exhaustive]
struct DealerState {
    green: GreenDeck,
    red: RedDeck,
}

#[repr(transparent)]
pub struct Amount(pub NonZeroUsize);

pub enum DealerMsg {
    DealRedCards {
        count: Amount,
        reply: RpcReplyPort<Vec<RedCard>>,
    },
    DealGreenCards {
        count: Amount,
        reply: RpcReplyPort<Vec<GreenCard>>,
    },
    Shuffle,
}
use crate::core::deck::Deck;

async fn load() -> anyhow::Result<DealerState> {
    const GC: &str = "./assets/original/greenApples.txt";
    let cards = parse_green_cards(GC).await?;
    let green: GreenDeck = cards.into();
    let RC: &str = "./assets/original/redApples.txt";
    let red: RedDeck = parse_red_cards(RC).await?.into();
    Ok(DealerState { green, red })
}

#[actor(msg=DealerMsg,state=DealerState,args=DealerArgs)]
struct Dealer;

impl Dealer {
    actor_pre_start!({
        let state = load().await?;
        Ok(state)
    });

    actor_handle!({
        match msg {
            DealerMsg::DealRedCards { count, reply } => {}
            DealerMsg::DealGreenCards { count, reply } => {
                todo!("deal cards")
            }
            DealerMsg::Shuffle => {
                state.green.shuffle();
                state.red.shuffle();
            }
        }
        Ok(())
    });
}
