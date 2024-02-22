use crate::cpu::{Cpu, Cycles};
use crate::memory::CpuMemory;
use crate::registers::CpuFlags;

macro_rules! ld_n_nn {
    ( $name:ident, $dst:ident ) => {
        pub fn $name(cpu: &mut Cpu, mem: &impl CpuMemory) -> Cycles {
            cpu.$dst(mem.get_u16(cpu.pc.wrapping_add(1)));
            cpu.pc += 3;
            Cycles(12)
        }
    };
}
ld_n_nn!(op01, set_bc);
ld_n_nn!(op11, set_de);
ld_n_nn!(op21, set_hl);
ld_n_nn!(op31, set_sp);

pub fn opf9(cpu: &mut Cpu, _mem: &impl CpuMemory) -> Cycles {
    cpu.set_sp(cpu.get_hl());
    cpu.pc += 1;
    Cycles(8)
}

pub fn opf8(cpu: &mut Cpu, mem: &impl CpuMemory) -> Cycles {
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

    cpu.set_hl(result);
    cpu.pc += 2;
    Cycles(12)
}

pub fn op08(cpu: &mut Cpu, mem: &mut impl CpuMemory) -> Cycles {
    let addr = mem.get_u16(cpu.pc.wrapping_add(1));
    mem.set_u16(addr, cpu.sp);
    cpu.pc += 3;
    Cycles(20)
}

macro_rules! push {
    ( $name:ident, $src:ident ) => {
        pub fn $name(cpu: &mut Cpu, mem: &mut impl CpuMemory) -> Cycles {
            cpu.sp -= 2;
            mem.set_u16(cpu.sp, cpu.$src());
            cpu.pc += 1;
            Cycles(16)
        }
    };
}
push!(opf5, get_af);
push!(opc5, get_bc);
push!(opd5, get_de);
push!(ope5, get_hl);

macro_rules! pop {
    ( $name:ident, $dst:ident ) => {
        pub fn $name(cpu: &mut Cpu, mem: &impl CpuMemory) -> Cycles {
            cpu.$dst(mem.get_u16(cpu.sp));
            cpu.sp += 2;
            cpu.pc += 1;
            Cycles(12)
        }
    };
}
pop!(opc1, set_bc);
pop!(opd1, set_de);
pop!(ope1, set_hl);
pub fn opf1(cpu: &mut Cpu, mem: &impl CpuMemory) -> Cycles {
    cpu.set_af(mem.get_u16(cpu.sp) & 0xfff0);
    cpu.sp += 2;
    cpu.pc += 1;
    Cycles(12)
}
