use std::sync::mpsc;
use std::time::{Duration, Instant};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowAttributes, WindowId};
use crate::cpu::Cpu;
use crate::tray::TrayIcon;

pub struct CpuApp {
    window: Option<Window>,
    tray_icon: Option<TrayIcon>,
    cpu_rx: mpsc::Receiver<u8>,
}

impl CpuApp {
    pub fn run() {
        let event_loop = EventLoop::new().unwrap();

        let (tx, rx) = mpsc::channel();

        std::thread::spawn(move || {
            loop {
                let cpu_load = Cpu::current_load();
                if tx.send(cpu_load).is_err() {
                    break;
                }
            }
        });

        let mut app = CpuApp {
            window: None,
            tray_icon: None,
            cpu_rx: rx,
        };

        event_loop.run_app(&mut app).unwrap();
    }
}

impl ApplicationHandler for CpuApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_none() {
            let attrs = WindowAttributes::default().with_visible(false);
            let window = event_loop.create_window(attrs).unwrap();
            self.tray_icon = Some(TrayIcon::new(&window));
            self.window = Some(window);
        }

        event_loop.set_control_flow(ControlFlow::WaitUntil(
            Instant::now() + Duration::from_millis(200),
        ));
    }

    fn about_to_wait(&mut self, event_loop: &ActiveEventLoop) {
        if let Ok(cpu_load) = self.cpu_rx.try_recv() {
            if let Some(tray) = &mut self.tray_icon {
                tray.update(cpu_load);
            }
        }

        event_loop.set_control_flow(ControlFlow::WaitUntil(
            Instant::now() + Duration::from_millis(200),
        ));
    }

    fn window_event(&mut self, _event_loop: &ActiveEventLoop, _window_id: WindowId, _event: WindowEvent) {
    }
}
