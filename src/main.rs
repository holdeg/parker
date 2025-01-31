use std::io::{self, Write};

use parker::{
    auction::AuctionBid,
    card::{Card, Rank, Suit},
    deck::Deck,
    error::ParseError,
};

fn main() {
    let ace_of_hearts = dbg!(Card {
        suit: Suit::Hearts,
        rank: Rank::Ace,
    });
    println!("{}\n", ace_of_hearts);

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

    loop {
        print!("\n\n> ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        if buffer.is_empty() {
            break;
        }

        buffer = buffer.trim().to_string();
        let parsed_bid: Result<AuctionBid, ParseError> = buffer.parse();
        let output = match parsed_bid {
            Ok(bid) => format!("{} ({:?})", bid, parsed_bid),
            Err(_) => format!("{:?}", parsed_bid),
        };
        println!("'{}' gives {}", buffer, output)
    }

    println!("All done.")
}
