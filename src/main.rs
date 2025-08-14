//#![windows_subsystem = "windows"]
//no console window

mod utils;
mod models;
mod math;
mod app;
mod loader;

use app::app::App;
use eframe::egui::IconData;

use crate::app::config::AppConfig;


fn main() {

    let options = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
        .with_inner_size([320.0, 240.0])
        .with_decorations(false)
        .with_active(true)
        .with_taskbar(true)
        ,//.with_icon(IconData::default()),
        centered: true,
        ..Default::default()
    };
    eframe::run_native(
        "Replace me!!!!!!!",
        options,
        Box::new(|_cc| Ok(Box::<loader::app::MyApp>::default())),
    ).unwrap();

    let proc = proc_mem::Process::with_name("Code.exe").unwrap();

    let hwnd = utils::windows::get_main_window_from_process_id(proc.process_id).unwrap();

    let app = App {
        init: false,
        exit: false,
        visible: true,
        config: AppConfig::from_file("configs/config1.toml"),
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
