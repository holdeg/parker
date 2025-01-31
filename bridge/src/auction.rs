use std::ops::{Add, Sub};

use deranged::RangedU8;
use strum::FromRepr;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, FromRepr)]
pub enum Seat {
    North = 0,
    East,
    South,
    West,
}

impl Add<usize> for Seat {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Seat::from_repr((self as usize + rhs) % 4).unwrap()
    }
}

impl Sub<usize> for Seat {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Seat::from_repr((((self as isize - rhs as isize) % 4) + 4) as usize % 4).unwrap()
    }
}

impl Sub<Seat> for Seat {
    type Output = usize;

    fn sub(self, rhs: Seat) -> Self::Output {
        ((self as isize - rhs as isize) + 4) as usize % 4
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Auction {
    pub dealer: Seat,
    pub sequence: Vec<AuctionBid>,
}

impl Default for Auction {
    fn default() -> Self {
        Self {
            dealer: Seat::North,
            sequence: Default::default(),
        }
    }
}

impl Auction {
    pub fn new(dealer: Seat) -> Self {
        Self {
            dealer,
            sequence: vec![],
        }
    }

    pub fn turn(&self) -> Seat {
        self.dealer + self.sequence.len()
    }
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

    #[test]
    fn seat_addition() {
        assert_eq!(Seat::South, Seat::North + 2);
        assert_eq!(Seat::South, Seat::North + 6);

        let mut auction = Auction::new(Seat::West);
        auction.sequence.append(&mut vec![AuctionBid::Pass; 3]);
        assert_eq!(Seat::South, auction.turn());
    }

    #[test]
    fn seat_subtraction() {
        assert_eq!(Seat::South, Seat::North - 2);
        assert_eq!(Seat::East, Seat::North - 3);
        assert_eq!(Seat::South, Seat::West - 5);

        assert_eq!(2, Seat::North - Seat::South);
        assert_eq!(Seat::East - Seat::West, Seat::West - Seat::East);
    }
}
