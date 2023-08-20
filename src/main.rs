use eframe::egui;
use screenshots::Screen;
use std::{fs, time::Duration};

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

struct MyApp {
    buffer: Option<Vec<u8>>,
    screen_rect: RectangleCrop,
    window_hidden: bool,
    mode: bool,
    mode_radio: Enum,
}

struct RectangleCrop {
    x_left: f32,
    y_left: f32,
    width: f32,
    height: f32,
}

#[derive(PartialEq)]
enum Enum {
    Screen,
    Selection,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            buffer: None,
            screen_rect: RectangleCrop {
                x_left: 0.0,
                y_left: 0.0,
                width: 0.0,
                height: 0.0,
            },
            window_hidden: false,
            mode: false,
            mode_radio: Enum::Screen,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.window_hidden {
            std::thread::sleep(Duration::from_secs(1));
            let screen = Screen::all().unwrap()[0];
            let image;
            if self.mode {
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
            self.window_hidden = false;
            frame.set_visible(true);
        }

        egui::Window::new("Screenshot")
            .title_bar(false)
            .anchor(egui::Align2::CENTER_BOTTOM, [0.0, -20.0])
            .frame(egui::Frame {
                fill: egui::Color32::GRAY,
                stroke: egui::Stroke::new(0.5, egui::Color32::BLACK),
                inner_margin: egui::style::Margin::same(15.0),
                rounding: egui::Rounding::same(20.0),
                ..Default::default()
            })
            .fixed_size([400.0, 30.0])
            .resizable(false)
            .show(ctx, |ui| {
                ui.with_layout(
                    egui::Layout {
                        main_dir: egui::Direction::LeftToRight,
                        main_wrap: false,
                        main_align: egui::Align::Center,
                        main_justify: false,
                        cross_align: egui::Align::Center,
                        cross_justify: true,
                    },
                    |ui| {
                        if ui
                            .selectable_value(&mut self.mode_radio, Enum::Screen, "  🖵  ")
                            .on_hover_text("Capture the entire screen")
                            .clicked()
                        {
                            self.mode = false;
                        };
                        if ui
                            .selectable_value(&mut self.mode_radio, Enum::Selection, "  ⛶  ")
                            .on_hover_text("Capture the selection")
                            .clicked()
                        {
                            self.mode = true;
                        };
                        if ui.button("  Options  ").clicked() {}
                        if ui.button("  Capture  ").clicked() {
                            frame.set_visible(false);
                            self.window_hidden = true;
                        }
                        if ui
                            .add(egui::Button::new("  X  ").rounding(egui::Rounding::same(50.0)))
                            .on_hover_text("Close")
                            .clicked()
                        {
                            frame.close();
                        }
                    },
                );
            });

        let w = egui::Window::new("resize")
            .title_bar(false)
            .default_size(egui::vec2(320.0, 240.0))
            .resizable(true)
            .movable(true)
            .default_pos(egui::Pos2::new(
                (frame.info().window_info.size[0] - 320.0) / 2.0,
                (frame.info().window_info.size[1] - 240.0) / 2.0,
            ))
            .open(&mut self.mode)
            .frame(egui::Frame {
                // fill: egui::Color32::TRANSPARENT,
                stroke: egui::Stroke::new(1.5, egui::Color32::WHITE),
                shadow: egui::epaint::Shadow::small_light(),
                ..Default::default()
            })
            .show(ctx, |ui| {
                ui.allocate_space(ui.available_size());
            });

        if self.mode == true {
            let r = w.unwrap().response.rect;
            self.screen_rect = RectangleCrop {
                x_left: r.left(),
                y_left: r.top() + frame.info().window_info.position.unwrap()[1],
                width: r.width(),
                height: r.height(),
            };
        }
    }
    // fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
    //     // to make the background of the app completely transparent
    //     egui::Rgba::TRANSPARENT.to_array()
    // }
}
