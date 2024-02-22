use crate::cpu::{Cpu, Cycles};
use crate::memory::CpuMemory;
use crate::registers::CpuFlags;

macro_rules! add16 {
    ( $name:ident, $src:ident ) => {
        pub fn $name(cpu: &mut Cpu, _mem: &mut impl CpuMemory) -> Cycles {
            let l = cpu.get_hl() as u32;
            let r = cpu.$src() as u32;
            let result = l + r;
            let h = (l & 0xfff) + (r & 0xfff) > 0xfff;
            cpu.f.set(CpuFlags::N, false);
            cpu.f.set(CpuFlags::H, h);
            cpu.f.set(CpuFlags::C, result & 0x10000 != 0);
            cpu.set_hl(result as u16);
            cpu.pc += 1;
            Cycles(8)
        }
    };
}
add16!(op09, get_bc);
add16!(op19, get_de);
add16!(op29, get_hl);
add16!(op39, get_sp);

pub fn ope8(cpu: &mut Cpu, mem: &mut impl CpuMemory) -> Cycles {
    let l = cpu.get_sp();
    let r_u8 = mem.get_u8(cpu.pc.wrapping_add(1));
    let r: i16 = (r_u8 as i8).into();

    let result = l.wrapping_add_signed(r);

    cpu.f.set(CpuFlags::Z, false);
    cpu.f.set(CpuFlags::N, false);

    let c = (l as u8).checked_add(r_u8).is_none();
    cpu.f.set(CpuFlags::C, c);

    let h = if r >= 0 {
        (l & 0xf) + ((r_u8 & 0xf) as u16) > 0xf
    } else {
        (result & 0xf) <= (l & 0xf)
    };
    cpu.f.set(CpuFlags::H, h);

    cpu.sp = result;
    cpu.pc += 2;
    Cycles(16)
}

macro_rules! inc16 {
    ( $name:ident, $dst:ident, $src:ident ) => {
        pub fn $name(cpu: &mut Cpu, _mem: &impl CpuMemory) -> Cycles {
            let val = cpu.$src().wrapping_add(1);
            cpu.$dst(val);
            cpu.pc += 1;
            Cycles(8)
        }
    };
}
inc16!(op03, set_bc, get_bc);
inc16!(op13, set_de, get_de);
inc16!(op23, set_hl, get_hl);
inc16!(op33, set_sp, get_sp);

macro_rules! dec16 {
    ( $name:ident, $dst:ident, $src:ident ) => {
        pub fn $name(cpu: &mut Cpu, _mem: &impl CpuMemory) -> Cycles {
            let val = cpu.$src().wrapping_sub(1);
            cpu.$dst(val);
            cpu.pc += 1;
            Cycles(8)
        }
    };
}
dec16!(op0b, set_bc, get_bc);
dec16!(op1b, set_de, get_de);
dec16!(op2b, set_hl, get_hl);
dec16!(op3b, set_sp, get_sp);
