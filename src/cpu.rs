extern crate cpu_monitor;

use cpu_monitor::CpuInstant;

pub struct Cpu;

impl Cpu {
    pub fn current_load() -> u8 {
        let cpu_before = CpuInstant::now().unwrap();

        std::thread::sleep(std::time::Duration::from_millis(1000));
        
        let cpu_after = CpuInstant::now().unwrap();

        let duration = cpu_after - cpu_before;

        (duration.non_idle() * 100.) as u8
    }
}