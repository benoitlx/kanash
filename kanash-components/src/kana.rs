use wana_kana::ConvertJapanese;

use super::*;

const TITLE: &str = " Hiragana ";
const STATS_TITLE: &str = " Stats ";
const LEFT_TITLE: &str = " Shown: ";
const RIGHT_TITLE: &str = " Correct: ";
const KEY_HELPER: &str = " ? ";
const HELP_STRINGS: [&str; 5] = [
    "Type the corresponding romaji. Good answers are logged automaticaly.",
    "",
    "? - toggle this popup",
    "esc - go back to main menu",
    "space - reveal answer",
];

#[derive(Debug, PartialEq, Eq)]
pub struct KanaModel {
    shown: u32,
    correct: u32,
    input: String,
    current_kana: (String, usize),
    display_answer: bool,
    show_help_popup: bool,
    pub mode: Mode,
    pub scroll_state: ScrollbarState,
    good_cnts: [usize; HIRAGANA_NUMBER + KATAKANA_NUMBER],
    bad_cnts: [usize; HIRAGANA_NUMBER + KATAKANA_NUMBER],
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum KanaMessage {
    /// reveal the answer
    Answer,

    /// When the user is typing
    TypingRoma(char),

    /// Delete roma
    DeleteRoma,

    /// Pass
    Pass,

    /// Help
    Help,

    ScrollUp,
    ScrollDown,
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
            show_help_popup: false,
            mode: Mode::Hira,
            scroll_state: ScrollbarState::new(HIRAGANA_NUMBER),
            good_cnts: [0; HIRAGANA_NUMBER + KATAKANA_NUMBER],
            bad_cnts: [0; HIRAGANA_NUMBER + KATAKANA_NUMBER],
        }
    }

    /// Handle Event (Mostly convert key event to message)
    fn handle_event(&self, key_event: &PlatformKeyEvent) -> Option<Message> {
        match key_event.code {
            KeyCode::Esc => Some(Message::Back),
            KeyCode::Backspace => Some(Message::Kana(KanaMessage::DeleteRoma)),
            KeyCode::Char('?') => Some(Message::Kana(KanaMessage::Help)),
            KeyCode::Char(' ') => Some(Message::Kana(KanaMessage::Answer)),
            KeyCode::Char(c) => Some(Message::Kana(KanaMessage::TypingRoma(c))),
            KeyCode::Up => Some(Message::Kana(KanaMessage::ScrollUp)),
            KeyCode::Down => Some(Message::Kana(KanaMessage::ScrollDown)),
            _ => None,
        }
    }

    fn update(&mut self, msg: Message) -> Option<Message> {
        if let Message::Kana(kana_msg) = msg {
            match kana_msg {
                KanaMessage::TypingRoma(c) => {
                    self.input.push(c);

                    if self.input == self.current_kana.0.to_romaji() {
                        if self.display_answer {
                            self.display_answer = false;
                            self.bad_cnts[self.current_kana.1] += 1;
                        } else {
                            self.good_cnts[self.current_kana.1] += 1;
                            self.correct += 1;
                        }
                        self.shown += 1;
                        return Some(Message::Kana(KanaMessage::Pass));
                    }
                }
                KanaMessage::Pass => {
                    self.input = String::new();
                    match self.mode {
                        Mode::Hira => self.current_kana = random_hiragana(),
                        Mode::Kata => self.current_kana = random_katakana(),
                        Mode::Both => self.current_kana = random_kana(),
                    }
                }
                KanaMessage::Answer => {
                    self.display_answer = true;
                    self.input = String::new();
                }
                KanaMessage::DeleteRoma => {
                    self.input.pop();
                }
                KanaMessage::Help => {
                    self.show_help_popup = !self.show_help_popup;
                }
                KanaMessage::ScrollUp => {
                    self.scroll_state.prev();
                }
                KanaMessage::ScrollDown => {
                    self.scroll_state.next();
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
        let [v_area] = Layout::vertical([Constraint::Length(7)])
            .flex(Flex::Center)
            .areas(frame.area());

        let [main_area, stats_area] =
            Layout::horizontal([Constraint::Length(43), Constraint::Length(10)])
                .flex(Flex::Center)
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

        let stats_block = Block::new()
            .title(Line::from(STATS_TITLE).fg(ColorPalette::TITLE).centered())
            .border_type(BorderType::Rounded)
            .borders(Borders::ALL);

        let text = vec![
            Line::from(self.current_kana.0.clone()),
            if self.display_answer {
                Line::from(self.current_kana.0.to_romaji()).fg(ColorPalette::ERROR)
            } else {
                Line::default()
            },
            Line::from(self.input.clone()),
        ];

        let p = Paragraph::new(text).block(block).centered();

        let stats_closure = |(i, u): (usize, &u16)| {
            format!(
                "{} {}/{}",
                String::from_utf16(&[*u]).expect("error"),
                self.good_cnts[i],
                self.bad_cnts[i]
            )
        };

        let stat_text = match self.mode {
            Mode::Hira => Text::from_iter(WANTED_HIRAGANA.iter().enumerate().map(stats_closure)),
            Mode::Kata => Text::from_iter(
                WANTED_KATAKANA
                    .iter()
                    .enumerate()
                    .map(|(i, u)| stats_closure((i + HIRAGANA_NUMBER, u))),
            ),
            Mode::Both => Text::from_iter(
                WANTED_HIRAGANA
                    .iter()
                    .chain(WANTED_KATAKANA.iter())
                    .enumerate()
                    .map(stats_closure),
            ),
        };

        let stats = Paragraph::new(stat_text)
            .block(stats_block)
            .scroll((self.scroll_state.get_position() as u16, 0));

        frame.render_widget(Clear, main_area);
        frame.render_widget(p, main_area);

        frame.render_widget(Clear, stats_area);
        frame.render_widget(stats, stats_area);
        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("╮"))
                .end_symbol(Some("╯")),
            stats_area,
            &mut self.scroll_state,
        );

        if self.show_help_popup {
            help_popup(HELP_STRINGS, 10, 30, frame);
        }
    }
}
