use crate::cpu::{Cpu, Cycles};
use crate::memory::ProgramMemory;
use crate::registers::CpuFlags;

macro_rules! op8_reg {
    ( $name:ident, $op:ident, $src:ident ) => {
        pub fn $name(cpu: &mut Cpu, _mem: &impl ProgramMemory) -> Cycles {
            let l = cpu.a;
            let r = cpu.$src;
            cpu.a = $op(&mut cpu.f, l, r);
            cpu.pc += 1;
            Cycles(4)
        }
    };
}
macro_rules! op8_mem {
    ( $name:ident, $op:ident ) => {
        pub fn $name(cpu: &mut Cpu, mem: &impl ProgramMemory) -> Cycles {
            let l = cpu.a;
            let r = mem.get_u8(cpu.get_hl());
            cpu.a = $op(&mut cpu.f, l, r);
            cpu.pc += 1;
            Cycles(8)
        }
    };
}
macro_rules! op8_imm {
    ( $name:ident, $op:ident ) => {
        pub fn $name(cpu: &mut Cpu, mem: &impl ProgramMemory) -> Cycles {
            let l = cpu.a;
            let r = mem.get_u8(cpu.pc.wrapping_add(1));
            cpu.a = $op(&mut cpu.f, l, r);
            cpu.pc += 2;
            Cycles(8)
        }
    };
}

#[inline]
fn add8(flags: &mut CpuFlags, l: u8, r: u8) -> u8 {
    let result = l as u16 + r as u16;
    flags.set(CpuFlags::N, false);
    flags.set(CpuFlags::H, ((l & 0xf) + (r & 0xf)) & 0x10 != 0);
    flags.set(CpuFlags::C, result & 0x100 != 0);
    let result = result as u8;
    flags.set(CpuFlags::Z, result == 0);
    result
}
op8_reg!(op87, add8, a);
op8_reg!(op80, add8, b);
op8_reg!(op81, add8, c);
op8_reg!(op82, add8, d);
op8_reg!(op83, add8, e);
op8_reg!(op84, add8, h);
op8_reg!(op85, add8, l);
op8_mem!(op86, add8);
op8_imm!(opc6, add8);

#[inline]
fn adc8(flags: &mut CpuFlags, l: u8, r: u8) -> u8 {
    let c = if flags.contains(CpuFlags::C) { 1 } else { 0 };
    let result = l as u16 + r as u16 + c as u16;
    flags.set(CpuFlags::N, false);
    flags.set(CpuFlags::H, ((l & 0xf) + (r & 0xf) + c) & 0x10 != 0);
    flags.set(CpuFlags::C, result & 0x100 != 0);
    let result = result as u8;
    flags.set(CpuFlags::Z, result == 0);
    result
}
op8_reg!(op8f, adc8, a);
op8_reg!(op88, adc8, b);
op8_reg!(op89, adc8, c);
op8_reg!(op8a, adc8, d);
op8_reg!(op8b, adc8, e);
op8_reg!(op8c, adc8, h);
op8_reg!(op8d, adc8, l);
op8_mem!(op8e, adc8);
op8_imm!(opce, adc8);

#[inline]
fn sub8(flags: &mut CpuFlags, l: u8, r: u8) -> u8 {
    flags.set(CpuFlags::N, true);
    flags.set(CpuFlags::H, (l & 0xf) < (r & 0xf));
    flags.set(CpuFlags::C, l < r);
    let result = l.wrapping_sub(r);
    flags.set(CpuFlags::Z, result == 0);
    result
}
op8_reg!(op97, sub8, a);
op8_reg!(op90, sub8, b);
op8_reg!(op91, sub8, c);
op8_reg!(op92, sub8, d);
op8_reg!(op93, sub8, e);
op8_reg!(op94, sub8, h);
op8_reg!(op95, sub8, l);
op8_mem!(op96, sub8);
op8_imm!(opd6, sub8);

#[inline]
pub const fn borrowing_sub(lhs: u8, rhs: u8, borrow: u8) -> (u8, bool) {
    let (a, b) = lhs.overflowing_sub(rhs);
    let (c, d) = a.overflowing_sub(borrow);
    (c, b != d)
}

#[inline]
fn sbc8(flags: &mut CpuFlags, l: u8, r: u8) -> u8 {
    let c = flags.contains(CpuFlags::C);
    let c = if c { 1 } else { 0 };
    let (result, borrow) = borrowing_sub(l, r, c);
    flags.set(CpuFlags::N, true);
    flags.set(CpuFlags::H, (l & 0xf) < ((r & 0xf) + c));
    flags.set(CpuFlags::C, borrow);
    flags.set(CpuFlags::Z, result == 0);
    result
}
op8_reg!(op9f, sbc8, a);
op8_reg!(op98, sbc8, b);
op8_reg!(op99, sbc8, c);
op8_reg!(op9a, sbc8, d);
op8_reg!(op9b, sbc8, e);
op8_reg!(op9c, sbc8, h);
op8_reg!(op9d, sbc8, l);
op8_mem!(op9e, sbc8);
op8_imm!(opde, sbc8);

#[inline]
fn and8(flags: &mut CpuFlags, l: u8, r: u8) -> u8 {
    let result = l & r;
    flags.set(CpuFlags::Z, result == 0);
    flags.set(CpuFlags::N, false);
    flags.set(CpuFlags::H, true);
    flags.set(CpuFlags::C, false);
    result
}
op8_reg!(opa7, and8, a);
op8_reg!(opa0, and8, b);
op8_reg!(opa1, and8, c);
op8_reg!(opa2, and8, d);
op8_reg!(opa3, and8, e);
op8_reg!(opa4, and8, h);
op8_reg!(opa5, and8, l);
op8_mem!(opa6, and8);
op8_imm!(ope6, and8);

