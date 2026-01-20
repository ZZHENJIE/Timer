use crate::widget::window::{WindowType, mainwindow};

#[derive(Clone)]
pub enum Message {
    CreateWindow(iced::window::Id, WindowType),
    MainWindow(iced::window::Id, mainwindow::Message),
}
