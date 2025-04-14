use std::time::Duration;
use ratatui::Frame;
use tui_rain::{CharacterSet, Rain, RainDensity, RainSpeed};

pub fn view(frame: &mut Frame, elapsed: Duration) {
    let rain = Rain::new_rain(elapsed)
        .with_rain_density(RainDensity::Relative {
            sparseness: 1,
        })
        .with_rain_speed(RainSpeed::Absolute {
            speed: 10.0,
        })
        .with_rain_speed_variance(0.6)
        .with_color(ratatui::style::Color::LightGreen)
        .with_noise_interval(Duration::from_secs(10));

    let kana_tail = rain.clone()
        .with_character_set(CharacterSet::HalfKana)
        .with_tail_lifespan(Duration::from_millis(120))
        .with_color(ratatui::style::Color::LightGreen);

    frame.render_widget(rain, frame.area());
    frame.render_widget(kana_tail, frame.area());
}