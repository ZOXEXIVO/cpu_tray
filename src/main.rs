#![windows_subsystem = "windows"]

mod cpu;
mod tray;
mod icon;
mod app;

use winreg::enums::{HKEY_CURRENT_USER, KEY_WRITE};
use winreg::RegKey;
use crate::app::CpuApp;

fn main() {
    add_to_autostart().expect("failed to set up autostart");
    
    CpuApp::run();
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