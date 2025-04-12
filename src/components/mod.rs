pub mod kana;
pub mod home;
pub mod helper;

pub use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Wrap},
    Frame,
    style::Stylize,
};

use home::HomeMessage;
use kana::KanaMessage;

#[derive(Debug, PartialEq, Eq)]
pub enum Message {
    /// Go to the previous page or quit the app
    Back,

    Home(HomeMessage),
    Kana(KanaMessage),
}

pub trait Components {
    fn new() -> Self;

    fn handle_event(&self) -> Option<Message>;

    fn update(&mut self, msg:Message) -> Option<Message>;

    fn view(&mut self, frame: &mut Frame);
}