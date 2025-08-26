use struct_iterable::Iterable;

use crate::app::visuals::BoxType;

#[derive(Iterable)]
pub struct Config {
    pub toggle_key: Option<device_query::Keycode>,
    pub aim_key: Option<device_query::Keycode>,
    pub esp_enabled: bool,
    pub esp_distance: f32,
    pub esp_box_type: BoxType,
    pub esp_color: egui::Color32,
    pub aim_enabled: bool,
    pub aim_smoothness: f64,
    pub time_dilation: f32, // New field for time dilation
}

impl Default for Config {
    fn default() -> Self {
        Config {
            toggle_key: None,
            aim_key: None,
            esp_enabled: true,
            esp_distance: 10000.0,
            esp_box_type: BoxType::Outline,
            esp_color: egui::Color32::from_rgb(255, 0, 0),
            aim_enabled: true,
            aim_smoothness: 0.1,
            time_dilation: 1.0, // Default value for time dilation
        }
    }
}