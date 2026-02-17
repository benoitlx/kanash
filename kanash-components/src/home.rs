use super::*;

const TITLE: &str = " KANA SH ";
const KEY_HELPER: &str = " help - <h?> ";
const HELP_STRING: &str = "h? - toggle help popup\njk - navigate up and down\nx - disable rain fx\nEnter - enter the selected mode\nq Escape - quit";
const SELECTED_STYLE: Style = Style::new()
    .bg(ColorPalette::SELECTION)
    .add_modifier(Modifier::BOLD);

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mode {
    Hira,
    Kata,
    Both,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BackgroundMode {
    Cycle,
    Disable,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HomeModel {
    page_list: Vec<String>,
    state: ListState,
    show_help_popup: bool,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HomeMessage {
    /// Launch a Page
    Enter(Mode),

    Up,
    Down,

    RainFx,

    Help,
}

impl Components for HomeModel {
    fn new() -> Self {
        let mut init_state = ListState::default();
        init_state.select_first();

        Self {
            page_list: vec![
                "Learn Hiragana あ".into(),
                "Learn Katakana ア".into(),
                "Learn Both".into(),
            ],
            state: init_state,
            show_help_popup: false,
        }
    }

    /// Handle Event (Mostly convert key event to message)
    fn handle_event(&self, event: &PlatformKeyEvent) -> Option<Message> {
        match event.code {
            KeyCode::Esc | KeyCode::Char('q') => Some(Message::Back),
            KeyCode::Enter => {
                if let Some(i) = self.state.selected() {
                    match i {
                        0 => Some(Message::Home(HomeMessage::Enter(Mode::Hira))),
                        1 => Some(Message::Home(HomeMessage::Enter(Mode::Kata))),
                        2 => Some(Message::Home(HomeMessage::Enter(Mode::Both))),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            KeyCode::Char('j') | KeyCode::Down => Some(Message::Home(HomeMessage::Down)),
            KeyCode::Char('k') | KeyCode::Up => Some(Message::Home(HomeMessage::Up)),
            KeyCode::Char('h') | KeyCode::Char('?') => Some(Message::Home(HomeMessage::Help)),
            KeyCode::Char('x') => Some(Message::Home(HomeMessage::RainFx)),
            _ => None,
        }
    }

    fn update(&mut self, msg: Message) -> Option<Message> {
        if let Message::Home(home_msg) = msg {
            match home_msg {
                HomeMessage::Down => {
                    self.state.select_next();
                }
                HomeMessage::Up => {
                    self.state.select_previous();
                }
                HomeMessage::Help => {
                    self.show_help_popup = !self.show_help_popup;
                }
                _ => {}
            }
        }
        None
    }

    fn view(&mut self, frame: &mut Frame, _elapsed: Duration) {
        let n_page: u16 = self.page_list.len().try_into().unwrap();
        let [_, vert_area, _] = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(n_page + 4),
            Constraint::Min(0),
        ])
        .areas(frame.area());

        let [_, main_area, _] =
            Layout::horizontal([Constraint::Min(0), Constraint::Max(23), Constraint::Min(0)])
                .areas(vert_area);

        let block = Block::new()
            .title(Line::from(TITLE).fg(ColorPalette::TITLE).centered())
            .title_bottom(Line::from(KEY_HELPER).fg(ColorPalette::KEY_HINT).centered())
            .border_type(BorderType::Rounded)
            .padding(Padding::vertical(1))
            .borders(Borders::ALL);

        let items: Vec<ListItem> = self
            .page_list
            .iter()
            .map(|item| ListItem::new(Line::from(item.clone()).centered()))
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_spacing(HighlightSpacing::Always);

        frame.render_widget(Clear, main_area);
        frame.render_stateful_widget(list, main_area, &mut self.state);

        if self.show_help_popup {
            let block = Block::new()
                .title("Help")
                .border_type(BorderType::Rounded)
                .borders(Borders::ALL);
            let p = Paragraph::new(HELP_STRING)
                .fg(ColorPalette::KEY_HINT)
                .block(block);

            let vertical = Layout::vertical([Constraint::Percentage(30)]).flex(Flex::Center);
            let horizontal = Layout::horizontal([Constraint::Percentage(50)]).flex(Flex::Center);
            let [area] = vertical.areas(frame.area());
            let [area] = horizontal.areas(area);

            frame.render_widget(Clear, area);
            frame.render_widget(p, area);
        }
    }
}
