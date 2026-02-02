#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use timer::app::App;

#[tokio::main]
async fn main() -> eframe::Result {
    timer::utils::log::init();

    let option = eframe::NativeOptions {
        vsync: true,
        ..Default::default()
    };

    eframe::run_native(
        "Timer",
        option,
        Box::new(|cx| {
            let app = App::new(&cx.egui_ctx);
            Ok(Box::new(app))
        }),
    )
}
