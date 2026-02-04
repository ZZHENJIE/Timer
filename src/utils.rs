use chrono::{Local, LocalResult, TimeZone};
use eframe::egui::{Color32, IconData};
use tracing::info;
use tracing_appender::rolling;
use tracing_subscriber::fmt::time::LocalTime;

pub fn app_icon() -> IconData {
    let icon_bytes = include_bytes!("../assets/icon.ico");

    match image::load_from_memory_with_format(icon_bytes, image::ImageFormat::Ico) {
        Ok(image) => IconData {
            width: image.width(),
            height: image.height(),
            rgba: image.into_rgba8().into_raw(),
        },
        Err(err) => {
            info!("load icon failed: {}", err);
            IconData::default()
        }
    }
}

pub fn format_timestamp_to_local(timestamp: i64) -> String {
    match Local.timestamp_opt(timestamp, 0) {
        LocalResult::Single(local_time) | LocalResult::Ambiguous(local_time, _) => {
            local_time.format("%H:%M:%S").to_string()
        }
        LocalResult::None => {
            info!("Invalid timestamp: no valid local time");
            "00:00:00".to_string()
        }
    }
}

pub fn string_to_color_hex(color: &str) -> Color32 {
    Color32::from_hex(color).unwrap_or_else(|_| {
        info!("Failed to parse color.");
        Color32::WHITE
    })
}

pub fn tracing_subscriber_init() {
    let file_appender = rolling::daily("logs", "timer.log");

    tracing_subscriber::fmt()
        .with_timer(LocalTime::rfc_3339())
        .with_writer(file_appender)
        .init();
}
