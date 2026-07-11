use std::cell::RefCell;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::mem::{size_of, zeroed};
use std::os::windows::ffi::OsStrExt;
use winapi::shared::windef::HWND;
use winapi::um::shellapi::*;
use winapi::um::winuser::WM_USER;
use crate::icon::IconGenerator;

/// Message the shell posts to our window when the tray icon is interacted with.
pub const TRAY_CALLBACK_MESSAGE: u32 = WM_USER + 1;

/// Stable identifier for our single tray icon (per owning window).
const TRAY_ICON_ID: u32 = 1;

pub struct TrayIcon {
    current_value: u8,
    icon_generator: IconGenerator,
    tooltip_generator: TooltipGenerator,
    nid: NOTIFYICONDATAW,
}

impl TrayIcon {
    pub fn new(hwnd: HWND) -> Self {
        let mut tray = TrayIcon {
            current_value: 0,
            icon_generator: IconGenerator::new(),
            tooltip_generator: TooltipGenerator::new(),
            nid: unsafe { zeroed() },
        };

        tray.nid.cbSize = size_of::<NOTIFYICONDATAW>() as u32;
        tray.nid.hWnd = hwnd;
        tray.nid.uID = TRAY_ICON_ID;
        tray.nid.uCallbackMessage = TRAY_CALLBACK_MESSAGE;
        tray.nid.hIcon = tray.icon_generator.generate(0);
        tray.nid.szTip = tray.tooltip_generator.generate_tooltip("CPU".to_string());
        tray.nid.uFlags = NIF_ICON | NIF_MESSAGE | NIF_TIP;

        tray.add();

        tray
    }

    /// (Re)register the icon with the shell. Idempotent: deletes any stale
    /// registration first, so it is safe to call on startup and whenever the
    /// taskbar is recreated (Explorer restart).
    fn add(&mut self) {
        unsafe {
            // Clear a possible leftover from a previous instance that died
            // without running Drop, otherwise NIM_ADD can silently fail.
            Shell_NotifyIconW(NIM_DELETE, &mut self.nid);

            if Shell_NotifyIconW(NIM_ADD, &mut self.nid) == 0 {
                // One more attempt after an explicit delete.
                Shell_NotifyIconW(NIM_DELETE, &mut self.nid);
                Shell_NotifyIconW(NIM_ADD, &mut self.nid);
            }

            *self.nid.u.uVersion_mut() = NOTIFYICON_VERSION_4;
            Shell_NotifyIconW(NIM_SETVERSION, &mut self.nid);
        }
    }

    /// Called when the taskbar is recreated (the `TaskbarCreated` broadcast).
    /// Re-adds the icon and restores the currently displayed state.
    pub fn restore(&mut self) {
        self.add();
        let value = self.current_value;
        self.apply(value);
    }

    pub fn update(&mut self, value: u8) {
        self.apply(value);
    }

    fn apply(&mut self, value: u8) {
        self.nid.hIcon = self.icon_generator.generate(value);
        self.nid.szTip = self.tooltip_generator
            .generate_tooltip(format!("CPU: {}%", value));
        self.nid.uFlags = NIF_ICON | NIF_MESSAGE | NIF_TIP;
        self.current_value = value;

        unsafe {
            if Shell_NotifyIconW(NIM_MODIFY, &mut self.nid) == 0 {
                // The icon no longer exists (e.g. Explorer was restarted and we
                // somehow missed the TaskbarCreated broadcast). Re-add it so the
                // icon self-heals within one update cycle.
                self.add();
                Shell_NotifyIconW(NIM_MODIFY, &mut self.nid);
            }
        }
    }
}

impl Drop for TrayIcon {
    fn drop(&mut self) {
        unsafe {
            Shell_NotifyIconW(NIM_DELETE, &mut self.nid);
        }
    }
}

pub struct TooltipGenerator {
    tooltip_cache: RefCell<HashMap<String, [u16; 128]>>,
}

impl TooltipGenerator {
    pub fn new() -> Self {
        TooltipGenerator {
            tooltip_cache: RefCell::new(HashMap::with_capacity(101)),
        }
    }

    fn generate_tooltip(&self, tooltip: String) -> [u16; 128] {
        let mut cache = self.tooltip_cache.borrow_mut();

        match cache.get_mut(&tooltip) {
            Some(cached_value) => *cached_value,
            None => {
                let tooltip_data = Self::create_tooltip_inner(&tooltip);
                cache.insert(tooltip, tooltip_data);
                tooltip_data
            }
        }
    }

    fn create_tooltip_inner(tooltip: &String) -> [u16; 128] {
        let mut tray_tool_tip_int: [u16; 128] = [0; 128];

        let tray_tool_tip_step_os = OsStr::new(tooltip);
        let tray_tool_tip_step_utf16 = tray_tool_tip_step_os.encode_wide().collect::<Vec<u16>>();

        // Leave room for the trailing NUL; truncate over-long tips defensively.
        let len = tray_tool_tip_step_utf16.len().min(127);
        tray_tool_tip_int[..len].copy_from_slice(&tray_tool_tip_step_utf16[..len]);

        tray_tool_tip_int
    }
}
