pub mod instr;

use std::fmt::Display;

use crate::cpu::instr::execute_instr;
use crate::memory::CpuMemory;
use crate::memory_map::MemoryMap;
use crate::registers::{CpuFlags, Interrupt};

pub struct InterruptChange {
    next_state: bool,
    change_in: usize,
}

impl InterruptChange {
    pub fn di() -> Self {
        Self {
            next_state: false,
            change_in: 2,
        }
    }
    pub fn ei() -> Self {
        Self {
            next_state: true,
            change_in: 2,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum State {
    Running,
    Stopped,
    Halted,
}

pub struct Cpu {
    pub a: u8,
    pub f: CpuFlags,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    pub sp: u16,
    pub pc: u16,

    pub wait: u8,
    pub ie: bool,
    pub interrupt_change: Option<InterruptChange>,

    pub state: State,
}

impl Display for Cpu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "a: {:#x}", self.a)?;
        writeln!(f, "f: {:#x}", self.f)?;
        writeln!(f, "bc: {:#x}", self.get_bc())?;
        writeln!(f, "de: {:#x}", self.get_de())?;
        writeln!(f, "hl: {:#x}", self.get_hl())?;
        writeln!(f, "sp: {:#x}", self.sp)?;
        writeln!(f, "pc: {:#x}", self.pc)?;
        write!(f, "state: {:?}", self.state)
    }
}

impl Default for Cpu {
    fn default() -> Self {
        Self {
            a: 0x01,
            f: CpuFlags::from_bits(0xb0).unwrap(),
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xd8,
            h: 0x01,
            l: 0x4d,
            sp: 0xfffe,
            pc: 0x0000,

            wait: 0,
            interrupt_change: None,
            ie: false,

            state: State::Running,
        }
    }
}

pub struct Cycles(u8);

impl Cpu {
    pub fn get_af(&self) -> u16 {
        (self.a as u16) << 8 | self.f.bits() as u16
    }

    pub fn set_af(&mut self, val: u16) {
        self.a = (val >> 8) as u8;
        self.f = CpuFlags::from_bits(val as u8).unwrap();
    }

    pub fn get_bc(&self) -> u16 {
        (self.b as u16) << 8 | self.c as u16
    }

    pub fn set_bc(&mut self, val: u16) {
        self.b = (val >> 8) as u8;
        self.c = val as u8;
    }

    pub fn get_de(&self) -> u16 {
        (self.d as u16) << 8 | self.e as u16
    }

    pub fn set_de(&mut self, val: u16) {
        self.d = (val >> 8) as u8;
        self.e = val as u8;
    }

    pub fn get_sp(&self) -> u16 {
        self.sp
    }

    pub fn set_sp(&mut self, val: u16) {
        self.sp = val;
    }

    pub fn get_hl(&self) -> u16 {
        (self.h as u16) << 8 | self.l as u16
    }

    pub fn set_hl(&mut self, val: u16) {
        self.h = (val >> 8) as u8;
        self.l = val as u8;
    }

    fn handle_interrupt(
        &mut self,
        mem: &mut impl CpuMemory,
        reg: &Interrupt,
        bit: Interrupt,
        vector: MemoryMap,
    ) -> bool {
        if reg.contains(bit) {
            self.state = State::Running;
            if self.ie {
                // Reset request flag
                mem.clear_reg_flag(MemoryMap::IF, bit);
                self.ie = false;

                self.sp -= 2;
                mem.set_u16(self.sp, self.pc);

                self.pc = vector.into();

                self.wait = 4;
                return true;
            }
        }
        false
    }

    // Handle interrupts
    // http://gbdev.gg8.se/wiki/articles/Interrupts
    fn handle_interrupts(&mut self, mem: &mut impl CpuMemory) -> bool {
        let enabled = Interrupt::from_bits(mem.get_u8(MemoryMap::IE)).unwrap();
        let requested = Interrupt::from_bits(mem.get_u8(MemoryMap::IF)).unwrap();
        let enabled_and_requested = enabled.intersection(requested);

        if self.handle_interrupt(
            mem,
            &enabled_and_requested,
            Interrupt::VBlank,
            MemoryMap::InterruptVBlank,
        ) {
            return true;
        }
        if self.handle_interrupt(
            mem,
            &enabled_and_requested,
            Interrupt::LcdStat,
            MemoryMap::InterruptLcdStat,
        ) {
            return true;
        }
        if self.handle_interrupt(
            mem,
            &enabled_and_requested,
            Interrupt::Timer,
            MemoryMap::InterruptTimer,
        ) {
            return true;
        }
        if self.handle_interrupt(
            mem,
            &enabled_and_requested,
            Interrupt::Serial,
            MemoryMap::InterruptSerial,
        ) {
            return true;
        }
        self.handle_interrupt(
            mem,
            &enabled_and_requested,
            Interrupt::Joypad,
            MemoryMap::InterruptJoypad,
        )
    }

    pub fn tick(&mut self, mem: &mut impl CpuMemory) {
        if self.wait > 0 {
            self.wait -= 1;
            return;
        }

        if self.state != State::Stopped && self.handle_interrupts(mem) {
            return;
        }

        if self.state != State::Running {
            return;
        }

        let Cycles(delay) = execute_instr(self, mem);
        self.wait = delay / 4 - 1;

        if let Some(ref mut change) = self.interrupt_change {
            if change.change_in > 0 {
                change.change_in -= 1;

                if change.change_in == 0 {
                    self.ie = change.next_state;
                    self.interrupt_change = None;
                }
            }
        };
    }
}
