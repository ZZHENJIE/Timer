use crate::{
    settings::{Theme, WindowLevel},
    utils::string_to_color_hex,
};
use eframe::egui::{self, Button, ComboBox, Layout, Ui, Vec2, ViewportCommand, Window, pos2};

pub struct SettingsWindow {}

impl SettingsWindow {
    pub fn new(ctx: &eframe::egui::Context, settings: &crate::Settings) -> Self {
        ctx.set_theme(settings.theme);
        ctx.send_viewport_cmd(ViewportCommand::InnerSize(Vec2::new(
            settings.window.width,
            settings.window.height,
        )));
        ctx.send_viewport_cmd(ViewportCommand::OuterPosition(pos2(
            settings.window.x,
            settings.window.y,
        )));
        ctx.send_viewport_cmd(ViewportCommand::WindowLevel(settings.window.level.into()));
        Self {}
    }
    pub fn update(&mut self, ui: &mut eframe::egui::Ui, settings: &mut crate::Settings) -> bool {
        let mut is_hide = false;
        Window::new("Settings").show(ui.ctx(), |ui| {
            let ui_builder = egui::UiBuilder::new();
            ui.scope_builder(ui_builder, |ui| {
                ui.with_layout(Layout::top_down(eframe::egui::Align::Center), |ui| {
                    egui::Grid::new("my_grid")
                        .num_columns(2)
                        .spacing([40.0, 4.0])
                        .striped(true)
                        .show(ui, |ui| {
                            ui.label("Window Level");
                            self.window_level_combobox(ui, settings);
                            ui.end_row();

                            self.timestamp_style_edit(ui, settings);
                            self.theme_select(ui, settings);
                        });

                    ui.horizontal(|ui| {
                        if ui.add(Button::new("Close")).clicked() {
                            is_hide = true;
                        }
                    });
                });
            });
        });
        is_hide
    }
}

impl SettingsWindow {
    fn timestamp_style_edit(&mut self, ui: &mut egui::Ui, settings: &mut crate::Settings) {
        let mut color = string_to_color_hex(&settings.style.timestamp_color);
        ui.label("Timestamp Color");
        if ui.color_edit_button_srgba(&mut color).changed() {
            settings.style.timestamp_color = color.to_hex();
        }
        ui.end_row();

        ui.label("Timestamp Font Size");
        ui.add(egui::DragValue::new(&mut settings.style.timestamp_font_size).speed(0.1));
        ui.end_row();
    }
    fn theme_select(&mut self, ui: &mut Ui, settings: &mut crate::Settings) {
        ui.label("Theme");
        let mut value = settings.theme.clone();
        ui.horizontal(|ui| {
            ui.radio_value(&mut value, Theme::Dark, "Dark");
            ui.radio_value(&mut value, Theme::Light, "Light");
            ui.radio_value(&mut value, Theme::System, "System");
        });
        ui.end_row();
        if value != settings.theme {
            settings.theme = value;
            ui.ctx().set_theme(value);
        }
    }
    fn window_level_combobox(&mut self, ui: &mut Ui, settings: &mut crate::Settings) {
        let level: &mut WindowLevel = &mut settings.window.level.clone();
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
            if level != &settings.window.level {
                settings.window.level = level.clone();
                ui.ctx()
                    .send_viewport_cmd(ViewportCommand::WindowLevel(level.clone().into()));
            }
        }
    }
}
