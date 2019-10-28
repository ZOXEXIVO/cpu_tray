mod cpu;
mod tray;
mod icon;
mod console;

use cpu::Cpu;
use tray::TrayIcon;
use icon::IconGenerator;
use console::Console;

fn main() {
    Console::hide();

    let default_tooltip = "CpuTray by Artemov Ivan (zoxexivo@gmail.com)".to_string();

    let mut tray_icon = TrayIcon::new(
        default_tooltip, 
        IconGenerator::new()
    );

    loop {
        let current_load = Cpu::get_load(None);

        tray_icon.update(current_load);  
    }
}
