use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq)]
pub enum WindowLevel {
    Normal,
    AlwaysOnBottom,
    AlwaysOnTop,
}

impl Into<eframe::egui::WindowLevel> for WindowLevel {
    fn into(self) -> eframe::egui::WindowLevel {
        match self {
            WindowLevel::Normal => eframe::egui::WindowLevel::Normal,
            WindowLevel::AlwaysOnBottom => eframe::egui::WindowLevel::AlwaysOnBottom,
            WindowLevel::AlwaysOnTop => eframe::egui::WindowLevel::AlwaysOnTop,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Copy)]
pub struct Window {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub level: WindowLevel,
}

#[derive(Deserialize, Serialize, Clone, Copy)]
pub struct Settings {
    pub window: Window,
}

impl Default for Settings {
    fn default() -> Self {
        let bytes = std::fs::read("default.toml")
            .unwrap_or_else(|err| panic!("Failed to read default.toml file: {}", err));
        toml::from_slice(&bytes)
            .unwrap_or_else(|err| panic!("Failed to parse default.toml file: {}", err))
    }
}

impl Settings {
    pub fn new() -> Self {
        let bytes = match std::fs::read("settings.toml") {
            Ok(bytes) => bytes,
            Err(err) => {
                info!("Failed to read settings.toml file: {}", err);
                return Self::default();
            }
        };
        toml::from_slice(&bytes).unwrap_or_else(|err| {
            info!("Failed to parse settings.toml file: {}", err);
            Self::default()
        })
    }
    pub fn save(&self) {
        let string = toml::to_string(&self)
            .unwrap_or_else(|err| panic!("Failed to serialize settings: {}", err));
        let _ = std::fs::write("settings.toml", string)
            .unwrap_or_else(|err| panic!("Failed to write settings.toml: {}", err));
    }
}
