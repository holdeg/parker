use std::fmt::Display;

use crate::card::{Card, Suit};

#[derive(Debug, Clone)]
pub struct Hand {
    spades: Vec<Card>,
    hearts: Vec<Card>,
    diamonds: Vec<Card>,
    clubs: Vec<Card>,
}

impl From<Vec<Card>> for Hand {
    fn from(value: Vec<Card>) -> Self {
        let (mut spades, mut hearts, mut diamonds, mut clubs) = (vec![], vec![], vec![], vec![]);

        for card in value {
            match card.suit {
                crate::card::Suit::Spades => spades.push(card),
                crate::card::Suit::Hearts => hearts.push(card),
                crate::card::Suit::Diamonds => diamonds.push(card),
                crate::card::Suit::Clubs => clubs.push(card),
            }
        }

        spades.sort_by(|a, b| b.rank.cmp(&a.rank));
        hearts.sort_by(|a, b| b.rank.cmp(&a.rank));
        diamonds.sort_by(|a, b| b.rank.cmp(&a.rank));
        clubs.sort_by(|a, b| b.rank.cmp(&a.rank));

        Self {
            spades,
            hearts,
            diamonds,
            clubs,
        }
    }
}

impl Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn collect(suit: &Vec<Card>) -> String {
            if suit.is_empty() {
                return "—".to_string();
            }
            suit.iter()
                .map(|card| card.rank.to_string())
                .collect::<Vec<String>>()
                .join(" ")
        }
        f.write_str(&format!(
            "{} {}  {} {}  {} {}  {} {}",
            Suit::Spades.to_string(),
            collect(&self.spades),
            Suit::Hearts.to_string(),
            collect(&self.hearts),
            Suit::Diamonds.to_string(),
            collect(&self.diamonds),
            Suit::Clubs.to_string(),
            collect(&self.clubs),
        ))
    }
}

impl Hand {
    pub fn len(&self) -> usize {
        self.spades.len() + self.hearts.len() + self.diamonds.len() + self.clubs.len()
    }
}
