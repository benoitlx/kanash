use super::{home::HomeModel, kana::KanaModel, *};
use crate::helper::rain;

#[cfg(not(target_arch = "wasm32"))]
use ansi_to_tui::IntoText;
#[cfg(not(target_arch = "wasm32"))]
use rascii_art::{render_to, RenderOptions};

#[cfg(not(target_arch = "wasm32"))]
use ratatui::text::Text;
#[cfg(target_arch = "wasm32")]
use ratzilla::ratatui::text::Text;

#[derive(Debug, PartialEq, Eq)]
enum AppPage {
    Home(HomeModel),
    Kana(KanaModel),
}

#[derive(Debug, PartialEq, Eq)]
pub struct App {
    pub exit: bool,
    background_widget: Box<Text<'static>>,
    page: AppPage,
    previous_height: u16,
    disable_rain: bool,
    pub disable_background: bool,
    background_number: usize,
    pub background_paths: Vec<String>,
}

impl Components for App {
    fn new() -> Self {
        let home = HomeModel::new();

        Self {
            exit: false,
            page: AppPage::Home(home),
            background_widget: Box::new(Text::default()),
            previous_height: 0,
            disable_rain: false,
            disable_background: false,
            background_paths: vec![],
            background_number: 0,
        }
    }

    fn handle_event(&self, event: &PlatformKeyEvent) -> Option<Message> {
        match &self.page {
            AppPage::Home(h) => h.handle_event(event),
            AppPage::Kana(k) => k.handle_event(event),
        }
    }

    fn update(&mut self, msg: Message) -> Option<Message> {
        match &mut self.page {
            AppPage::Home(ref mut h) => {
                // quit if msg == Message::Back
                if msg == Message::Back {
                    self.exit = true;
                    return None;
                }

                if msg == Message::Home(HomeMessage::RainFx) {
                    self.disable_rain = !self.disable_rain;
                    return None;
                }

                if msg == Message::Home(HomeMessage::Background) {
                    match h.background_state {
                        BackgroundMode::Cycle => {
                            self.background_number += 1;
                            self.previous_height = 0;

                            if self.background_number + 1 == self.background_paths.len() {
                                h.key_helper_state = BackgroundMode::Disable
                            }

                            if self.background_number >= self.background_paths.len() {
                                self.disable_background = true;
                                self.background_number = 0;
                                h.background_state = BackgroundMode::Disable;
                                h.key_helper_state = BackgroundMode::Cycle;
                            }
                        }
                        BackgroundMode::Disable => {
                            h.background_state = BackgroundMode::Cycle;
                            self.disable_background = false;
                        }
                    }

                    return None;
                }

                if msg == Message::Home(HomeMessage::Up) || msg == Message::Home(HomeMessage::Down)
                {
                    let response = h.update(msg.clone());
                    return response;
                }

                // transform self en App::Kana(new_kana(selected)) if msg == Message::Home(Enter)
                if let Message::Home(HomeMessage::Enter(mode)) = msg {
                    let mut new_kana = KanaModel::new();

                    new_kana.mode = mode;
                    new_kana.update(Message::Kana(KanaMessage::Pass));

                    self.page = AppPage::Kana(new_kana);
                }

                None
            }
            AppPage::Kana(ref mut k) => {
                let response = k.update(msg.clone());

                // transform self en App::Home(new_home) if msg == Message::Back
                if msg == Message::Back {
                    let home = HomeModel::new();

                    self.page = AppPage::Home(home);
                }

                response
            }
        }
    }

    fn view(&mut self, frame: &mut Frame, elapsed: Duration) {
        #[cfg(not(target_arch = "wasm32"))]
        if !self.disable_background {
            self.background(frame);
        }

        if !self.disable_rain {
            rain::view(frame, elapsed);
        }

        match &mut self.page {
            AppPage::Home(ref mut h) => h.view(frame, elapsed),
            AppPage::Kana(ref mut k) => k.view(frame, elapsed),
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl App {
    fn background(&mut self, frame: &mut Frame) {
        let actual_height = frame.area().height;
        if self.previous_height != actual_height {
            self.write_background(actual_height.into());
            self.previous_height = actual_height;
        }

        frame.render_widget((*self.background_widget).clone(), frame.area());
    }

    fn write_background(&mut self, height: u32) {
        // needed because otherwise the render_to function take a while to overwrite the previous string
        let mut buffer = String::new();

        if !self.background_paths.is_empty() {
            render_to(
                self.background_paths[self.background_number].clone(),
                &mut buffer,
                &RenderOptions::new().height(height).colored(true),
            )
            .unwrap();
        } else {
            buffer = String::from("No asset directory")
        }

        self.background_widget = Box::new(buffer.into_text().unwrap().centered());
    }
}
