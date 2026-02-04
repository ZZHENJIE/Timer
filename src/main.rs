#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tokio::main(flavor = "multi_thread", worker_threads = 2)]
async fn main() -> eframe::Result {
    timer::utils::tracing_subscriber_init();

    eframe::run_native(
        "Timer",
        native_option(),
        Box::new(|cx| {
            let app = timer::App::new(&cx.egui_ctx);
            Ok(Box::new(app))
        }),
    )
}

#[cfg(target_os = "windows")]
fn native_option() -> eframe::NativeOptions {
    let icon = timer::utils::app_icon();
    eframe::NativeOptions {
        vsync: true,
        viewport: eframe::egui::ViewportBuilder::default().with_icon(icon),
        ..Default::default()
    }
}

fn native_option() -> eframe::NativeOptions {
    eframe::NativeOptions {
        vsync: true,
        ..Default::default()
    }
}
