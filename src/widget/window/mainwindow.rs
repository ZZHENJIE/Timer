use crate::widget::widget::{Widget, Window};
use iced::{
    Task,
    widget::{button, column},
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
    fn new(id: &iced::window::Id) -> Self {
        MainWindow { id: id.clone() }
    }
    fn id(&self) -> &iced::window::Id {
        &self.id
    }
    fn settings(settings: &crate::Settings) -> iced::window::Settings {
        iced::window::Settings::default()
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
        match message {
            Message::StartClicked => {
                crate::Message::MainWindow(self.id.clone(), Message::StartClicked)
            }
            Message::StopClicked => {
                crate::Message::MainWindow(self.id.clone(), Message::StopClicked)
            }
        }
    }
}
