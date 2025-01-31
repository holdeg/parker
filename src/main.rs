use parker::{
    card::{Card, Rank, Suit},
    deck::Deck,
};

fn main() {
    let ace_of_hearts = dbg!(Card {
        suit: Suit::Hearts,
        rank: Rank::Ace,
    });
    println!("{}\n", ace_of_hearts);

    // let cards: Vec<Card> = vec!["AH", "2S", "4C", "8C", "TH", "5S"]
    //     .iter()
    //     .filter_map(|card_string| card_string.parse().ok())
    //     .collect();

    let mut deck = Deck::default();
    deck.shuffle();

    let hands = deck.deal();
    let north = &hands[0];
    println!(
        "Hand: {}\nLength: {}\nDistribution: {:?}\nHCP: {}",
        north,
        north.len(),
        north.distribution(),
        north.hcp()
    );
}
