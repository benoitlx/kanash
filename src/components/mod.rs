pub mod app;
pub mod helper;
pub mod home;
pub mod kana;

pub use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout},
    style::Stylize,
    style::{palette::tailwind::SLATE, Modifier, Style},
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Wrap},
    widgets::{BorderType, Borders, HighlightSpacing, List, ListItem, ListState, Padding},
    Frame,
};

use home::HomeMessage;
use kana::KanaMessage;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Message {
    /// Go to the previous page or quit the app
    Back,

    Home(HomeMessage),
    Kana(KanaMessage),
}

pub trait Components {
    fn new() -> Self;

    fn handle_event(&self) -> Option<Message>;

    fn update(&mut self, msg: Message) -> Option<Message>;

    fn view(&mut self, frame: &mut Frame);
}
