pub trait Widget: Sized {
    type Message: Clone;

    fn into_message(message: Self::Message) -> crate::Message;
    fn update(&mut self, message: Self::Message) -> iced::Task<crate::Message>;
    fn view(&self) -> iced::Element<'_, crate::Message>;

    fn default_update(
        widget: &mut Option<Self>,
        message: Self::Message,
    ) -> iced::Task<crate::Message> {
        if let Some(widget) = widget {
            widget.update(message)
        } else {
            iced::Task::none()
        }
    }
    fn default_view(widget: &Option<Self>) -> iced::Element<'_, crate::Message> {
        if let Some(widget) = widget {
            widget.view()
        } else {
            iced::widget::space().into()
        }
    }
}
