use std::io;

use model::Model;
use update::handle_event;

pub mod model;
pub mod update;
pub mod view;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let mut model = Model::new();

    while !model.exit {
        terminal.draw(|frame| view::view(&model, frame))?;

        let mut message = handle_event(&model)?;
        while let Some(inner) = message {
            message = update::update(&mut model, inner)
        }
    }

    ratatui::restore();
    Ok(())
}
