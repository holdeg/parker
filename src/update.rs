use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use crate::model::Model;

pub enum Message {
    Exit,
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
        KeyCode::Char('q') => Some(Message::Exit),
        _ => None,
    }
}

pub fn update(model: &mut Model, message: Message) -> Option<Message> {
    match message {
        Message::Exit => {
            model.exit = true;
        }
    }
    None
}
