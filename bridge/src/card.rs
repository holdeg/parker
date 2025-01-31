use std::{
    fmt::{Display, Write},
    str::FromStr,
};

use strum::EnumIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter)]
pub enum Suit {
    Spades = 3,
    Hearts = 2,
    Diamonds = 1,
    Clubs = 0,
}

impl FromStr for Suit {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "spades" | "s" => Ok(Self::Spades),
            "hearts" | "h" => Ok(Self::Hearts),
            "diamonds" | "d" => Ok(Self::Diamonds),
            "clubs" | "c" => Ok(Self::Clubs),
            _ => Err("not a suit".to_string()),
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
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Suit::Spades),
            'H' => Ok(Suit::Hearts),
            'D' => Ok(Suit::Diamonds),
            'C' => Ok(Suit::Clubs),
            _ => Err(format!("provided char {} is not a suit", value)),
        }
    }
}

impl Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            Suit::Spades => '♠',
            Suit::Hearts => '♥',
            Suit::Diamonds => '♦',
            Suit::Clubs => '♣',
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
            Rank::Jack => 1,
            Rank::Queen => 2,
            Rank::King => 3,
            Rank::Ace => 4,
            _ => 0,
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "10",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
            Rank::Ace => "A",
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
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(Rank::Two),
            '3' => Ok(Rank::Three),
            '4' => Ok(Rank::Four),
            '5' => Ok(Rank::Five),
            '6' => Ok(Rank::Six),
            '7' => Ok(Rank::Seven),
            '8' => Ok(Rank::Eight),
            '9' => Ok(Rank::Nine),
            'T' => Ok(Rank::Ten),
            'J' => Ok(Rank::Jack),
            'Q' => Ok(Rank::Queen),
            'K' => Ok(Rank::King),
            'A' => Ok(Rank::Ace),
            _ => Err(format!("provided char {} is not a rank", value)),
        }
    }
}

impl TryFrom<u8> for Rank {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            2 => Ok(Rank::Two),
            3 => Ok(Rank::Three),
            4 => Ok(Rank::Four),
            5 => Ok(Rank::Five),
            6 => Ok(Rank::Six),
            7 => Ok(Rank::Seven),
            8 => Ok(Rank::Eight),
            9 => Ok(Rank::Nine),
            10 => Ok(Rank::Ten),
            11 => Ok(Rank::Jack),
            12 => Ok(Rank::Queen),
            13 => Ok(Rank::King),
            14 => Ok(Rank::Ace),
            _ => Err(()),
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
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut char_iter = s.chars();
        let rank = char_iter.next().ok_or("couldn't determine rank char")?;
        let suit = char_iter.next().ok_or("couldn't determine suit char")?;
        if char_iter.next().is_some() {
            Err("too many chars provided")?;
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
