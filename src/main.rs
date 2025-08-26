#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release



mod utils;
mod models;
mod math;
mod app;
mod loader;

use app::app::App;


fn main() {

    let proc = match proc_mem::Process::with_name("SurrounDead-Win64-Shipping.exe") {
        Ok(process) => {process},
        Err(_) => {autherium_loader::loader::start::error("Game not running!");return;},
    };

    let hwnd = match utils::windows::get_main_window_from_process_id(proc.process_id) {
        Some(hwnd) => {hwnd},
        None => {autherium_loader::loader::start::error("Error: 0xH011");return;},
    };

    //autherium_loader::loader::start::start();

    let app = App::new(hwnd, proc);
    
    //start actor reading thread

    egui_overlay::start(app);
}
