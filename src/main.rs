use iced::executor;
use iced::theme;
use iced::widget::{button, column, container, horizontal_space, image, row, text};
use iced::window;
use iced::{Alignment, Application, Command, Element, Length, Settings, Theme};
use native_dialog::FileDialog;
use screenshots::Screen;
use std::fs;

pub fn main() -> iced::Result {
    Screenshot::run(Settings {
        window: window::Settings {
            size: (700, 600),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug)]
struct Screenshot {
    image: Option<image::Handle>,
    buffer: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    TakeScreenshotPressed,
    TakeScreenshot,
    ViewWindow,
    SaveImage,
    Settings,
}

impl Application for Screenshot {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Screenshot, Command<Message>) {
        (
            Screenshot {
                image: None,
                buffer: None,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Screenshot")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::TakeScreenshotPressed => Command::batch(vec![
                window::change_mode(window::Mode::Hidden),
                Command::perform(async {}, |()| Message::TakeScreenshot),
            ]),
            Message::TakeScreenshot => {
                let screen = Screen::all().unwrap()[0];
                let image = screen.capture().unwrap();
                self.buffer = Some(image.to_png(None).unwrap());
                self.image = Some(image::Handle::from_memory(self.buffer.clone().unwrap()));
                Command::perform(async {}, |()| Message::ViewWindow)
            }
            Message::ViewWindow => window::change_mode(window::Mode::Windowed),
            Message::SaveImage => {
                let result = FileDialog::new()
                    .add_filter("PNG Image", &["png"])
                    .add_filter("JPEG Image", &["jpg", "jpeg"])
                    .add_filter("GIF Image", &["gif"])
                    .show_save_single_file()
                    .unwrap();
                let result = match result {
                    Some(result) => {
                        fs::write(result.clone(), self.buffer.clone().unwrap()).unwrap();
                        Command::none()
                    }
                    None => Command::none(),
                };

                result
            }
            Message::Settings => Command::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        let image_viewer = self
            .image
            .as_ref()
            .map(|image_handle| image::viewer(image_handle.clone()).into());

        let image_viewer_element: Element<Message> =
            image_viewer.unwrap_or_else(|| text::Text::new("No screenshot taken yet").into());

        let button_save = if self.buffer.is_some() {
            button("Save").on_press(Message::SaveImage)
        } else {
            button("Save")
        };

        let content = container(
            column![
                row![
                    button("+ New").on_press(Message::TakeScreenshotPressed),
                    horizontal_space(Length::Fill),
                    button("Settings")
                        .on_press(Message::Settings)
                        .style(theme::Button::Secondary),
                    horizontal_space(10),
                    button_save
                ]
                .align_items(Alignment::Start)
                .padding(10),
                container(image_viewer_element)
                    .center_x()
                    .center_y()
                    .width(Length::Fill)
                    .height(Length::Fill),
            ]
            .height(Length::Fill),
        )
        .padding(10)
        .width(Length::Fill)
        .height(Length::Fill);

        content.into()
    }
}
