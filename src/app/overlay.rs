use device_query::DeviceQuery;
use egui_overlay::EguiOverlay;
use egui::{Color32, LayerId, Pos2, Rect, Rounding, Vec2};
use winapi::um::winuser::{GetMonitorInfoA, GetWindowRect, IsWindow, MonitorFromWindow, SetWindowLongA, ShowWindow, GWL_EXSTYLE, MONITORINFO, MONITOR_DEFAULTTONEAREST, SW_HIDE, SW_SHOW, WS_EX_TOOLWINDOW };

use crate::app::app::App;

impl EguiOverlay for App {
    fn gui_run(
            &mut self,
            egui_context: &egui::Context,
            _default_gfx_backend: &mut egui_render_three_d::ThreeDBackend,
            glfw_backend: &mut egui_window_glfw_passthrough::GlfwBackend,
        ) {
        
        let keys = self.device_state.get_keys();
        if keys.contains(&self.toggle_key){
            self.visible = !self.visible;
            std::thread::sleep(std::time::Duration::from_millis(200));
        }

        //if game is closed or exit is set, quit
        if self.exit || unsafe{IsWindow(self.game_hwnd) == 0} {
            glfw_backend.window.set_should_close(true);
            return;
        }

        //if neither game window nor overlay window is focused, return
        let foreground_window = unsafe { winapi::um::winuser::GetForegroundWindow() };
        if foreground_window != self.game_hwnd  && foreground_window != glfw_backend.window.get_win32_window() as winapi::shared::windef::HWND {
            return;
        }

        // Set window size to match game window size (x axis+1 to avoid glfw passthrough blackout bug)
        // basically if a transparent glfw window is focused and fullscreen, it won't be transparent
        // but if we set the size to be slightly larger than the monitor size, it wont count as fullscreen
        // so we just always set it to be slightly larger than the game window size.
        // this should always work unless someone has a game window that is exactly their monitor size but
        // -1 on the x-axis, and if that happens thats a them problem :nod:
        let mut window_rect = winapi::shared::windef::RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        }; unsafe { GetWindowRect(self.game_hwnd, &mut window_rect)};

        self.window_size = [(window_rect.right - window_rect.left) as u32 + 1, (window_rect.bottom - window_rect.top) as u32];
        self.window_pos = [window_rect.left, window_rect.top];

        glfw_backend.window_size_virtual = self.window_size;
        glfw_backend.window.set_size(self.window_size[0] as i32, self.window_size[1] as i32);
        glfw_backend.window.set_pos(self.window_pos[0], self.window_pos[1]);

        //hide overlay from taskbar and alt+tab list
        if !self.init {
            self.init = true;
            unsafe { ShowWindow(glfw_backend.window.get_win32_window() as winapi::shared::windef::HWND, SW_HIDE) };
            unsafe { SetWindowLongA(glfw_backend.window.get_win32_window() as winapi::shared::windef::HWND, GWL_EXSTYLE, WS_EX_TOOLWINDOW as i32) };
            unsafe { ShowWindow(glfw_backend.window.get_win32_window() as winapi::shared::windef::HWND, SW_SHOW) };
        }

        //make it so that windows won't have drop shadow behind them (could look cool in future?)
        let mut visuals = egui::Visuals::default();

        visuals.window_shadow = egui::epaint::Shadow {
            offset: Vec2::default(),
            blur: 0.0,
            spread: 0.0,
            color: Color32::from_rgb(0, 0, 0),
        };

        visuals.popup_shadow = egui::epaint::Shadow {
            offset: Vec2::default(),
            blur: 0.0,
            spread: 0.0,
            color: Color32::from_rgb(0, 0, 0),
        };

        visuals.window_rounding = Rounding::same(0.0);

        visuals.window_fill = Color32::BLACK;
        visuals.window_stroke = egui::Stroke { width: 0.5, color: Color32::DARK_BLUE };

        visuals.override_text_color = Some(Color32::WHITE);

        egui_context.set_visuals(visuals);

        if self.visible{
            self.gui(egui_context);
        }

        //make sure painter obj is properly sized (probably not super necessary to run every tick?)
        let monitor = unsafe { MonitorFromWindow(self.game_hwnd, MONITOR_DEFAULTTONEAREST) };
        self.monitor_info = unsafe { std::mem::zeroed() };
        self.monitor_info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
        unsafe { GetMonitorInfoA(monitor, &mut self.monitor_info as *mut MONITORINFO) };
        let painter = egui::Painter::new(
            egui_context.clone(),
            LayerId::debug(),
            Rect{
                min: Pos2 {
                    x:0.0,
                    y:0.0,
                },
                max: Pos2 {
                    x:(self.monitor_info.rcMonitor.right - self.monitor_info.rcMonitor.left) as f32,
                    y:(self.monitor_info.rcMonitor.bottom - self.monitor_info.rcMonitor.top) as f32,
                }
            },
        );
        //main game logic loop goes here!!!!
        self.debug(format!("game base address: 0x{:x}",self.game_proc.process_base_address));

        self.mock_esp(painter.clone());

        //set passthrough enabling and request egui_repaint
        if egui_context.wants_pointer_input() || egui_context.wants_keyboard_input() {
            glfw_backend.set_passthrough(false);
        } else {
            glfw_backend.set_passthrough(true)
        }
        egui_context.request_repaint();
    }
}