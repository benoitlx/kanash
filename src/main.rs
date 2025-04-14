mod components;
use components::app::App;
use components::Components;

use ratatui::layout::{Constraint, Layout};
use ratatui::{restore, style::Style};

use std::time::{Duration, Instant};

use ratatui::style::Color;
use tachyonfx::{fx, EffectRenderer, Interpolation};
use tui_big_text::{BigText, PixelSize};

/*
Color Palette
#ff33ff
#ff3399
#ff3333
#ff9933
#ffff33
#99ff33
 */

fn main() {
    let mut terminal = ratatui::init();

    let mut app = App::new();

    let start_time = Instant::now();

    let mut fade_effect = fx::dissolve((20000, Interpolation::QuadOut));

    // Splash Screen Rendering
    while start_time.elapsed() < Duration::from_millis(2000) {
        // let p = Paragraph::new(Line::from("~ Kana SH ~")).centered();
        let p = BigText::builder()
            .pixel_size(PixelSize::HalfHeight)
            .lines(vec![">> Kana SH <<".into()])
            .centered()
            .style(Style::fg(Style::new(), Color::from_u32(0x00ff33ff)))
            .build();

        let _ = terminal.draw(|frame| {
            let [_, area, _] = Layout::vertical([
                Constraint::Min(0),
                Constraint::Length(4),
                Constraint::Min(0),
            ])
            .areas(frame.area());

            frame.render_widget(p, area);
            if start_time.elapsed() > Duration::from_secs(1) {
                frame.render_effect(&mut fade_effect, area, tachyonfx::Duration::from_millis(33));
            }
        });
    }

    // Main app rendering
    while app != App::Exit {
        let _ = terminal.draw(|frame| app.view(frame, start_time.elapsed()));

        let mut current_msg = app.handle_event();

        while current_msg.is_some() {
            current_msg = app.update(current_msg.unwrap());
        }
    }

    restore();
}
