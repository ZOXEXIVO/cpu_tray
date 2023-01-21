#![windows_subsystem = "windows"]

mod cpu;
mod tray;
mod icon;
mod app;

use cpu::Cpu;
use tray::TrayIcon;
use icon::IconGenerator;
use winit::{
    event_loop::{ EventLoop},
    window::WindowBuilder,
};
use crate::app::CpuApp;

fn main() {
    CpuApp::new().run();
}