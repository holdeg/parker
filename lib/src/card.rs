use std::{
    fmt::{Display, Write},
    str::FromStr,
};

use strum::EnumIter;

use crate::error::ParseError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum Suit {
    Spades = 3,
    Hearts = 2,
    Diamonds = 1,
    Clubs = 0,
}

impl FromStr for Suit {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "spades" | "s" | "spade" => Ok(Self::Spades),
            "hearts" | "h" | "heart" => Ok(Self::Hearts),
            "diamonds" | "d" | "diamond" => Ok(Self::Diamonds),
            "clubs" | "c" | "club" => Ok(Self::Clubs),
            _ => Err(ParseError::SuitNotValid),
        }
    }
}

impl From<Suit> for char {
    fn from(value: Suit) -> Self {
        match value {
            Suit::Spades => 'S',
            Suit::Hearts => 'H',
            Suit::Diamonds => 'D',
            Suit::Clubs => 'C',
        }
    }
}

impl TryFrom<char> for Suit {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Self::Spades),
            'H' => Ok(Self::Hearts),
            'D' => Ok(Self::Diamonds),
            'C' => Ok(Self::Clubs),
            _ => Err(ParseError::SuitNotValid),
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Self::Spades => '♠',
            Self::Hearts => '♥',
            Self::Diamonds => '♦',
            Self::Clubs => '♣',
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum Rank {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    pub fn high_card_points(&self) -> u8 {
        match self {
            Self::Jack => 1,
            Self::Queen => 2,
            Self::King => 3,
            Self::Ace => 4,
            _ => 0,
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Two => "2",
            Self::Three => "3",
            Self::Four => "4",
            Self::Five => "5",
            Self::Six => "6",
            Self::Seven => "7",
            Self::Eight => "8",
            Self::Nine => "9",
            Self::Ten => "10",
            Self::Jack => "J",
            Self::Queen => "Q",
            Self::King => "K",
            Self::Ace => "A",
        })
    }
}

impl From<Rank> for char {
    fn from(value: Rank) -> Self {
        match value {
            Rank::Two => '2',
            Rank::Three => '3',
            Rank::Four => '4',
            Rank::Five => '5',
            Rank::Six => '6',
            Rank::Seven => '7',
            Rank::Eight => '8',
            Rank::Nine => '9',
            Rank::Ten => 'T',
            Rank::Jack => 'J',
            Rank::Queen => 'Q',
            Rank::King => 'K',
            Rank::Ace => 'A',
        }
    }
}

impl TryFrom<char> for Rank {
    type Error = ParseError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            'T' => Ok(Self::Ten),
            'J' => Ok(Self::Jack),
            'Q' => Ok(Self::Queen),
            'K' => Ok(Self::King),
            'A' => Ok(Self::Ace),
            _ => Err(ParseError::RankNotValid),
        }
    }
}

impl TryFrom<u8> for Rank {
    type Error = ParseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Four),
            5 => Ok(Self::Five),
            6 => Ok(Self::Six),
            7 => Ok(Self::Seven),
            8 => Ok(Self::Eight),
            9 => Ok(Self::Nine),
            10 => Ok(Self::Ten),
            11 => Ok(Self::Jack),
            12 => Ok(Self::Queen),
            13 => Ok(Self::King),
            14 => Ok(Self::Ace),
            _ => Err(ParseError::RankNotValid),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}{}", self.suit, self.rank))
    }
}

impl FromStr for Card {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut char_iter = s.chars();
        let rank = char_iter.next().ok_or(ParseError::TooShort)?;
        let suit = char_iter.next().ok_or(ParseError::TooShort)?;
        if char_iter.next().is_some() {
            Err(ParseError::TooLong)?;
        }

        Ok(Self {
            suit: suit.try_into()?,
            rank: rank.try_into()?,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rank_ordering() {
        assert!(Rank::Two < Rank::Five);
        assert!(Rank::Ace > Rank::King);
        assert!(Rank::Four <= Rank::Ten);
        assert!(Rank::Jack >= Rank::Jack);
        assert!(Rank::Three != Rank::Queen);
        assert!(Rank::Seven == Rank::Seven);
    }

    #[test]
    fn rank_casting() {
        assert_eq!(2, Rank::Two as isize);
        assert_eq!(14, Rank::Ace as usize);
        assert_eq!(10, Rank::Ten as u64);
    }

    #[test]
    fn card_parse() {
        assert_eq!(
            Ok(Card {
                suit: Suit::Hearts,
                rank: Rank::Ten,
            }),
            "TH".parse()
        );
        assert_eq!(
            Ok(Card {
                suit: Suit::Clubs,
                rank: Rank::Two,
            }),
            "2C".parse()
        );
        assert_eq!(
            Ok(Card {
                suit: Suit::Diamonds,
                rank: Rank::Queen,
            }),
            "QD".parse()
        );
        assert!("Kx".parse::<Card>().is_err());
        assert!("#H".parse::<Card>().is_err());
        assert!("1S".parse::<Card>().is_err());
        assert!("".parse::<Card>().is_err());
        assert!("3C7H".parse::<Card>().is_err());
        assert!("6".parse::<Card>().is_err());
    }
}
