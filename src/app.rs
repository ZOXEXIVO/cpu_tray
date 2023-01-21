use std::time::Duration;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};
use crate::cpu::Cpu;
use crate::tray::TrayIcon;

pub struct CpuApp {
    window: Window
}

impl CpuApp {
    pub fn new() -> Self {
        CpuApp {
            window: WindowBuilder::new()
                .with_visible(false)
                .build(&EventLoop::new()).unwrap()
        }
    }

    pub fn run(self) {
        let mut tray_icon = TrayIcon::new(&self.window);

        loop {
            let cpu_load = Cpu::current_load();

            tray_icon.update(cpu_load);
        }
    }
}