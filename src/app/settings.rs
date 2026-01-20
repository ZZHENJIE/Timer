use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Deserialize, Serialize, Clone, Copy)]
pub enum WindowLevel {
    Normal,
    AlwaysOnBottom,
    AlwaysOnTop,
}

impl From<WindowLevel> for iced::window::Level {
    fn from(level: WindowLevel) -> Self {
        match level {
            WindowLevel::Normal => iced::window::Level::Normal,
            WindowLevel::AlwaysOnBottom => iced::window::Level::AlwaysOnBottom,
            WindowLevel::AlwaysOnTop => iced::window::Level::AlwaysOnTop,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Window {
    pub width: f32,
    pub height: f32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub level: WindowLevel,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Settings {
    pub window: Window,
}

impl Default for Settings {
    fn default() -> Self {
        let bytes = std::fs::read("default.toml").unwrap_or_else(|err| {
            info!("Failed to read default.toml file: {}", err);
            panic!("{}", err);
        });
        toml::from_slice(&bytes).unwrap_or_else(|err| {
            info!("Failed to parse default.toml file: {}", err);
            panic!("{}", err);
        })
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
        let string = toml::to_string(&self).unwrap_or_else(|err| {
            info!("Failed to serialize settings: {}", err);
            panic!("{}", err);
        });
        let _ = std::fs::write("settings.toml", string).unwrap_or_else(|err| {
            info!("Failed to write settings.toml: {}", err);
            panic!("{}", err);
        });
    }
    pub fn window(&self) -> iced::window::Settings {
        let mut default = iced::window::Settings::default();
        default.level = self.window.level.into();
        default
    }
    pub fn settings(&self) -> iced::Settings {
        iced::Settings::default()
    }
}
