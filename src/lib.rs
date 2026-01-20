pub mod app;
pub mod widget;
pub mod utils {
    pub mod log;
}

pub use app::message::Message;
pub use app::settings::Settings;
pub use app::state::State;
use iced::{Daemon, Program, Theme};

pub fn application() -> Daemon<impl Program<State = State, Message = Message, Theme = Theme>> {
    iced::daemon(State::new, State::update, State::view)
}
