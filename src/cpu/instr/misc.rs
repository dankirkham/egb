use crate::cpu::{Cpu, Cycles, InterruptChange, State};
use crate::memory::ProgramMemory;
use crate::registers::CpuFlags;

#[inline]
fn swap_impl(flags: &mut CpuFlags, reg: u8) -> u8 {
    let result = ((reg << 4) & 0xf0) | ((reg >> 4) & 0x0f);
    flags.set(CpuFlags::Z, result == 0);
    flags.set(CpuFlags::N, false);
    flags.set(CpuFlags::H, false);
    flags.set(CpuFlags::C, false);
    result
}
macro_rules! swap {
    ( $name:ident, $dst:ident ) => {
        pub fn $name(cpu: &mut Cpu, _mem: &impl ProgramMemory) -> Cycles {
            let reg = cpu.$dst;
            cpu.$dst = swap_impl(&mut cpu.f, reg);
            cpu.pc += 2;
            Cycles(8)
        }
    };
}
swap!(opcb37, a);
swap!(opcb30, b);
swap!(opcb31, c);
swap!(opcb32, d);
swap!(opcb33, e);
swap!(opcb34, h);
swap!(opcb35, l);
pub fn opcb36(cpu: &mut Cpu, mem: &mut impl ProgramMemory) -> Cycles {
    let reg = mem.get_u8(cpu.get_hl());
    mem.set_u8(cpu.get_hl(), swap_impl(&mut cpu.f, reg));
    cpu.pc += 2;
    Cycles(16)
}

pub fn op27(cpu: &mut Cpu, _mem: &impl ProgramMemory) -> Cycles {
    if !cpu.f.contains(CpuFlags::N) {
        // Last operation was an additon.
        if cpu.f.contains(CpuFlags::C) || cpu.a > 0x99 {
            cpu.a = cpu.a.wrapping_add(0x60);
            cpu.f.set(CpuFlags::C, true);
        }
        if cpu.f.contains(CpuFlags::H) || ((cpu.a & 0xf) > 0x9) {
            cpu.a = cpu.a.wrapping_add(0x6);
        }
    } else {
        // Last operation was a subtracton.
        if cpu.f.contains(CpuFlags::C) {
            cpu.a = cpu.a.wrapping_sub(0x60);
        }
        if cpu.f.contains(CpuFlags::H) {
            cpu.a = cpu.a.wrapping_sub(0x6);
        }
    }
    cpu.f.set(CpuFlags::Z, cpu.a == 0);
    cpu.f.set(CpuFlags::H, false);
    cpu.pc += 1;
    Cycles(4)
}

pub fn op2f(cpu: &mut Cpu, _mem: &impl ProgramMemory) -> Cycles {
    cpu.a = !cpu.a;
    cpu.f.set(CpuFlags::N, true);
    cpu.f.set(CpuFlags::H, true);
    cpu.pc += 1;
    Cycles(4)
}

pub fn op3f(cpu: &mut Cpu, _mem: &impl ProgramMemory) -> Cycles {
    cpu.f.set(CpuFlags::N, false);
    cpu.f.set(CpuFlags::H, false);
    cpu.f.set(CpuFlags::C, !cpu.f.contains(CpuFlags::C));
    cpu.pc += 1;
    Cycles(4)
}

pub fn op37(cpu: &mut Cpu, _mem: &impl ProgramMemory) -> Cycles {
    cpu.f.set(CpuFlags::N, false);
    cpu.f.set(CpuFlags::H, false);
    cpu.f.set(CpuFlags::C, true);
    cpu.pc += 1;
    Cycles(4)
}

// Nop
pub fn op00(cpu: &mut Cpu, _mem: &impl ProgramMemory) -> Cycles {
    cpu.pc += 1;
    Cycles(4)
}

// Halt
pub fn op76(cpu: &mut Cpu, _mem: &impl ProgramMemory) -> Cycles {
    cpu.state = State::Halted;
    cpu.pc += 1;
    Cycles(4)
}

// Stop
pub fn op10(cpu: &mut Cpu, _mem: &mut impl ProgramMemory) -> Cycles {
    cpu.state = State::Stopped;
    cpu.pc += 1;
    Cycles(4)
}

// DI
pub fn opf3(cpu: &mut Cpu, _mem: &impl ProgramMemory) -> Cycles {
    cpu.interrupt_change = Some(InterruptChange::di());
    cpu.pc += 1;
    Cycles(4)
}

// EI
pub fn opfb(cpu: &mut Cpu, _mem: &impl ProgramMemory) -> Cycles {
    cpu.interrupt_change = Some(InterruptChange::ei());
    cpu.pc += 1;
    Cycles(4)
}
