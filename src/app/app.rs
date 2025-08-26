use std::{fmt::Debug, sync::Mutex};

use winapi::{shared::windef::HWND, um::winuser::MONITORINFO};

use crate::app::main::{AActorData, Offsets};

pub struct App {
    pub init: bool,
    pub exit: bool,
    pub window_size: [u32;2],
    pub window_pos: [i32;2],
    pub game_hwnd: winapi::shared::windef::HWND,
    pub game_proc: proc_mem::Process,
    pub device_state: device_query::DeviceState,
    pub keys: Vec<device_query::Keycode>,
    pub toggle_key: Option<device_query::Keycode>,
    pub visible: bool,
    pub monitor_info: MONITORINFO,
    pub debug: String,

    pub dsapi: dumpspace_api::DSAPI,

    pub fname_cache: std::collections::HashMap<i32, String>,
    pub config: crate::app::config::Config,
    pub offsets: crate::app::main::Offsets,
    pub aactors: Mutex<Vec<AActorData>>,
}


impl App {
    pub fn new(hwnd: HWND,proc:proc_mem::Process) -> Self {
        App {
            init: false,
            exit: false,
            visible: true,
            window_size: [0;2],
            window_pos: [0;2],
            game_hwnd: hwnd,
            game_proc: proc,
            device_state: device_query::DeviceState::new(),
            keys: Vec::new(),
            toggle_key: None,
            monitor_info: unsafe { std::mem::zeroed() },
            debug: String::new(),
            dsapi: {let mut dsapi = dumpspace_api::DSAPI::new("c87411ed",None);dsapi.download_content();dsapi},
            fname_cache: std::collections::HashMap::new(),
            config: crate::app::config::Config::default(),
            aactors: Mutex::new(Vec::new()),
            offsets: Offsets::default(),
        }
    }
    pub fn debug<T: Debug>(&mut self, content: T) {
        self.debug = format!("Debug: {:?}",content)
    }
    pub fn add_debug<T: Debug>(&mut self, content: T) {
        self.debug += format!("\nDebug: {:?}",content).as_str();
    }
    pub fn hex_debug<T: Debug>(&mut self, content: T, name:&str) {
        self.debug += format!("\n{}: {:x?}",name, content).as_str();
    }
}