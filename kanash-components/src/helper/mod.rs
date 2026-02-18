pub mod ja;
pub mod rain;

use super::{
    Block, BorderType, Borders, Clear, Color, ColorPalette, Constraint, Flex, Frame, Layout, Line,
    Paragraph, Span, Stylize, Text, Wrap,
};

pub fn help_popup(help_strings: [&str; 5], max_height: u16, max_width: u16, frame: &mut Frame) {
    let block = Block::new()
        .title(Line::from(" Help ").fg(ColorPalette::TITLE).centered())
        .border_type(BorderType::Rounded)
        .fg(ColorPalette::KEY_HINT)
        .borders(Borders::ALL);

    let p = Paragraph::new(parse_strings_to_text(help_strings))
        .wrap(Wrap { trim: true })
        .fg(Color::White);

    let vertical = Layout::vertical([Constraint::Max(max_height)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Max(max_width)]).flex(Flex::Center);
    let [area] = vertical.areas(frame.area());
    let [area] = horizontal.areas(area);

    frame.render_widget(Clear, area);
    frame.render_widget(p.block(block), area);
}

fn parse_strings_to_text(strings: [&str; 5]) -> Text<'_> {
    strings
        .into_iter()
        .map(|s| {
            let mut sub_strings = s.split(" - ");
            let key = Span::from(sub_strings.next().unwrap_or("")).bg(ColorPalette::SELECTION);
            let function = Span::from(sub_strings.next().unwrap_or(""));
            Line::from(vec![key, " - ".into(), function]).centered()
        })
        .collect()
}
