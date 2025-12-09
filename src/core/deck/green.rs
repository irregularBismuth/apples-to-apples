use super::super::cards::GreenCard;

#[derive(Default)]
#[non_exhaustive]
pub struct GreenDeck(Vec<GreenCard>);

impl GreenDeck {
    ///Create a new GreenDeck
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl From<Vec<GreenCard>> for GreenDeck {
    fn from(v: Vec<GreenCard>) -> Self {
        Self(v)
    }
}
