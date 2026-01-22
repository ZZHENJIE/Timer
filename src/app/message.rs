use crate::widget::window;

#[derive(Clone)]
pub enum Message {
    WindowClose(iced::window::Id),
    MainWindow(window::main::InnerMessage),
    SettingsWindow(window::settings::InnerMessage),
    None,
}
