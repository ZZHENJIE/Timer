use crate::widget::window::{self, Windows};

#[derive(Clone)]
pub enum Message {
    CreateWindow(iced::window::Id, Windows),
    WindowClose(iced::window::Id),
    MainWindow(iced::window::Id, window::main::InnerMessage),
    SettingsWindow(iced::window::Id, window::settings::InnerMessage),
}
