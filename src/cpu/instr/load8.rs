use crate::cpu::{Cpu, Cycles};
use crate::memory::CpuMemory;

macro_rules! l8_imm {
    ( $name:ident, $dst:ident ) => {
        pub fn $name(cpu: &mut Cpu, mem: &impl CpuMemory) -> Cycles {
            cpu.$dst = mem.get_u8(cpu.pc.wrapping_add(1));
            cpu.pc += 2;
            Cycles(8)
        }
    };
}
l8_imm!(op06, b);
l8_imm!(op0e, c);
l8_imm!(op16, d);
l8_imm!(op1e, e);
l8_imm!(op26, h);
l8_imm!(op2e, l);

macro_rules! l8_reg {
    ( $name:ident, $dst:ident, $src:ident ) => {
        pub fn $name(cpu: &mut Cpu, _mem: &impl CpuMemory) -> Cycles {
            cpu.$dst = cpu.$src;
            cpu.pc += 1;
            Cycles(4)
        }
    };
}
l8_reg!(op7f, a, a);
l8_reg!(op78, a, b);
l8_reg!(op79, a, c);
l8_reg!(op7a, a, d);
l8_reg!(op7b, a, e);
l8_reg!(op7c, a, h);
l8_reg!(op7d, a, l);
l8_reg!(op40, b, b);
l8_reg!(op41, b, c);
l8_reg!(op42, b, d);
l8_reg!(op43, b, e);
l8_reg!(op44, b, h);
l8_reg!(op45, b, l);
l8_reg!(op47, b, a);
l8_reg!(op48, c, b);
l8_reg!(op49, c, c);
l8_reg!(op4a, c, d);
l8_reg!(op4b, c, e);
l8_reg!(op4c, c, h);
l8_reg!(op4d, c, l);
l8_reg!(op4f, c, a);
l8_reg!(op50, d, b);
l8_reg!(op51, d, c);
l8_reg!(op52, d, d);
l8_reg!(op53, d, e);
l8_reg!(op54, d, h);
l8_reg!(op55, d, l);
l8_reg!(op57, d, a);
l8_reg!(op58, e, b);
l8_reg!(op59, e, c);
l8_reg!(op5a, e, d);
l8_reg!(op5b, e, e);
l8_reg!(op5c, e, h);
l8_reg!(op5d, e, l);
l8_reg!(op5f, e, a);
l8_reg!(op60, h, b);
l8_reg!(op61, h, c);
l8_reg!(op62, h, d);
l8_reg!(op63, h, e);
l8_reg!(op64, h, h);
l8_reg!(op65, h, l);
l8_reg!(op67, h, a);
l8_reg!(op68, l, b);
l8_reg!(op69, l, c);
l8_reg!(op6a, l, d);
l8_reg!(op6b, l, e);
l8_reg!(op6c, l, h);
l8_reg!(op6d, l, l);
l8_reg!(op6f, l, a);

macro_rules! l8_from_mem {
    ( $name:ident, $dst:ident, $src:ident ) => {
        pub fn $name(cpu: &mut Cpu, mem: &impl CpuMemory) -> Cycles {
            cpu.$dst = mem.get_u8(cpu.$src());
            cpu.pc += 1;
            Cycles(8)
        }
    };
}
l8_from_mem!(op7e, a, get_hl);
l8_from_mem!(op46, b, get_hl);
l8_from_mem!(op4e, c, get_hl);
l8_from_mem!(op56, d, get_hl);
l8_from_mem!(op5e, e, get_hl);
l8_from_mem!(op66, h, get_hl);
l8_from_mem!(op6e, l, get_hl);

macro_rules! l8_to_mem {
    ( $name:ident, $src:ident, $dst:ident ) => {
        pub fn $name(cpu: &mut Cpu, mem: &mut impl CpuMemory) -> Cycles {
            mem.set_u8(cpu.$dst(), cpu.$src);
            cpu.pc += 1;
            Cycles(8)
        }
    };
}
l8_to_mem!(op70, b, get_hl);
l8_to_mem!(op71, c, get_hl);
l8_to_mem!(op72, d, get_hl);
l8_to_mem!(op73, e, get_hl);
l8_to_mem!(op74, h, get_hl);
l8_to_mem!(op75, l, get_hl);

