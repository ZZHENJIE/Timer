use timer::State;

fn main() -> iced::Result {
    timer::utils::log::init();
    iced::daemon(State::new, State::update, State::view)
        .subscription(State::subscription)
        .run()
}
