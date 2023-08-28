pub mod shortcut {
    use crate::action::Action;
    use eframe::egui;
    use egui::{Key, KeyboardShortcut, Modifiers};

    pub struct ShortCut {
        name: String,
        shortcut: KeyboardShortcut,
        pub is_active: bool,
        wants_image_viewer: bool,
        action: Action,
    }

    pub struct ShortcutSet {
        set: Vec<ShortCut>,
        pub show: bool,
    }

    impl ShortCut {
        fn listener_shortcut(&self, ctx: &egui::Context) -> Option<Action> {
            if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut)) && self.is_active {
                Some(self.action)
            } else {
                None
            }
        }

        pub fn to_string(&self, ctx: &egui::Context) -> String {
            let mut output = self.name.clone();
            output.push_str(&" -> ".to_string());
            output.push_str(&ctx.format_shortcut(&self.shortcut));
            output
        }
    }

    impl ShortcutSet {
        pub fn default() -> Self {
            let mut output = Vec::new();
            {
                let s_set_entire_screen = ShortCut {
                    name: "Full screen".to_string(),
                    shortcut: KeyboardShortcut::new(Modifiers::COMMAND, Key::F),
                    is_active: true,
                    wants_image_viewer: false,
                    action: Action::SetEntireScreen,
                };
                let s_set_selection = ShortCut {
                    name: "Set the selection".to_string(),
                    shortcut: KeyboardShortcut::new(Modifiers::COMMAND, Key::ArrowDown),
                    is_active: true,
                    wants_image_viewer: false,
                    action: Action::SetSelection,
                };

                let s_start_timer = ShortCut {
                    name: "Start timer".to_string(),
                    shortcut: KeyboardShortcut::new(Modifiers::COMMAND, Key::T),
                    is_active: true,
                    wants_image_viewer: false,
                    action: Action::SettingTimer,
                };
                let s_cancel_timer = ShortCut {
                    name: "Cancel timer".to_string(),
                    shortcut: KeyboardShortcut::new(Modifiers::COMMAND | Modifiers::ALT, Key::T),
                    is_active: true,
                    wants_image_viewer: false,
                    action: Action::CancelTimer,
                };
                let s_options = ShortCut {
                    name: "Open Options".to_string(),
                    shortcut: KeyboardShortcut::new(Modifiers::COMMAND, Key::O),
                    is_active: true,
                    wants_image_viewer: false,
                    action: Action::Options,
                };
                let s_capture = ShortCut {
                    name: "Do the screenshot".to_string(),
                    shortcut: KeyboardShortcut::new(Modifiers::COMMAND, Key::C),
                    is_active: true,
                    wants_image_viewer: false,
                    action: Action::Capture,
                };
                let s_close = ShortCut {
                    name: "Close".to_string(),
                    shortcut: KeyboardShortcut::new(Modifiers::COMMAND, Key::X),
                    is_active: true,
                    wants_image_viewer: false,
                    action: Action::Close,
                };
                let s_modify = ShortCut {
                    name: "Modify".to_string(),
                    shortcut: KeyboardShortcut::new(Modifiers::COMMAND, Key::M),
                    is_active: true,
                    wants_image_viewer: true,
                    action: Action::Modify,
                };
                let s_another_screenshot = ShortCut {
                    name: "Take another screenshot".to_string(),
                    shortcut: KeyboardShortcut::new(Modifiers::COMMAND, Key::A),
                    is_active: true,
                    wants_image_viewer: true,
                    action: Action::TakeAnotherScreenshot,
                };
                let s_save = ShortCut {
                    name: "Save".to_string(),
                    shortcut: KeyboardShortcut::new(Modifiers::COMMAND, Key::S),
                    is_active: true,
                    wants_image_viewer: true,
                    action: Action::Save,
                };
                output.push(s_save);
                output.push(s_set_entire_screen);
                output.push(s_set_selection);
                output.push(s_start_timer);
                output.push(s_cancel_timer);
                output.push(s_options);
                output.push(s_capture);
                output.push(s_close);
                output.push(s_modify);
                output.push(s_another_screenshot);
            }

            Self {
                set: output,
                show: false,
            }
        }

        pub fn listener(&self, ctx: &egui::Context, is_image: bool) -> Option<Action> {
            for sc in self.set.iter() {
                if sc.wants_image_viewer == is_image && sc.is_active {
                    if let Some(opt_action) = sc.listener_shortcut(ctx) {
                        return Some(opt_action);
                    }
                }
            }
            None
        }

        /*   pub fn to_string(&self, ctx: &egui::Context) -> Vec<String> {
            let mut output = Vec::new();
            for sc in self.set.iter() {
                let sc_name = sc.to_string(ctx);
                output.push(sc_name);
            }
            output
        } */

        pub fn to_vec_mut(&mut self) -> Vec<&mut ShortCut> {
            let mut output = Vec::new();
            for sc in self.set.iter_mut() {
                output.push(sc);
            }
            output
        }
    }
}
