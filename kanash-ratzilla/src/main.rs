use kanash_components::{app::App, ColorPalette, Components};

use ratzilla::backend::webgl2::WebGl2BackendOptions;
use ratzilla::ratatui::{
    layout::{Constraint, Layout},
    style::Style,
    text::Line,
    Terminal,
};
use ratzilla::{CanvasBackend, DomBackend, WebGl2Backend, WebRenderer};

use tachyonfx::{fx, EffectRenderer};
use tui_big_text::{BigText, PixelSize};

use std::io;
use web_time::Instant;

use std::{cell::RefCell, rc::Rc};

type Outbox<T> = Rc<RefCell<Option<T>>>;

fn new_outbox<T>() -> Outbox<T> {
    Rc::new(RefCell::new(None))
}

fn main() -> io::Result<()> {
    let webgl2_options = WebGl2BackendOptions::new().fallback_glyph("@");
    let backend = WebGl2Backend::new_with_options(webgl2_options)?;
    let terminal = Terminal::new(backend)?;

    let mut render_app = App::new();
    let mut event_app = App::new();
    let start_time = Instant::now();

    let mut fade_effect = fx::dissolve(2000); //(20000, Interpolation::QuadOut));

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
        let p = BigText::builder()
            .pixel_size(PixelSize::HalfHeight)
            .lines(vec![">> Kana SH <<".into()])
            .centered()
            .style(Style::fg(Style::new(), ColorPalette::TITLE))
            .build();

        let credit = Line::from("@benoitlx")
            .centered()
            .style(Style::fg(Style::new(), ColorPalette::SUBTITLE));

        let [_, area, _, bottom] = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(4),
            Constraint::Min(0),
            Constraint::Length(1),
        ])
        .areas(f.area());

        let [_, creds_area, _] = Layout::horizontal([
            Constraint::Min(0),
            Constraint::Percentage(10),
            Constraint::Min(0),
        ])
        .areas(bottom);

        if start_time.elapsed() < web_time::Duration::from_millis(1000) {
            f.render_widget(p, area);
            f.render_widget(credit, creds_area);
        } else if start_time.elapsed() < web_time::Duration::from_millis(1800) {
            f.render_widget(p, area);
            f.render_widget(credit, creds_area);
            f.render_effect(&mut fade_effect, area, tachyonfx::Duration::from_millis(33));
        } else {
            render_app.view(f, start_time.elapsed());

            let mut current_msg = outbox.borrow_mut().take();
            while current_msg.is_some() {
                current_msg = render_app.update(current_msg.unwrap());
            }
        }
    });

    Ok(())
}
