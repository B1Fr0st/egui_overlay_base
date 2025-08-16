use eframe::egui::{self, include_image, vec2, Color32, ImageButton, Pos2, Rect, RichText, Stroke, Style};
use core::f32;
use std::sync::mpsc;


#[derive(Default, Debug)]
pub(crate) struct MyApp {
    pub ui_state: UiState,
    frame: u64,
    load: bool,
    pub license: String,
    pub failed_reason: String,
    // Channel for async license verification
    pub license_receiver: Option<mpsc::Receiver<LicenseResult>>,
}
#[derive(Default, PartialEq, Debug)]
pub enum UiState{
    Verifying,
    #[default]
    LicenseInput,
    Verified,
    Error
}

// Result type for license verification
#[derive(Debug, Clone)]
pub enum LicenseResult {
    Success,
    Error(String),
}

fn hsv_to_color32(h: f32, s: f32, v: f32) -> Color32 {
    let c = v * s; // Chroma
    let h_prime = h / 60.0;
    let x = c * (1.0 - ((h_prime % 2.0) - 1.0).abs());
    let m = v - c;
    
    let (r_prime, g_prime, b_prime) = if h_prime >= 0.0 && h_prime < 1.0 {
        (c, x, 0.0)
    } else if h_prime >= 1.0 && h_prime < 2.0 {
        (x, c, 0.0)
    } else if h_prime >= 2.0 && h_prime < 3.0 {
        (0.0, c, x)
    } else if h_prime >= 3.0 && h_prime < 4.0 {
        (0.0, x, c)
    } else if h_prime >= 4.0 && h_prime < 5.0 {
        (x, 0.0, c)
    } else if h_prime >= 5.0 && h_prime < 6.0 {
        (c, 0.0, x)
    } else {
        (0.0, 0.0, 0.0) // Fallback for invalid hue
    };
    
    // Convert to 0-255 range
    let r = ((r_prime + m) * 255.0).round() as u8;
    let g = ((g_prime + m) * 255.0).round() as u8;
    let b = ((b_prime + m) * 255.0).round() as u8;
    
    Color32::from_rgb(r, g, b)
}

impl MyApp {
    fn color_cycle(&self) -> Color32{
        hsv_to_color32(self.frame as f32 % 360.0,1.0,1.0)
    }
}


impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui_extras::install_image_loaders(ctx);
        // Check for async license verification result
        self.check_license_result();
        self.frame += 1;


        let mut visuals = egui::Visuals::default();
        
        visuals.window_shadow = egui::epaint::Shadow {
            offset: [0,0],
            blur: 0,
            spread: 0,
            color: Color32::BLACK,
        };

        visuals.popup_shadow = egui::epaint::Shadow {
            offset: [0,0],
            blur: 0,
            spread: 0,
            color: Color32::BLACK,
        };

        visuals.widgets.hovered.bg_stroke = Stroke::new(0.1,Color32::BLACK);
        visuals.widgets.active.bg_stroke = Stroke::new(0.1,Color32::BLACK);
        visuals.widgets.inactive.bg_stroke = Stroke::new(0.1,Color32::BLACK);

        visuals.widgets.hovered.weak_bg_fill = Color32::from_rgb(110,0,0);
        visuals.widgets.inactive.weak_bg_fill = Color32::from_rgb(32, 16, 16);

        
        visuals.extreme_bg_color = Color32::BLACK;

        visuals.panel_fill = Color32::BLACK;

        visuals.window_stroke = egui::Stroke{width:0.5, color:Color32::from_rgb(54,1,63)};

        ctx.set_visuals(visuals);
        
        

        egui::CentralPanel::default().show(ctx, |ui| {
            let painter = ui.painter();

            painter.rect_filled(Rect {min:[100.,100.].into(),max:[150.,150.].into()}, 0.0, Color32::from_rgba_unmultiplied(65, 125, 125, 100));

            match self.ui_state{
                UiState::Verifying => {
                    ctx.style_mut(|s|{s.spacing.item_spacing = vec2(16.0, 64.0);s.spacing.indent=16.0;});
                    ui.vertical_centered(|ui| {
                        ui.label(RichText::new("Verifying license...").size(16.0).color(Color32::LIGHT_BLUE));
                        ui.add(egui::Spinner::new().size(50.0).color(self.color_cycle()));
                    });
                },
                UiState::LicenseInput => {
                    ctx.style_mut(|s|{*s = Style::default()});
                    // if ui.button("verifying").clicked(){
                    //     self.ui_state = UiState::Verifying;
                    // }
                    
                    if self.license_regex() {
                            self.verify_license_async();
                            self.ui_state = UiState::Verifying;
                        }
                    else {
                        ui.vertical_centered(|ui|{
                            ui.add_space(5.0);
                            ui.label(RichText::new("License Key").size(24.0).color(Color32::LIGHT_BLUE));
                            ui.add_space(10.0);
                            ui.add(egui::TextEdit::singleline(&mut self.license));
                            self.license = self.license.trim().to_string();
                            if !self.license.is_empty() && !self.license_regex() {
                                self.failed_reason = String::new();
                                ui.label(RichText::new("License not in correct format!").size(16.0).color(Color32::LIGHT_RED));
                            }
                            if ui.button("Exit").clicked() {
                                self.load = false;
                                ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                            }

                            let discord = ui.add_sized([50.0,50.0],ImageButton::new(include_image!("../../assets/discord.png")).frame(false));

                            if discord.clicked(){
                                ctx.open_url(egui::OpenUrl::new_tab( "https://google.com"));
                            }
                            if discord.hovered(){
                                discord.with_new_rect(Rect::from_two_pos(Pos2::new(0.0,0.0), Pos2::new(100.0,100.0)));
                            }

                        });
                    }
                    
                    if !self.failed_reason.is_empty() {
                        ui.label(RichText::new(format!("Failed: {}", self.failed_reason)).size(16.0).color(Color32::LIGHT_RED));
                    }
                },
                UiState::Verified => {
                    ctx.style_mut(|s|{*s = Style::default()});
                    if ui.button("Load").clicked() {
                        self.load = true;
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                    if ui.button("Exit").clicked() {
                        self.load = false;
                        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                },
                UiState::Error => {
                    ctx.style_mut(|s|{*s = Style::default()});
                    ui.vertical_centered(|ui|{
                        ui.add_space(50.0);
                        ui.label(RichText::new(self.failed_reason.clone()).size(24.0).color(Color32::DARK_RED));
                        ui.add_space(100.0);
                        if ui.button("Exit").clicked() {
                            self.load = false;
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                }
            }
        });

        if ctx.input(|i| i.viewport().close_requested()) && !self.load {
            std::process::exit(0);
        }
    }
}