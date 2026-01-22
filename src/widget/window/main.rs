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
        window::Settings::default()
    }
    fn settings(&self) -> iced::Task<crate::Message> {
        Task::none()
    }
    fn title(&self) -> String {
        "Timer".into()
    }
}

impl Widget for MainWindow {
    type Message = crate::Message;

    fn update(&mut self, message: Self::Message) -> iced::Task<crate::Message> {
        if let crate::Message::MainWindow(msg) = message {
            match msg {
                InnerMessage::StartClicked => Task::future(async move {
                    let client = reqwest::Client::new();
                    let response = client.get("https://time.akamai.com").send().await.unwrap();
                    let response_text = response.text().await.unwrap();
                    let time = response_text.parse::<u64>().unwrap();
                    crate::Message::MainWindow(InnerMessage::SetValue(time))
                }),
                InnerMessage::StopClicked => {
                    println!("StopClicked");
                    Task::none()
                }
                InnerMessage::SetValue(value) => {
                    self.value = value;
                    Task::none()
                }
            }
        } else {
            Task::none()
        }
    }

    fn view(&self) -> iced::Element<'_, crate::Message> {
        column![
            button("Start").on_press(crate::Message::MainWindow(InnerMessage::StartClicked)),
            button("Stop").on_press(crate::Message::MainWindow(InnerMessage::StopClicked)),
            text(self.value)
        ]
        .into()
    }
}
