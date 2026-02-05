use eframe::egui::RichText;
use egui_inbox::UiInbox;
use std::time::Duration;
use tracing::info;

use crate::{
    economy::finviz,
    settings::Style,
    utils::{format_timestamp_to_local, string_to_color_hex},
};

pub struct Content {
    timestamp: i64,
    inbox_timestamp: UiInbox<i64>,
    info: String,
    inbox_info: UiInbox<String>,
    settings_window_is_visible: bool,
}

impl Content {
    fn timestamp_render(&self, style: &Style) -> RichText {
        RichText::new(format_timestamp_to_local(self.timestamp))
            .color(string_to_color_hex(&style.timestamp_color))
            .size(style.timestamp_font_size)
    }
    fn info_render(&self, style: &Style) -> RichText {
        RichText::new(&self.info)
            .color(string_to_color_hex(&style.info_color))
            .size(style.info_font_size)
    }
}

impl Content {
    pub fn new(_: &eframe::egui::Context, _: &crate::Settings) -> Self {
        let value = Self {
            timestamp: 0,
            inbox_timestamp: UiInbox::new(),
            info: String::new(),
            inbox_info: UiInbox::new(),
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
        ui.label(self.info_render(&settings.style));
        if let Some(response) = self.inbox_timestamp.read(ui).last() {
            self.timestamp = response;
        };
        if let Some(response) = self.inbox_info.read(ui).last() {
            self.info = response;
        };
        self.settings_window_is_visible
    }
    pub fn close_settings_window(&mut self) {
        self.settings_window_is_visible = false;
    }
}

impl Content {
    fn start(&self) -> tokio::task::JoinHandle<()> {
        let timestamp_sender = self.inbox_timestamp.sender().clone();
        let info_sender = self.inbox_info.sender().clone();
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
            let economy = match finviz::fetch(&client, time).await {
                Ok(economy) => economy,
                Err(err) => {
                    info!("Error fetching economy data: {}", err);
                    return;
                }
            };
            timestamp_sender
                .send(time)
                .expect("Error sending timestamp");

            let mut interval = tokio::time::interval(Duration::from_secs(1));
            loop {
                interval.tick().await;
                let mut count = 0;
                let mut max_importance: u8 = 0;
                time += 1;
                timestamp_sender
                    .send(time)
                    .expect("Error sending timestamp");
                for item in &economy {
                    let value_difference = item.date_timestamp - time;
                    if value_difference > 0 && value_difference <= 300 {
                        count += 1;
                        if max_importance < item.importance {
                            max_importance = item.importance;
                        }
                    }
                }
                let info_result: String;
                if count > 0 {
                    info_result = format!("Count:{} Max:{}", count, max_importance);
                } else {
                    info_result = String::new();
                }
                info_sender.send(info_result).expect("Error sending info");
            }
        })
    }
}
