use tracing::level_filters::LevelFilter;
use tracing_subscriber::{Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};

pub fn init() {
    // open log file
    let log_file = std::fs::File::options()
        .create(true)
        .append(true)
        .open("log.json")
        .expect("create/open log file");
    // set log level filter
    let file_layer = fmt::layer()
        .json()
        .with_writer(log_file)
        .with_timer(fmt::time::LocalTime::rfc_3339())
        .with_filter(LevelFilter::INFO);
    tracing_subscriber::registry().with(file_layer).init();
}
