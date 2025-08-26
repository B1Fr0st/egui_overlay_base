use device_query::DeviceQuery;
use egui::color_picker::{color_picker_color32, Alpha};

impl crate::app::app::App{
    pub fn gui(&mut self,context: &egui::Context){
        //.default_pos(Pos2::new(1000.0,1000.0))
        egui::Window::new("Misc")
        .title_bar(false)
        .resizable(false)
        .default_pos([300.0,1000.0])
        .show(context, |ui| {
            ui.set_width(300.0);
            ui.label("Made by B1Fr0st");
            if ui.button("exit").clicked(){
                self.exit = true;
            }
            if ui.button("bind toggle key").clicked(){
                if let Some(key) = self.keys.first() {
                    self.toggle_key = Some(*key);
                    self.add_debug(format!("Toggle key set to: {:?}", self.toggle_key));
                } else {
                    self.add_debug("No keys pressed to bind toggle key.".to_string());
                }
            }
            ui.label(self.debug.clone());
        });

        egui::Window::new("Config")
        .title_bar(false)
        .resizable(false)
        .default_pos([600.0,1000.0])
        .show(context, |ui| {
            ui.checkbox(&mut self.config.esp_enabled, "ESP Enabled");
            if self.config.esp_enabled {
                ui.add(egui::Slider::new(&mut self.config.esp_distance, 0.0..=10000.0).text("ESP Distance"));
                color_picker_color32(ui, &mut self.config.esp_color, Alpha::Opaque);
                ui.selectable_value(&mut self.config.esp_box_type, crate::app::visuals::BoxType::Outline, "Outline");
                ui.selectable_value(&mut self.config.esp_box_type, crate::app::visuals::BoxType::Corners, "Corners");
            }
            ui.checkbox(&mut self.config.aim_enabled, "Aimbot Enabled");
            if self.config.aim_enabled {
                ui.add(egui::Slider::new(&mut self.config.aim_smoothness, 0.0..=1.0).text("Aim Smoothness"));
                if ui.button("bind aim key").clicked() {
                    if let Some(key) = self.keys.first() {
                        self.config.aim_key = Some(*key);
                        self.add_debug(format!("Aim key set to: {:?}", self.config.aim_key));
                    } else {
                        self.add_debug("No keys pressed to bind aim key.".to_string());
                    }
                }
            }
            ui.add(egui::Slider::new(&mut self.config.time_dilation, 0.1..=50.0).text("Time Dilation"));
        });
        
    }
}