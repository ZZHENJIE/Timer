use crate::widget::widget::{Widget, Window};
use iced::{
    Task,
    widget::{button, column},
    window,
};

#[derive(Clone)]
pub enum Message {
    StartClicked,
    StopClicked,
}

#[derive(Clone)]
pub struct MainWindow {
    id: iced::window::Id,
}

impl Window for MainWindow {
    fn new(id: iced::window::Id) -> Self {
        MainWindow { id }
    }
    fn id(&self) -> &iced::window::Id {
        &self.id
    }
    fn reload_settings(&self, settings: &crate::Settings) -> iced::Task<crate::Message> {
        let main_window = &settings.main_window;
        let pos = iced::Point::new(main_window.pos_x, main_window.pos_y);
        let size = iced::Size::new(main_window.width, main_window.height);
        window::move_to(self.id.clone(), pos)
            .chain(window::resize(self.id.clone(), size))
            .chain(window::set_level(self.id.clone(), main_window.level.into()))
    }
}

impl Widget for MainWindow {
    type Message = Message;

    fn update(&self, message: Self::Message) -> iced::Task<crate::Message> {
        match message {
            Message::StartClicked => println!("StartClicked"),
            Message::StopClicked => println!("StopClicked"),
        }
        Task::none()
    }

    fn view(&self) -> iced::Element<'_, crate::Message> {
        column![
            button("Start").on_press(self.into_message(Message::StartClicked)),
            button("Stop").on_press(self.into_message(Message::StopClicked))
        ]
        .into()
    }

    fn into_message(&self, message: Self::Message) -> crate::Message {
        crate::Message::MainWindow(message)
    }
}
