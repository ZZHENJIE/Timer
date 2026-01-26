use crate::widget::{Widget, Window};
pub mod main;
pub mod settings;

#[derive(Clone)]
pub enum Windows {
    Main(main::MainWindow),
    Settings(settings::SettingsWindow),
}

impl Windows {
    pub fn view(&self) -> iced::Element<'_, crate::Message> {
        match self {
            Windows::Main(window) => window.view(),
            Windows::Settings(window) => window.view(),
        }
    }
    pub fn close(&self) -> iced::Task<crate::Message> {
        match self {
            Windows::Main(window) => window.close(),
            Windows::Settings(window) => window.close(),
        }
    }
}
