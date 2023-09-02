pub mod shortcut {
    use crate::action::Action;
    use eframe::egui;
    use egui::{Key, KeyboardShortcut, Modifiers};

    pub struct NewShortcut {
        pub modifier: Modifiers,
        pub key: Option<Key>,
        pub action: Option<Action>,
        pub is_default: bool,
    }

    impl NewShortcut {
        pub fn default() -> Self {
            Self {
                modifier: Modifiers {
                    alt: false,
                    ctrl: false,
                    shift: false,
                    mac_cmd: false,
                    command: false,
                },
                key: None,
                action: None,
                is_default: true,
            }
        }
    }
    #[derive(Clone)]
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
    pub struct AllKeyArr {
        pub all_key: Vec<Key>,
    }
    impl AllKeyArr {
        pub fn new() -> Self {
            Self {
                all_key: vec![
                    Key::ArrowDown,
                    Key::ArrowLeft,
                    Key::ArrowRight,
                    Key::ArrowUp,
                    Key::Escape,
                    Key::Tab,
                    Key::Backspace,
                    Key::Enter,
                    Key::Space,
                    Key::Insert,
                    Key::Delete,
                    Key::Home,
                    Key::End,
                    Key::PageUp,
                    Key::PageDown,
                    Key::Minus,
                    Key::PlusEquals,
                    Key::Num0,
                    Key::Num1,
                    Key::Num2,
                    Key::Num3,
                    Key::Num4,
                    Key::Num5,
                    Key::Num6,
                    Key::Num7,
                    Key::Num8,
                    Key::Num9,
                    Key::A,
                    Key::B,
                    Key::C,
                    Key::D,
                    Key::E,
                    Key::F,
                    Key::G,
                    Key::H,
                    Key::I,
                    Key::J,
                    Key::K,
                    Key::L,
                    Key::M,
                    Key::N,
                    Key::O,
                    Key::P,
                    Key::Q,
                    Key::R,
                    Key::S,
                    Key::T,
                    Key::U,
                    Key::V,
                    Key::W,
                    Key::X,
                    Key::Y,
                    Key::Z,
                    Key::F1,
                    Key::F2,
                    Key::F3,
                    Key::F4,
                    Key::F5,
                    Key::F6,
                    Key::F7,
                    Key::F8,
                    Key::F9,
                    Key::F10,
                    Key::F11,
                    Key::F12,
                    Key::F13,
                    Key::F14,
                    Key::F15,
                    Key::F16,
                    Key::F17,
                    Key::F18,
                    Key::F19,
                    Key::F20,
                ],
            }
        }
    }

    impl ShortCut {
        fn listener_shortcut(&self, ctx: &egui::Context) -> Option<Action> {
            if ctx.input_mut(|i| i.consume_shortcut(&self.shortcut)) && self.is_active {
                Some(self.action)
            } else {
                None
            }
        }
        fn change_active(&mut self){
            let active= self.is_active;
            self.is_active= !active;
        }

        fn shortcut_builder(modifiers: Modifiers, key: Key, action: Action) -> Self {
            Self {
                name: action.to_string(),
                shortcut: KeyboardShortcut { modifiers, key },
                is_active: true,
                wants_image_viewer: action.wants_image_viewer(),
                action: action,
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
                    name: Action::SetEntireScreen.to_string(),
                    shortcut: KeyboardShortcut::new(Modifiers::COMMAND, Key::F),
                    is_active: true,
                    wants_image_viewer: false,
                    action: Action::SetEntireScreen,
                };
                let s_set_selection = ShortCut {
                    name: Action::SetSelection.to_string(),
                    shortcut: KeyboardShortcut::new(Modifiers::COMMAND, Key::ArrowDown),
                    is_active: true,
                    wants_image_viewer: false,
                    action: Action::SetSelection,
                };

                let s_open_timer = ShortCut {
                    name: Action::SettingTimer.to_string(),
                    shortcut: KeyboardShortcut::new(Modifiers::COMMAND, Key::T),
                    is_active: true,
                    wants_image_viewer: false,
                    action: Action::SettingTimer,
                };
                let s_cancel_timer = ShortCut {
                    name: Action::CancelTimer.to_string(),
                    shortcut: KeyboardShortcut::new(Modifiers::COMMAND | Modifiers::ALT, Key::T),
                    is_active: true,
                    wants_image_viewer: false,
                    action: Action::CancelTimer,
                };
                let s_options = ShortCut {
                    name: Action::Options.to_string(),
                    shortcut: KeyboardShortcut::new(Modifiers::COMMAND, Key::O),
                    is_active: true,
                    wants_image_viewer: false,
                    action: Action::Options,
                };
                let s_capture = ShortCut {
                    name: Action::Capture.to_string(),
                    shortcut: KeyboardShortcut::new(Modifiers::COMMAND, Key::Enter),
                    is_active: true,
                    wants_image_viewer: false,
                    action: Action::Capture,
                };
                let s_close = ShortCut {
                    name: Action::Close.to_string(),
                    shortcut: KeyboardShortcut::new(Modifiers::COMMAND, Key::X),
                    is_active: true,
                    wants_image_viewer: false,
                    action: Action::Close,
                };
                let s_modify = ShortCut {
                    name: Action::Modify.to_string(),
                    shortcut: KeyboardShortcut::new(Modifiers::COMMAND, Key::M),
                    is_active: true,
                    wants_image_viewer: true,
                    action: Action::Modify,
                };
                let s_another_screenshot = ShortCut {
                    name: Action::TakeAnotherScreenshot.to_string(),
                    shortcut: KeyboardShortcut::new(Modifiers::COMMAND, Key::A),
                    is_active: true,
                    wants_image_viewer: true,
                    action: Action::TakeAnotherScreenshot,
                };
                let s_save = ShortCut {
                    name: Action::Save.to_string(),
                    shortcut: KeyboardShortcut::new(Modifiers::COMMAND, Key::S),
                    is_active: true,
                    wants_image_viewer: true,
                    action: Action::Save,
                };
                let s_copy = ShortCut {
                    name: Action::Copy.to_string(),
                    shortcut: KeyboardShortcut::new(Modifiers::COMMAND, Key::C),
                    is_active: true,
                    wants_image_viewer: true,
                    action: Action::Copy,
                };
                let s_undo = ShortCut {
                    name: Action::Undo.to_string(),
                    shortcut: KeyboardShortcut::new(Modifiers::COMMAND, Key::Z),
                    is_active: true,
                    wants_image_viewer: true,
                    action: Action::Undo,
                };
                let s_start_timer = ShortCut {
                    name: Action::StartTimer.to_string(),
                    shortcut: KeyboardShortcut::new(Modifiers::COMMAND | Modifiers::SHIFT, Key::T),
                    is_active: true,
                    wants_image_viewer: false,
                    action: Action::StartTimer,
                };
                output.push(s_save);
                output.push(s_set_entire_screen);
                output.push(s_set_selection);
                output.push(s_open_timer);
                output.push(s_start_timer);
                output.push(s_cancel_timer);
                output.push(s_options);
                output.push(s_capture);
                output.push(s_close);
                output.push(s_modify);
                output.push(s_another_screenshot);
                output.push(s_copy);
                output.push(s_undo);
                
            }

            Self {
                set: output,
                show: false,
            }
        }


       

        pub fn insert_new_shortcut(&mut self, new_shortcut: &mut NewShortcut) -> Option<ShortCut> {
            if let Some(_) = new_shortcut.action {
                if let Some(_) = new_shortcut.key {
                    if !new_shortcut.modifier.is_none() {
                        let new_sc = ShortCut::shortcut_builder(
                            new_shortcut.modifier,
                            new_shortcut.key.unwrap(),
                            new_shortcut.action.unwrap(),
                        );
                        for sc in self.set.iter() {
                            if sc.shortcut.eq(&new_sc.shortcut) {
                                return None;
                            }
                        }
                        self.set.push(new_sc.clone());
                        Some(new_sc.clone())
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }
        pub fn delete_shotucut(&mut self, shortcut: &mut ShortCut) {
            let mut delete_index = 0;
            for (i, sc) in self.set.iter().enumerate() {
                if sc.shortcut.eq(&shortcut.shortcut) {
                    delete_index = i
                }
            }
            self.set.remove(delete_index);
        }
        pub fn listener(&self, ctx: &egui::Context, is_image: bool) -> Option<Action> {
            for sc in self.set.iter() {
                if sc.action == Action::Options || sc.action ==Action::Close {        
                    if sc.is_active {
                        if let Some(opt_action) = sc.listener_shortcut(ctx) {
                            return Some(opt_action);
                        }
                    }
                } else {
                    if sc.wants_image_viewer == is_image && sc.is_active {
                        if let Some(opt_action) = sc.listener_shortcut(ctx) {
                            return Some(opt_action);
                        }
                    }
                }
            }
            None
        }

        pub fn change_active(&mut self, shortcut: &mut ShortCut){

            for sc in self.set.iter_mut(){

                if sc.shortcut.eq(&shortcut.shortcut){

                    sc.change_active();
                }
            }
        }

        pub fn to_vec_mut(&mut self) -> Vec<&mut ShortCut> {
            let mut output = Vec::new();
            for sc in self.set.iter_mut() {
                output.push(sc);
            }
            output
        }
    }
}
