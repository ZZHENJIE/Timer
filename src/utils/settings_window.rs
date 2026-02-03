use crate::utils::{
    assets::app_icon,
    settings::{Settings, Theme, WindowLevel},
};
use eframe::egui::{
    self, Button, Color32, ComboBox, Context, Layout, RichText, Ui, Vec2, ViewportCommand, Window,
    pos2,
};
use std::sync::Arc;
use tracing::info;

pub struct SettingsWindow {
    settings: Settings,
    is_visible: bool,
}

impl SettingsWindow {
    pub fn new() -> Self {
        Self {
            settings: Settings::new(),
            is_visible: false,
        }
    }
    pub fn init(&self, ctx: &Context) {
        let window_settings = &self.settings.window;
        ctx.set_theme(self.settings.theme);
        ctx.send_viewport_cmd(ViewportCommand::Icon(Some(Arc::new(app_icon()))));
        ctx.send_viewport_cmd(ViewportCommand::InnerSize(Vec2::new(
            window_settings.width,
            window_settings.height,
        )));
        ctx.send_viewport_cmd(ViewportCommand::OuterPosition(pos2(
            window_settings.x,
            window_settings.y,
        )));
        ctx.send_viewport_cmd(ViewportCommand::WindowLevel(window_settings.level.into()));
    }
    pub fn open(&mut self) {
        self.is_visible = true;
    }
    pub fn save_settings(&self) {
        self.settings.save();
    }
    pub fn color_hex(color: &str) -> Color32 {
        Color32::from_hex(color).unwrap_or_else(|_| {
            info!("Failed to parse color.");
            Color32::WHITE
        })
    }
    pub fn timestamp_style(&self, text: String) -> RichText {
        let color = Self::color_hex(&self.settings.style.timestamp_color);
        RichText::new(text)
            .color(color)
            .size(self.settings.style.timestamp_font_size)
    }
}

impl SettingsWindow {
    pub fn update_window_input(&mut self, input: &mut egui::RawInput) {
        if let Some(rect) = input.viewport().inner_rect {
            self.settings.window.height = rect.height();
            self.settings.window.width = rect.width();
        }
        if let Some(rect) = input.viewport().outer_rect {
            self.settings.window.x = rect.min.x;
            self.settings.window.y = rect.min.y;
        }
    }
    pub fn timestamp_style_edit(&mut self, ui: &mut egui::Ui) {
        let mut color = Self::color_hex(&self.settings.style.timestamp_color);
        ui.label("Timestamp Color");
        if ui.color_edit_button_srgba(&mut color).changed() {
            self.settings.style.timestamp_color = color.to_hex();
        }
        ui.end_row();

        ui.label("Timestamp Font Size");
        ui.add(egui::DragValue::new(&mut self.settings.style.timestamp_font_size).speed(0.1));
        ui.end_row();
    }
    pub fn render(&mut self, ctx: &Context) {
        if self.is_visible {
            Window::new("Settings").show(ctx, |ui| {
                let ui_builder = egui::UiBuilder::new();
                ui.scope_builder(ui_builder, |ui| {
                    ui.with_layout(Layout::top_down(eframe::egui::Align::Center), |ui| {
                        egui::Grid::new("my_grid")
                            .num_columns(2)
                            .spacing([40.0, 4.0])
                            .striped(true)
                            .show(ui, |ui| {
                                ui.label("Window Level");
                                self.window_level_combobox(ui);
                                ui.end_row();

                                self.timestamp_style_edit(ui);
                                self.theme_select(ui);
                            });

                        ui.horizontal(|ui| {
                            if ui.add(Button::new("Close")).clicked() {
                                self.is_visible = false;
                            }
                            if ui.add(Button::new("Save")).clicked() {
                                self.save_settings();
                            }
                        });
                    });
                });
            });
        }
    }
    fn theme_select(&mut self, ui: &mut Ui) {
        ui.label("Theme");
        let mut value = self.settings.theme.clone();
        ui.horizontal(|ui| {
            ui.radio_value(&mut value, Theme::Dark, "Dark");
            ui.radio_value(&mut value, Theme::Light, "Light");
            ui.radio_value(&mut value, Theme::System, "System");
        });
        ui.end_row();
        if value != self.settings.theme {
            self.settings.theme = value;
            ui.ctx().set_theme(value);
        }
    }
    fn window_level_combobox(&mut self, ui: &mut Ui) {
        let level: &mut WindowLevel = &mut self.settings.window.level.clone();
        let is_hover = ComboBox::from_label("")
            .selected_text(format!("{:#?}", level))
            .show_ui(ui, |ui| {
                ui.selectable_value(level, WindowLevel::Normal, "Normal");
                ui.selectable_value(level, WindowLevel::AlwaysOnTop, "AlwaysOnTop");
                ui.selectable_value(level, WindowLevel::AlwaysOnBottom, "AlwaysOnBottom");
            })
            .inner
            .is_some();
        if is_hover {
            if level != &self.settings.window.level {
                self.settings.window.level = level.clone();
                ui.ctx()
                    .send_viewport_cmd(ViewportCommand::WindowLevel(level.clone().into()));
            }
        }
    }
}
