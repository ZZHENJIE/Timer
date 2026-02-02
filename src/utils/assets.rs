use eframe::egui::IconData;
use tracing::info;

pub fn app_icon() -> IconData {
    let icon_bytes = include_bytes!("../../assets/icon.ico");

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
