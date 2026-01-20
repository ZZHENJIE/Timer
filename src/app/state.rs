use crate::{
    Message, Settings,
    widget::{
        widget::{Widget, Window},
        window::{WindowType, mainwindow::MainWindow},
    },
};
use iced::Task;
use std::collections::HashMap;

pub struct State {
    settings: Settings,
    windows: HashMap<iced::window::Id, WindowType>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            settings: Settings::new(),
            windows: HashMap::new(),
        }
    }
}

impl State {
    pub fn new() -> (Self, iced::Task<Message>) {
        let state = Self::default();
        let (_, task) = iced::window::open(MainWindow::settings(&state.settings));
        (
            state,
            task.map(move |id| Message::CreateWindow(id, WindowType::Main(MainWindow::new(&id)))),
        )
    }

    pub fn name() -> &'static str {
        "Timer"
    }

    pub fn settings(&self) -> iced::Settings {
        self.settings.settings()
    }

    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::Subscription::none()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::CreateWindow(id, window) => {
                self.windows.insert(id, window);
                Task::none()
            }
            Message::MainWindow(id, message) => {
                if let Some(window) = self.windows.get(&id) {
                    match window {
                        WindowType::Main(window) => window.update(message),
                    }
                } else {
                    Task::none()
                }
            }
        }
    }

    pub fn view(&self, id: iced::window::Id) -> iced::Element<'_, Message> {
        if let Some(window) = self.windows.get(&id) {
            match window {
                WindowType::Main(window) => window.view(),
            }
        } else {
            iced::widget::space().into()
        }
    }
}
