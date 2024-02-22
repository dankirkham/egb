use std::ops::Not;

use crate::cpu::{Cpu, CpuFlags, Cycles};
use crate::memory::CpuMemory;

pub fn opc3(cpu: &mut Cpu, mem: &impl CpuMemory) -> Cycles {
    let addr = mem.get_u16(cpu.pc.wrapping_add(1));
    cpu.pc = addr;
    Cycles(16)
}

macro_rules! jp_cc {
    ( $name:ident, $bit:ident, $tf:literal ) => {
        pub fn $name(cpu: &mut Cpu, mem: &impl CpuMemory) -> Cycles {
            if cpu.f.contains(CpuFlags::$bit) == $tf {
                let addr = mem.get_u16(cpu.pc.wrapping_add(1));
                cpu.pc = addr;
                Cycles(16)
            } else {
                cpu.pc += 3;
                Cycles(12)
            }
        }
    };
}
jp_cc!(opc2, Z, false);
jp_cc!(opca, Z, true);
jp_cc!(opd2, C, false);
jp_cc!(opda, C, true);

pub fn ope9(cpu: &mut Cpu, _mem: &impl CpuMemory) -> Cycles {
    cpu.pc = cpu.get_hl();
    Cycles(4)
}

pub fn op18(cpu: &mut Cpu, mem: &impl CpuMemory) -> Cycles {
    let n_u8 = mem.get_u8(cpu.pc.wrapping_add(1));
    let n_u16 = n_u8 as u16;
    let n_i8 = n_u8 as i8;
    if n_i8 < 0 {
        cpu.pc -= n_u8.not().wrapping_add(1) as u16;
    } else {
        cpu.pc += n_u16;
    }
    cpu.pc += 2;
    Cycles(12)
}

macro_rules! jp_cc_n {
    ( $name:ident, $bit:ident, $tf:literal ) => {
        pub fn $name(cpu: &mut Cpu, mem: &impl CpuMemory) -> Cycles {
            let cycles = if cpu.f.contains(CpuFlags::$bit) == $tf {
                let n_u8 = mem.get_u8(cpu.pc.wrapping_add(1));
                let n_u16 = n_u8 as u16;
                let n_i8 = n_u8 as i8;
                if n_i8 < 0 {
                    cpu.pc -= n_u8.not().wrapping_add(1) as u16;
                } else {
                    cpu.pc += n_u16;
                }
                Cycles(20)
            } else {
                Cycles(8)
            };

            cpu.pc += 2;
            cycles
        }
    };
}
jp_cc_n!(op20, Z, false);
jp_cc_n!(op28, Z, true);
jp_cc_n!(op30, C, false);
jp_cc_n!(op38, C, true);

pub fn opcd(cpu: &mut Cpu, mem: &mut impl CpuMemory) -> Cycles {
    cpu.sp -= 2;
    let next_inst = cpu.pc.wrapping_add(3);
    mem.set_u16(cpu.sp, next_inst);
    let addr = mem.get_u16(cpu.pc.wrapping_add(1));
    cpu.pc = addr;
    Cycles(24)
}

macro_rules! call_cc_n {
    ( $name:ident, $bit:ident, $tf:literal ) => {
        pub fn $name(cpu: &mut Cpu, mem: &mut impl CpuMemory) -> Cycles {
            if cpu.f.contains(CpuFlags::$bit) == $tf {
                cpu.sp -= 2;
                let next_inst = cpu.pc.wrapping_add(3);
                mem.set_u16(cpu.sp, next_inst);
                let addr = mem.get_u16(cpu.pc.wrapping_add(1));
                cpu.pc = addr;
                Cycles(24)
            } else {
                cpu.pc += 3;
                Cycles(12)
            }
        }
    };
}
call_cc_n!(opc4, Z, false);
call_cc_n!(opcc, Z, true);
call_cc_n!(opd4, C, false);
call_cc_n!(opdc, C, true);

macro_rules! rst {
    ( $name:ident, $offset:literal ) => {
        pub fn $name(cpu: &mut Cpu, mem: &mut impl CpuMemory) -> Cycles {
            cpu.sp -= 2;
            let next_inst = cpu.pc.wrapping_add(1);
            mem.set_u16(cpu.sp, next_inst);
            cpu.pc = $offset;
            Cycles(16)
        }
    };
}
rst!(opc7, 0x00);
rst!(opcf, 0x08);
rst!(opd7, 0x10);
rst!(opdf, 0x18);
rst!(ope7, 0x20);
rst!(opef, 0x28);
rst!(opf7, 0x30);
rst!(opff, 0x38);

// ret
pub fn opc9(cpu: &mut Cpu, mem: &impl CpuMemory) -> Cycles {
    let addr = mem.get_u16(cpu.sp);
    cpu.sp += 2;
    cpu.pc = addr;
    Cycles(16)
}

macro_rules! ret_cc {
    ( $name:ident, $bit:ident, $tf:literal ) => {
        pub fn $name(cpu: &mut Cpu, mem: &impl CpuMemory) -> Cycles {
            if cpu.f.contains(CpuFlags::$bit) == $tf {
                let addr = mem.get_u16(cpu.sp);
                cpu.sp += 2;
                cpu.pc = addr;
                Cycles(20)
            } else {
                cpu.pc += 1;
                Cycles(8)
            }
        }
    };
}
ret_cc!(opc0, Z, false);
ret_cc!(opc8, Z, true);
ret_cc!(opd0, C, false);
ret_cc!(opd8, C, true);

// reti
pub fn opd9(cpu: &mut Cpu, mem: &impl CpuMemory) -> Cycles {
    let addr = mem.get_u16(cpu.sp);
    cpu.sp += 2;
    cpu.pc = addr;
    cpu.ie = true;
    Cycles(16)
}
