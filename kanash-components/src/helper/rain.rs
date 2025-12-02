#[cfg(not(target_arch = "wasm32"))]
use ratatui::{style::Color, Frame};

#[cfg(not(target_arch = "wasm32"))]
use std::time::Duration;

#[cfg(target_arch = "wasm32")]
use ratzilla::ratatui::{style::Color, Frame};

#[cfg(target_arch = "wasm32")]
use web_time::Duration;

use tui_rain::{CharacterSet, Rain, RainDensity, RainSpeed};

const TAIL_COLOR: Color = Color::from_u32(0x0008dbbe);

pub fn view(frame: &mut Frame, elapsed: Duration) {
    let rain = Rain::new_rain(elapsed)
        .with_rain_density(RainDensity::Relative { sparseness: 50 })
        .with_rain_speed(RainSpeed::Absolute { speed: 14.0 })
        .with_rain_speed_variance(0.6)
        .with_color(TAIL_COLOR)
        .with_noise_interval(Duration::from_secs(10));

    let kana_tail = rain
        .clone()
        .with_character_set(CharacterSet::HalfKana)
        .with_tail_lifespan(Duration::from_millis(120))
        .with_color(Color::LightGreen);

    frame.render_widget(rain, frame.area());
    frame.render_widget(kana_tail, frame.area());
}
