use super::{home::HomeModel, kana::KanaModel, *};
use crate::helper::rain;

#[derive(Debug, PartialEq, Eq)]
enum AppPage {
    Home(HomeModel),
    Kana(KanaModel),
}

#[derive(Debug, PartialEq, Eq)]
pub struct App {
    pub exit: bool,
    page: AppPage,
    previous_height: u16,
    disable_rain: bool,
}

impl Components for App {
    fn new() -> Self {
        let home = HomeModel::new();

        Self {
            exit: false,
            page: AppPage::Home(home),
            previous_height: 0,
            disable_rain: false,
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

                // transform self en App::Kana(new_kana(selected)) if msg == Message::Home(Enter)
                if let Message::Home(HomeMessage::Enter(mode)) = msg {
                    let mut new_kana = KanaModel::new();

                    new_kana.mode = mode;
                    let response = new_kana.update(Message::Kana(KanaMessage::Pass));

                    self.page = AppPage::Kana(new_kana);
                    return response;
                }

                h.update(msg.clone())
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
        if !self.disable_rain {
            rain::view(frame, elapsed);
        }

        match &mut self.page {
            AppPage::Home(ref mut h) => h.view(frame, elapsed),
            AppPage::Kana(ref mut k) => k.view(frame, elapsed),
        }
    }
}
