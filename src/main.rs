#![windows_subsystem = "windows"]

mod cpu;
mod tray;
mod icon;
mod app;

use std::ffi::OsStr;
use std::os::windows::ffi::OsStrExt;
use std::ptr::null_mut;

use winapi::shared::winerror::ERROR_ALREADY_EXISTS;
use winapi::um::errhandlingapi::GetLastError;
use winapi::um::synchapi::CreateMutexW;
use winreg::enums::{HKEY_CURRENT_USER, KEY_WRITE};
use winreg::RegKey;

use crate::app::CpuApp;

fn main() {
    // Only one instance may run, otherwise autostart plus a manual launch
    // would leave two icons in the tray.
    if already_running() {
        return;
    }

    // Autostart registration is best-effort — a failure here (e.g. locked-down
    // registry) must never stop the app from actually running.
    let _ = add_to_autostart();

    CpuApp::run();
}

fn already_running() -> bool {
    let name = OsStr::new("CpuTray_SingleInstance_Mutex")
        .encode_wide()
        .chain(std::iter::once(0))
        .collect::<Vec<u16>>();

    unsafe {
        // Held for the whole process lifetime; the handle is intentionally not
        // closed. If creation itself fails we cannot tell, so err on the side
        // of running.
        let handle = CreateMutexW(null_mut(), 0, name.as_ptr());
        !handle.is_null() && GetLastError() == ERROR_ALREADY_EXISTS
    }
}

fn add_to_autostart() -> Result<(), Box<dyn std::error::Error>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let run = hkcu.open_subkey_with_flags(
        "Software\\Microsoft\\Windows\\CurrentVersion\\Run",
        KEY_WRITE,
    )?;

    let exe_path = std::env::current_exe()?;
    run.set_value("MyRustApp", &exe_path.to_string_lossy().to_string())?;

    Ok(())
}
