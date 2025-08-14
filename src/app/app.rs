use std::fmt::Debug;

use winapi::um::winuser::MONITORINFO;

use crate::app::config::AppConfig;

pub struct App {
    pub init: bool,
    pub exit: bool,
    pub window_size: [u32;2],
    pub window_pos: [i32;2],
    pub config: AppConfig,
    pub game_hwnd: winapi::shared::windef::HWND,
    pub game_proc: proc_mem::Process,
    pub device_state: device_query::DeviceState,
    pub toggle_key: device_query::Keycode,
    pub visible: bool,
    pub monitor_info: MONITORINFO,
    pub debug: String,
}

impl App {
    pub fn debug<T: Debug>(&mut self, content: T) {
        self.debug = format!("{:?}",content)
    }
}