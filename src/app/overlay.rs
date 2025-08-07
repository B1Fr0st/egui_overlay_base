use egui_overlay::EguiOverlay;
use egui::{Color32, LayerId, Pos2, Rect, Vec2};
use winapi::um::winuser::{GetMonitorInfoA, GetWindowRect, IsWindow, MonitorFromWindow, SetWindowLongA, ShowWindow, GWL_EXSTYLE, MONITORINFO, MONITOR_DEFAULTTONEAREST, SW_HIDE, SW_SHOW, WS_EX_TOOLWINDOW };

use crate::app::app::App;

impl EguiOverlay for App {
    fn gui_run(
            &mut self,
            egui_context: &egui::Context,
            _default_gfx_backend: &mut egui_render_three_d::ThreeDBackend,
            glfw_backend: &mut egui_window_glfw_passthrough::GlfwBackend,
        ) {
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

            glfw_backend.window.set_title("FPSChess Cheat");
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

        egui_context.set_visuals(visuals);

        //windows go here!!
        egui::Window::new("Placeholder").show(egui_context, |ui| {
            ui.set_width(300.0);
            ui.label("Made by B1Fr0st");
            if ui.button("exit").clicked(){
                self.exit = true;
            }
        });

        //main game logic loop
        let monitor = unsafe { MonitorFromWindow(self.game_hwnd, MONITOR_DEFAULTTONEAREST) };
        let mut monitor_info: MONITORINFO = unsafe { std::mem::zeroed() };
        monitor_info.cbSize = std::mem::size_of::<MONITORINFO>() as u32;
        unsafe { GetMonitorInfoA(monitor, &mut monitor_info as *mut MONITORINFO) };
        let painter = egui::Painter::new(
            egui_context.clone(),
            LayerId::debug(),
            Rect{
                min: Pos2 {
                    x:0.0,
                    y:0.0,
                },
                max: Pos2 {
                    x:(monitor_info.rcMonitor.right - monitor_info.rcMonitor.left) as f32,
                    y:(monitor_info.rcMonitor.bottom - monitor_info.rcMonitor.top) as f32,
                }
            },
        );
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