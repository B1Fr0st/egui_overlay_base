use crate::{app::app::App, models::vector2::Vector2, app::visuals::BoxType};

impl App{
    pub fn mock_esp(&self, painter: egui::Painter){
        self.draw_box(
            &painter,
            BoxType::Corners,
            Vector2 {
                x: 2000.0,
                y: 200.0
            },
            50.0,
            100.0,
            egui::Color32::WHITE
        );
        self.draw_box(
            &painter,
            BoxType::Outline,
            Vector2 {
                x: 1500.0,
                y: 200.0
            },
            50.0,
            100.0,
            egui::Color32::WHITE
        )
    }
}