use crate::core::cards::RedCard;

#[derive(Default)]
#[non_exhaustive]
pub struct RedDeck(Vec<RedCard>);

impl RedDeck {
    ///Create a empty deck
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl From<Vec<RedCard>> for RedDeck {
    fn from(v: Vec<RedCard>) -> Self {
        Self(v)
    }
}
