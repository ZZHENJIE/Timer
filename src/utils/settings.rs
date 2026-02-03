use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Deserialize, Serialize, Clone, Copy, PartialEq)]
pub enum Theme {
    Light,
    Dark,
    System,
}

#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq)]
pub enum WindowLevel {
    Normal,
    AlwaysOnBottom,
    AlwaysOnTop,
}

#[derive(Deserialize, Serialize)]
pub struct Window {
    pub width: f32,
    pub height: f32,
    pub x: f32,
    pub y: f32,
    pub level: WindowLevel,
}

#[derive(Deserialize, Serialize)]
pub struct Style {
    pub timestamp_color: String,
    pub timestamp_font_size: f32,
    pub info_color: String,
    pub info_font_size: f32,
}

#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub window: Window,
    pub style: Style,
    pub theme: Theme,
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

impl Into<eframe::egui::WindowLevel> for WindowLevel {
    fn into(self) -> eframe::egui::WindowLevel {
        match self {
            WindowLevel::Normal => eframe::egui::WindowLevel::Normal,
            WindowLevel::AlwaysOnBottom => eframe::egui::WindowLevel::AlwaysOnBottom,
            WindowLevel::AlwaysOnTop => eframe::egui::WindowLevel::AlwaysOnTop,
        }
    }
}

impl Into<eframe::egui::ThemePreference> for Theme {
    fn into(self) -> eframe::egui::ThemePreference {
        match self {
            Theme::Dark => eframe::egui::ThemePreference::Dark,
            Theme::Light => eframe::egui::ThemePreference::Light,
            Theme::System => eframe::egui::ThemePreference::System,
        }
    }
}
