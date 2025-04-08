use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};

use wana_kana::ConvertJapanese;

use rand::Rng;

#[derive(Debug)]
pub struct App {
    shown: u16,
    correct: u16,
    exit: bool,
    input: String,
    current_kana: String,
    index: usize,
}

fn get_hiragana() -> String {
    let mut rng = rand::rng();

    String::from_utf16(&[rng.random_range(12353..12438)]).expect("error")
}

fn get_kana() -> String {
    get_hiragana()
}

impl App {
    pub fn new(kana: String) -> App {
        App {
            shown: 0,
            correct: 0,
            exit: false,
            input: String::new(),
            current_kana: kana, 
            index: 0,
        }
    }

    pub fn random(s: u16, c: u16) -> App {
        App {
            shown: s,
            correct: c,
            exit: false,
            input: String::new(),
            current_kana: get_kana(),
            index: 0,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            // it's important to check that the event is a key press event as
            // crossterm also emits key release and repeat events on Windows.
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Esc => self.exit = true,
            KeyCode::Backspace => {self.input.pop();}
            KeyCode::Char(chr_to_push) => self.check_char(chr_to_push), 
            _ => {}
        }
    }

    fn check_char(&mut self, c: char) {
        self.input.push(c);

        if self.input == self.current_kana.to_romaji() {
            self.correct += 1;
            self.shown += 1;
            self.index = 0;
            self.input = String::new();
            self.current_kana = get_kana();
        }
    }

}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Kana TUI ".bold());
        let left_title = Line::from(vec![
            " Shown: ".into(),
            self.shown.to_string().yellow(),
            " ".into(),
        ]).left_aligned();
        let right_title = Line::from(vec![
            " Correct: ".into(),
            self.correct.to_string().yellow(),
            " ".into(),
        ]).right_aligned();

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
        let romaji = Line::from(self.current_kana.to_romaji()).centered().red();
        let user_input = Line::from(self.input.clone());


        Paragraph::new(vec![kana, romaji, user_input])
            .centered()
            .block(block)
            .render(area, buf);
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::random(0, 0).run(&mut terminal);

    ratatui::restore();
    app_result
}
