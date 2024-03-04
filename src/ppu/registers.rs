use crate::memory::Memory;
use crate::memory_map::MemoryMap;
use crate::registers::graphics::*;
use crate::registers::Interrupt;

pub struct Registers {
    pub lcdc: LcdControl,
    pub stat: LcdStatus,
    pub scy: u8,
    pub scx: u8,
    pub ly: u8,
    pub lyc: u8,
    pub if_reg: Interrupt,
}

impl Registers {
    pub fn read(mem: &Memory) -> Self {
        let lcdc = mem.get_reg::<LcdControl>(MemoryMap::LCDC);
        let stat = mem.get_reg::<LcdStatus>(MemoryMap::STAT);
        let scy = mem.get_u8(MemoryMap::SCY);
        let scx = mem.get_u8(MemoryMap::SCX);
        let ly = mem.get_u8(MemoryMap::LY);
        let lyc = mem.get_u8(MemoryMap::LYC);
        let if_reg = mem.get_reg::<Interrupt>(MemoryMap::IF);

        Self {
            lcdc,
            stat,
            scy,
            scx,
            ly,
            lyc,
            if_reg,
        }
    }

    pub fn write(self, mem: &mut Memory) {
        mem.set_u8(MemoryMap::LY, self.ly);
        mem.set_reg(MemoryMap::STAT, self.stat);
        mem.set_reg(MemoryMap::IF, self.if_reg);
    }
}
