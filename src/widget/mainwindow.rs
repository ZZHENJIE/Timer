use crate::widget::widget::Widget;
use iced::{
    Task,
    widget::{button, column, text},
    window,
};

#[derive(Clone)]
pub enum Message {
    StartClicked,
    StopClicked,
    SetValue(u64),
}

#[derive(Clone)]
pub struct MainWindow {
    id: iced::window::Id,
    value: u64,
}

impl MainWindow {
    pub fn new(id: iced::window::Id) -> Self {
        MainWindow { id, value: 0 }
    }
    pub fn id(&self) -> &iced::window::Id {
        &self.id
    }
    pub fn reload_settings(&self, settings: &crate::Settings) -> iced::Task<crate::Message> {
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

    fn update(&mut self, message: Self::Message) -> iced::Task<crate::Message> {
        match message {
            Message::StartClicked => Task::future(async move {
                let client = reqwest::Client::new();
                let response = client.get("https://time.akamai.com").send().await.unwrap();
                let response_text = response.text().await.unwrap();
                let time = response_text.parse::<u64>().unwrap();
                Self::into_message(Message::SetValue(time))
            }),
            Message::StopClicked => {
                println!("StopClicked");
                Task::none()
            }
            Message::SetValue(value) => {
                self.value = value;
                Task::none()
            }
        }
    }

    fn view(&self) -> iced::Element<'_, crate::Message> {
        column![
            button("Start").on_press(Self::into_message(Message::StartClicked)),
            button("Stop").on_press(Self::into_message(Message::StopClicked)),
            text(self.value)
        ]
        .into()
    }

    fn into_message(message: Self::Message) -> crate::Message {
        crate::Message::MainWindow(message)
    }
}
