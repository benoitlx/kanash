pub mod app;
pub mod helper;
pub mod home;
pub mod kana;

pub use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout},
    style::Stylize,
    style::{palette::tailwind::SLATE, Color, Modifier, Style},
    text::Line,
    widgets::{Block, Paragraph},
    widgets::{BorderType, Borders, Clear, HighlightSpacing, List, ListItem, ListState, Padding},
    Frame,
};

use std::time::Duration;

use home::{BackgroundMode, HomeMessage, Mode};
use kana::KanaMessage;

pub struct ColorPalette;

impl ColorPalette {
    pub const TITLE: Color = Color::from_u32(0x00ff33ff);
    pub const SUBTITLE: Color = Color::from_u32(0x00ff3399);
    pub const ERROR: Color = Color::from_u32(0x00ff3333);
    pub const KEY_HINT: Color = Color::from_u32(0x00ff9933);
    pub const SELECTION: Color = SLATE.c800;
    // 0x00ffff33
    // #99ff33
}

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

    fn view(&mut self, frame: &mut Frame, elapsed: Duration);
}
