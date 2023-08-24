use eframe::egui::{self};
use native_dialog::FileDialog;
use screenshots::Screen;
use std::{fs, time::Duration};
use chrono::prelude::*;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        maximized: true,
        decorated: true,
        transparent: true,
        resizable: false,
        ..Default::default()
    };
    
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

#[derive(Default, Clone)]
struct MyApp {
    screen_rect: RectangleCrop,
    window_hidden: u8,
    delay: u64,
}

#[derive(Default, Clone)]
struct RectangleCrop {
    x_left: f32,
    y_left: f32,
    width: f32,
    height: f32
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::Window::new("Screenshot").show(ctx, |ui| {

            if self.window_hidden != 0 {
                if self.delay == 0 {self.delay += 1;}
                std::thread::sleep(Duration::from_secs(self.delay));
                let coord = self.screen_rect.clone();
                let screen = Screen::all().unwrap()[0];
                let image;
                if self.window_hidden == 1 {
                    image = screen.capture_area(
                        coord.x_left.floor() as i32, 
                        coord.y_left.floor() as i32, 
                    coord.width.floor() as u32, 
                    coord.height.floor() as u32).unwrap();
                }
                else {
                    image = screen.capture().unwrap();
                }
                // let t = std::thread::spawn(move || {
                    let buffer = Some(image.to_png(None).unwrap());
                    let today = Utc::now().to_string()
                        .replace("-", "")
                        .replace(":", "_")
                        .replace(" ", "")
                        .to_string();
                    let concatenated_string = format!("screenshot_{}", today);
                    let result = FileDialog::new()
                        .set_filename(&concatenated_string[..27])
                        .add_filter("PNG Image", &["png"])
                        .add_filter("JPEG Image", &["jpg", "jpeg"])
                        .add_filter("GIF Image", &["gif"])
                        .show_save_single_file()
                        .unwrap();
                    match result {
                        Some(result) => {
                            fs::write(result.clone(), buffer.clone().unwrap()).unwrap();
                        }
                        None => (), 
                    };
                // });
                    self.window_hidden = 0;
                    frame.set_visible(true);
                // t.join().and_then(|()| Ok({self.window_hidden = 0;})).unwrap();

            }
            ui.horizontal(|ui|{
                if ui.button("Take screenshot").clicked() {

                    frame.set_visible(false);
                    self.window_hidden = 1;   
                    
                }
                if ui.button("Whole screen").clicked() {

                    frame.set_visible(false);
                    self.window_hidden = 2;

            }});

            ui.add(egui::Slider::new(&mut self.delay, 0..=120).text("delay (s)"));
            
        });
        let w = egui::Window::new("ciao")
            .title_bar(false)
            .default_size(egui::vec2(320.0, 240.0))
            .resizable(true)
            .frame(egui::Frame {
                fill: egui::Color32::TRANSPARENT,
                stroke: egui::Stroke::new(1.5, egui::Color32::DARK_GRAY),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.allocate_space(ui.available_size());
            });

        let r = w.unwrap().response.rect;
        self.screen_rect = RectangleCrop { x_left: r.left(), y_left: r.top(), width: r.width(), height: r.height() }
    }   

    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        // to make the background of the app completely transparent
        egui::Rgba::TRANSPARENT.to_array()
    } 
}
