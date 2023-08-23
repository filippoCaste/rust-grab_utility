use eframe::egui;
use screenshots::Screen;
use std::{fs, time::Duration};
use native_dialog::FileDialog;

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

#[derive(Default)]
struct MyApp {
    buffer: Option<Vec<u8>>,
    screen_rect: RectangleCrop,
    window_hidden: u8,
    texture: Option<eframe::epaint::TextureHandle>,
    image_size: Option<(f32, f32)>,
    imagePNG:Option<Vec<u8>>
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
            std::thread::sleep(Duration::from_secs(1));
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
            self.buffer = Some(image.rgba().to_vec());

            self.imagePNG= Some(image.to_png(None).unwrap());

            let size = (image.width() as usize, image.height() as usize);

            self.image_size = Some((size.0 as f32, size.1 as f32));

           
            let _texture = self.texture.get_or_insert_with(|| {
                ctx.load_texture(
                    "new_screen",
                    egui::ColorImage::from_rgba_unmultiplied(
                        size.into(),
                        self.buffer.as_ref().unwrap(),
                    ),
                    Default::default(),
                )
            });

            self.window_hidden = 0;
            frame.set_visible(true);
        }

        egui::Window::new("Screenshot").show(ctx, |ui| {
            
            if let Some(_buffer) = &self.buffer {


                if ui.button("save").clicked(){

                    let result = FileDialog::new()
                    .add_filter("PNG Image", &["png"])
                    .add_filter("JPEG Image", &["jpg", "jpeg"])
                    .add_filter("GIF Image", &["gif"])
                    .show_save_single_file()
                    .unwrap();
                 match result {
                    Some(result) => {
                        fs::write(result.clone(), self.imagePNG.clone().unwrap()).unwrap();
                       
                    }
                    None => {;}
                };

                }

                if ui.button("crop image").clicked(){
                    
                }

                ui.image(
                    egui::TextureId::from(self.texture.as_ref().unwrap()),
                    egui::Vec2::from(&self.image_size.unwrap()),
                );

             //   println!("{:?}",self.image_size.unwrap())
            }else{

                if ui.button("Take screenshot").clicked() {
                    frame.set_visible(false);
                    self.window_hidden = 1;
                }
                if ui.button("Whole screen").clicked() {
                    frame.set_visible(false);
                    self.window_hidden = 2;
                }

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
