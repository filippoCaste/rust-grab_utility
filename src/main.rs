use iced::executor;
use iced::widget::{button, column, image, text};
use iced::window;
use iced::{Alignment, Application, Command, Element, Settings, Theme};
use screenshots::Screen;
use std::fs;

pub fn main() -> iced::Result {
    Counter::run(Settings {
        window: window::Settings {
            size: (700, 600),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}

#[derive(Debug)]
struct Counter {
    value: i32,
    image: Option<image::Handle>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    TakeScreenshotPressed,
    TakeScreenshot,
}

impl Application for Counter {
    type Message = Message;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Counter, Command<Message>) {
        (
            Counter {
                value: 0,
                image: None,
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
                self.value += 1;
                let screen = Screen::all().unwrap()[0];
                let image = screen.capture().unwrap();
                let filename = format!("screen_{}.png", self.value);
                let buffer = image.to_png(None).unwrap();
                fs::write(&filename, buffer).unwrap();
                self.image = Some(image::Handle::from_path(&filename));
                window::change_mode(window::Mode::Windowed)
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let image_viewer = self
            .image
            .as_ref()
            .map(|image_handle| image::viewer(image_handle.clone()).into());

        let image_viewer_element: Element<Message> =
            image_viewer.unwrap_or_else(|| text::Text::new("No screenshot taken yet").into());

        column![
            button("Take Screenshot").on_press(Message::TakeScreenshotPressed),
            text(self.value).size(50),
            image_viewer_element,
        ]
        .padding(20)
        .align_items(Alignment::Center)
        .into()
    }
}
