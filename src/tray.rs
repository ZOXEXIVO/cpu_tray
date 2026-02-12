use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{OsStr};
use std::mem::{size_of, zeroed};
use std::os::windows::ffi::OsStrExt;
use winapi::shared::guiddef::GUID;
use winapi::um::shellapi::*;
use winapi::um::winuser::WM_USER;
use crate::icon::IconGenerator;
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use winit::window::Window;

pub struct TrayIcon {
    current_value: u8,
    icon_generator: IconGenerator,
    tooltip_generator: TooltipGenerator,
    nid: winapi::um::shellapi::NOTIFYICONDATAW
}

impl TrayIcon {
    pub fn new(parent_window: &Window) -> Self {
        TrayIcon {
            current_value: 0,
            icon_generator: IconGenerator::new(),
            tooltip_generator: TooltipGenerator::new(),
            nid: TrayIcon::create_icon(parent_window),
        }
    }

    fn create_icon(parent_window: &Window) -> NOTIFYICONDATAW {
        let mut nid: NOTIFYICONDATAW = unsafe { zeroed() };

        let hwnd = match parent_window.window_handle().unwrap().as_raw() {
            RawWindowHandle::Win32(handle) => handle.hwnd.get() as _,
            _ => panic!("unsupported platform"),
        };

        nid.cbSize = size_of::<NOTIFYICONDATAW>() as u32;
        nid.hWnd = hwnd;

        // REQUIRED when using NIF_GUID
        nid.guidItem = GUID {
            Data1: 0x12345678,
            Data2: 0x1234,
            Data3: 0x5678,
            Data4: [0x12, 0x34, 0x56, 0x78, 0x9A, 0xBC, 0xDE, 0xF0],
        };

        nid.hIcon = IconGenerator::new().generate(0);

        nid.uCallbackMessage = WM_USER + 1;

        nid.uFlags = NIF_GUID | NIF_ICON | NIF_MESSAGE;

        let ok = unsafe { Shell_NotifyIconW(NIM_ADD, &mut nid) };

        // REQUIRED on Windows 10+
        unsafe {
            *nid.u.uVersion_mut() = winapi::um::shellapi::NOTIFYICON_VERSION_4;
            Shell_NotifyIconW(NIM_SETVERSION, &mut nid);
        }

        nid
    }

    pub fn update(&mut self, value: u8) {
        self.nid.hIcon = self.icon_generator.generate(value);

        let tip = self.tooltip_generator.generate_tooltip(
            format!("CPU: {}%", value)
        );

        self.nid.szTip = tip;

        self.nid.uFlags = NIF_GUID | NIF_ICON | NIF_MESSAGE | NIF_TIP;

        self.current_value = value;

        unsafe {
            Shell_NotifyIconW(NIM_MODIFY, &mut self.nid);
        }
    }
}

impl Drop for TrayIcon {
    fn drop(&mut self) {
        unsafe {
            winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_DELETE, &mut self.nid)
        };
    }
}

pub struct TooltipGenerator{
    tooltip_cache: RefCell<HashMap<String, [u16; 128]>>
}

impl TooltipGenerator {
    pub fn new() -> Self {
        TooltipGenerator{
            tooltip_cache: RefCell::new(HashMap::with_capacity(101))
        }
    }

    fn generate_tooltip(&self, tooltip: String) -> [u16; 128] {
        let mut cache = self.tooltip_cache.borrow_mut();

        return match cache.get_mut(&tooltip) {
            Some(cached_value) => {
                *cached_value
            },
            None => {
                let tooltip_data = Self::create_tooltip_inner(&tooltip);

                cache.insert(tooltip, tooltip_data);

                tooltip_data
            }
        };
    }

    fn create_tooltip_inner(tooltip: &String) -> [u16; 128]{
        let mut tray_tool_tip_int: [u16; 128] = [0; 128];

        let tray_tool_tip_step_os = OsStr::new(tooltip);
        let tray_tool_tip_step_utf16 = tray_tool_tip_step_os.encode_wide().collect::<Vec<u16>>();

        tray_tool_tip_int[..tray_tool_tip_step_utf16.len()].copy_from_slice(&tray_tool_tip_step_utf16);

        tray_tool_tip_int
    }
}
