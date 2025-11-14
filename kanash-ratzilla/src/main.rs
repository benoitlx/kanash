use kanash_components::{app::App, Components};

use ratzilla::ratatui::Terminal;
use ratzilla::{DomBackend, WebGl2Backend, WebRenderer};

use std::io;
use web_time::Instant;

use std::{cell::RefCell, rc::Rc};

type Outbox<T> = Rc<RefCell<Option<T>>>;

fn new_outbox<T>() -> Outbox<T> {
    Rc::new(RefCell::new(None))
}

fn main() -> io::Result<()> {
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;

    let mut render_app = App::new();
    let mut event_app = App::new();
    let start_time = Instant::now();

    let outbox: Outbox<kanash_components::Message> = new_outbox();
    let outbox_event = Rc::clone(&outbox);

    terminal.on_key_event({
        move |key_event| {
            let current_msg = event_app.handle_event(&key_event);

            if let Some(msg) = current_msg {
                *outbox_event.borrow_mut() = Some(msg);
                event_app.update(current_msg.unwrap());
            }
        }
    });

    terminal.draw_web(move |f| {
        render_app.view(f, start_time.elapsed());

        let mut current_msg = outbox.borrow_mut().take();
        while current_msg.is_some() {
            current_msg = render_app.update(current_msg.unwrap());
        }
    });

    Ok(())
}
