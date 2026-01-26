pub trait Widget {
    type Message: Clone;
    fn into_message(&self, message: Self::Message) -> crate::Message;
    fn update(&mut self, message: Self::Message) -> iced::Task<crate::Message>;
    fn view(&self) -> iced::Element<'_, crate::Message>;
}

pub trait Window: Widget {
    fn title(&self) -> String;
    fn load_settings(settings: &crate::Settings) -> iced::window::Settings
    where
        Self: Sized;
    fn id(&self) -> iced::window::Id;
    fn new(id: iced::window::Id) -> Self
    where
        Self: Sized;
    fn settings(&self) -> iced::Task<crate::Message>;
    fn close(&self) -> iced::Task<crate::Message>;
}
