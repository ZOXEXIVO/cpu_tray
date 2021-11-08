#![windows_subsystem = "windows"]

mod cpu;
mod tray;
mod icon;

use cpu::Cpu;
use tray::TrayIcon;
use icon::IconGenerator;
use winit::{
    event_loop::{ EventLoop},
    window::WindowBuilder,
};

fn main() {
    let default_invisible_window = WindowBuilder::new()
        .with_visible(false)
        .build(&EventLoop::new()).unwrap();

    let mut tray_icon = TrayIcon::new(&default_invisible_window);

    loop {
        let current_load = Cpu::current_load(None);

        tray_icon.update(current_load);  
    }
}