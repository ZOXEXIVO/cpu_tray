pub struct Console;

use std::ptr;

impl Console {
    pub fn hide() {
        unsafe {
            let window = winapi::um::wincon::GetConsoleWindow();

            if window != ptr::null_mut() {
                winapi::um::winuser::ShowWindow(window, winapi::um::winuser::SW_HIDE);
            }
        }
    }
}
