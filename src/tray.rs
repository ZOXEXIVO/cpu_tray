extern crate winapi;

use std::ffi::OsStr;
use std::mem::{size_of, zeroed};
use std::os::windows::ffi::OsStrExt;

use crate::IconGenerator;

pub struct TrayIcon {
    current_value: u8,
    icon_generator: IconGenerator,
    nid: winapi::um::shellapi::NOTIFYICONDATAW,
}

impl TrayIcon {
    pub fn new(tooltip: String, icon_generator: IconGenerator) -> Self {
        TrayIcon {
            current_value: 0,
            icon_generator: icon_generator,
            nid: TrayIcon::create_icon(tooltip),
        }
    }

    fn create_icon(tray_tool_tip: String) -> winapi::um::shellapi::NOTIFYICONDATAW {
        let mut tray_tool_tip_int: [u16; 128] = [0; 128];

        let tray_tool_tip_step_os = OsStr::new(&*tray_tool_tip);
        let tray_tool_tip_step_utf16 = tray_tool_tip_step_os.encode_wide().collect::<Vec<u16>>();

        tray_tool_tip_int[..tray_tool_tip_step_utf16.len()].copy_from_slice(&tray_tool_tip_step_utf16);

        let mut nid: winapi::um::shellapi::NOTIFYICONDATAW = unsafe { zeroed() };
        unsafe {
            nid.cbSize = size_of::<winapi::um::shellapi::NOTIFYICONDATAW>() as u32;
            nid.hWnd = winapi::um::wincon::GetConsoleWindow();
            nid.uID = 1001;
            nid.uCallbackMessage = winapi::um::winuser::WM_APP + 100;

            nid.hIcon = IconGenerator::new().generate(0);

            nid.szTip = tray_tool_tip_int;
            nid.uFlags = winapi::um::shellapi::NIF_MESSAGE
                | winapi::um::shellapi::NIF_ICON
                | winapi::um::shellapi::NIF_TIP;
        };

        unsafe { winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_ADD, &mut nid) };

        nid
    }

    pub fn update(&mut self, value: u8) {
        if self.current_value == value {
            return;
        }

        self.nid.hIcon = self.icon_generator.generate(value);

        self.current_value = value;

        unsafe {
            winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_MODIFY, &mut self.nid)
        };
    }
}

impl Drop for TrayIcon {
    fn drop(&mut self) {
        unsafe {
            winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_DELETE, &mut self.nid)
        };
    }
}
