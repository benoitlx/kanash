use super::{home::HomeModel, kana::KanaModel, *};
use crate::components::helper::rain;
use ansi_to_tui::IntoText;
use rascii_art::{render_to, RenderOptions};

#[derive(Debug, PartialEq, Eq)]
pub enum App {
    Home(HomeModel),
    Kana(KanaModel),
    Exit,
}

impl Components for App {
    fn new() -> Self {
        let home = HomeModel::new();
        App::Home(home)
    }

    fn handle_event(&self) -> Option<Message> {
        match self {
            App::Home(h) => h.handle_event(),
            App::Kana(k) => k.handle_event(),
            _ => None,
        }
    }

    fn update(&mut self, msg: Message) -> Option<Message> {
        match self {
            App::Home(h) => {
                // quit if msg == Message::Back
                if msg == Message::Back {
                    *self = App::Exit;
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

                    *self = App::Kana(new_kana);
                }

                None
            }
            App::Kana(k) => {
                let response = k.update(msg.clone());

                // transform self en App::Home(new_home) if msg == Message::Back
                if msg == Message::Back {
                    *self = App::new()
                }

                response
            }
            _ => None,
        }
    }

    fn view(&mut self, frame: &mut Frame, elapsed: Duration) {
        self.background(frame, elapsed);
        match self {
            App::Home(h) => h.view(frame, elapsed),
            App::Kana(k) => k.view(frame, elapsed),
            _ => {}
        }
    }
}

impl App {
    fn background(&mut self, frame: &mut Frame, elapsed: Duration) {
        let mut buffer = String::new();

        render_to(
            r"./assets/gate_low_res.jpg",
            &mut buffer,
            &RenderOptions::new()
                .height(frame.area().height.into())
                .colored(true),
        )
        .unwrap();

        let widget = buffer.into_text().unwrap().centered();

        frame.render_widget(widget, frame.area());
        rain::view(frame, elapsed);
    }
}
