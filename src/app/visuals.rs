use crate::app::app::App;
use crate::models::vector2::Vector2;

pub enum BoxType{
    Outline,
    Corners
}

impl App {
    pub fn draw_line(&self, painter: &egui::Painter,start: Vector2, end: Vector2, color: egui::Color32) {
        painter.line_segment(
            [start.to_egui(), end.to_egui()],
            egui::Stroke {
                width: 1.0,
                color,
            },
        );
    }

    pub fn draw_box(&self, painter: &egui::Painter, box_type:BoxType, top_left: Vector2, width: f32, height: f32, color: egui::Color32) {

        match box_type {
            BoxType::Outline => {
                let top_right = Vector2 {
                    x: top_left.x + width,
                    y: top_left.y,
                };
                let bottom_left = Vector2 {
                    x: top_left.x,
                    y: top_left.y + height,
                };
                let bottom_right = Vector2 {
                    x: top_left.x + width,
                    y: top_left.y + height,
                };

                self.draw_line(painter, top_left, top_right, color);
                self.draw_line(painter, top_right, bottom_right, color);
                self.draw_line(painter, bottom_right, bottom_left, color);
                self.draw_line(painter, bottom_left, top_left, color);
            }
            // 2, almost the same as 1 but the connection lines have some space between them
            BoxType::Corners => {
                let top_right = Vector2 {
                    x: top_left.x + width,
                    y: top_left.y,
                };
                let bottom_left = Vector2 {
                    x: top_left.x,
                    y: top_left.y + height,
                };
                let bottom_right = Vector2 {
                    x: top_left.x + width,
                    y: top_left.y + height,
                };

                let line_len = width / 2.5;

                // Top horizontal lines
                self.draw_line(
                    painter,
                    top_left,
                    Vector2 {
                        x: top_left.x + line_len,
                        y: top_left.y,
                    },
                    color
                );
                self.draw_line(
                    painter,
                    Vector2 {
                        x: top_right.x - line_len,
                        y: top_right.y,
                    },
                    top_right,
                    color
                );

                // Bottom horizontal lines
                self.draw_line(
                    painter,
                    bottom_left,
                    Vector2 {
                        x: bottom_left.x + line_len,
                        y: bottom_left.y,
                    },
                    color
                );
                self.draw_line(
                    painter,
                    Vector2 {
                        x: bottom_right.x - line_len,
                        y: bottom_right.y,
                    },
                    bottom_right,
                    color
                );

                // Left vertical lines
                self.draw_line(
                    painter,
                    top_left,
                    Vector2 {
                        x: top_left.x,
                        y: top_left.y + line_len,
                    },
                    color
                );
                self.draw_line(
                    painter,
                    Vector2 {
                        x: bottom_left.x,
                        y: bottom_left.y - line_len,
                    },
                    bottom_left,
                    color
                );

                // Right vertical lines
                self.draw_line(
                    painter,
                    top_right,
                    Vector2 {
                        x: top_right.x,
                        y: top_right.y + line_len,
                    },
                    color
                );
                self.draw_line(
                    painter,
                    Vector2 {
                        x: bottom_right.x,
                        y: bottom_right.y - line_len,
                    },
                    bottom_right,
                    color
                );
            }
        }
    }
    
}