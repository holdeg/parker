use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

use crate::{
    card::{Card, Rank, Suit},
    hand::Hand,
};

#[derive(Debug)]
pub struct Deck(Vec<Card>);

impl Default for Deck {
    fn default() -> Self {
        Self(
            Suit::iter()
                .map(|suit| {
                    Rank::iter()
                        .map(|rank| Card { suit, rank })
                        .collect::<Vec<_>>()
                })
                .flatten()
                .collect(),
        )
    }
}

impl Deck {
    pub fn shuffle(&mut self) {
        let mut rng = rand::rng();
        self.0.shuffle(&mut rng);
    }

    pub fn deal(mut self) -> [Hand; 4] {
        [
            Hand::from(self.0.split_off(13 * 3)),
            Hand::from(self.0.split_off(13 * 2)),
            Hand::from(self.0.split_off(13 * 1)),
            Hand::from(self.0),
        ]
    }
}
