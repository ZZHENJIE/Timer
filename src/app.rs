use crate::utils::settings::{Settings, WindowLevel};
use eframe::egui::{self, Vec2, ViewportCommand, Window, pos2};
use egui_inbox::{UiInbox, UiInboxSender};
use std::time::Duration;
use tracing::info;

pub struct App {
    timestamp: u64,
    inbox_timestamp: UiInbox<u64>,
    settings: Settings,
    visible_settings_window: bool,
}

impl App {
    pub fn new(settings: Settings) -> Self {
        Self {
            timestamp: 0,
            settings,
            visible_settings_window: false,
            inbox_timestamp: UiInbox::new(),
        }
    }
}

impl App {
    fn window_level_combobox(&mut self, ui: &mut egui::Ui) {
        let level: &mut WindowLevel = &mut self.settings.window.level.clone();
        let is_hover = egui::ComboBox::from_label("Level")
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
    pub fn init(&self, context: &eframe::egui::Context) {
        refresh_timestamp(self.inbox_timestamp.sender().clone());
        //init window
        let window_settings = &self.settings.window;
        context.send_viewport_cmd(ViewportCommand::InnerSize(Vec2::new(
            window_settings.width,
            window_settings.height,
        )));
        context.send_viewport_cmd(ViewportCommand::OuterPosition(pos2(
            window_settings.x,
            window_settings.y,
        )));
        context.send_viewport_cmd(ViewportCommand::WindowLevel(window_settings.level.into()));
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.label(format_timestamp_to_utc8(self.timestamp));
                ui.horizontal_centered(|ui| {
                    if ui.button("Refresh").clicked() {
                        refresh_timestamp(self.inbox_timestamp.sender().clone());
                    }
                    if ui.button("Settings").clicked() {
                        self.visible_settings_window = true
                    }
                    self.window_level_combobox(ui);
                });
            });
            if let Some(response) = self.inbox_timestamp.read(ui).last() {
                self.timestamp = response;
            };
        });
        Window::new("Settings")
            .open(&mut self.visible_settings_window)
            .show(ctx, |ui| ctx.settings_ui(ui));
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

fn refresh_timestamp(sender: UiInboxSender<u64>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let client = reqwest::Client::new();
        let response = match client.get("https://time.akamai.com").send().await {
            Ok(response) => response,
            Err(err) => {
                info!("Error fetching timestamp: {}", err);
                return;
            }
        };
        let response_text = match response.text().await {
            Ok(text) => text,
            Err(err) => {
                info!("Error fetching timestamp text: {}", err);
                return;
            }
        };
        let mut time = match response_text.parse::<u64>() {
            Ok(time) => time,
            Err(err) => {
                info!("Error parsing timestamp: {}", err);
                return;
            }
        };
        match sender.send(time) {
            Ok(()) => {
                let mut interval = tokio::time::interval(Duration::from_secs(1));
                loop {
                    interval.tick().await;
                    time += 1;
                    sender
                        .send(time)
                        .unwrap_or_else(|_| info!("Error sending timestamp"));
                }
            }
            Err(_) => {
                info!("Error sending timestamp");
            }
        };
    })
}

fn format_timestamp_to_utc8(timestamp: u64) -> String {
    // 转换为 UTC+8 (偏移 8 小时 = 28800 秒)
    let utc8_offset = 8 * 3600;
    let total_seconds = timestamp + utc8_offset;

    // 计算时区修正后的时间
    let hours = (total_seconds % 86400) / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}
