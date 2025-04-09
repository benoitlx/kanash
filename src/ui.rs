use crate::app::App;
use crate::app::get_hiragana;
use wana_kana::ConvertJapanese;

use ratatui::{
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Wrap},
    Frame,
};

pub fn ui(frame: &mut Frame, app: &App) {
    let title = Line::from(" Kana TUI ".bold());
    let left_title = Line::from(vec![
        " Shown: ".into(),
        app.shown.to_string().yellow(),
        " ".into(),
    ])
    .left_aligned();
    let right_title = Line::from(vec![
        " Correct: ".into(),
        app.correct.to_string().yellow(),
        " ".into(),
    ])
    .right_aligned();

    let commands = Line::from(vec![
        " Quit ".into(),
        "<Esc> ".blue().bold(),
        "| Show answer ".into(),
        "<Space> ".blue().bold(),
    ]);

    let block = Block::bordered()
        .title(title.centered())
        .title(left_title)
        .title(right_title)
        .title_bottom(commands.centered())
        .border_set(border::THICK);

    let kana = Line::from(app.current_kana.clone()).centered();

    let romaji = if app.display_answer {
        Line::from(app.current_kana.to_romaji()).centered().red()
    } else {
        Line::from("\n")
    };
    
    let user_input = Line::from(app.input.clone());
    let kana_list = Line::from(get_hiragana());

    let p = Paragraph::new(vec![kana, romaji, user_input, kana_list])
        .centered()
        .block(block)
        .wrap(Wrap { trim: true });

    frame.render_widget(p, frame.area());
}
