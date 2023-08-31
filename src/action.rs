#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Action {
    SetEntireScreen, //ctrl+F
    SetSelection,    //ctrl+ArrowDown
    SettingTimer,    //ctrl+T
    StartTimer,
    HandleTimer,
    CancelTimer,           //ctrl+alt+T
    Options,               //ctrl+O
    Capture,               //ctrl+enter
    Close,                 //ctrl+X
    Modify,                //ctrl+M
    TakeAnotherScreenshot, //ctrl+A
    Save,                  //ctrl+S
    Copy,                  //Ctrl+C
    Undo,                  //Ctrl+Z
}

impl Action {

    
    pub fn to_string(self) -> String {
        match self {
            Action::SetEntireScreen => String::from("Set entire screen"),
            Action::SetSelection => String::from("Set selection"),
            Action::SettingTimer => String::from("Open timer"),
            Action::StartTimer => String::from("Start timer"),
            Action::HandleTimer =>String::from("handle timer"),
            Action::CancelTimer => String::from("Cancel timer"),
            Action::Options => String::from("Options"),
            Action::Capture => String::from("Capture"),
            Action::Close => String::from("Close"),
            Action::Modify => String::from("Modify"),
            Action::TakeAnotherScreenshot => String::from("Take another screenshot"),
            Action::Save => String::from("Save"),
            Action::Copy => String::from("Copy"),
            Action::Undo => String::from("Undo modify"),
        }
    }
    pub fn wants_image_viewer(self) -> bool {
        match self {
            Action::SetEntireScreen => false,
            Action::SetSelection => false,
            Action::SettingTimer => false,
            Action::StartTimer => false,
            Action::HandleTimer => false,
            Action::CancelTimer => false,
            Action::Options => false,
            Action::Capture => false,
            Action::Close => false,
            Action::Modify => true,
            Action::TakeAnotherScreenshot => true,
            Action::Save => true,
            Action::Copy => true,
            Action::Undo => true,
        }
    }
}
pub struct AllActionArr {
    pub all_action: Vec<Action>,
}

impl AllActionArr {
    pub fn new() -> Self {
        Self {
            all_action: vec![
                Action::SetEntireScreen,
                Action::SetSelection,
                Action::SettingTimer,
                Action::StartTimer,
               
                Action::CancelTimer,
                Action::Options,
                Action::Capture,
                Action::Close,
                Action::Modify,
                Action::TakeAnotherScreenshot,
                Action::Save,
                Action::Copy,
                Action::Undo,
            ],
        }
    }
}
