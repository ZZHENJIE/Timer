use crate::widget::{Widget, Window};
use iced::{
    Task,
    widget::{button, column, text},
    window,
};

#[derive(Clone)]
pub enum InnerMessage {
    StartClicked,
    StopClicked,
    SetValue(u64),
}

#[derive(Clone)]
pub struct MainWindow {
    id: iced::window::Id,
    value: u64,
}

impl Window for MainWindow {
    fn new(id: iced::window::Id) -> Self {
        MainWindow { id, value: 0 }
    }
    fn id(&self) -> iced::window::Id {
        self.id.clone()
    }
    fn load_settings(_: &crate::Settings) -> window::Settings {
        let default = window::Settings::default();
        default
    }
    fn settings(&self) -> iced::Task<crate::Message> {
        Task::none()
    }
    fn title(&self) -> String {
        "Timer".into()
    }
    fn close(&self) -> iced::Task<crate::Message> {
        iced::exit()
    }
}

impl Widget for MainWindow {
    type Message = InnerMessage;

    fn into_message(&self, message: Self::Message) -> crate::Message {
        crate::Message::MainWindow(self.id(), message)
    }
    fn update(&mut self, message: Self::Message) -> iced::Task<crate::Message> {
        match message {
            InnerMessage::StartClicked => {
                let id = self.id();
                Task::future(async move {
                    let client = reqwest::Client::new();
                    let response = client.get("https://time.akamai.com").send().await.unwrap();
                    let response_text = response.text().await.unwrap();
                    let time = response_text.parse::<u64>().unwrap();
                    crate::Message::MainWindow(id, InnerMessage::SetValue(time))
                })
            }
            InnerMessage::StopClicked => {
                println!("StopClicked");
                Task::none()
            }
            InnerMessage::SetValue(value) => {
                self.value = value;
                Task::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, crate::Message> {
        column![
            button("Start").on_press(self.into_message(InnerMessage::StartClicked)),
            button("Stop").on_press(self.into_message(InnerMessage::StopClicked)),
            text(self.value)
        ]
        .into()
    }
}
