use std::time::{Duration, Instant};

use sysinfo::{Pid, ProcessesToUpdate, System, get_current_pid};

use crate::meter::process_memory::ProcessMemory;

pub struct MemoryMeter {
    system: Option<System>,
    pid: Option<Pid>,
    last: Option<(Instant, ProcessMemory)>,
}

impl MemoryMeter {
    pub fn new() -> Self {
        Self {
            system: System::new().ok(),
            pid: get_current_pid().ok(),
            last: None,
        }
    }

    pub fn sample(&mut self) -> Option<ProcessMemory> {
        let pid = self.pid?;
        let system = self.system.as_mut()?;
        system.refresh_processes(ProcessesToUpdate::Some(&[pid]), true);
        let process = system.process(pid)?;
        let memory = ProcessMemory {
            resident_bytes: process.memory(),
            virtual_bytes: process.virtual_memory(),
        };
        self.last = Some((Instant::now(), memory));
        Some(memory)
    }

    pub fn sample_if_stale(&mut self, max_age: Duration) -> Option<ProcessMemory> {
        if let Some((taken, memory)) = self.last
            && taken.elapsed() < max_age
        {
            return Some(memory);
        }
        self.sample()
    }
}

impl Default for MemoryMeter {
    fn default() -> Self {
        Self::new()
    }
}
