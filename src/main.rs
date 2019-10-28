mod cpu;
mod tray;
mod icon;

use cpu::Cpu;
use tray::TrayIcon;
use icon::IconGenerator;

fn main() {
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
