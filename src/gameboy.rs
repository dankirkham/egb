use crate::cpu::Cpu;
use crate::debugger::Debugger;
use crate::memory::Memory;
use crate::ppu::Ppu;
use crate::serial::Serial;
use crate::timer::Timer;

pub const CLOCK_SPEED_HZ: u64 = 4_194_304 / 4;

pub struct Gameboy {
    pub cpu: Cpu,
    pub mem: Memory,
    serial: Serial,
    timer: Timer,
    pub debugger: Option<Debugger>,
    pub ppu: Ppu,
}

impl Gameboy {
    pub fn new(mem: Memory) -> Self {
        let serial = Serial;
        let timer = Timer::default();
        let cpu = Cpu::default();
        let ppu = Ppu::default();

        Self {
            cpu,
            mem,
            serial,
            timer,
            debugger: None,
            ppu,
        }
    }

    /// Run Gameboy for one M-cycle
    pub fn tick(&mut self) -> Option<u8> {
        if let Some(ref mut debugger) = self.debugger {
            if !debugger.tick(&self.cpu, &self.mem) {
                return None;
            }
        }

        self.timer.tick(&mut self.mem);
        self.ppu.tick(&mut self.mem);
        let out = self.serial.tick(&mut self.mem);
        self.cpu.tick(&mut self.mem);
        out
    }

    pub fn attach_debugger(&mut self, debugger: Option<Debugger>) {
        self.debugger = debugger;
    }

    pub fn detach_debugger(&mut self) -> Option<Debugger> {
        self.debugger.take()
    }
}
