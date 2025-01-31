use std::{
    ops::{Add, Sub},
    str::FromStr,
};

use deranged::RangedU8;
use strum::FromRepr;

use crate::{card::Suit, error::ParseError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BiddingSuit {
    Suit(Suit),
    NoTrumps,
}

impl FromStr for BiddingSuit {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "nt" | "notrumps" | "no trumps" => Ok(Self::NoTrumps),
            _ => Ok(Self::Suit(s.parse()?)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bid {
    level: RangedU8<1, 7>,
    suit: BiddingSuit,
}

impl Bid {
    pub fn new(level: u8, suit: BiddingSuit) -> Result<Self, String> {
        Ok(Self {
            level: RangedU8::new(level).ok_or("bid level must be between 1 and 7, inclusive")?,
            suit,
        })
    }
}

impl FromStr for Bid {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (level, suit) = s.split_at_checked(1).ok_or(ParseError::TooShort)?;

        Ok(Self {
            level: RangedU8::new(
                level
                    .parse()
                    .map_err(|_| ParseError::BidLevelNotAnInteger)?,
            )
            .ok_or(ParseError::BidLevelOutOfBounds)?,
            suit: suit.parse()?,
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
    pub fn suit_bid(level: u8, suit: BiddingSuit) -> Result<Self, String> {
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

impl FromStr for AuctionBid {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pass" | "p" => Ok(Self::Pass),
            "double" | "x" => Ok(Self::Double),
            "redouble" | "xx" => Ok(Self::Redouble),
            _ => Ok(AuctionBid::Bid(s.parse()?)),
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
    dealer: Seat,
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

    pub fn bids_for(&self, seat: Seat) -> Vec<&AuctionBid> {
        self.sequence
            .iter()
            .skip(seat - self.dealer)
            .step_by(4)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_auction_bid() {
        assert_eq!(Ok(AuctionBid::Pass), "pass".parse());
        assert_eq!(
            Ok(AuctionBid::suit_bid(4, BiddingSuit::NoTrumps).unwrap()),
            "4NT".parse()
        );
        assert_eq!(Ok(AuctionBid::Redouble), "xx".parse());
        assert_eq!(
            Ok(AuctionBid::suit_bid(1, BiddingSuit::Suit(Suit::Spades)).unwrap()),
            "1s".parse()
        );

        assert_eq!(
            Err(ParseError::BidLevelOutOfBounds),
            "8s".parse::<AuctionBid>()
        );
        assert_eq!(Err(ParseError::SuitNotValid), "5a".parse::<AuctionBid>());
        assert_eq!(
            Err(ParseError::BidLevelNotAnInteger),
            "ant".parse::<AuctionBid>()
        );
        assert_eq!(Err(ParseError::TooShort), "".parse::<AuctionBid>());
    }

    #[test]
    fn bid_comparison() {
        assert!("6H".parse::<Bid>().unwrap() > "4NT".parse().unwrap());
        assert!("1D".parse::<Bid>().unwrap() > "1C".parse().unwrap());
        assert!("2NT".parse::<Bid>().unwrap() > "2S".parse().unwrap());
        assert!("3H".parse::<Bid>().unwrap() == "3H".parse().unwrap());
        assert!("5C".parse::<Bid>().unwrap() != "4C".parse().unwrap());
    }

    #[test]
    fn auction_bid_ordering() {
        assert!("6H".parse::<AuctionBid>().unwrap() > "4NT".parse().unwrap());

        let one_diamond: AuctionBid = "1D".parse().unwrap();
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

    #[test]
    fn auction_by_seat() {
        let mut auction = Auction::new(Seat::South);
        auction.sequence.append(&mut vec![AuctionBid::Pass; 2]);
        auction.sequence.push("1D".parse().unwrap());
        auction.sequence.push("1H".parse().unwrap());
        auction.sequence.push("1NT".parse().unwrap());
        auction.sequence.push(AuctionBid::Pass);
        auction.sequence.push("2NT".parse().unwrap());
        auction.sequence.push(AuctionBid::Pass);
        auction.sequence.push("3NT".parse().unwrap());
        auction.sequence.append(&mut vec![AuctionBid::Pass; 3]);

        assert_eq!(
            vec![
                &AuctionBid::Pass,
                &"1NT".parse().unwrap(),
                &"3NT".parse().unwrap(),
            ],
            auction.bids_for(Seat::South)
        );
        assert_eq!(vec![&AuctionBid::Pass; 3], auction.bids_for(Seat::West));
        assert_eq!(
            vec![
                &"1D".parse().unwrap(),
                &"2NT".parse().unwrap(),
                &AuctionBid::Pass,
            ],
            auction.bids_for(Seat::North)
        );
        assert_eq!(
            vec![&"1H".parse().unwrap(), &AuctionBid::Pass, &AuctionBid::Pass],
            auction.bids_for(Seat::East)
        );
    }
}
