use crate::core::cards::RedCard;
use crate::core::hand::Hand;
use crate::core::player::PlayerId;
use crate::core::player::player::Player;

struct Bot {
    hand: Hand,
    id: PlayerId,
}

impl Player for Bot {
    fn play_card(&mut self) -> anyhow::Result<RedCard> {
        self.hand.remove_card(0)
    }

    fn hand_mut(&mut self) -> &mut Hand {
        &mut self.hand
    }

    fn id(&self) -> PlayerId {
        self.id
    }
}
