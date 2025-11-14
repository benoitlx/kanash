// use crate::components::helper::image;
use crate::helper::ja::random_kana;
use rand::SeedableRng;
use rand_pcg::{Mcg128Xsl64, Pcg64Mcg};
use wana_kana::ConvertJapanese;

use super::{
    helper::ja::{random_hiragana, random_katakana},
    *,
};

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
    rng: Mcg128Xsl64,
    pub mode: Mode,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum KanaMessage {
    /// reveal the answer
    Answer,

    /// When the user is typing
    TypingRoma(char),

    /// Delete roma
    DeleteRoma,

    /// Pass,
    Pass,
}

impl Components for KanaModel {
    /// Create a new kana model
    fn new() -> Self {
        let mut r = Pcg64Mcg::seed_from_u64(
            #[cfg(not(target_arch = "wasm32"))]
            std::time::SystemTime::now()
                .duration_since(std::time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            #[cfg(target_arch = "wasm32")]
            web_time::SystemTime::now()
                .duration_since(web_time::SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        );
        Self {
            shown: 0,
            correct: 0,
            input: String::new(),
            current_kana: random_kana(&mut r),
            display_answer: false,
            rng: r,
            mode: Mode::Hira,
        }
    }

    /// Handle Event (Mostly convert key event to message)
    #[cfg(not(target_arch = "wasm32"))]
    fn handle_event(&self) -> Option<Message> {
        if event::poll(Duration::from_millis(10)).unwrap() {
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
        } else {
            None
        }

        #[cfg(target_arch = "wasm32")]
        None
    }

    #[cfg(target_arch = "wasm32")]
    fn handle_event(&self, key_event: &ratzilla::event::KeyEvent) -> Option<Message> {
        use ratzilla::event::KeyCode;

        match key_event.code {
            KeyCode::Esc => Some(Message::Back),
            KeyCode::Backspace => Some(Message::Kana(KanaMessage::DeleteRoma)),
            KeyCode::Char(' ') => Some(Message::Kana(KanaMessage::Answer)),
            KeyCode::Char(c) => Some(Message::Kana(KanaMessage::TypingRoma(c))),
            _ => None,
        }
    }

    fn update(&mut self, msg: Message) -> Option<Message> {
        if let Message::Kana(kana_msg) = msg {
            return match kana_msg {
                KanaMessage::TypingRoma(c) => {
                    self.input.push(c);

                    if self.input == self.current_kana.to_romaji() {
                        if self.display_answer {
                            self.display_answer = false;
                        } else {
                            self.correct += 1;
                        }
                        self.shown += 1;
                        return Some(Message::Kana(KanaMessage::Pass));
                    }
                    None
                }
                KanaMessage::Pass => {
                    self.input = String::new();
                    match self.mode {
                        Mode::Hira => self.current_kana = random_hiragana(&mut self.rng),
                        Mode::Kata => self.current_kana = random_katakana(&mut self.rng),
                        Mode::Both => self.current_kana = random_kana(&mut self.rng),
                    }

                    None
                }
                KanaMessage::Answer => {
                    self.display_answer = true;
                    self.input = String::new();
                    None
                }
                KanaMessage::DeleteRoma => {
                    self.input.pop();
                    None
                }
            };
        }
        None
    }

    fn view(&mut self, frame: &mut Frame, _elapsed: Duration) {
        self.learning_zone(frame);
    }
}

impl KanaModel {
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
            LEFT_TITLE.fg(ColorPalette::SUBTITLE).into(),
            self.shown.to_string().yellow(),
            " ".into(),
        ])
        .left_aligned();

        let right_title = Line::from(vec![
            RIGHT_TITLE.fg(ColorPalette::SUBTITLE).into(),
            self.correct.to_string().yellow(),
            " ".into(),
        ])
        .right_aligned();

        let block = Block::new()
            .title(Line::from(TITLE).fg(ColorPalette::TITLE).centered())
            .title(left_title)
            .title(right_title)
            .title_bottom(
                Line::from(KEY_HELPER)
                    .fg(ColorPalette::KEY_HINT)
                    .bold()
                    .centered(),
            )
            .border_type(BorderType::Rounded)
            .padding(Padding::vertical(1))
            .borders(Borders::ALL);

        let text = vec![
            Line::from(self.current_kana.clone()),
            if self.display_answer {
                Line::from(self.current_kana.to_romaji()).fg(ColorPalette::ERROR)
            } else {
                Line::default()
            },
            Line::from(self.input.clone()),
        ];

        let p = Paragraph::new(text).block(block).centered();

        frame.render_widget(Clear, main_area);
        frame.render_widget(p, main_area);
    }
}
