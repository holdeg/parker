use bridge::{
    card::{Card, Rank, Suit},
    hand::Hand,
};

fn main() {
    let ace_of_hearts = dbg!(Card {
        suit: Suit::Hearts,
        rank: Rank::Ace,
    });
    println!("{}\n", ace_of_hearts);

    let cards: Vec<Card> = vec!["AH", "2S", "4C", "8C", "TH", "5S"]
        .iter()
        .filter_map(|card_string| card_string.parse().ok())
        .collect();

    let hand: Hand = dbg!(cards.into());
    println!("Hand: {}\nLength: {}", hand, hand.len());
}
