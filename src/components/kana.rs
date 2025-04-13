use crate::components::helper::ja::{get_hiragana, random_kana};
use wana_kana::ConvertJapanese;

use super::*;

#[derive(Debug, PartialEq, Eq)]
pub struct KanaModel {
    shown: u32,
    correct: u32,
    input: String,
    current_kana: String,
    display_answer: bool,
    pub exit: bool,
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
            exit: false,
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
        } else {
            if msg == Message::Back {
                self.exit = true;
            }
        }
        None
    }

    fn view(&mut self, frame: &mut Frame) {
        let title = Line::from(" Kana TUI ".bold());
        let left_title = Line::from(vec![
            " Shown: ".into(),
            self.shown.to_string().yellow(),
            " ".into(),
        ])
        .left_aligned();
        let right_title = Line::from(vec![
            " Correct: ".into(),
            self.correct.to_string().yellow(),
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

        let kana = Line::from(self.current_kana.clone()).centered();

        let romaji = if self.display_answer {
            Line::from(self.current_kana.to_romaji()).centered().red()
        } else {
            Line::from("\n")
        };

        let user_input = Line::from(self.input.clone());
        let kana_list = Line::from(get_hiragana());

        let p = Paragraph::new(vec![kana, romaji, user_input, kana_list])
            .centered()
            .block(block)
            .wrap(Wrap { trim: true });

        frame.render_widget(p, frame.area());
    }
}
