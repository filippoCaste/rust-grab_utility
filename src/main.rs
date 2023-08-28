
use eframe::egui;
use egui::{RichText,Color32};
use image;
use native_dialog::FileDialog;
use std::{fs, time::Duration};
use chrono::Utc;
use std::time::Instant;

mod action;
mod shortcut;
mod timer;
mod schermi;
 

use action::Action;
use shortcut::shortcut::ShortcutSet;
use timer::timer::Timer;
use schermi::schermi::Schermi;


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
    texture: Option<egui::TextureHandle>,
    buffer: Option<Vec<u8>>,
    screen_rect: RectangleCrop,
    window_hidden: bool,
    mode: bool,
    mode_radio: SelectionMode,
    image_viewer: bool,
    timer:Timer,
    show_options: bool,
    shortcut_set: ShortcutSet,
    schermi: Schermi,
    default_location:String,
    mac_bug: bool,
    annotation: bool,
    selection_annotation: SelectionAnnotation,
    annotation_element: AnnotationElement,
    last_modify: Vec<SelectionAnnotation>,
}

struct RectangleCrop {
    x_left: f32,
    y_left: f32,
    width: f32,
    height: f32,
}

#[derive(PartialEq)]
enum SelectionMode {
    Screen,
    Selection,
}

#[derive(PartialEq, Debug)]
enum SelectionAnnotation {
    NotSelected,
    Pen,
    Rect,
    Arrow,
    Text,
    Crop,
    Line,
    Circle,
}

