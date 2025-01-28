use std::fmt::{Display, Write};

#[derive(Debug, Clone, Copy)]
enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
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
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Suit::Spades),
            'H' => Ok(Suit::Hearts),
            'D' => Ok(Suit::Diamonds),
            'C' => Ok(Suit::Clubs),
            _ => Err(()),
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

#[derive(Debug, Clone, Copy)]
enum Rank {
    Two,
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
    type Error = ();

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
            _ => Err(()),
        }
    }
}

impl From<Rank> for u8 {
    fn from(value: Rank) -> Self {
        match value {
            Rank::Two => 2,
            Rank::Three => 3,
            Rank::Four => 4,
            Rank::Five => 5,
            Rank::Six => 6,
            Rank::Seven => 7,
            Rank::Eight => 8,
            Rank::Nine => 9,
            Rank::Ten => 10,
            Rank::Jack => 11,
            Rank::Queen => 12,
            Rank::King => 13,
            Rank::Ace => 14,
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
