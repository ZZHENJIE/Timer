use crate::{
    Message, Settings,
    widget::{
        Widget, Window,
        window::{Windows, main::MainWindow},
    },
};
use iced::{Task, window};
use std::collections::HashMap;

pub struct State {
    settings: Settings,
    windows: HashMap<iced::window::Id, Windows>,
}

impl State {
    pub fn new() -> (Self, Task<Message>) {
        let state = Self {
            settings: Settings::new(),
            windows: HashMap::new(),
        };
        let (_, task) = window::open(MainWindow::load_settings(&state.settings));
        (
            state,
            task.map(|id| Message::CreateWindow(id, Windows::Main(MainWindow::new(id)))),
        )
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        window::close_events().map(Message::WindowClose)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::CreateWindow(id, window) => {
                self.windows.insert(id, window);
                Task::none()
            }
            Message::MainWindow(id, message) => {
                if let Some(window_type) = self.windows.get_mut(&id) {
                    if let Windows::Main(window) = window_type {
                        window.update(message)
                    } else {
                        Task::none()
                    }
                } else {
                    Task::none()
                }
            }
            Message::SettingsWindow(id, message) => {
                if let Some(window_type) = self.windows.get_mut(&id) {
                    if let Windows::Settings(window) = window_type {
                        window.update(message)
                    } else {
                        Task::none()
                    }
                } else {
                    Task::none()
                }
            }
            Message::WindowClose(id) => {
                if let Some(window) = self.windows.remove(&id) {
                    window.close()
                } else {
                    Task::none()
                }
            }
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