struct AnnotationElement {
    stroke: egui::Stroke,
    pen: Vec<Vec<(egui::Pos2, egui::Stroke)>>,
    rect: Vec<Vec<(egui::Pos2, egui::Stroke)>>,
    circle: Vec<Vec<(egui::Pos2, egui::Stroke)>>,
    arrow: Vec<Vec<(egui::Pos2, egui::Stroke)>>,
    line: Vec<Vec<(egui::Pos2, egui::Stroke)>>,
    text: Vec<(egui::Pos2, String, egui::Stroke)>,
    text2: String,
    pos_text: bool,
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
            mode_radio: SelectionMode::Screen,
            image_viewer: false,
            timer:Timer::new(),
            show_options:false,
            shortcut_set:ShortcutSet::default(),
            default_location: "~".to_string(),
            schermi: Schermi::new(),
            mac_bug: false,
            selection_annotation: SelectionAnnotation::NotSelected,
            last_modify: Default::default(),
            annotation: false,
            annotation_element: AnnotationElement {
                pen: Default::default(),
                rect: Default::default(),
                circle: Default::default(),
                arrow: Default::default(),
                line: Default::default(),
                text: Default::default(),
                stroke: egui::Stroke::new(1.0, egui::Color32::BLACK),
                text2: "Edit this text".to_owned(),
                pos_text: false,
            },
        }
    }
}
impl MyApp{
    fn run_action(&mut self, action: Action, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match action {
            Action::SetEntireScreen => {

                self.mode_radio=SelectionMode::Screen;
                self.mode = false;
            }
            Action::SetSelection => {
                self.mode_radio=SelectionMode::Selection;
                self.mode = true;
            }
            Action::SettingTimer => {
                
                 if self.timer.get_seconds() > 0 {
                    self.timer.start_timer();
                } else {
                    frame.set_visible(false);
                    self.window_hidden = true;
                }


            }
            Action::StartTimer => {
              
                let now = Instant::now();
                if now.duration_since(self.timer.last_decrement().unwrap()).as_secs_f32() >= 1.0 {
                    self.timer.handle_positive_timer();
                    if self.timer.seconds == 0 {
                       self.timer.handle_negative_timer(); 
                       self.window_hidden = true;
                       frame.set_visible(false);
                       self.timer.close_timer_form();
                    }
                    self.timer.last_decrement_time = Some(now);
                }
                ctx.request_repaint();

               
            }
            Action::CancelTimer => {
                self.timer.cancel_timer();
            }
            Action::Options => {
                self.show_options = true;
                if self.show_options {
                    
                    egui::Window::new("Options")
                    .title_bar(true)
                    .frame(egui::Frame {
                        fill: egui::Color32::GRAY,
                        stroke: egui::Stroke::new(0.5, egui::Color32::BLACK),
                        inner_margin: egui::style::Margin::same(15.0),
                        rounding: egui::Rounding::same(20.0),
                        ..Default::default()
                    })
                    .movable(true)
                    .open(&mut self.show_options)
                    .show(ctx, |ui| {

                        if ui.button("> Show shortcut options").clicked() || self.shortcut_set.show {
                            self.shortcut_set.show = true;
                            for shortcut in self.shortcut_set.to_vec_mut() {
                                ui.label(shortcut.to_string(ctx));
                                ui.checkbox(&mut shortcut.is_active, "actived");
                            }
                            if ui.button("Close").clicked() {
                                self.shortcut_set.show = false;
                            }
                        }


                        if ui.button("> Change location").clicked() || self.schermi.show_screen_options{

                            self.schermi.show_screen_options=true;
                         
                                ui.label(RichText::new("Path to which save the screenshots: ").color(Color32::BLACK));
                                let set_path_text = ui.text_edit_singleline(&mut self.default_location)
                                        .on_hover_text(RichText::new("If path does not exist, the default location will be 'home'").color(Color32::DARK_RED));
                                if set_path_text.changed() {
                                    if self.default_location == "" {
                                        self.default_location = "~".to_string();
                                    }
                                }

                                if ui.button("Close").clicked() {
                                    self.schermi.show_screen_options = false;
                                }
                        }

                        egui::ComboBox::from_id_source("Schermi")
                            .selected_text("> Change screen")
                            .show_ui(ui, |ui| {
                                for i in 0..self.schermi.no_screens() {
                                    let txt = format!("Screen {}", i);
                                    ui.selectable_value(&mut self.schermi.screen_no, i, txt);
                                }
                            });
                    });
                }
            }
            Action::Capture => {
                self.window_hidden = true;
                frame.set_visible(false);
            }
            Action::Close => {
                frame.close();
            }
            Action::Modify => {
                         self.annotation=true;
            }
            Action::TakeAnotherScreenshot => {
                self.image_viewer = false;
                self.mode_radio = SelectionMode::Screen;
                self.mode = false;
                frame.set_visible(false);
                self.mac_bug = true;
            }
            Action::Save => {
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
        }
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::light());
        if self.mac_bug {
            std::thread::sleep(Duration::from_millis(100));
            frame.set_visible(true);
            self.mac_bug = false;
        }
        if self.window_hidden {
            std::thread::sleep(Duration::from_millis(300));
            let screen = self.schermi.get_screen();
            let image;
            if self.mode || self.annotation {
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
            //fs::write("screen.png", self.buffer.clone().unwrap()).unwrap();
            self.window_hidden = false;
            self.image_viewer = true;
            self.mode = false;
            self.annotation = false;
            self.annotation_element.pen.clear();
            self.annotation_element.rect.clear();
            self.annotation_element.text.clear();
            self.annotation_element.arrow.clear();
            self.annotation_element.line.clear();
            self.annotation_element.circle.clear();
            self.last_modify.clear();
            frame.set_visible(false);
            self.mac_bug=true;
            //frame.set_visible(true);
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

                        match self.shortcut_set.listener(ctx, self.image_viewer) {
                            Some(action) => self.run_action(action, ctx, frame),
                            None => {}
                        }

                        if !self.image_viewer {
                            if ui
                                .selectable_value(
                                    &mut self.mode_radio,
                                    SelectionMode::Screen,
                                    "  🖵  ",
                                )
                                .on_hover_text("Capture the entire screen")
                                .clicked()
                            {
                              self.run_action(Action::SetEntireScreen, ctx, frame)
                            };
                            if ui
                                .selectable_value(
                                    &mut self.mode_radio,
                                    SelectionMode::Selection,
                                    "  ⛶  ",
                                )
                                .on_hover_text("Capture the selection")
                                .clicked()
                            {
                               self.run_action(Action::SetSelection, ctx, frame)
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
                            ui.add(egui::Slider::new(&mut self.timer.seconds, 0..=60));

                            if ui.button("Start Timer").clicked() {
                                /*
                                N.B: il bottone ha il solo compito di comunicare l'intenzione di avviare il timer.
                                     Sarà poi il blocco 'if self.timer.is_timer_running { }' ad attivare
                                     effettivamente il timer decrementando i secondi.
                                 */
                                self.run_action(Action::SettingTimer, ctx, frame);
                            }

                            if ui.button("Cancel").clicked() {
                                self.run_action(Action::CancelTimer, ctx, frame);
                            }
                        }

                        if self.timer.is_timer_running() {
                          //  ui.label(format!("Screenshot tra: {}", self.timer.seconds - 1));
                            self.run_action(Action::StartTimer, ctx, frame);

                            if ui.button("Cancel").clicked() {
                                self.run_action(Action::CancelTimer, ctx, frame);
                            }
                        }
                            if ui.button("  Options  ").clicked() {
                                self.run_action(Action::Options, ctx, frame)
                            }
                            if self.show_options {
                                self.run_action(Action::Options, ctx, frame)
                            }
                            if ui.button("  Capture  ").clicked() {
                               self.run_action(Action::Capture, ctx, frame)
                            }
                            if ui
                                .add(
                                    egui::Button::new("  X  ").rounding(egui::Rounding::same(50.0)),
                                )
                                .on_hover_text("Close")
                                .clicked()
                            {
                                self.run_action(Action::Close, ctx, frame)
                            }
                        } else if self.image_viewer && !self.annotation {
                            if ui.button("  Modify  ").clicked() {
                                self.run_action(Action::Modify, ctx, frame)
                            }
                            if ui.button("  Take another Screenshot  ").clicked() {
                            self.run_action(Action::TakeAnotherScreenshot, ctx, frame)
                            }
                            if ui.button("  Save  ").clicked() {
                                self.run_action(Action::Save, ctx, frame)
                            }
                            if ui
                                .add(
                                    egui::Button::new("  X  ").rounding(egui::Rounding::same(50.0)),
                                )
                                .on_hover_text("Close")
                                .clicked()
                            {
                                self.run_action(Action::Close, ctx, frame)
                            }
                        } else {
                            ui.selectable_value(
                                &mut self.selection_annotation,
                                SelectionAnnotation::Pen,
                                "  🖊  ",
                            )
                            .on_hover_text("Draw");
                            ui.selectable_value(
                                &mut self.selection_annotation,
                                SelectionAnnotation::Line,
                                "  /  ",
                            )
                            .on_hover_text("Draw a line");
                            ui.selectable_value(
                                &mut self.selection_annotation,
                                SelectionAnnotation::Arrow,
                                "  ↖  ",
                            )
                            .on_hover_text("Draw an arrow");
                            ui.selectable_value(
                                &mut self.selection_annotation,
                                SelectionAnnotation::Rect,
                                "  ☐  ",
                            )
                            .on_hover_text("Draw a rectangle");
                            ui.selectable_value(
                                &mut self.selection_annotation,
                                SelectionAnnotation::Circle,
                                "  ⭕  ",
                            )
                            .on_hover_text("Draw a circle");
                            ui.selectable_value(
                                &mut self.selection_annotation,
                                SelectionAnnotation::Text,
                                "  Text  ",
                            )
                            .on_hover_text("Text");
                            ui.label("|");
                            ui.selectable_value(
                                &mut self.selection_annotation,
                                SelectionAnnotation::Crop,
                                "  ⛶  ",
                            )
                            .on_hover_text("Crop");
                            if self.selection_annotation==SelectionAnnotation::Crop{
                                if ui.button("  Save crop  ").clicked() {
                                    self.selection_annotation = SelectionAnnotation::NotSelected;
                                    self.window_hidden=true;
                                }
                            }
                            ui.label("|");
                            egui::stroke_ui(ui, &mut self.annotation_element.stroke, "Stroke");
                            ui.label("|");
                            if ui.button("  ⟲  ").clicked() {
                                if let Some(last) = self.last_modify.pop() {
                                    match last {
                                        SelectionAnnotation::NotSelected => {}
                                        SelectionAnnotation::Pen => {
                                            self.annotation_element
                                                .pen
                                                .remove(self.annotation_element.pen.len() - 2);
                                        }
                                        SelectionAnnotation::Line => {
                                            self.annotation_element
                                                .line
                                                .remove(self.annotation_element.line.len() - 2);
                                        }
                                        SelectionAnnotation::Arrow => {
                                            self.annotation_element
                                                .arrow
                                                .remove(self.annotation_element.arrow.len() - 2);
                                        }
                                        SelectionAnnotation::Rect => {
                                            self.annotation_element
                                                .rect
                                                .remove(self.annotation_element.rect.len() - 2);
                                        }
                                        SelectionAnnotation::Circle => {
                                            self.annotation_element
                                                .circle
                                                .remove(self.annotation_element.circle.len() - 2);
                                        }
                                        SelectionAnnotation::Text => {
                                            self.annotation_element.text.pop();
                                        }
                                        SelectionAnnotation::Crop => {}
                                    }
                                }
                            }
                            if ui.button("  Cancel  ").clicked() {
                                self.annotation_element.pen.clear();
                                self.annotation_element.rect.clear();
                                self.annotation_element.text.clear();
                                self.annotation_element.arrow.clear();
                                self.annotation_element.line.clear();
                                self.annotation_element.circle.clear();
                                self.last_modify.clear();
                                self.selection_annotation = SelectionAnnotation::NotSelected;
                                self.annotation = false;
                            }
                            if ui.button("  Save modify  ").clicked() {
                                let dim_image = resize_image_to_fit_container(
                                    1000.0,
                                    600.0,
                                    self.texture.clone().unwrap().size_vec2()[0],
                                    self.texture.clone().unwrap().size_vec2()[1],
                                );
                                self.screen_rect = RectangleCrop {
                                    x_left: ((frame.info().window_info.size[0]) / 2.0)-(dim_image.0/2.0),
                                    y_left: ((frame.info().window_info.size[1]) / 2.0)-(dim_image.1/2.0)+ frame.info().window_info.position.unwrap()[1],
                                    width: dim_image.0,
                                    height: dim_image.1,
                                };
                                self.selection_annotation = SelectionAnnotation::NotSelected;
                                self.window_hidden=true;
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
                // ui.image(
                //     &self.texture.clone().unwrap(),
                //     resize_image_to_fit_container(
                //         1000.0,
                //         600.0,
                //         self.texture.clone().unwrap().size_vec2()[0],
                //         self.texture.clone().unwrap().size_vec2()[1],
                //     ),
                // );
                let dim_image = resize_image_to_fit_container(
                    1000.0,
                    600.0,
                    self.texture.clone().unwrap().size_vec2()[0],
                    self.texture.clone().unwrap().size_vec2()[1],
                );
                let (mut response, painter) =
                    ui.allocate_painter(egui::vec2(dim_image.0, dim_image.1), egui::Sense::drag());
                painter.image(
                    self.texture.clone().unwrap().id(),
                    egui::Rect::from_center_size(
                        egui::Pos2::new(
                            (frame.info().window_info.size[0]) / 2.0,
                            (frame.info().window_info.size[1]) / 2.0,
                        ),
                        egui::Vec2::new(dim_image.0, dim_image.1),
                    ),
                    egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                    egui::Color32::WHITE,
                );
                if self.annotation {
                    match self.selection_annotation {
                        SelectionAnnotation::NotSelected => {}
                        SelectionAnnotation::Pen => {
                            response
                                .clone()
                                .on_hover_cursor(egui::output::CursorIcon::PointingHand);
                            if self.annotation_element.pen.is_empty() {
                                self.annotation_element.pen.push(vec![]);
                            }

                            let current_line = self.annotation_element.pen.last_mut().unwrap();

                            if let Some(pointer_pos) = response.interact_pointer_pos() {
                                if current_line.last()
                                    != Some(&(pointer_pos, self.annotation_element.stroke))
                                {
                                    current_line
                                        .push((pointer_pos, self.annotation_element.stroke));
                                    response.mark_changed();
                                }
                            } else if !current_line.is_empty() {
                                self.annotation_element.pen.push(vec![]);
                                response.mark_changed();
                                self.last_modify.push(SelectionAnnotation::Pen);
                            }
                        }
                        SelectionAnnotation::Line => {
                            response
                                .clone()
                                .on_hover_cursor(egui::output::CursorIcon::Crosshair);
                            if self.annotation_element.line.is_empty() {
                                self.annotation_element.line.push(vec![]);
                            }

                            let current_line = self.annotation_element.line.last_mut().unwrap();

                            if let Some(pointer_pos) = response.interact_pointer_pos() {
                                if current_line.last()
                                    != Some(&(pointer_pos, self.annotation_element.stroke))
                                {
                                    current_line
                                        .push((pointer_pos, self.annotation_element.stroke));
                                    response.mark_changed();
                                }
                            } else if !current_line.is_empty() {
                                self.annotation_element.line.push(vec![]);
                                response.mark_changed();
                                self.last_modify.push(SelectionAnnotation::Line);
                            }
                        }
                        SelectionAnnotation::Arrow => {
                            response
                                .clone()
                                .on_hover_cursor(egui::output::CursorIcon::Crosshair);
                            if self.annotation_element.arrow.is_empty() {
                                self.annotation_element.arrow.push(vec![]);
                            }

                            let current_line = self.annotation_element.arrow.last_mut().unwrap();

                            if let Some(pointer_pos) = response.interact_pointer_pos() {
                                if current_line.last()
                                    != Some(&(pointer_pos, self.annotation_element.stroke))
                                {
                                    current_line
                                        .push((pointer_pos, self.annotation_element.stroke));
                                    response.mark_changed();
                                }
                            } else if !current_line.is_empty() {
                                self.annotation_element.arrow.push(vec![]);
                                response.mark_changed();
                                self.last_modify.push(SelectionAnnotation::Arrow);
                            }
                        }
                        SelectionAnnotation::Rect => {
                            response
                                .clone()
                                .on_hover_cursor(egui::output::CursorIcon::Crosshair);
                            if self.annotation_element.rect.is_empty() {
                                self.annotation_element.rect.push(vec![]);
                            }

                            let current_line = self.annotation_element.rect.last_mut().unwrap();

                            if let Some(pointer_pos) = response.interact_pointer_pos() {
                                if current_line.last()
                                    != Some(&(pointer_pos, self.annotation_element.stroke))
                                {
                                    current_line
                                        .push((pointer_pos, self.annotation_element.stroke));
                                    response.mark_changed();
                                }
                            } else if !current_line.is_empty() {
                                self.annotation_element.rect.push(vec![]);
                                response.mark_changed();
                                self.last_modify.push(SelectionAnnotation::Rect);
                            }
                        }
                        SelectionAnnotation::Circle => {
                            response
                                .clone()
                                .on_hover_cursor(egui::output::CursorIcon::Crosshair);
                            if self.annotation_element.circle.is_empty() {
                                self.annotation_element.circle.push(vec![]);
                            }

                            let current_line = self.annotation_element.circle.last_mut().unwrap();

                            if let Some(pointer_pos) = response.interact_pointer_pos() {
                                if current_line.last()
                                    != Some(&(pointer_pos, self.annotation_element.stroke))
                                {
                                    current_line
                                        .push((pointer_pos, self.annotation_element.stroke));
                                    response.mark_changed();
                                }
                            } else if !current_line.is_empty() {
                                self.annotation_element.circle.push(vec![]);
                                response.mark_changed();
                                self.last_modify.push(SelectionAnnotation::Circle);
                            }
                        }
                        SelectionAnnotation::Text => {
                            let res = egui::Area::new("text")
                                .movable(true)
                                .default_pos(egui::Pos2::new(
                                    (frame.info().window_info.size[0] - 20.0) / 2.0,
                                    (frame.info().window_info.size[1] - 20.0) / 2.0,
                                ))
                                .drag_bounds(egui::Rect::from_center_size(
                                    egui::Pos2::new(
                                        (frame.info().window_info.size[0]) / 2.0,
                                        (frame.info().window_info.size[1]) / 2.0,
                                    ),
                                    egui::Vec2::new(dim_image.0, dim_image.1),
                                ))
                                .order(egui::layers::Order::Foreground)
                                .show(ctx, |ui| {
                                    ui.vertical(|ui| {
                                        ui.label(
                                            egui::RichText::new(format!(
                                                "{}",
                                                self.annotation_element.text2,
                                            ))
                                            .color(self.annotation_element.stroke.color)
                                            .size(
                                                self.annotation_element.stroke.width * 20.0 + 0.1,
                                            ),
                                        );
                                        ui.horizontal(|ui| {
                                            egui::TextEdit::multiline(
                                                &mut self.annotation_element.text2,
                                            )
                                            .hint_text("Hello!")
                                            .show(ui);
                                            if ui.button("save").clicked() {
                                                self.annotation_element.pos_text = true;
                                                self.last_modify.push(SelectionAnnotation::Text);
                                                self.selection_annotation = SelectionAnnotation::NotSelected;
                                            };
                                        })
                                    });
                                });
                            if self.annotation_element.pos_text {
                                self.annotation_element.pos_text = false;
                                let r = res.response.rect;
                                self.annotation_element.text.push((
                                    egui::Pos2::new(r.left(), r.top()),
                                    self.annotation_element.text2.clone(),
                                    self.annotation_element.stroke.clone(),
                                ));
                                self.annotation_element.text2 = "Edit this text".to_string();
                            }
                        }
                        SelectionAnnotation::Crop => {
                           let pos= egui::Window::new("resize2")
                                .title_bar(false)
                                .default_size(egui::vec2(320.0, 240.0))
                                .resizable(true)
                                .movable(true)
                                .default_pos(egui::Pos2::new(
                                    (frame.info().window_info.size[0] - 320.0) / 2.0,
                                    (frame.info().window_info.size[1] - 240.0) / 2.0,
                                ))
                                .drag_bounds(egui::Rect::from_center_size(
                                    egui::Pos2::new(
                                        (frame.info().window_info.size[0]) / 2.0,
                                        (frame.info().window_info.size[1]) / 2.0,
                                    ),
                                    egui::Vec2::new(dim_image.0, dim_image.1),
                                ))
                                .frame(egui::Frame {
                                    // fill: egui::Color32::TRANSPARENT,
                                    stroke: egui::Stroke::new(1.5, egui::Color32::WHITE),
                                    shadow: egui::epaint::Shadow::small_light(),
                                    ..Default::default()
                                })
                                .show(ctx, |ui| {
                                    ui.allocate_space(ui.available_size());
                                });
                                let r = pos.unwrap().response.rect;
                                self.screen_rect = RectangleCrop {
                                    x_left: r.left(),
                                    y_left: r.top() + frame.info().window_info.position.unwrap()[1],
                                    width: r.width(),
                                    height: r.height(),
                                };
                        }
                    }
                }
                let pen = self
                    .annotation_element
                    .pen
                    .iter()
                    .filter(|line| line.len() >= 2)
                    .map(|line| {
                        let points: Vec<egui::Pos2> = line.iter().map(|p| p.0).collect();
                        let stroke = line[0].1;
                        egui::Shape::line(points, stroke)
                    });
                let rect = self
                    .annotation_element
                    .rect
                    .iter()
                    .filter(|line| line.len() >= 2)
                    .map(|line| {
                        let rect = egui::Rect::from_two_pos(
                            line.first().unwrap().0,
                            line.last().unwrap().0,
                        );
                        egui::Shape::rect_stroke(rect, egui::Rounding::none(), line[0].1)
                    });
                let circle = self
                    .annotation_element
                    .circle
                    .iter()
                    .filter(|line| line.len() >= 2)
                    .map(|line| {
                        egui::Shape::circle_stroke(
                            line.first().unwrap().0,
                            line.first().unwrap().0.distance(line.last().unwrap().0),
                            line[0].1,
                        )
                    });
                let line = self
                    .annotation_element
                    .line
                    .iter()
                    .filter(|line| line.len() >= 2)
                    .map(|line| {
                        let vec = [line.first().unwrap().0, line.last().unwrap().0];
                        egui::Shape::line_segment(vec, line[0].1)
                    });

                for el in self.annotation_element.arrow.clone() {
                    if el.first().is_some() && el.last().is_some() {
                        let vec = el.first().unwrap().0 - el.last().unwrap().0;
                        painter.arrow(el.first().unwrap().0, -vec, el[0].1);
                    }
                }

                for el in self.annotation_element.text.clone() {
                    painter.text(
                        el.0,
                        egui::Align2::LEFT_TOP,
                        el.1,
                        egui::FontId::proportional(el.2.width * 20.0 + 0.1),
                        el.2.color,
                    );
                }

                painter.extend(pen);
                painter.extend(line);
                painter.extend(rect);
                painter.extend(circle);
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
    let container_ratio = container_width / container_height;
    let image_ratio = image_width / image_height;

    if container_ratio > image_ratio {
        let new_height = container_height;
        let new_width = new_height * image_ratio;
        (new_width, new_height)
    } else {
        let new_width = container_width;
        let new_height = new_width / image_ratio;
        (new_width, new_height)
    }
}
