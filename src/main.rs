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

#[derive(Debug, Default)]
pub struct App {
    shown: u16,
    correct: u16,
    exit: bool,
    input: String,
}

impl App {
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
            KeyCode::Left => self.shown += 1, 
            KeyCode::Right => self.shown -= 1,
            KeyCode::Backspace => {self.input.pop();}
            KeyCode::Char(chr_to_push) => self.input.push(chr_to_push),
            _ => {}
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

        let kana = Line::from("そこで犬が寝ている").centered();
        let romaji = Line::from("romaji").centered().red();
        let user_input = Line::from(self.input.clone());


        Paragraph::new(vec![kana, romaji, user_input])
            .centered()
            .block(block)
            .render(area, buf);
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);

    ratatui::restore();
    app_result
}
