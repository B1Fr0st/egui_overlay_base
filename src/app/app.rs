pub struct App {
    pub init: bool,
    pub exit: bool,
    pub window_size: [u32;2],
    pub window_pos: [i32;2],
    pub game_hwnd: winapi::shared::windef::HWND,
    pub game_proc: proc_mem::Process,
}