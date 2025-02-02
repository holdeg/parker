use std::rc::Rc;

use parker::auction::{AuctionBid, ContractBid};
use ratatui::layout::{Constraint, Direction, Layout, Position, Rect};
use ratatui::widgets::Wrap;
use ratatui::Frame;

use ratatui::{
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph},
};

use crate::model::Model;

pub fn view(model: &Model, frame: &mut Frame) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(Constraint::from_percentages([75, 25]))
        .split(area);

    view_auction_area(model, frame, chunks[0]);
    view_typing_area(model, frame, chunks[1]);
}

fn view_auction_area(model: &Model, frame: &mut Frame, display_area: Rect) {
    // Separate the main display area into nine boxes. The central one is slightly larger
    // and will display the auction; the boxes adjacent to the auction display box will
    // display the four hands.
    let columns = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(Constraint::from_percentages([30, 40, 30]))
        .split(display_area);
    let rects: Vec<Rc<[Rect]>> = columns
        .iter()
        .map(|column| {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints(Constraint::from_percentages([30, 40, 30]))
                .split(*column)
        })
        .collect();

    let title = Line::from(" Auction ".bold());
    let instructions = Line::from(vec![" Quit ".into(), "<Ctrl-c> ".blue().bold()]);
    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .border_set(border::THICK);

    let auction_display = model.auction.to_string();
    let auction = Paragraph::new(
        auction_display
            .lines()
            .map(|line| Line::from(line))
            .collect::<Vec<Line>>(),
    )
    .centered();

    frame.render_widget(block, display_area);
    frame.render_widget(auction, rects[1][1]);
}

fn view_typing_area(model: &Model, frame: &mut Frame, display_area: Rect) {
    let rows = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Fill(1), Constraint::Max(3)])
        .split(display_area);

    let result_display = Paragraph::new(Line::from(match model.parsed_bid {
        Some(Ok(bid)) => vec![
            "You bid ".into(),
            match bid {
                AuctionBid::Bid(ContractBid { level, suit }) => format!("{} {:?}", level, suit),
                AuctionBid::Pass => "Pass".to_string(),
                AuctionBid::Double => "Double".to_string(),
                AuctionBid::Redouble => "Redouble".to_string(),
            }
            .bold(),
            format!(" ({})", bid).into(),
        ],
        Some(Err(err)) => vec!["Parsing error: ".into(), format!("{:?}", err).bold()],
        None => vec!["".into()],
    }))
    .wrap(Wrap { trim: true })
    .block(Block::bordered().title("Output"));
    frame.render_widget(result_display, rows[0]);

    let parsing_window = Paragraph::new(Line::from(format!("> {}", model.typed)))
        .block(Block::bordered().title("Prompt"));

    frame.render_widget(parsing_window, rows[1]);
    frame.set_cursor_position(Position::new(
        rows[1].x + 3 + model.typed.len() as u16,
        rows[1].y + 1,
    ));
}
