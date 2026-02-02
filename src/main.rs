use timer::{app::App, utils::settings::Settings};

#[tokio::main]
async fn main() -> eframe::Result {
    timer::utils::log::init();

    eframe::run_native(
        "Timer",
        eframe::NativeOptions::default(),
        Box::new(|cx| {
            let settings = Settings::new();
            let app = App::new(settings);
            app.init(&cx.egui_ctx);
            Ok(Box::new(app))
        }),
    )
}
