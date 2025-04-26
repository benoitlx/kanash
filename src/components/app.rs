use super::{home::HomeModel, kana::KanaModel, *};
use crate::components::helper::rain;
use ansi_to_tui::IntoText;
use rascii_art::{render_to, RenderOptions};

#[derive(Debug, PartialEq, Eq)]
enum AppPage {
    Home(HomeModel),
    Kana(KanaModel),
}

#[derive(Debug, PartialEq, Eq)]
pub struct App {
    pub exit: bool,
    background_buffer: String,
    page: AppPage,
    previous_height: u16,
}

impl Components for App {
    fn new() -> Self {
        let home = HomeModel::new();

        Self {
            exit: false,
            page: AppPage::Home(home),
            background_buffer: String::new(),
            previous_height: 0,
        }
    }

    fn handle_event(&self) -> Option<Message> {
        match &self.page {
            AppPage::Home(h) => h.handle_event(),
            AppPage::Kana(k) => k.handle_event(),
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
                    *self = App::new()
                }

                response
            }
        }
    }

    fn view(&mut self, frame: &mut Frame, elapsed: Duration) {
        self.background(frame, elapsed);
        match &mut self.page {
            AppPage::Home(ref mut h) => h.view(frame, elapsed),
            AppPage::Kana(ref mut k) => k.view(frame, elapsed),
        }
    }
}

impl App {
    fn background(&mut self, frame: &mut Frame, elapsed: Duration) {
        let actual_height = frame.area().height;
        if self.previous_height != actual_height {
            self.write_background(actual_height.into());
            self.previous_height = actual_height;
        }

        let widget = self.background_buffer.into_text().unwrap().centered();

        frame.render_widget(widget, frame.area());
        rain::view(frame, elapsed);
    }

    fn write_background(&mut self, height: u32) {
        // needed because otherwise the render_to function take a while to overwrite the previous string
        self.background_buffer = String::new();

        render_to(
            r"./assets/gate_low_res.jpg",
            &mut self.background_buffer,
            &RenderOptions::new().height(height).colored(true),
        )
        .unwrap();
    }
}
