use ratatui::{
    layout::{Constraint, Layout},
    style::{palette::tailwind::SLATE, Modifier, Style},
    widgets::{BorderType, Borders, HighlightSpacing, List, ListItem, ListState, Padding},
};

use super::*;

const SELECTED_STYLE: Style = Style::new().bg(SLATE.c800).add_modifier(Modifier::BOLD);

pub struct HomeModel {
    page_list: Vec<String>,
    state: ListState,
    pub exit: bool,
}

pub enum HomeMessage {
    /// Exit the app
    Back,

    /// Launch a Page
    Enter,

    Up,
    Down,
}

impl HomeModel {
    pub fn new() -> Self {
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
    pub fn handle_event(&self) -> Option<HomeMessage> {
        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Esc => Some(HomeMessage::Back),
                KeyCode::Enter => Some(HomeMessage::Enter),
                KeyCode::Char('j') | KeyCode::Down => Some(HomeMessage::Down),
                KeyCode::Char('k') | KeyCode::Up => Some(HomeMessage::Up),
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn update(&mut self, msg: HomeMessage) -> Option<HomeMessage> {
        match msg {
            HomeMessage::Back => {
                self.exit = true;
            }
            HomeMessage::Down => {
                self.state.select_next();
            }
            HomeMessage::Up => {
                self.state.select_previous();
            }
            HomeMessage::Enter => {}
        }
        None
    }

    pub fn view(&mut self, frame: &mut Frame) {
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
