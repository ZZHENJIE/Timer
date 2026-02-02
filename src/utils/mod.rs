pub mod assets;
pub mod log;
pub mod settings;
pub mod settings_window;

use chrono::{Local, LocalResult, TimeZone};
use tracing::info;

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
