use crate::components::helper::ja::{get_hiragana, random_kana};
use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Wrap},
    Frame,
    style::Stylize,
};
use wana_kana::ConvertJapanese;

pub struct KanaModel {
    shown: u32,
    correct: u32,
    input: String,
    current_kana: String,
    display_answer: bool,
    pub exit: bool,
}

pub enum KanaMessage {
    /// reveal the answer
    Answer,

    /// go back to the main menu
    Back,

    /// When the user is typing
    TypingRoma(char),

    /// Delete roma
    DeleteRoma,
}

impl KanaModel {
    /// Create a new kana model
    pub fn new() -> KanaModel {
        KanaModel {
            shown: 0,
            correct: 0,
            input: String::new(),
            current_kana: random_kana(),
            display_answer: false,
            exit: false,
        }
    }

    /// Handle Event (Mostly convert key event to message)
    pub fn handle_event(&self) -> Option<KanaMessage> {
        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Esc => Some(KanaMessage::Back),
                KeyCode::Backspace => Some(KanaMessage::DeleteRoma),
                KeyCode::Char(' ') => Some(KanaMessage::Answer),
                KeyCode::Char(c) => Some(KanaMessage::TypingRoma(c)),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn update(&mut self, msg: KanaMessage) -> Option<KanaMessage> {
        match msg {
            KanaMessage::Back => {
                self.exit = true;
            }
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
        None
    }

    pub fn view(&self, frame: &mut Frame) {
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
