use crate::memory::Memory;
use crate::memory_map::MemoryMap;
use crate::registers::timer::*;
use crate::registers::Interrupt;

#[derive(Debug)]
struct TimerRegisters {
    div: u8,
    tima: u8,
    tma: u8,
    tac: TimerControl,
    if_reg: Interrupt,
}

impl TimerRegisters {
    pub fn read(mem: &Memory) -> Self {
        let div = mem.get_u8(MemoryMap::DIV);
        let tima = mem.get_u8(MemoryMap::TIMA);
        let tma = mem.get_u8(MemoryMap::TMA);
        let tac = mem.get_reg::<TimerControl>(MemoryMap::TAC);
        let if_reg = mem.get_reg::<Interrupt>(MemoryMap::IF);

        Self {
            div,
            tima,
            tma,
            tac,
            if_reg,
        }
    }

    pub fn write(self, mem: &mut Memory) {
        mem.set_u8(MemoryMap::DIV, self.div);
        mem.set_u8(MemoryMap::TIMA, self.tima);
        mem.set_reg(MemoryMap::IF, self.if_reg);
    }
}

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
    pub fn tick(&mut self, mem: &mut Memory) {
        let mut regs = TimerRegisters::read(mem);

        for _ in 0..4 {
            self.t_cycle(&mut regs);
        }

        regs.write(mem);
    }

    fn t_cycle(&mut self, regs: &mut TimerRegisters) {
        self.counter = self.counter.wrapping_add(1);
        regs.div = (self.counter >> 8) as u8;

        if !regs.tac.contains(TimerControl::Enable) {
            return;
        }

        let falling_edge = self
            .divider_fed
            .tick(self.counter & regs.tac.get_speed() != 0);
        if falling_edge {
            regs.tima = regs.tima.wrapping_add(1);
            if regs.tima == 0 {
                self.interrupt_in = 4;
            }
        }

        if self.interrupt_in > 0 {
            self.interrupt_in -= 1;
            if self.interrupt_in == 0 {
                regs.if_reg.set(Interrupt::Timer, true);
                regs.tima = regs.tma;
            }
        }
    }
}