pub fn op36(cpu: &mut Cpu, mem: &mut impl CpuMemory) -> Cycles {
    mem.set_u8(cpu.get_hl(), mem.get_u8(cpu.pc.wrapping_add(1)));
    cpu.pc += 2;
    Cycles(12)
}

l8_from_mem!(op0a, a, get_bc);
l8_from_mem!(op1a, a, get_de);
pub fn opfa(cpu: &mut Cpu, mem: &impl CpuMemory) -> Cycles {
    let low = mem.get_u8(cpu.pc.wrapping_add(1));
    let high = mem.get_u8(cpu.pc.wrapping_add(2));
    let addr = (high as u16) << 8 | low as u16;
    cpu.a = mem.get_u8(addr);
    cpu.pc += 3;
    Cycles(16)
}
pub fn op3e(cpu: &mut Cpu, mem: &impl CpuMemory) -> Cycles {
    cpu.a = mem.get_u8(cpu.pc.wrapping_add(1));
    cpu.pc += 2;
    Cycles(8)
}

l8_to_mem!(op02, a, get_bc);
l8_to_mem!(op12, a, get_de);
l8_to_mem!(op77, a, get_hl);
pub fn opea(cpu: &mut Cpu, mem: &mut impl CpuMemory) -> Cycles {
    let low = mem.get_u8(cpu.pc.wrapping_add(1));
    let high = mem.get_u8(cpu.pc.wrapping_add(2));
    let addr = (high as u16) << 8 | low as u16;
    mem.set_u8(addr, cpu.a);
    cpu.pc += 3;
    Cycles(16)
}

pub fn opf2(cpu: &mut Cpu, mem: &impl CpuMemory) -> Cycles {
    let low = cpu.c;
    let high = 0xff;
    let addr = (high as u16) << 8 | low as u16;
    cpu.a = mem.get_u8(addr);
    cpu.pc += 1;
    Cycles(8)
}

pub fn ope2(cpu: &mut Cpu, mem: &mut impl CpuMemory) -> Cycles {
    let low = cpu.c;
    let high = 0xff;
    let addr = (high as u16) << 8 | low as u16;
    mem.set_u8(addr, cpu.a);
    cpu.pc += 1;
    Cycles(8)
}

pub fn op3a(cpu: &mut Cpu, mem: &impl CpuMemory) -> Cycles {
    let hl = cpu.get_hl();
    cpu.a = mem.get_u8(hl);
    cpu.set_hl(hl - 1);
    cpu.pc += 1;
    Cycles(8)
}

pub fn op32(cpu: &mut Cpu, mem: &mut impl CpuMemory) -> Cycles {
    let hl = cpu.get_hl();
    mem.set_u8(hl, cpu.a);
    cpu.set_hl(hl - 1);
    cpu.pc += 1;
    Cycles(8)
}

pub fn op2a(cpu: &mut Cpu, mem: &impl CpuMemory) -> Cycles {
    let hl = cpu.get_hl();
    cpu.a = mem.get_u8(hl);
    cpu.set_hl(hl.wrapping_add(1));
    cpu.pc += 1;
    Cycles(8)
}

pub fn op22(cpu: &mut Cpu, mem: &mut impl CpuMemory) -> Cycles {
    let hl = cpu.get_hl();
    mem.set_u8(hl, cpu.a);
    cpu.set_hl(hl.wrapping_add(1));
    cpu.pc += 1;
    Cycles(8)
}

pub fn ope0(cpu: &mut Cpu, mem: &mut impl CpuMemory) -> Cycles {
    let low = mem.get_u8(cpu.pc.wrapping_add(1));
    let high = 0xff;
    let addr = (high as u16) << 8 | low as u16;
    mem.set_u8(addr, cpu.a);
    cpu.pc += 2;
    Cycles(12)
}

pub fn opf0(cpu: &mut Cpu, mem: &impl CpuMemory) -> Cycles {
    let low = mem.get_u8(cpu.pc.wrapping_add(1));
    let high = 0xff;
    let addr = (high as u16) << 8 | low as u16;
    cpu.a = mem.get_u8(addr);
    cpu.pc += 2;
    Cycles(12)
}
