use crate::memory::PrivilegedMemory;
use crate::memory_map::MemoryMap;
use crate::registers::timer::*;
use crate::registers::Interrupt;

#[derive(Default)]
pub struct FallingEdgeDetector {
    last: bool,
}

impl FallingEdgeDetector {
    pub fn tick(&mut self, next: bool) -> bool {
        let result = self.last && !next;
        self.last = next;
        result
    }
}

#[derive(Default)]
pub struct Timer {
    counter: u16,
    divider_fed: FallingEdgeDetector,
    interrupt_in: usize,
}

impl Timer {
    pub fn tick(&mut self, mem: &mut impl PrivilegedMemory) {
        self.counter = self.counter.wrapping_add(1);
        mem.set_u8(MemoryMap::DIV, (self.counter >> 8) as u8);

        let tac = mem.get_reg::<TimerControl>(MemoryMap::TAC);
        if !tac.contains(TimerControl::Enable) {
            return;
        }

        let falling_edge = self.divider_fed.tick(self.counter & tac.get_speed() != 0);
        if falling_edge && mem.inc_u8(MemoryMap::TIMA) {
            self.interrupt_in = 4;
        }

        if self.interrupt_in > 0 {
            self.interrupt_in -= 1;
            if self.interrupt_in == 0 {
                mem.set_reg_flag(MemoryMap::IF, Interrupt::Timer);
                mem.set_u8(MemoryMap::TIMA, mem.get_u8(MemoryMap::TMA));
            }
        }
    }
}
