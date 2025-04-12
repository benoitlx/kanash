mod components;
use components::kana::KanaModel;
use components::home::HomeModel;
use components::Components;

use ratatui::restore;

#[tokio::main]
async fn main() {
    let mut terminal = ratatui::init();

    let mut app = KanaModel::new();

    while !app.exit {
        let _ = terminal.draw(|frame| app.view(frame));

        let mut current_msg = app.handle_event();

        while current_msg.is_some() {
            current_msg = app.update(current_msg.unwrap());
        }
    }

    restore();
}