use parker::{
    auction::{Auction, AuctionBid, Seat},
    deck::Deck,
    error::ParseError,
    hand::Hand,
};

#[derive(Debug)]
pub struct Model {
    pub auction: Auction,
    pub hands: [Hand; 4],
    pub typed: String,
    pub parsed_bid: Option<Result<AuctionBid, ParseError>>,
    pub exit: bool,
}

impl Default for Model {
    /// Returns a new Model.
    /// **NOTE** that this implementation is not deterministic and uses RNG both to
    /// shuffle the initial deck, and to pick the dealer.
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
            typed: String::default(),
            parsed_bid: None,
            exit: false,
        }
    }
}
