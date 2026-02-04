use crate::{
    Settings,
    widgets::{content::Content, windows::settings::SettingsWindow},
};
use eframe::egui;

pub struct App {
    settings: Settings,
    content: Content,
    settings_window: SettingsWindow,
}

impl App {
    pub fn new(ctx: &eframe::egui::Context) -> Self {
        let settings = Settings::new();
        Self {
            settings_window: SettingsWindow::new(ctx, &settings),
            content: Content::new(ctx, &settings),
            settings,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let ui_builder = egui::UiBuilder::new();
            ui.scope_builder(ui_builder, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    if self.content.update(ui, &mut self.settings) {
                        if self.settings_window.update(ui, &mut self.settings) {
                            self.content.close_settings_window();
                        }
                    }
                });
            });
        });
    }
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.settings.save();
    }
    fn raw_input_hook(&mut self, _ctx: &egui::Context, _raw_input: &mut egui::RawInput) {
        if let Some(rect) = _raw_input.viewport().inner_rect {
            self.settings.window.height = rect.height();
            self.settings.window.width = rect.width();
        }
        if let Some(rect) = _raw_input.viewport().outer_rect {
            self.settings.window.x = rect.min.x;
            self.settings.window.y = rect.min.y;
        }
    }
}
