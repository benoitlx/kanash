use ratatui::{
    layout::{Constraint, Layout},
    style::{palette::tailwind::SLATE, Modifier, Style},
    widgets::{BorderType, Borders, HighlightSpacing, List, ListItem, ListState, Padding},
};

use super::*;

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

#[derive(Debug, PartialEq, Eq)]
pub struct HomeModel {
    page_list: Vec<String>,
    state: ListState,
    pub exit: bool,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum HomeMessage {
    /// Launch a Page
    Enter,

    Up,
    Down,
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
            exit: false,
        }
    }

    /// Handle Event (Mostly convert key event to message)
    fn handle_event(&self) -> Option<Message> {
        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Esc => Some(Message::Back),
                KeyCode::Enter => Some(Message::Home(HomeMessage::Enter)),
                KeyCode::Char('j') | KeyCode::Down => Some(Message::Home(HomeMessage::Down)),
                KeyCode::Char('k') | KeyCode::Up => Some(Message::Home(HomeMessage::Up)),
                _ => None,
            }
        } else {
            None
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
                HomeMessage::Enter => {}
            }
        } else {
            if msg == Message::Back {
                self.exit = true;
            }
        }
        None
    }

    fn view(&mut self, frame: &mut Frame) {
        let n_page: u16 = self.page_list.len().try_into().unwrap();
        let [_, main_area, _] = Layout::vertical([
            Constraint::Min(0),
            Constraint::Length(n_page + 4),
            Constraint::Min(0),
        ])
        .areas(frame.area());

        let block = Block::new()
            .title(Line::from(" KANA SH ").red().centered())
            .title_bottom(
                Line::from(" Quit <Esc> | ⌃ <j,Down> | ⌄ <k,Up> | Select <Enter> ")
                    .blue()
                    .bold()
                    .centered(),
            )
            .border_type(BorderType::Rounded)
            .padding(Padding::vertical(1))
            .borders(Borders::ALL);

        let items: Vec<ListItem> = self
            .page_list
            .iter()
            .enumerate()
            .map(|(i, item)| ListItem::new(Line::from(item.clone()).centered()))
            .collect();

        let list = List::new(items)
            .block(block)
            .highlight_style(SELECTED_STYLE)
            .highlight_spacing(HighlightSpacing::Always);

        frame.render_stateful_widget(list, main_area, &mut self.state);
    }
}
