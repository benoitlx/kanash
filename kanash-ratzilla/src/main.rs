use kanash_components::{app::App, Components};

use ratzilla::ratatui::Terminal;
use ratzilla::{DomBackend, WebGl2Backend, WebRenderer};

use std::io;
use web_time::Instant;

use web_sys::console;

use std::{cell::RefCell, rc::Rc};

type Outbox<T> = Rc<RefCell<Option<T>>>;

fn new_outbox<T>() -> Outbox<T> {
    Rc::new(RefCell::new(None))
}

fn main() -> io::Result<()> {
    let backend = DomBackend::new()?;
    let terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let new_app = App::new();
    let start_time = Instant::now();

    let outbox: Outbox<kanash_components::Message> = new_outbox();
    let outbox_cb = Rc::clone(&outbox);

    // TODO) synchronise two apps state

    terminal.on_key_event({
        move |key_event| {
            let current_msg = new_app.handle_event(&key_event);
            console::log_1(&format!("{:?}", current_msg).into());

            if let Some(msg) = current_msg {
                *outbox_cb.borrow_mut() = Some(msg);
            }
        }
    });

    terminal.draw_web(move |f| {
        app.view(f, start_time.elapsed());

        let mut current_msg = outbox.borrow_mut().take();
        while current_msg.is_some() {
            current_msg = app.update(current_msg.unwrap());
        }
    });

    Ok(())
}
