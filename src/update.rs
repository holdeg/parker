use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use parker::auction::AuctionBid;

use crate::model::Model;

pub enum Message {
    Exit,
    Typed(char),
    Clear,
    Backspace,
    Enter,
    Bid(AuctionBid),
}

pub fn handle_event(_model: &Model) -> io::Result<Option<Message>> {
    match event::read()? {
        Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
            return Ok(handle_key_event(key_event));
        }
        _ => {}
    };
    Ok(None)
}

fn handle_key_event(key_event: KeyEvent) -> Option<Message> {
    match key_event.code {
        KeyCode::Char('c') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
            Some(Message::Exit)
        }
        KeyCode::Char('u') if key_event.modifiers.contains(KeyModifiers::CONTROL) => {
            Some(Message::Clear)
        }
        KeyCode::Char(key) => Some(Message::Typed(key)),
        KeyCode::Backspace => Some(Message::Backspace),
        KeyCode::Enter => Some(Message::Enter),
        _ => None,
    }
}

pub fn update(model: &mut Model, message: Message) -> Option<Message> {
    match message {
        Message::Exit => {
            model.exit = true;
        }
        Message::Typed(character) => {
            model.typed.push(character);
        }
        Message::Clear => {
            model.typed = "".to_string();
        }
        Message::Backspace => {
            model.typed.pop();
        }
        Message::Enter => {
            model.parsed_bid = Some(model.typed.parse::<AuctionBid>());
            model.typed = "".to_string();

            if let Some(Ok(auction_bid)) = model.parsed_bid {
                return Some(Message::Bid(auction_bid));
            }
        }
        Message::Bid(auction_bid) => {
            model.auction.sequence.push(auction_bid);
        }
    }
    None
}
