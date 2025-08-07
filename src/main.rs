#![windows_subsystem = "windows"]
//no console window

mod utils;
mod models;
mod math;
mod app;

use app::app::App;


fn main() {
    let proc = proc_mem::Process::with_name("Code.exe").unwrap();

    let hwnd = utils::windows::get_main_window_from_process_id(proc.process_id).unwrap();

    let app = App {
        init: false,
        exit: false,
        window_size: [0;2],
        window_pos: [0;2],
        game_hwnd: hwnd,
        game_proc: proc,
    };

    egui_overlay::start(app);
}
