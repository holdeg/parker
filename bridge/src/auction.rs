use deranged::RangedU8;

use crate::card::Suit;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BiddingSuit {
    Suit(Suit),
    NoTrumps,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bid {
    level: RangedU8<1, 7>,
    suit: BiddingSuit,
}

impl Bid {
    pub fn new(level: u8, suit: BiddingSuit) -> Result<Self, String> {
        Ok(Self {
            level: RangedU8::new(level).ok_or("Bid level must be between 1 and 7, inclusive")?,
            suit,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuctionBid {
    Bid(Bid),
    Pass,
    Double,
    Redouble,
}

impl AuctionBid {
    pub fn new(level: u8, suit: BiddingSuit) -> Result<Self, String> {
        Ok(Self::Bid(Bid::new(level, suit)?))
    }
}

impl PartialOrd for AuctionBid {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Bid(self_bid), Self::Bid(other_bid)) => Some(self_bid.cmp(other_bid)),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Seat {
    North,
    East,
    South,
    West,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bid_ordering() {
        assert!(
            Bid::new(6, BiddingSuit::Suit(Suit::Hearts)).unwrap()
                > Bid::new(4, BiddingSuit::NoTrumps).unwrap()
        );
        assert!(
            Bid::new(1, BiddingSuit::Suit(Suit::Diamonds)).unwrap()
                > Bid::new(1, BiddingSuit::Suit(Suit::Clubs)).unwrap()
        );
        assert!(
            Bid::new(2, BiddingSuit::NoTrumps).unwrap()
                > Bid::new(2, BiddingSuit::Suit(Suit::Spades)).unwrap()
        );
    }

    #[test]
    fn auction_bid_ordering() {
        assert!(
            AuctionBid::new(6, BiddingSuit::Suit(Suit::Hearts)).unwrap()
                > AuctionBid::new(4, BiddingSuit::NoTrumps).unwrap()
        );

        let one_diamond = AuctionBid::new(1, BiddingSuit::Suit(Suit::Diamonds)).unwrap();
        let pass = AuctionBid::Pass;

        assert!(!(one_diamond < pass));
        assert!(!(one_diamond > pass));
        assert!(!(one_diamond <= pass));
        assert!(!(one_diamond >= pass));
    }
}