#[inline]
fn or8(flags: &mut CpuFlags, l: u8, r: u8) -> u8 {
    let result = l | r;
    flags.set(CpuFlags::Z, result == 0);
    flags.set(CpuFlags::N, false);
    flags.set(CpuFlags::H, false);
    flags.set(CpuFlags::C, false);
    result
}
op8_reg!(opb7, or8, a);
op8_reg!(opb0, or8, b);
op8_reg!(opb1, or8, c);
op8_reg!(opb2, or8, d);
op8_reg!(opb3, or8, e);
op8_reg!(opb4, or8, h);
op8_reg!(opb5, or8, l);
op8_mem!(opb6, or8);
op8_imm!(opf6, or8);

#[inline]
fn xor8(flags: &mut CpuFlags, l: u8, r: u8) -> u8 {
    let result = l ^ r;
    flags.set(CpuFlags::Z, result == 0);
    flags.set(CpuFlags::N, false);
    flags.set(CpuFlags::H, false);
    flags.set(CpuFlags::C, false);
    result
}
op8_reg!(opaf, xor8, a);
op8_reg!(opa8, xor8, b);
op8_reg!(opa9, xor8, c);
op8_reg!(opaa, xor8, d);
op8_reg!(opab, xor8, e);
op8_reg!(opac, xor8, h);
op8_reg!(opad, xor8, l);
op8_mem!(opae, xor8);
op8_imm!(opee, xor8);

macro_rules! cp8_reg {
    ( $name:ident, $src:ident ) => {
        pub fn $name(cpu: &mut Cpu, _mem: &impl ProgramMemory) -> Cycles {
            let l = cpu.a;
            let r = cpu.$src;
            cp8(&mut cpu.f, l, r);
            cpu.pc += 1;
            Cycles(4)
        }
    };
}
macro_rules! cp8_mem {
    ( $name:ident ) => {
        pub fn $name(cpu: &mut Cpu, mem: &impl ProgramMemory) -> Cycles {
            let l = cpu.a;
            let r = mem.get_u8(cpu.get_hl());
            cp8(&mut cpu.f, l, r);
            cpu.pc += 1;
            Cycles(8)
        }
    };
}
macro_rules! cp8_imm {
    ( $name:ident ) => {
        pub fn $name(cpu: &mut Cpu, mem: &impl ProgramMemory) -> Cycles {
            let l = cpu.a;
            let r = mem.get_u8(cpu.pc.wrapping_add(1));
            cp8(&mut cpu.f, l, r);
            cpu.pc += 2;
            Cycles(8)
        }
    };
}
#[inline]
fn cp8(flags: &mut CpuFlags, l: u8, r: u8) {
    flags.set(CpuFlags::N, true);
    flags.set(CpuFlags::H, (l & 0xf) < (r & 0xf));
    flags.set(CpuFlags::C, l < r);
    let result = l.wrapping_sub(r);
    flags.set(CpuFlags::Z, result == 0);
}
cp8_reg!(opbf, a);
cp8_reg!(opb8, b);
cp8_reg!(opb9, c);
cp8_reg!(opba, d);
cp8_reg!(opbb, e);
cp8_reg!(opbc, h);
cp8_reg!(opbd, l);
cp8_mem!(opbe);
cp8_imm!(opfe);

#[inline]
fn inc_flags(flags: &mut CpuFlags, input: u8) -> u8 {
    let result = input.wrapping_add(1);
    flags.set(CpuFlags::Z, result == 0);
    flags.set(CpuFlags::N, false);
    flags.set(CpuFlags::H, ((input & 0xf) + 1) & 0x10 != 0);
    result
}
macro_rules! inc_reg {
    ( $name:ident, $dst:ident ) => {
        pub fn $name(cpu: &mut Cpu, _mem: &impl ProgramMemory) -> Cycles {
            cpu.$dst = inc_flags(&mut cpu.f, cpu.$dst);
            cpu.pc += 1;
            Cycles(4)
        }
    };
}
inc_reg!(op3c, a);
inc_reg!(op04, b);
inc_reg!(op0c, c);
inc_reg!(op14, d);
inc_reg!(op1c, e);
inc_reg!(op24, h);
inc_reg!(op2c, l);
pub fn op34(cpu: &mut Cpu, mem: &mut impl ProgramMemory) -> Cycles {
    let input = mem.get_u8(cpu.get_hl());
    mem.set_u8(cpu.get_hl(), inc_flags(&mut cpu.f, input));
    cpu.pc += 1;
    Cycles(12)
}

#[inline]
fn dec_flags(flags: &mut CpuFlags, input: u8) -> u8 {
    let result = input.wrapping_sub(1);
    flags.set(CpuFlags::Z, result == 0);
    flags.set(CpuFlags::N, true);
    flags.set(CpuFlags::H, (input & 0xf) < (result & 0xf));
    result
}
macro_rules! dec_reg {
    ( $name:ident, $dst:ident ) => {
        pub fn $name(cpu: &mut Cpu, _mem: &impl ProgramMemory) -> Cycles {
            cpu.$dst = dec_flags(&mut cpu.f, cpu.$dst);
            cpu.pc += 1;
            Cycles(4)
        }
    };
}
dec_reg!(op3d, a);
dec_reg!(op05, b);
dec_reg!(op0d, c);
dec_reg!(op15, d);
dec_reg!(op1d, e);
dec_reg!(op25, h);
dec_reg!(op2d, l);
pub fn op35(cpu: &mut Cpu, mem: &mut impl ProgramMemory) -> Cycles {
    let input = mem.get_u8(cpu.get_hl());
    mem.set_u8(cpu.get_hl(), dec_flags(&mut cpu.f, input));
    cpu.pc += 1;
    Cycles(12)
}
