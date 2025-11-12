use kanash_components::{app::App, Components};

use std::{cell::RefCell, io, rc::Rc};

use ratzilla::ratatui::{
    layout::Alignment,
    style::Color,
    widgets::{Block, Paragraph},
    Terminal,
};

use ratzilla::{event::KeyCode, DomBackend, WebRenderer};

use std::time::{Duration, Instant};

fn main() -> io::Result<()> {
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let start_time = Instant::now();

    let _ = terminal.draw_web(move |f| {
        f.render_widget(
            Paragraph::new(format!("Hey there"))
                .alignment(Alignment::Center)
                .block(
                    Block::bordered()
                        .title("Ratzilla")
                        .title_alignment(Alignment::Center)
                        .border_style(Color::Yellow),
                ),
            f.area(),
        );
    });

    Ok(())
}
