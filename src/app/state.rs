use crate::{
    Message, Settings,
    widget::{mainwindow::MainWindow, widget::Widget},
};
use iced::Task;

pub struct State {
    settings: Settings,
    main_window: Option<MainWindow>,
}

impl Default for State {
    fn default() -> Self {
        Self {
            settings: Settings::new(),
            main_window: None,
        }
    }
}

impl State {
    pub fn subscription(&self) -> iced::Subscription<Message> {
        iced::window::open_events().map(Message::WindowOpened)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::WindowOpened(id) => {
                self.main_window = Some(MainWindow::new(id));
                if let Some(window) = &self.main_window {
                    window.reload_settings(&self.settings)
                } else {
                    Task::none()
                }
            }
            Message::MainWindow(message) => Widget::default_update(&self.main_window, message),
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        Widget::default_view(&self.main_window)
    }
}
