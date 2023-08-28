
#[derive( Clone, Copy)]
pub enum Action {
    SetEntireScreen, //ctrl+F
    SetSelection,    //ctrl+ArrowDown
    SettingTimer,          //ctrl+T
    StartTimer,            //ctrl+T+S
    CancelTimer,           //ctrl+T+X
    Options,               //ctrl+O
    Capture,               //ctrl+C
    Close,                 //ctrl+X
    Modify,                //ctrl+M
    TakeAnotherScreenshot, //ctrl+A
    Save,                  //ctrl+S
}