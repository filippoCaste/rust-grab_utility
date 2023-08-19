use eframe::egui;
use screenshots::Screen;
use std::fs;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        maximized: true,
        decorated: false,
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
    buffer: Option<Vec<u8>>,
    screen_rect: RectangleCrop,
    window_hidden: u8,
}

#[derive(Default)]
struct RectangleCrop {
    x_left: f32,
    y_left: f32,
    width: f32,
    height: f32,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.window_hidden != 0 {
            let screen = Screen::all().unwrap()[0];
            let image;
            if self.window_hidden == 1 {
                image = screen
                    .capture_area(
                        self.screen_rect.x_left.floor() as i32,
                        self.screen_rect.y_left.floor() as i32,
                        self.screen_rect.width.floor() as u32,
                        self.screen_rect.height.floor() as u32,
                    )
                    .unwrap();
            } else {
                image = screen.capture().unwrap();
            }
            self.buffer = Some(image.to_png(None).unwrap());
            fs::write("screen.png", self.buffer.clone().unwrap()).unwrap();
            self.window_hidden = 0;
            frame.set_visible(true);
        }

        egui::Window::new("Screenshot").show(ctx, |ui| {
            if ui.button("Take screenshot").clicked() {
                frame.set_visible(false);
                self.window_hidden = 1;
            }
            if ui.button("Whole screen").clicked() {
                frame.set_visible(false);
                self.window_hidden = 2;
            }
        });

        let w = egui::Window::new("resize")
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
        self.screen_rect = RectangleCrop {
            x_left: r.left(),
            y_left: r.top() + 25.0,
            width: r.width(),
            height: r.height(),
        };
    }
}
