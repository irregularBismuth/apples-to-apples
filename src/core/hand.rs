use crate::core::cards::RedCard;

const HAND_SIZE: usize = 7;

pub struct Hand {
    cards: Vec<RedCard>,
}

impl Hand {
    pub fn new() -> Self {
        Self {
            cards: Vec::with_capacity(HAND_SIZE),
        }
    }

    pub fn add_card(&mut self, card: RedCard) -> anyhow::Result<()> {
        if self.is_full() {
            anyhow::bail!("Hand size full")
        }
        self.cards.push(card);
        return Ok(());
    }

    pub fn remove_card(&mut self, index: usize) -> anyhow::Result<RedCard> {
        if index >= self.cards.len() {
            return anyhow::bail!("Out of bounds");
        } else {
            Ok(self.cards.remove(index))
        }
    }

    #[inline]
    pub fn is_full(&self) -> bool {
        self.cards.len() >= HAND_SIZE
    }

    #[inline]
    pub fn cards(&self) -> &[RedCard] {
        &self.cards
    }
}
