use super::PlayerId;
use crate::core::cards::RedCard;
use crate::core::hand::Hand;

pub trait Player {
    fn play_card(&mut self) -> anyhow::Result<RedCard>;
    fn hand_mut(&mut self) -> &mut Hand;
    fn id(&self) -> PlayerId;
}

#[inline]
fn deal_card_to_player<P: Player>(player: &mut P, card: RedCard) -> anyhow::Result<()> {
    player.hand_mut().add_card(card)
}
