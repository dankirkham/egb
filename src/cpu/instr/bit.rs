use crate::cpu::{Cpu, Cycles};
use crate::memory::ProgramMemory;
use crate::registers::CpuFlags;

macro_rules! bit {
    ( $name:ident, hl, $bit:literal ) => {
        pub fn $name(cpu: &mut Cpu, mem: &impl ProgramMemory) -> Cycles {
            let reg = mem.get_u8(cpu.get_hl()) & 1 << $bit;
            cpu.f.set(CpuFlags::Z, reg == 0);
            cpu.f.set(CpuFlags::N, false);
            cpu.f.set(CpuFlags::H, true);
            cpu.pc += 2;
            Cycles(16)
        }
    };
    ( $name:ident, $dst:ident, $bit:literal ) => {
        pub fn $name(cpu: &mut Cpu, _mem: &impl ProgramMemory) -> Cycles {
            let reg = cpu.$dst & 1 << $bit;
            cpu.f.set(CpuFlags::Z, reg == 0);
            cpu.f.set(CpuFlags::N, false);
            cpu.f.set(CpuFlags::H, true);
            cpu.pc += 2;
            Cycles(8)
        }
    };
}
bit!(opcb47, a, 0);
bit!(opcb40, b, 0);
bit!(opcb41, c, 0);
bit!(opcb42, d, 0);
bit!(opcb43, e, 0);
bit!(opcb44, h, 0);
bit!(opcb45, l, 0);
bit!(opcb46, hl, 0);
bit!(opcb4f, a, 1);
bit!(opcb48, b, 1);
bit!(opcb49, c, 1);
bit!(opcb4a, d, 1);
bit!(opcb4b, e, 1);
bit!(opcb4c, h, 1);
bit!(opcb4d, l, 1);
bit!(opcb4e, hl, 1);
bit!(opcb57, a, 2);
bit!(opcb50, b, 2);
bit!(opcb51, c, 2);
bit!(opcb52, d, 2);
bit!(opcb53, e, 2);
bit!(opcb54, h, 2);
bit!(opcb55, l, 2);
bit!(opcb56, hl, 2);
bit!(opcb5f, a, 3);
bit!(opcb58, b, 3);
bit!(opcb59, c, 3);
bit!(opcb5a, d, 3);
bit!(opcb5b, e, 3);
bit!(opcb5c, h, 3);
bit!(opcb5d, l, 3);
bit!(opcb5e, hl, 3);
bit!(opcb67, a, 4);
bit!(opcb60, b, 4);
bit!(opcb61, c, 4);
bit!(opcb62, d, 4);
bit!(opcb63, e, 4);
bit!(opcb64, h, 4);
bit!(opcb65, l, 4);
bit!(opcb66, hl, 4);
bit!(opcb6f, a, 5);
bit!(opcb68, b, 5);
bit!(opcb69, c, 5);
bit!(opcb6a, d, 5);
bit!(opcb6b, e, 5);
bit!(opcb6c, h, 5);
bit!(opcb6d, l, 5);
bit!(opcb6e, hl, 5);
bit!(opcb77, a, 6);
bit!(opcb70, b, 6);
bit!(opcb71, c, 6);
bit!(opcb72, d, 6);
bit!(opcb73, e, 6);
bit!(opcb74, h, 6);
bit!(opcb75, l, 6);
bit!(opcb76, hl, 6);
bit!(opcb7f, a, 7);
bit!(opcb78, b, 7);
bit!(opcb79, c, 7);
bit!(opcb7a, d, 7);
bit!(opcb7b, e, 7);
bit!(opcb7c, h, 7);
bit!(opcb7d, l, 7);
bit!(opcb7e, hl, 7);

