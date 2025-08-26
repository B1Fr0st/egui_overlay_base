#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release



mod utils;
mod models;
mod math;
mod app;
mod loader;

use app::app::App;


fn main() {

    let proc = match proc_mem::Process::with_name("Code.exe") {
        Ok(process) => {process},
        Err(_) => {crate::loader::start::error("Game not running!");return;},
    };

    let hwnd = match utils::windows::get_main_window_from_process_id(proc.process_id) {
        Some(hwnd) => {hwnd},
        None => {crate::loader::start::error("Error: 0xH011");return;},
    };

    crate::loader::start::start();

    let app = App {
        init: false,
        exit: false,
        visible: true,
        window_size: [0;2],
        window_pos: [0;2],
        game_hwnd: hwnd,
        game_proc: proc,
        device_state: device_query::DeviceState::new(),
        toggle_key: device_query::Keycode::Insert,
        monitor_info: unsafe { std::mem::zeroed() },
        debug: String::new(),
    };

    egui_overlay::start(app);
}
