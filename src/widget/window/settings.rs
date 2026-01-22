use crate::widget::{Widget, Window};
use iced::{window, Task};

#[derive(Clone)]
pub enum InnerMessage {}

#[derive(Clone)]
pub struct SettingsWindow {
    id: iced::window::Id,
}

impl Window for SettingsWindow {
    fn new(id: iced::window::Id) -> Self {
        Self { id }
    }
    fn id(&self) -> iced::window::Id {
        self.id.clone()
    }
    fn load_settings(_: &crate::Settings) -> iced::window::Settings {
        window::Settings::default()
    }
    fn settings(&self) -> iced::Task<crate::Message> {
        Task::none()
    }
    fn title(&self) -> String {
        "Settings".into()
    }
}

impl Widget for SettingsWindow {
    type Message = crate::Message;
    fn update(&mut self, _: Self::Message) -> iced::Task<crate::Message> {
        Task::none()
    }
    fn view(&self) -> iced::Element<'_, crate::Message> {
        iced::widget::space().into()
    }
}
