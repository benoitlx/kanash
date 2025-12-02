use kanash_components::app::App;
use kanash_components::{ColorPalette, Components};

use ratatui::crossterm::event::{self, *};
use ratatui::layout::{Constraint, Layout};
use ratatui::text::Line;
use ratatui::{restore, style::Style};

use std::time::{Duration, Instant};

use tachyonfx::{fx, EffectRenderer, Interpolation};
use tui_big_text::{BigText, PixelSize};

use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    /// Path to the assets directory
    #[arg(short, long)]
    path: Option<String>,
}

fn main() {
    let arg = Args::parse();

    let mut terminal = ratatui::init();

    let mut fade_effect = fx::dissolve((20000, Interpolation::QuadOut));

    let start_time = Instant::now();

    // Splash Screen Rendering
    while start_time.elapsed() < Duration::from_millis(2000) {
        // let p = Paragraph::new(Line::from("~ Kana SH ~")).centered();
        let p = BigText::builder()
            .pixel_size(PixelSize::HalfHeight)
            .lines(vec![">> Kana SH <<".into()])
            .centered()
            .style(Style::fg(Style::new(), ColorPalette::TITLE))
            .build();

        let credit = Line::from("@benoitlx")
            .centered()
            .style(Style::fg(Style::new(), ColorPalette::SUBTITLE));

        let _ = terminal.draw(|frame| {
            let [_, area, _, bottom] = Layout::vertical([
                Constraint::Min(0),
                Constraint::Length(4),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .areas(frame.area());

            let [_, img_area, _] = Layout::horizontal([
                Constraint::Min(0),
                Constraint::Percentage(10),
                Constraint::Min(0),
            ])
            .areas(bottom);

            frame.render_widget(p, area);
            // crate::components::helper::image::view(frame, "./assets/rezo.png".to_string(), img_area);
            frame.render_widget(credit, img_area);
            if start_time.elapsed() > Duration::from_secs(1) {
                frame.render_effect(&mut fade_effect, area, tachyonfx::Duration::from_millis(33));
            }
        });
    }

    let mut app = App::new();
    if let Some(path) = arg.path {
        let assets = std::fs::read_dir(path)
            .unwrap()
            .map(|entry| entry.unwrap().path().to_str().unwrap().to_string())
            .collect::<Vec<_>>();
        app.background_paths = assets;
    }

    // Main app rendering
    while !app.exit {
        let _ = terminal.draw(|frame| app.view(frame, start_time.elapsed()));

        let mut current_msg = None;
        if event::poll(Duration::from_millis(10)).unwrap() {
            if let Event::Key(key) = event::read().unwrap() {
                current_msg = app.handle_event(&key);
            }
        }

        while current_msg.is_some() {
            current_msg = app.update(current_msg.unwrap());
        }
    }

    restore();
}
