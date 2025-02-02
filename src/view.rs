use std::rc::Rc;

use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::Frame;

use ratatui::{
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Padding, Paragraph},
};

use crate::model::Model;

pub fn view(model: &Model, frame: &mut Frame) {
    let area = frame.area();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(Constraint::from_percentages([75, 25]))
        .split(area);

    let display_area = chunks[0];

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
    let instructions = Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]);
    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .padding(Padding::top(area.height / 2))
        .border_set(border::THICK);

    let auction_display = model.auction.to_string();
    let lines = auction_display.lines();
    let paragraph =
        Paragraph::new(lines.map(|line| Line::from(line)).collect::<Vec<Line>>()).centered();

    frame.render_widget(block, display_area);
    frame.render_widget(paragraph, rects[1][1]);
}