macro_rules! set {
    ( $name:ident, hl, $bit:literal ) => {
        pub fn $name(cpu: &mut Cpu, mem: &mut impl ProgramMemory) -> Cycles {
            mem.set_u8(cpu.get_hl(), mem.get_u8(cpu.get_hl()) | 1 << $bit);
            cpu.pc += 2;
            Cycles(16)
        }
    };
    ( $name:ident, $dst:ident, $bit:literal ) => {
        pub fn $name(cpu: &mut Cpu, _mem: &impl ProgramMemory) -> Cycles {
            cpu.$dst |= 1 << $bit;
            cpu.pc += 2;
            Cycles(8)
        }
    };
}
set!(opcbc7, a, 0);
set!(opcbc0, b, 0);
set!(opcbc1, c, 0);
set!(opcbc2, d, 0);
set!(opcbc3, e, 0);
set!(opcbc4, h, 0);
set!(opcbc5, l, 0);
set!(opcbc6, hl, 0);
set!(opcbcf, a, 1);
set!(opcbc8, b, 1);
set!(opcbc9, c, 1);
set!(opcbca, d, 1);
set!(opcbcb, e, 1);
set!(opcbcc, h, 1);
set!(opcbcd, l, 1);
set!(opcbce, hl, 1);
set!(opcbd7, a, 2);
set!(opcbd0, b, 2);
set!(opcbd1, c, 2);
set!(opcbd2, d, 2);
set!(opcbd3, e, 2);
set!(opcbd4, h, 2);
set!(opcbd5, l, 2);
set!(opcbd6, hl, 2);
set!(opcbdf, a, 3);
set!(opcbd8, b, 3);
set!(opcbd9, c, 3);
set!(opcbda, d, 3);
set!(opcbdb, e, 3);
set!(opcbdc, h, 3);
set!(opcbdd, l, 3);
set!(opcbde, hl, 3);
set!(opcbe7, a, 4);
set!(opcbe0, b, 4);
set!(opcbe1, c, 4);
set!(opcbe2, d, 4);
set!(opcbe3, e, 4);
set!(opcbe4, h, 4);
set!(opcbe5, l, 4);
set!(opcbe6, hl, 4);
set!(opcbef, a, 5);
set!(opcbe8, b, 5);
set!(opcbe9, c, 5);
set!(opcbea, d, 5);
set!(opcbeb, e, 5);
set!(opcbec, h, 5);
set!(opcbed, l, 5);
set!(opcbee, hl, 5);
set!(opcbf7, a, 6);
set!(opcbf0, b, 6);
set!(opcbf1, c, 6);
set!(opcbf2, d, 6);
set!(opcbf3, e, 6);
set!(opcbf4, h, 6);
set!(opcbf5, l, 6);
set!(opcbf6, hl, 6);
set!(opcbff, a, 7);
set!(opcbf8, b, 7);
set!(opcbf9, c, 7);
set!(opcbfa, d, 7);
set!(opcbfb, e, 7);
set!(opcbfc, h, 7);
set!(opcbfd, l, 7);
set!(opcbfe, hl, 7);

macro_rules! res {
    ( $name:ident, hl, $bit:literal ) => {
        pub fn $name(cpu: &mut Cpu, mem: &mut impl ProgramMemory) -> Cycles {
            let mut reg = mem.get_u8(cpu.get_hl());
            reg &= !(1 << $bit);
            mem.set_u8(cpu.get_hl(), reg);
            cpu.pc += 2;
            Cycles(16)
        }
    };
    ( $name:ident, $dst:ident, $bit:literal ) => {
        pub fn $name(cpu: &mut Cpu, _mem: &impl ProgramMemory) -> Cycles {
            cpu.$dst &= !(1 << $bit);
            cpu.pc += 2;
            Cycles(8)
        }
    };
}
res!(opcb87, a, 0);
res!(opcb80, b, 0);
res!(opcb81, c, 0);
res!(opcb82, d, 0);
res!(opcb83, e, 0);
res!(opcb84, h, 0);
res!(opcb85, l, 0);
res!(opcb86, hl, 0);
res!(opcb8f, a, 1);
res!(opcb88, b, 1);
res!(opcb89, c, 1);
res!(opcb8a, d, 1);
res!(opcb8b, e, 1);
res!(opcb8c, h, 1);
res!(opcb8d, l, 1);
res!(opcb8e, hl, 1);
res!(opcb97, a, 2);
res!(opcb90, b, 2);
res!(opcb91, c, 2);
res!(opcb92, d, 2);
res!(opcb93, e, 2);
res!(opcb94, h, 2);
res!(opcb95, l, 2);
res!(opcb96, hl, 2);
res!(opcb9f, a, 3);
res!(opcb98, b, 3);
res!(opcb99, c, 3);
res!(opcb9a, d, 3);
res!(opcb9b, e, 3);
res!(opcb9c, h, 3);
res!(opcb9d, l, 3);
res!(opcb9e, hl, 3);
res!(opcba7, a, 4);
res!(opcba0, b, 4);
res!(opcba1, c, 4);
res!(opcba2, d, 4);
res!(opcba3, e, 4);
res!(opcba4, h, 4);
res!(opcba5, l, 4);
res!(opcba6, hl, 4);
res!(opcbaf, a, 5);
res!(opcba8, b, 5);
res!(opcba9, c, 5);
res!(opcbaa, d, 5);
res!(opcbab, e, 5);
res!(opcbac, h, 5);
res!(opcbad, l, 5);
res!(opcbae, hl, 5);
res!(opcbb7, a, 6);
res!(opcbb0, b, 6);
res!(opcbb1, c, 6);
res!(opcbb2, d, 6);
res!(opcbb3, e, 6);
res!(opcbb4, h, 6);
res!(opcbb5, l, 6);
res!(opcbb6, hl, 6);
res!(opcbbf, a, 7);
res!(opcbb8, b, 7);
res!(opcbb9, c, 7);
res!(opcbba, d, 7);
res!(opcbbb, e, 7);
res!(opcbbc, h, 7);
res!(opcbbd, l, 7);
res!(opcbbe, hl, 7);
