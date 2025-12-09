use super::{super::cards::GreenCard, Deck};

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct GreenDeck(Vec<GreenCard>);

impl GreenDeck {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl From<Vec<GreenCard>> for GreenDeck {
    fn from(v: Vec<GreenCard>) -> Self {
        Self(v)
    }
}

impl Deck for GreenDeck {
    type Card = GreenCard;

    #[inline]
    fn as_mut_slice(&mut self) -> &mut [Self::Card] {
        &mut self.0
    }

    fn pop_card(&mut self) -> Option<Self::Card> {
        self.0.pop()
    }
}

impl GreenDeck {
    pub fn iter(&self) -> std::slice::Iter<'_, GreenCard> {
        self.0.iter()
    }
}
