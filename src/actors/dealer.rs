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

#[actor(msg=DealerMsg,state=DealerState,args=DealerArgs)]
struct Dealer;

impl Dealer {
    actor_pre_start!({
        const GC: &str = "./assets/original/greenApples.txt";
        let cards = parse_green_cards(GC).await?;
        let green: GreenDeck = cards.into();
        let RC: &str = "./assets/original/redApples.txt";
        let red: RedDeck = parse_red_cards(RC).await?.into();
        Ok(DealerState { green, red })
    });

    actor_handle!({
        match msg {
            DealerMsg::DealRedCards { count, reply } => {
                todo!("deal cards")
            }
            DealerMsg::DealGreenCards { count, reply } => {
                todo!("deal cards")
            }
            DealerMsg::Shuffle => {
                todo!("shuffle decks");
            }
        }
        Ok(())
    });
}
