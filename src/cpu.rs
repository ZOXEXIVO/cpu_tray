extern crate cpu_monitor;

use std::thread;
use std::time::Duration;

use cpu_monitor::CpuInstant;

const DEFAULT_DURATION: Duration = Duration::from_millis(500);

pub struct Cpu;

impl Cpu {
    pub fn current_load(usage_duration: Option<Duration>) -> u8 {
        let cpu_before = CpuInstant::now().unwrap();

        let sleep_duration = match usage_duration {
            Some(duration) => duration,
            None => DEFAULT_DURATION
        };

        thread::sleep(sleep_duration);

        let cpu_after = CpuInstant::now().unwrap();

        let duration = cpu_after - cpu_before;

        (duration.non_idle() * 100.) as u8
    }
}