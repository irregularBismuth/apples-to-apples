use crate::core::cards::{GreenCard, RedCard};
use crate::core::deck::{GreenDeck, RedDeck};
use core::num::NonZeroUsize;
use dsl_ractor::*;
use ractor::RpcReplyPort;
use std::path::Path;

#[non_exhaustive]
struct DealerState {}

#[repr(transparent)]
struct Amount(pub NonZeroUsize);

pub enum DealerMsg {
    DealRedCards {
        count: Amount,
        reply: RpcReplyPort<Vec<RedCard>>,
    },
    DealGreenCards {
        count: Amount,
        reply: RpcReplyPort<Vec<GreenCard>>,
    },
    LoadDecks {
        red_path: Box<Path>,
        green_path: Box<Path>,
    },
    Shuffle,
}

#[actor(msg=(),state=())]
struct Dealer;

impl Dealer {
    actor_pre_start!({ Ok(()) });
    actor_handle!({ Ok(()) });
}
