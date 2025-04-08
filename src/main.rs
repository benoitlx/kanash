use std::io;

use crossterm::event::{self, Event, KeyCode};
use ratatui::backend::Backend;
use ratatui::Terminal;

mod app;
mod ui;
use crate::{app::App, ui::ui};

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == event::KeyEventKind::Release {
                continue;
            }

            match key.code {
                KeyCode::Esc => return Ok(()),
                KeyCode::Backspace => {
                    app.input.pop();
                }
                KeyCode::Char(' ') => app.reveal_roma(),
                KeyCode::Char(c) => app.check_char(c),
                _ => {}
            }
        }
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();

    let mut app = App::random(0, 0);
    let app_result = run_app(&mut terminal, &mut app);

    ratatui::restore();
    app_result
}
