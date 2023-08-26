use chrono::Utc;
use eframe::{egui::{self, RichText}, epaint::Color32};
use image;
use native_dialog::FileDialog;
use std::{fs, time::Duration};

pub mod schermi;
pub mod timer;

use schermi::schermi::Schermi;
use timer::timer::Timer;

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

struct MyApp {
    texture: Option<egui::TextureHandle>,
    buffer: Option<Vec<u8>>,
    screen_rect: RectangleCrop,
    window_hidden: bool,
    mode: bool,
    mode_radio: Enum,
    image_viewer: bool,
    timer: Timer,
    default_location: String,
    show_options: bool,
    schermi: Schermi,
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
            texture: None,
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
            image_viewer: false,
            timer: Timer::new(),
            default_location: "~".to_string(),
            show_options: false,
            schermi: Schermi::new(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        if self.window_hidden {
            std::thread::sleep(Duration::from_millis(300));
            let screen = self.schermi.get_screen();
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
            self.texture = Some(ctx.load_texture(
                "my-image",
                load_image_from_memory(&self.buffer.clone().unwrap()).unwrap(),
                Default::default(),
            ));

            self.window_hidden = false;
            self.image_viewer = true;
            self.mode = false;
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
                        //  let mut text = self.timer.get_seconds().to_string();

                        if !self.image_viewer {
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
                            if ui
                                .button(" 🕓 ")
                                .on_hover_text("Take a screenshot with timer")
                                .clicked()
                            {
                                self.timer.open_timer_form();
                            }

                            if self.timer.is_timer_form_open() {
                                ui.label("Timer (seconds):");
                                ui.add(egui::Slider::new(&mut self.timer.seconds, 0..=240));

                                if ui.button("Start Timer").clicked() {
                                    if self.timer.get_seconds() > 0 {
                                        self.timer.start_timer();
                                    } else {
                                        frame.set_visible(false);
                                        self.window_hidden = true;
                                    }
                                }

                                if ui.button("Cancel").clicked() {
                                    self.timer.cancel_timer();
                                }
                            }

                            if self.timer.is_timer_running() {
                                /*
                                Metodo coi thread -- la label non appare

                                 ui.label(format!("screenshot tra: {}", self.timer.get_seconds()));

                                   let seconds = self.timer.get_seconds();

                                   let (sx, rx) = std::sync::mpsc::channel::<u32>();
                                   let timer_thread = thread::spawn(move || {
                                       for _ in 1..=seconds {
                                           sx.send(1).unwrap();
                                           thread::sleep(Duration::from_secs(1));
                                       }
                                   });

                                   for _ in rx {
                                       self.timer.get_seconds() -= 1;
                                       ctx.request_repaint();
                                       if self.timer.get_seconds() == 0 {
                                           frame.set_visible(false);
                                           self.window_hidden = true;
                                       }
                                   }

                                   timer_thread.join().unwrap();

                                   Metodo col ciclo -- la label non compare

                                   if let Some(_) = self.timer.last_decrement_time {
                                       let mut start_time = self.timer.last_decrement_time.unwrap();
                                       while self.timer.get_seconds() > 0 {
                                           let elapsed_time = start_time.elapsed().as_secs() as u32;

                                           if elapsed_time >= 1 {
                                               self.timer.get_seconds() -= elapsed_time;

                                               start_time = std::time::Instant::now();
                                               if self.timer.get_seconds() <= 0 {
                                                   self.timer.get_seconds() = 0;
                                                   self.timer.is_timer_running = false;
                                                   frame.set_visible(false);
                                                   self.window_hidden = true;
                                               }
                                           }
                                       }

                                   }
                                   */
                                // ui.label(format!("Screenshot tra: {}", self.timer.get_seconds() - 1));

                                if self.timer.get_seconds() > 0 {
                                    std::thread::sleep(Duration::from_secs(1));
                                    self.timer.handle_positive_timer();
                                    ctx.request_repaint();
                                }

                                if self.timer.get_seconds() <= 0 {
                                    self.timer.handle_negative_timer();
                                    frame.set_visible(false);
                                    self.window_hidden = true;
                                }
                            }

                            if ui.button("  Options  ").clicked() {
                                self.show_options = true;
                            }
                            if ui.button("  Capture  ").clicked() {
                                frame.set_visible(false);
                                self.window_hidden = true;
                            }
                            if ui
                                .add(
                                    egui::Button::new("  X  ").rounding(egui::Rounding::same(50.0)),
                                )
                                .on_hover_text("Close")
                                .clicked()
                            {
                                frame.close();
                            }
                        } else {
                            if ui.button("  Modify  ").clicked() {}
                            if ui.button("  Take another Screenshot  ").clicked() {
                                self.image_viewer = false;
                                if self.mode_radio == Enum::Selection {
                                    self.mode = true;
                                } else {
                                    self.mode = false;
                                }
                            }
                            if ui.button("  Save  ").clicked() {                    
                                let default_name = std::thread::spawn(move || {
                                    let today = Utc::now().to_string()
                                    .replace("-", "")
                                    .replace(":", "_")
                                    .replace(" ", "")
                                    .to_string();
                                format!("screenshot_{}", today)
                                }).join().expect("Fail to compute date");
                                let result = match FileDialog::new()
                                    .set_location(&self.default_location)
                                    .set_filename(&default_name[..27])
                                    .add_filter("PNG Image", &["png"])
                                    .add_filter("JPEG Image", &["jpg", "jpeg"])
                                    .add_filter("GIF Image", &["gif"])
                                    .show_save_single_file() {
                                Ok(res) => {res},
                                Err(_) => {
                                    // uncorrect path set by user
                                    FileDialog::new()
                                    .set_location("~")
                                    .set_filename(&default_name[..27])
                                    .add_filter("PNG Image", &["png"])
                                    .add_filter("JPEG Image", &["jpg", "jpeg"])
                                    .add_filter("GIF Image", &["gif"])
                                    .show_save_single_file()
                                    .unwrap()
                                }
                            };
                                match result {
                                    Some(result) => {
                                        fs::write(result.clone(), self.buffer.clone().unwrap())
                                            .unwrap();
                                    }
                                    None => {}
                                };
                            }
                            if ui
                                .add(
                                    egui::Button::new("  X  ").rounding(egui::Rounding::same(50.0)),
                                )
                                .on_hover_text("Close")
                                .clicked()
                            {
                                frame.close();
                            }
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

        egui::Window::new("image_viewer")
            .title_bar(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .frame(egui::Frame {
                fill: egui::Color32::GRAY,
                stroke: egui::Stroke::new(0.5, egui::Color32::BLACK),
                inner_margin: egui::style::Margin::same(15.0),
                rounding: egui::Rounding::same(20.0),
                ..Default::default()
            })
            .fixed_size([1000.0, 600.0])
            .resizable(false)
            .open(&mut self.image_viewer)
            .show(ctx, |ui| {
                ui.image(
                    &self.texture.clone().unwrap(),
                    resize_image_to_fit_container(
                        1000.0,
                        600.0,
                        self.texture.clone().unwrap().size_vec2()[0],
                        self.texture.clone().unwrap().size_vec2()[1],
                    ),
                );
            });

        egui::Window::new("Options")
            .title_bar(true)
            .frame(egui::Frame {
                fill: egui::Color32::GRAY,
                stroke: egui::Stroke::new(0.5, egui::Color32::BLACK),
                inner_margin: egui::style::Margin::same(15.0),
                rounding: egui::Rounding::same(20.0),
                ..Default::default()
            })
            .resizable(true)
            .movable(true)
            .open(&mut self.show_options)
            .show(ctx, |ui| {
                ui.label(RichText::new("Inserisci il percorso nel quale salvare gli screenshots: ").color(Color32::BLACK));
                let set_path_text = ui.text_edit_singleline(&mut self.default_location);
                if set_path_text.changed() {
                    if self.default_location == "" {
                        self.default_location = "~".to_string();
                    }
                }
                ui.label(RichText::new("Se il percorso indicato non è corretto, si verrà reindirizzati a 'home'").color(Color32::RED));

            egui::ComboBox::from_id_source("Schermi")
                .selected_text("Schermo da catturare")
                .show_ui(ui, |ui| {
                    for i in 0..self.schermi.no_screens() {
                        let txt = format!("Schermo {}", i);
                        ui.selectable_value(&mut self.schermi.screen_no, i, txt);
                    }
                });
            });

        if self.timer.is_timer_running() {
            egui::Window::new("Countdown")
                .title_bar(false)
                .anchor(egui::Align2::RIGHT_TOP, [0.0, 10.0])
                .frame(egui::Frame {
                    fill: egui::Color32::GRAY,
                    stroke: egui::Stroke::new(0.5, egui::Color32::BLACK),
                    inner_margin: egui::style::Margin::same(15.0),
                    rounding: egui::Rounding::same(20.0),
                    ..Default::default()
                })
                .resizable(false)
                .show(ctx, |ui| {
                    let txt = format!("  {}  ", self.timer.get_seconds() - 1);
                    ui.label(
                        RichText::new(txt)
                            .size(40.0)
                            .color(Color32::DARK_RED)
                    );
                });
        }

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
}

fn load_image_from_memory(image_data: &[u8]) -> Result<egui::ColorImage, image::ImageError> {
    let image = image::load_from_memory(image_data)?;
    let size = [image.width() as _, image.height() as _];
    let image_buffer = image.to_rgba8();
    let pixels = image_buffer.as_flat_samples();
    Ok(egui::ColorImage::from_rgba_unmultiplied(
        size,
        pixels.as_slice(),
    ))
}

fn resize_image_to_fit_container(
    container_width: f32,
    container_height: f32,
    image_width: f32,
    image_height: f32,
) -> (f32, f32) {
    let res = std::thread::spawn(move || {
        let container_ratio = container_width / container_height;
        let image_ratio = image_width / image_height;

        if container_ratio > image_ratio {
            // Il contenitore è più largo rispetto all'immagine, quindi adattiamo l'altezza dell'immagine.
            let new_height = container_height;
            let new_width = new_height * image_ratio;
            (new_width, new_height)
        } else {
            // Il contenitore è più alto o ha lo stesso rapporto dell'immagine, quindi adattiamo la larghezza dell'immagine.
            let new_width = container_width;
            let new_height = new_width / image_ratio;
            (new_width, new_height)
        }
    });

    res.join().unwrap()
}