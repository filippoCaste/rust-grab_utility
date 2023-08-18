use eframe::egui::{self, Image};
use native_dialog::FileDialog;
use screenshots::Screen;
use std::fs;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        fullscreen: true,
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

#[derive(Default)]
struct MyApp {
    image: Option<Image>,
    buffer: Option<Vec<u8>>,
    screen_rect: RectangleCrop
}

#[derive(Default)]
struct RectangleCrop {
    x_left: f32,
    y_left: f32,
    width: f32,
    height: f32
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::Window::new("Screenshot").show(ctx, |ui| {
            if ui.button("Take screenshot").clicked() {
                                ui.set_visible(false);
                let screen = Screen::all().unwrap()[0];
                let image = screen.capture_area(self.screen_rect.x_left.floor() as i32, self.screen_rect.y_left.floor() as i32, 
                                                        self.screen_rect.width.floor() as u32, self.screen_rect.height.floor() as u32).unwrap();
                self.buffer = Some(image.to_png(None).unwrap());
                let result = FileDialog::new()
                    .add_filter("PNG Image", &["png"])
                    .add_filter("JPEG Image", &["jpg", "jpeg"])
                    .add_filter("GIF Image", &["gif"])
                    .show_save_single_file()
                    .unwrap();
                match result {
                    Some(result) => {
                        fs::write(result.clone(), self.buffer.clone().unwrap()).unwrap();
                    }
                    None => (),
                };
                ui.set_visible(true);
            }
            if ui.button("Whole screen").clicked() {
                ui.set_visible(false);
                let screen = Screen::all().unwrap()[0];
                let image = screen.capture().unwrap();
                self.buffer = Some(image.to_png(None).unwrap());
                let result = FileDialog::new()
                    .add_filter("PNG Image", &["png"])
                    .add_filter("JPEG Image", &["jpg", "jpeg"])
                    .add_filter("GIF Image", &["gif"])
                    .show_save_single_file()
                    .unwrap();
                match result {
                    Some(result) => {
                        fs::write(result.clone(), self.buffer.clone().unwrap()).unwrap();
                    }
                    None => (),
                };
                ui.set_visible(true);
            }
        });
        let w = egui::Window::new("ciao")
            .title_bar(false)
            .default_size(egui::vec2(320.0, 240.0))
            .resizable(true)
            .show(ctx, |ui| {
                ui.allocate_space(ui.available_size());
            });

        let r = w.unwrap().response.rect;
        self.screen_rect = RectangleCrop { x_left: r.left(), y_left: r.top(), width: r.width(), height: r.height() }
    }    
}
