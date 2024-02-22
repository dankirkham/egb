use crate::cpu::{Cpu, Cycles};
use crate::memory::CpuMemory;
use crate::registers::CpuFlags;

fn rlca_flags(flags: &mut CpuFlags, value: u8) -> u8 {
    let result = value.rotate_left(1);
    flags.set(CpuFlags::Z, false);
    flags.set(CpuFlags::N, false);
    flags.set(CpuFlags::H, false);
    flags.set(CpuFlags::C, value & 0x80 != 0);
    result
}
fn rlc_flags(flags: &mut CpuFlags, value: u8) -> u8 {
    let result = value.rotate_left(1);
    flags.set(CpuFlags::Z, result == 0);
    flags.set(CpuFlags::N, false);
    flags.set(CpuFlags::H, false);
    flags.set(CpuFlags::C, value & 0x80 != 0);
    result
}
macro_rules! rlc {
    ( $name:ident, hl, $size:literal ) => {
        pub fn $name(cpu: &mut Cpu, mem: &mut impl CpuMemory) -> Cycles {
            let value = mem.get_u8(cpu.get_hl());
            mem.set_u8(cpu.get_hl(), rlc_flags(&mut cpu.f, value));
            cpu.pc += $size;
            Cycles(16)
        }
    };
    ( $name:ident, $dst:ident, $size:literal, $flags:ident ) => {
        pub fn $name(cpu: &mut Cpu, _mem: &impl CpuMemory) -> Cycles {
            let value = cpu.$dst;
            cpu.$dst = $flags(&mut cpu.f, value);
            cpu.pc += $size;
            Cycles(8)
        }
    };
}
rlc!(op07, a, 1, rlca_flags);
rlc!(opcb07, a, 2, rlc_flags);
rlc!(opcb00, b, 2, rlc_flags);
rlc!(opcb01, c, 2, rlc_flags);
rlc!(opcb02, d, 2, rlc_flags);
rlc!(opcb03, e, 2, rlc_flags);
rlc!(opcb04, h, 2, rlc_flags);
rlc!(opcb05, l, 2, rlc_flags);
rlc!(opcb06, hl, 2);

fn rla_flags(flags: &mut CpuFlags, value: u8) -> u8 {
    let c_in = if flags.contains(CpuFlags::C) { 1 } else { 0 };
    let result = value << 1 | c_in;
    flags.set(CpuFlags::Z, false);
    flags.set(CpuFlags::N, false);
    flags.set(CpuFlags::H, false);
    flags.set(CpuFlags::C, value & 0x80 != 0);
    result
}
fn rl_flags(flags: &mut CpuFlags, value: u8) -> u8 {
    let c_in = if flags.contains(CpuFlags::C) { 1 } else { 0 };
    let result = value << 1 | c_in;
    flags.set(CpuFlags::Z, result == 0);
    flags.set(CpuFlags::N, false);
    flags.set(CpuFlags::H, false);
    flags.set(CpuFlags::C, value & 0x80 != 0);
    result
}
macro_rules! rl {
    ( $name:ident, hl, $size:literal ) => {
        pub fn $name(cpu: &mut Cpu, mem: &mut impl CpuMemory) -> Cycles {
            let value = mem.get_u8(cpu.get_hl());
            mem.set_u8(cpu.get_hl(), rl_flags(&mut cpu.f, value));
            cpu.pc += $size;
            Cycles(16)
        }
    };
    ( $name:ident, $dst:ident, $size:literal, $cycles:literal, $flags:ident ) => {
        pub fn $name(cpu: &mut Cpu, _mem: &impl CpuMemory) -> Cycles {
            let value = cpu.$dst;
            cpu.$dst = $flags(&mut cpu.f, value);
            cpu.pc += $size;
            Cycles($cycles)
        }
    };
}
rl!(op17, a, 1, 4, rla_flags);
rl!(opcb17, a, 2, 8, rl_flags);
rl!(opcb10, b, 2, 8, rl_flags);
rl!(opcb11, c, 2, 8, rl_flags);
rl!(opcb12, d, 2, 8, rl_flags);
rl!(opcb13, e, 2, 8, rl_flags);
rl!(opcb14, h, 2, 8, rl_flags);
rl!(opcb15, l, 2, 8, rl_flags);
rl!(opcb16, hl, 2);

