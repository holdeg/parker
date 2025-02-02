use parker::{
    auction::{Auction, Seat},
    deck::Deck,
    hand::Hand,
};

#[derive(Debug)]
pub struct Model {
    pub auction: Auction,
    pub hands: [Hand; 4],
    pub exit: bool,
}

impl Default for Model {
    fn default() -> Self {
        Self::new()
    }
}

impl Model {
    pub fn new() -> Self {
        let mut deck = Deck::default();
        deck.shuffle();

        Self {
            auction: Auction::new(Seat::from_repr(rand::random_range(0..3)).unwrap()),
            hands: deck.deal(),
            exit: false,
        }
    }
}
