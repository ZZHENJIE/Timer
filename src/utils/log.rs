use tracing_appender::rolling;
use tracing_subscriber::fmt::time::LocalTime;

pub fn init() {
    let file_appender = rolling::daily("logs", "timer.log");

    tracing_subscriber::fmt()
        .with_timer(LocalTime::rfc_3339())
        .with_writer(file_appender)
        .init();
}
