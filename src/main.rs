use timer::State;

fn main() -> iced::Result {
    timer::utils::log::init();
    iced::application(State::default, State::update, State::view)
        .subscription(State::subscription)
        .title("Timer")
        .run()
}
