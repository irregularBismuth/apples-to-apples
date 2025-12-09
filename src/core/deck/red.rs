use super::Deck;
use crate::core::cards::RedCard;

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct RedDeck(Vec<RedCard>);

impl RedDeck {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<Vec<RedCard>> for RedDeck {
    fn from(v: Vec<RedCard>) -> Self {
        Self(v)
    }
}

impl Deck for RedDeck {
    type Card = RedCard;

    #[inline]
    fn pop_card(&mut self) -> Option<Self::Card> {
        self.0.pop()
    }

    #[inline]
    fn as_mut_slice(&mut self) -> &mut [Self::Card] {
        &mut self.0
    }
}

impl RedDeck {
    pub fn iter(&self) -> std::slice::Iter<'_, RedCard> {
        self.0.iter()
    }
}
