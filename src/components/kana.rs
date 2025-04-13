use crate::components::helper::ja::{get_hiragana, random_kana};
use wana_kana::ConvertJapanese;

use super::*;

const TITLE: &str = " Hiragana ";
const LEFT_TITLE: &str = " Shown: ";
const RIGHT_TITLE: &str = " Correct: ";
const KEY_HELPER: &str = " Main Menu <Esc> | Show answer <Space> ";

#[derive(Debug, PartialEq, Eq)]
pub struct KanaModel {
    shown: u32,
    correct: u32,
    input: String,
    current_kana: String,
    display_answer: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum KanaMessage {
    /// reveal the answer
    Answer,

    /// When the user is typing
    TypingRoma(char),

    /// Delete roma
    DeleteRoma,
}

impl Components for KanaModel {
    /// Create a new kana model
    fn new() -> Self {
        Self {
            shown: 0,
            correct: 0,
            input: String::new(),
            current_kana: random_kana(),
            display_answer: false,
        }
    }

    /// Handle Event (Mostly convert key event to message)
    fn handle_event(&self) -> Option<Message> {
        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Esc => Some(Message::Back),
                KeyCode::Backspace => Some(Message::Kana(KanaMessage::DeleteRoma)),
                KeyCode::Char(' ') => Some(Message::Kana(KanaMessage::Answer)),
                KeyCode::Char(c) => Some(Message::Kana(KanaMessage::TypingRoma(c))),
                _ => None,
            }
        } else {
            None
        }
    }

    fn update(&mut self, msg: Message) -> Option<Message> {
        if let Message::Kana(kana_msg) = msg {
            match kana_msg {
                KanaMessage::TypingRoma(c) => {
                    self.input.push(c);

                    if self.input == self.current_kana.to_romaji() {
                        if self.display_answer {
                            self.display_answer = false;
                        } else {
                            self.correct += 1;
                        }
                        self.shown += 1;
                        self.input = String::new();
                        self.current_kana = random_kana();
                    }
                }
                KanaMessage::Answer => {
                    self.display_answer = true;
                    self.input = String::new();
                }
                KanaMessage::DeleteRoma => {
                    self.input.pop();
                }
            }
        }
        None
    }

    fn view(&mut self, frame: &mut Frame) {
        self.learning_zone(frame);
    }
}

impl KanaModel {
    fn background(&mut self, frame: &mut Frame) {
        todo!()
    }

    fn learning_zone(&mut self, frame: &mut Frame) {
        let [_, v_area, _] = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(7),
            Constraint::Min(0),
        ])
        .areas(frame.area());

        let [_, main_area, _] = Layout::horizontal([
            Constraint::Min(0),
            Constraint::Length(43),
            Constraint::Min(0),
        ])
        .areas(v_area);

        let left_title = Line::from(vec![
            LEFT_TITLE.into(),
            self.shown.to_string().yellow(),
            " ".into(),
        ])
        .left_aligned();

        let right_title = Line::from(vec![
            RIGHT_TITLE.into(),
            self.correct.to_string().yellow(),
            " ".into(),
        ])
        .right_aligned();

        let block = Block::new()
            .title(Line::from(TITLE).centered())
            .title(left_title)
            .title(right_title)
            .title_bottom(Line::from(KEY_HELPER).blue().bold().centered())
            .border_type(BorderType::Rounded)
            .padding(Padding::vertical(1))
            .borders(Borders::ALL);

        let text = vec![
            Line::from(self.current_kana.clone()),
            if self.display_answer {
                Line::from(self.current_kana.to_romaji()).red()
            } else {
                Line::default()
            },
            Line::from(self.input.clone()),
        ];

        let p = Paragraph::new(text).block(block).centered();

        frame.render_widget(p, main_area);
    }
}
