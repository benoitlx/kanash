pub mod app;
pub mod helper;
pub mod home;
pub mod kana;

#[cfg(not(target_arch = "wasm32"))]
pub use ratatui::{
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Flex, Layout, Rect},
    style::Stylize,
    style::{palette::tailwind::SLATE, Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Borders, Clear, HighlightSpacing, List, ListItem, ListState, Padding,
        Paragraph, Wrap,
    },
    Frame,
};

#[cfg(not(target_arch = "wasm32"))]
pub type PlatformKeyEvent = ratatui::crossterm::event::KeyEvent;

#[cfg(target_arch = "wasm32")]
pub use ratzilla::ratatui::{
    layout::{Constraint, Flex, Layout},
    style::Stylize,
    style::{palette::tailwind::SLATE, Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, BorderType, Borders, Clear, HighlightSpacing, List, ListItem, ListState, Padding,
        Paragraph, Wrap,
    },
    Frame,
};

#[cfg(target_arch = "wasm32")]
pub type PlatformKeyEvent = ratzilla::event::KeyEvent;

#[cfg(target_arch = "wasm32")]
pub use ratzilla::{event::KeyCode, web_sys::console};

use std::time::Duration;

use helper::{help_popup, ja::*};
use home::{HomeMessage, Mode};
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

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Message {
    /// Go to the previous page or quit the app
    Back,

    Home(HomeMessage),
    Kana(KanaMessage),
}

pub trait Components {
    fn new() -> Self;

    fn handle_event(&self, event: &PlatformKeyEvent) -> Option<Message>;

    fn update(&mut self, msg: Message) -> Option<Message>;

    fn view(&mut self, frame: &mut Frame, elapsed: Duration);
}
