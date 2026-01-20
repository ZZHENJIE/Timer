pub trait Widget {
    type Message: Clone;

    fn into_message(&self, message: Self::Message) -> crate::Message;
    fn update(&self, message: Self::Message) -> iced::Task<crate::Message>;
    fn view(&self) -> iced::Element<'_, crate::Message>;
}

pub trait Window: Widget + Clone {
    fn new(id: &iced::window::Id) -> Self;
    fn id(&self) -> &iced::window::Id;
    fn settings(settings: &crate::Settings) -> iced::window::Settings;
}
