pub mod schermi {
    use screenshots::Screen;

    pub struct Schermi {
        screens: Vec<Screen>,
        pub screen_no: usize,
        pub show_screen_options:bool
    }

    impl Schermi {
        pub fn new() -> Self {
            Schermi { screens: Screen::all().unwrap(), screen_no: 0, show_screen_options:false }
        }

        pub fn no_screens(&self) -> usize {
            self.screens.len() as usize
        }

        pub fn get_screen(&self) -> Screen {
            self.screens[self.screen_no]
        }

    }
}