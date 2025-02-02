use ratatui::Frame;

use ratatui::{
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Padding, Paragraph},
};

use crate::model::Model;

pub fn view(model: &Model, frame: &mut Frame) {
    let area = frame.area();

    let title = Line::from(" Auction ".bold());
    let instructions = Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]);
    let block = Block::bordered()
        .title(title.centered())
        .title_bottom(instructions.centered())
        .padding(Padding::top(area.height / 2))
        .border_set(border::THICK);

    let dealer = model.auction.dealer();
    let counter_text = Text::from(vec![
        Line::from(vec![
            format!("{:?}: ", dealer).into(),
            model.hands[*model.auction.dealer() as usize]
                .to_string()
                .into(),
        ]),
        Line::from(vec![
            "N: ".into(),
            model.hands[0].to_string().into(),
            "E: ".into(),
            model.hands[1].to_string().into(),
            "S: ".into(),
            model.hands[2].to_string().into(),
            "W: ".into(),
            model.hands[3].to_string().into(),
        ]),
    ]);

    let paragraph = Paragraph::new(counter_text).centered().block(block);

    frame.render_widget(paragraph, area);
}
