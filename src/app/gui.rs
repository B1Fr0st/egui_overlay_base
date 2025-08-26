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
            ui.label(self.debug.clone());
        });
        
    }
}