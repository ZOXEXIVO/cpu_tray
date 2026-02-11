#![windows_subsystem = "windows"]

mod cpu;
mod tray;
mod icon;
mod app;

use crate::app::CpuApp;

fn main() {
    CpuApp::run();
}
