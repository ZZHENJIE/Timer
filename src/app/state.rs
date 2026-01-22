use crate::{
    widget::{window::main::MainWindow, Window},
    Message, Settings,
};
use iced::{window, Task};
use std::collections::HashMap;

pub struct State {
    settings: Settings,
    windows: HashMap<iced::window::Id, Box<dyn Window<Message = crate::Message>>>,
}

impl State {
    pub fn new() -> (Self, Task<Message>) {
        let mut state = Self {
            settings: Settings::new(),
            windows: HashMap::new(),
        };
        let (id, task) = window::open(MainWindow::load_settings(&state.settings));
        state.windows.insert(id, Box::new(MainWindow::new(id)));
        (state, task.map(|_| Message::None))
    }
    pub fn subscription(&self) -> iced::Subscription<Message> {
        window::close_events().map(Message::WindowClose)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::MainWindow(_) => Task::none(),
            Message::SettingsWindow(_) => Task::none(),
            Message::WindowClose(_) => Task::none(),
            Message::None => Task::none(),
        }
    }

    pub fn view(&self, id: window::Id) -> iced::Element<'_, Message> {
        if let Some(window) = self.windows.get(&id) {
            window.view()
        } else {
            iced::widget::space().into()
        }
    }
}
