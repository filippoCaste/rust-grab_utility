pub mod schermi {
    use eframe::WindowInfo;
    use screenshots::Screen;

    pub struct Schermi {
        screens: Vec<Screen>,
        pub screen_no: usize,
        pub show_screen_options: bool,
        pub default_screen_no: usize,
    }

    impl Schermi {
        pub fn new() -> Self {
            Schermi {
                screens: Screen::all().unwrap(),
                screen_no: 0,
                show_screen_options: false,
                default_screen_no: 0,
            }
        }

        pub fn set_screen_no(&mut self, info: WindowInfo) {
            let x = info.position.unwrap().x as i32;
            let y = info.position.unwrap().y as i32
                - (info.monitor_size.unwrap().y as i32 - info.size.y as i32);
            if info.position.unwrap().x.abs() > 100.0 || info.position.unwrap().y.abs() > 100.0 {
                match Screen::from_point(x, y) {
                    Ok(screen_info) => {
                        let id_screen = screen_info.display_info.id;
                        let list_screen = Screen::all().unwrap();
                        if let Some(position) = list_screen
                            .iter()
                            .position(|&screen| screen.display_info.id == id_screen)
                        {
                            self.screen_no = position;
                            self.default_screen_no = position;
                        }
                    }
                    Err(_) => {}
                }
            }
        }

        pub fn no_screens(&self) -> usize {
            self.screens.len() as usize
        }

        pub fn get_screen(&self) -> Screen {
            self.screens[self.screen_no]
        }

        pub fn get_default_screen(&self) -> Screen {
            self.screens[self.default_screen_no]
        }
    }
}
