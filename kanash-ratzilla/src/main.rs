use kanash_components::{app::App, Components};

use ratzilla::ratatui::Terminal;
use ratzilla::{event::KeyCode, DomBackend, WebRenderer};

use std::io;
use web_time::Instant;

fn main() -> io::Result<()> {
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let start_time = Instant::now();

    terminal.draw_web(move |f| {
        app.view(f, start_time.elapsed());

        let mut current_msg = app.handle_event();

        while current_msg.is_some() {
            current_msg = app.update(current_msg.unwrap());
        }
    });
    Ok(())
}
