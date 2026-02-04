use eframe::egui::RichText;
use egui_inbox::UiInbox;
use std::time::Duration;
use tracing::info;

use crate::{
    settings::Style,
    utils::{format_timestamp_to_local, string_to_color_hex},
};

pub struct Content {
    timestamp: i64,
    inbox_timestamp: UiInbox<i64>,
    settings_window_is_visible: bool,
}

impl Content {
    fn timestamp_render(&self, style: &Style) -> RichText {
        RichText::new(format_timestamp_to_local(self.timestamp))
            .color(string_to_color_hex(&style.timestamp_color))
            .size(style.timestamp_font_size)
    }
}

impl Content {
    pub fn new(_: &eframe::egui::Context, _: &crate::Settings) -> Self {
        let value = Self {
            timestamp: 0,
            inbox_timestamp: UiInbox::new(),
            settings_window_is_visible: false,
        };
        let _ = value.start();
        value
    }
    pub fn update(&mut self, ui: &mut eframe::egui::Ui, settings: &mut crate::Settings) -> bool {
        if ui
            .label(self.timestamp_render(&settings.style))
            .clicked_by(eframe::egui::PointerButton::Secondary)
        {
            self.settings_window_is_visible = true;
        }
        if let Some(response) = self.inbox_timestamp.read(ui).last() {
            self.timestamp = response;
        };
        self.settings_window_is_visible
    }
    pub fn close_settings_window(&mut self) {
        self.settings_window_is_visible = false;
    }
}

impl Content {
    fn start(&self) -> tokio::task::JoinHandle<()> {
        let sender = self.inbox_timestamp.sender().clone();
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
        })
    }
}
