mod green;
mod red;

pub use green::GreenDeck;
pub use red::RedDeck;

pub trait Deck {
    type Card;

    fn as_mut_slice(&mut self) -> &mut [Self::Card];

    fn pop_card(&mut self) -> Option<Self::Card>;

    fn get_mut(&mut self, idx: usize) -> Option<&mut Self::Card> {
        self.as_mut_slice().get_mut(idx)
    }

    fn shuffle(&mut self) {
        use rand::rng;
        use rand::seq::SliceRandom;

        let mut rng = rng();
        self.as_mut_slice().shuffle(&mut rng);
    }

    fn draw(&mut self) -> Option<Self::Card> {
        self.pop_card()
    }
}
