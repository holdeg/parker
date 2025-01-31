use std::{
    cmp::max,
    collections::VecDeque,
    fmt::Display,
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

impl Display for BiddingSuit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Suit(suit) => suit.fmt(f),
            Self::NoTrumps => f.write_str("NT"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct ContractBid {
    level: RangedU8<1, 7>,
    suit: BiddingSuit,
}

impl ContractBid {
    pub fn new(level: u8, suit: BiddingSuit) -> Result<Self, String> {
        Ok(Self {
            level: RangedU8::new(level).ok_or("bid level must be between 1 and 7, inclusive")?,
            suit,
        })
    }
}

impl FromStr for ContractBid {
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

impl Display for ContractBid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}{}", self.level, self.suit))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuctionBid {
    Bid(ContractBid),
    Pass,
    Double,
    Redouble,
}

impl AuctionBid {
    pub fn suit_bid(level: u8, suit: BiddingSuit) -> Result<Self, String> {
        Ok(Self::Bid(ContractBid::new(level, suit)?))
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
        let mut massaged = s.to_lowercase();
        massaged.retain(|c| !c.is_whitespace());
        match massaged.as_str() {
            "pass" | "p" | "nobid" => Ok(Self::Pass),
            "double" | "x" | "dbl" => Ok(Self::Double),
            "redouble" | "xx" | "redbl" => Ok(Self::Redouble),
            _ => Ok(AuctionBid::Bid(massaged.parse()?)),
        }
    }
}

impl Display for AuctionBid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bid(bid) => bid.fmt(f),
            AuctionBid::Pass => f.write_str("Pass"),
            AuctionBid::Double => f.write_str("Dbl"),
            AuctionBid::Redouble => f.write_str("Redbl"),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
    Undoubled,
    Doubled,
    Redoubled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Contract {
    pub bid: ContractBid,
    pub status: Status,
}

impl From<ContractBid> for Contract {
    fn from(value: ContractBid) -> Self {
        Self {
            bid: value,
            status: Status::Undoubled,
        }
    }
}

impl FromStr for Contract {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut massaged = s.to_lowercase();
        massaged.retain(|c| !c.is_whitespace());

        let mut chars = massaged.chars().rev().peekable();
        let mut status = Status::Undoubled;

        if *chars.peek().ok_or(ParseError::TooShort)? == 'x' {
            status = Status::Doubled;
            chars.next();
            if *chars.peek().ok_or(ParseError::TooShort)? == 'x' {
                status = Status::Redoubled;
                chars.next();
            }
        }

        Ok(Self {
            bid: chars.rev().collect::<String>().parse()?,
            status,
        })
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

impl Display for Auction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let headers = "+-- N --+-- E --+-- S --+-- W --+";
        let divider = "+-------+-------+-------+-------+";

        let mut buffer = headers.to_string();

        let mut string_bids: VecDeque<_> = self
            .sequence
            .clone()
            .into_iter()
            .map(|bid| bid.to_string())
            .collect();

        for _ in 0..self.dealer as usize {
            string_bids.push_front("".to_string());
        }

        let mut bid_iterator = string_bids.into_iter().peekable();
        while bid_iterator.peek().is_some() {
            buffer += &format!(
                "\n| {0: <5} | {1: <5} | {2: <5} | {3: <5} |\n",
                bid_iterator.next().unwrap_or_default(),
                bid_iterator.next().unwrap_or_default(),
                bid_iterator.next().unwrap_or_default(),
                bid_iterator.next().unwrap_or_default()
            );
            buffer += divider;
        }

        f.write_str(&buffer)
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

    pub fn closed(&self) -> bool {
        self.sequence.len() >= 4
            && self
                .sequence
                .iter()
                .rev()
                .take(3)
                .all(|bid| *bid == AuctionBid::Pass)
    }

    pub fn contract(&self) -> Option<Contract> {
        let mut status = Status::Undoubled;
        let mut contract_bid = None;
        for auction_bid in self.sequence.iter().rev() {
            match auction_bid {
                AuctionBid::Pass => continue,
                AuctionBid::Double => status = max(status, Status::Doubled),
                AuctionBid::Redouble => status = max(status, Status::Redoubled),
                AuctionBid::Bid(bid) => {
                    contract_bid = Some(*bid);
                    break;
                }
            }
        }
        Some(Contract {
            bid: contract_bid?,
            status,
        })
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
            Ok(AuctionBid::suit_bid(1, BiddingSuit::Suit(Suit::Clubs)).unwrap()),
            "1c".parse()
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
    fn parse_contract() {
        assert_eq!(
            Ok(Contract {
                bid: "1s".parse().unwrap(),
                status: Status::Redoubled
            }),
            "1sxx".parse()
        );
        assert_eq!(
            Ok(Contract {
                bid: "2nt".parse().unwrap(),
                status: Status::Undoubled
            }),
            "2nt".parse()
        );
        assert_eq!(
            Ok(Contract {
                bid: "3h".parse().unwrap(),
                status: Status::Doubled
            }),
            "3h x".parse()
        );
        assert_eq!(
            Ok(Contract {
                bid: "4d".parse().unwrap(),
                status: Status::Redoubled
            }),
            "4    diamond x    x   ".parse()
        );

        assert_eq!(Err(ParseError::TooShort), "".parse::<Contract>());
        assert_eq!(
            Err(ParseError::BidLevelOutOfBounds),
            "8cxx".parse::<Contract>()
        );
        assert_eq!(Err(ParseError::SuitNotValid), "5dxxx".parse::<Contract>());
    }

    fn game_with_small_interference() -> Auction {
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

        auction
    }

    fn three_passes() -> Auction {
        let mut auction = Auction::new(Seat::West);
        auction.sequence.append(&mut vec![AuctionBid::Pass; 3]);

        auction
    }

    #[test]
    fn bid_comparison() {
        assert!("6H".parse::<ContractBid>().unwrap() > "4NT".parse().unwrap());
        assert!("1D".parse::<ContractBid>().unwrap() > "1C".parse().unwrap());
        assert!("2NT".parse::<ContractBid>().unwrap() > "2S".parse().unwrap());
        assert!("3H".parse::<ContractBid>().unwrap() == "3H".parse().unwrap());
        assert!("5C".parse::<ContractBid>().unwrap() != "4C".parse().unwrap());
    }

    #[test]
    fn auction_bid_displays() {
        assert_eq!("6♥", "6H".parse::<AuctionBid>().unwrap().to_string());
        assert_eq!(
            "7NT",
            "7 no trumps".parse::<AuctionBid>().unwrap().to_string()
        );
        assert_eq!("Pass", "no bid".parse::<AuctionBid>().unwrap().to_string());
        assert_eq!("Dbl", "x".parse::<AuctionBid>().unwrap().to_string());
        assert_eq!(
            "Redbl",
            "redouble".parse::<AuctionBid>().unwrap().to_string()
        );
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

        let auction = three_passes();
        assert_eq!(Seat::South, auction.turn());

        assert!(!auction.closed());
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
        let auction = game_with_small_interference();

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

        assert!(auction.closed());
    }

    #[test]
    fn auction_completion() {
        let game_auction = game_with_small_interference();

        assert!(game_auction.closed());
        assert_eq!(
            Some("3NT".parse::<Contract>().unwrap()),
            game_auction.contract()
        );

        let three_passes = three_passes();

        assert!(!three_passes.closed());
        assert_eq!(None, three_passes.contract());
    }

    #[test]
    fn auction_display() {
        assert_eq!(
            "+-- N --+-- E --+-- S --+-- W --+\n\
             |       |       | Pass  | Pass  |\n\
             +-------+-------+-------+-------+\n\
             | 1♦    | 1♥    | 1NT   | Pass  |\n\
             +-------+-------+-------+-------+\n\
             | 2NT   | Pass  | 3NT   | Pass  |\n\
             +-------+-------+-------+-------+\n\
             | Pass  | Pass  |       |       |\n\
             +-------+-------+-------+-------+",
            game_with_small_interference().to_string()
        );

        assert_eq!(
            "+-- N --+-- E --+-- S --+-- W --+\n\
             |       |       |       | Pass  |\n\
             +-------+-------+-------+-------+\n\
             | Pass  | Pass  |       |       |\n\
             +-------+-------+-------+-------+",
            three_passes().to_string()
        );
    }
}
