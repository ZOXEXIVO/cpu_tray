use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::{OsStr};
use std::mem::{size_of, zeroed};
use std::os::windows::ffi::OsStrExt;

use crate::IconGenerator;
use winapi::shared::windef::{HWND};
use winit::platform::windows::WindowExtWindows;
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

    fn create_icon(parent_window: &Window) -> winapi::um::shellapi::NOTIFYICONDATAW {
        let mut nid: winapi::um::shellapi::NOTIFYICONDATAW = unsafe { zeroed() };
        unsafe {
            nid.cbSize = size_of::<winapi::um::shellapi::NOTIFYICONDATAW>() as u32;
            nid.hWnd = std::mem::transmute::<isize, winapi::shared::windef::HWND>(parent_window.hwnd());
            
            nid.hIcon = IconGenerator::new().generate(0);

            nid.uFlags = winapi::um::shellapi::NIF_GUID | winapi::um::shellapi::NIF_ICON;
        };

        unsafe { winapi::um::shellapi::Shell_NotifyIconW(winapi::um::shellapi::NIM_ADD, &mut nid) };

        nid
    }    

    pub fn update(&mut self, value: u8) {
        self.nid.hIcon = self.icon_generator.generate(value);
        self.nid.szTip = self.tooltip_generator.generate_tooltip(format!("CPU: {}%", value));
                
        self.nid.uFlags = winapi::um::shellapi::NIF_ICON | winapi::um::shellapi::NIF_TIP;
        
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