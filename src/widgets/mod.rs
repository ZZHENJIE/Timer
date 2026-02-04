pub mod windows {
    pub mod settings;
}
pub mod content;

pub trait Widget {
    fn new(ctx: &eframe::egui::Context, settings: &crate::Settings) -> Self;
    fn update(&mut self, ui: &mut eframe::egui::Ui, settings: &mut crate::Settings);
}
