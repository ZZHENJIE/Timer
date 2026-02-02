use crate::utils::{format_timestamp_to_local, settings_window::SettingsWindow};
use eframe::egui::{self, Context};
use egui_inbox::UiInbox;
use std::time::Duration;
use tracing::info;

pub struct App {
    timestamp: i64,
    inbox_timestamp: UiInbox<i64>,
    settings_window: SettingsWindow,
}

impl App {
    pub fn new(ctx: &Context) -> Self {
        let app = Self {
            timestamp: 0,
            settings_window: SettingsWindow::new(),
            inbox_timestamp: UiInbox::new(),
        };
        app.init(ctx);
        app
    }
}

impl App {
    pub fn init(&self, ctx: &Context) {
        self.settings_window.init(ctx);

        let sender = self.inbox_timestamp.sender().clone();
        let _ = tokio::spawn(async move {
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
            let mut time = match response_text.parse::<i64>() {
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
        });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let ui_builder = egui::UiBuilder::new();
            ui.scope_builder(ui_builder, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                    ui.label(
                        self.settings_window
                            .timestamp_style(format_timestamp_to_local(self.timestamp)),
                    );
                    if ui.button("Settings").clicked() {
                        self.settings_window.open();
                    }
                });
            });
            if let Some(response) = self.inbox_timestamp.read(ui).last() {
                self.timestamp = response;
            };
        });

        self.settings_window.render(ctx);
    }
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        self.settings_window.save_settings();
    }
    fn raw_input_hook(&mut self, _ctx: &egui::Context, _raw_input: &mut egui::RawInput) {
        self.settings_window.update_window_input(_raw_input);
    }
}
