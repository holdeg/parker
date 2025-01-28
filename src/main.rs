use bridge::card::{Card, Rank, Suit};

fn main() {
    let ace_of_hearts = dbg!(Card {
        suit: Suit::Hearts,
        rank: Rank::Ace,
    });
    println!("{}", ace_of_hearts);
}