fn rrca_flags(flags: &mut CpuFlags, value: u8) -> u8 {
    let result = value.rotate_right(1);
    flags.set(CpuFlags::Z, false);
    flags.set(CpuFlags::N, false);
    flags.set(CpuFlags::H, false);
    flags.set(CpuFlags::C, value & 0x1 != 0);
    result
}
fn rrc_flags(flags: &mut CpuFlags, value: u8) -> u8 {
    let result = value.rotate_right(1);
    flags.set(CpuFlags::Z, result == 0);
    flags.set(CpuFlags::N, false);
    flags.set(CpuFlags::H, false);
    flags.set(CpuFlags::C, value & 0x1 != 0);
    result
}
macro_rules! rrc {
    ( $name:ident, hl, $size:literal ) => {
        pub fn $name(cpu: &mut Cpu, mem: &mut impl CpuMemory) -> Cycles {
            let value = mem.get_u8(cpu.get_hl());
            mem.set_u8(cpu.get_hl(), rrc_flags(&mut cpu.f, value));
            cpu.pc += $size;
            Cycles(16)
        }
    };
    ( $name:ident, $dst:ident, $size:literal, $cycles: literal, $flags:ident ) => {
        pub fn $name(cpu: &mut Cpu, _mem: &impl CpuMemory) -> Cycles {
            let value = cpu.$dst;
            cpu.$dst = $flags(&mut cpu.f, value);
            cpu.pc += $size;
            Cycles($cycles)
        }
    };
}
rrc!(op0f, a, 1, 4, rrca_flags);
rrc!(opcb0f, a, 2, 8, rrc_flags);
rrc!(opcb08, b, 2, 8, rrc_flags);
rrc!(opcb09, c, 2, 8, rrc_flags);
rrc!(opcb0a, d, 2, 8, rrc_flags);
rrc!(opcb0b, e, 2, 8, rrc_flags);
rrc!(opcb0c, h, 2, 8, rrc_flags);
rrc!(opcb0d, l, 2, 8, rrc_flags);
rrc!(opcb0e, hl, 2);

fn rra_flags(flags: &mut CpuFlags, value: u8) -> u8 {
    let c_in = if flags.contains(CpuFlags::C) { 1 } else { 0 };
    let result = c_in << 7 | value >> 1;
    flags.set(CpuFlags::Z, false);
    flags.set(CpuFlags::N, false);
    flags.set(CpuFlags::H, false);
    flags.set(CpuFlags::C, value & 0x1 != 0);
    result
}
fn rr_flags(flags: &mut CpuFlags, value: u8) -> u8 {
    let c_in = if flags.contains(CpuFlags::C) { 1 } else { 0 };
    let result = c_in << 7 | value >> 1;
    flags.set(CpuFlags::Z, result == 0);
    flags.set(CpuFlags::N, false);
    flags.set(CpuFlags::H, false);
    flags.set(CpuFlags::C, value & 0x1 != 0);
    result
}
macro_rules! rr {
    ( $name:ident, hl, $size:literal ) => {
        pub fn $name(cpu: &mut Cpu, mem: &mut impl CpuMemory) -> Cycles {
            let value = mem.get_u8(cpu.get_hl());
            mem.set_u8(cpu.get_hl(), rr_flags(&mut cpu.f, value));
            cpu.pc += $size;
            Cycles(16)
        }
    };
    ( $name:ident, $dst:ident, $size:literal, $cycles:literal, $flags:ident ) => {
        pub fn $name(cpu: &mut Cpu, _mem: &impl CpuMemory) -> Cycles {
            let value = cpu.$dst;
            cpu.$dst = $flags(&mut cpu.f, value);
            cpu.pc += $size;
            Cycles($cycles)
        }
    };
}
rr!(op1f, a, 1, 4, rra_flags);
rr!(opcb1f, a, 2, 8, rr_flags);
rr!(opcb18, b, 2, 8, rr_flags);
rr!(opcb19, c, 2, 8, rr_flags);
rr!(opcb1a, d, 2, 8, rr_flags);
rr!(opcb1b, e, 2, 8, rr_flags);
rr!(opcb1c, h, 2, 8, rr_flags);
rr!(opcb1d, l, 2, 8, rr_flags);
rr!(opcb1e, hl, 2);

