fn main() -> iced::Result {
    timer::utils::log::init();
    timer::application().run()
}
