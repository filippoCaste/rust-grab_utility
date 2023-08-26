pub mod schermi {
    use screenshots::Screen;

    pub struct Schermi {
        screens: Vec<Screen>,
        pub screen_no: usize
    }

    impl Schermi {
        pub fn new() -> Self {
            Schermi { screens: Screen::all().unwrap(), screen_no: 0 }
        }

        pub fn no_screens(&self) -> usize {
            self.screens.len() as usize
        }

        pub fn get_screen(&self) -> Screen {
            self.screens[self.screen_no]
        }

    }
}