fn sla_flags(flags: &mut CpuFlags, value: u8) -> u8 {
    let result = value << 1;
    flags.set(CpuFlags::Z, result == 0);
    flags.set(CpuFlags::N, false);
    flags.set(CpuFlags::H, false);
    flags.set(CpuFlags::C, value & 0x80 != 0);
    result
}
macro_rules! sla {
    ( $name:ident, hl, $size:literal ) => {
        pub fn $name(cpu: &mut Cpu, mem: &mut impl CpuMemory) -> Cycles {
            let value = mem.get_u8(cpu.get_hl());
            mem.set_u8(cpu.get_hl(), sla_flags(&mut cpu.f, value));
            cpu.pc += $size;
            Cycles(16)
        }
    };
    ( $name:ident, $dst:ident, $size:literal ) => {
        pub fn $name(cpu: &mut Cpu, _mem: &impl CpuMemory) -> Cycles {
            let value = cpu.$dst;
            cpu.$dst = sla_flags(&mut cpu.f, value);
            cpu.pc += $size;
            Cycles(8)
        }
    };
}
sla!(opcb27, a, 2);
sla!(opcb20, b, 2);
sla!(opcb21, c, 2);
sla!(opcb22, d, 2);
sla!(opcb23, e, 2);
sla!(opcb24, h, 2);
sla!(opcb25, l, 2);
sla!(opcb26, hl, 2);

fn sra_flags(flags: &mut CpuFlags, value: u8) -> u8 {
    let result = value & 0x80 | value >> 1;
    flags.set(CpuFlags::Z, result == 0);
    flags.set(CpuFlags::N, false);
    flags.set(CpuFlags::H, false);
    flags.set(CpuFlags::C, value & 0x1 != 0);
    result
}
macro_rules! sra {
    ( $name:ident, hl, $size:literal ) => {
        pub fn $name(cpu: &mut Cpu, mem: &mut impl CpuMemory) -> Cycles {
            let value = mem.get_u8(cpu.get_hl());
            mem.set_u8(cpu.get_hl(), sra_flags(&mut cpu.f, value));
            cpu.pc += $size;
            Cycles(16)
        }
    };
    ( $name:ident, $dst:ident, $size:literal ) => {
        pub fn $name(cpu: &mut Cpu, _mem: &impl CpuMemory) -> Cycles {
            let value = cpu.$dst;
            cpu.$dst = sra_flags(&mut cpu.f, value);
            cpu.pc += $size;
            Cycles(8)
        }
    };
}
sra!(opcb2f, a, 2);
sra!(opcb28, b, 2);
sra!(opcb29, c, 2);
sra!(opcb2a, d, 2);
sra!(opcb2b, e, 2);
sra!(opcb2c, h, 2);
sra!(opcb2d, l, 2);
sra!(opcb2e, hl, 2);

fn srl_flags(flags: &mut CpuFlags, value: u8) -> u8 {
    let result = value >> 1;
    flags.set(CpuFlags::Z, result == 0);
    flags.set(CpuFlags::N, false);
    flags.set(CpuFlags::H, false);
    flags.set(CpuFlags::C, value & 0x1 != 0);
    result
}
macro_rules! srl {
    ( $name:ident, hl, $size:literal ) => {
        pub fn $name(cpu: &mut Cpu, mem: &mut impl CpuMemory) -> Cycles {
            let value = mem.get_u8(cpu.get_hl());
            mem.set_u8(cpu.get_hl(), srl_flags(&mut cpu.f, value));
            cpu.pc += $size;
            Cycles(16)
        }
    };
    ( $name:ident, $dst:ident, $size:literal ) => {
        pub fn $name(cpu: &mut Cpu, _mem: &impl CpuMemory) -> Cycles {
            let value = cpu.$dst;
            cpu.$dst = srl_flags(&mut cpu.f, value);
            cpu.pc += $size;
            Cycles(8)
        }
    };
}
srl!(opcb3f, a, 2);
srl!(opcb38, b, 2);
srl!(opcb39, c, 2);
srl!(opcb3a, d, 2);
srl!(opcb3b, e, 2);
srl!(opcb3c, h, 2);
srl!(opcb3d, l, 2);
srl!(opcb3e, hl, 2);
