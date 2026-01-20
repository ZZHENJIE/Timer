use crate::widget::mainwindow;

#[derive(Clone)]
pub enum Message {
    WindowOpened(iced::window::Id),
    MainWindow(mainwindow::Message),
}
