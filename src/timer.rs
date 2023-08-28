pub mod timer {
    use std::time::Instant;

    #[derive(Clone)]
    pub struct Timer {
        pub seconds: u32,
        text: String,
        timer_form_open: bool,
        is_timer_running: bool,
      pub  last_decrement_time:Option<Instant>,
    }

    impl Timer {

        pub fn new() -> Self {
            Timer {    
                seconds: 0,
                text: "".to_string(),
                timer_form_open: false,
                is_timer_running: false,
                last_decrement_time: None,
            }
        }

        pub fn handle_negative_timer(&mut self) {
            self.seconds = 0;
            self.text = "".to_string();
            self.is_timer_running = false;
        }

        pub fn handle_positive_timer(&mut self) {
            self.seconds -= 1;
        }

        pub fn start_timer(&mut self) {
            self.timer_form_open = false;
            self.is_timer_running = true;
            self.last_decrement_time= Some(Instant::now());
        }

        pub fn cancel_timer(&mut self) {
            self.timer_form_open = false;
            self.seconds = 0;
            self.text = "".to_string();
            self.is_timer_running = false;
        }

        pub fn is_timer_running(&self) -> bool {
            self.is_timer_running
        }

        pub fn get_seconds(&self) -> u32 {
            self.seconds
        }

        pub fn open_timer_form(&mut self) {
            self.timer_form_open = true;
        }

        pub fn is_timer_form_open(&self) -> bool {
            self.timer_form_open
        }

        pub fn close_timer_form(&mut self) {
            self.timer_form_open = false;
        }
        pub fn last_decrement(&self)->Option<Instant>{
            self.last_decrement_time
        }
    }

}