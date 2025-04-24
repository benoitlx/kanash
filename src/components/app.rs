use super::{home::HomeModel, kana::KanaModel, *};

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
                if msg == Message::Home(HomeMessage::Up) || msg == Message::Home(HomeMessage::Down)
                {
                    let response = h.update(msg.clone());
                    return response;
                }

                // quit if msg == Message::Back
                if msg == Message::Back {
                    *self = App::Exit;
                }

                // transform self en App::Kana(new_kana(selected)) if msg == Message::Home(Enter)
                if let Message::Home(home_msg) = msg {
                    let mut new_kana = KanaModel::new();

                    match home_msg {
                        HomeMessage::EnterHira => new_kana.mode = Mode::Hira,
                        HomeMessage::EnterKata => new_kana.mode = Mode::Kata,
                        HomeMessage::EnterBoth => new_kana.mode = Mode::Both,
                        _ => {}
                    }

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
        match self {
            App::Home(h) => h.view(frame, elapsed),
            App::Kana(k) => k.view(frame, elapsed),
            _ => {}
        }
    }
}
