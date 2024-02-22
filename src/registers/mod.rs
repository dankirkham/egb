pub mod graphics;
pub mod timer;

use bitflags::bitflags;

bitflags! {
    pub struct CpuFlags: u8 {
        /// Zero Flag
        const Z = 1 << 7;
        /// Subtract Flag
        const N = 1 << 6;
        /// Half Carry Flag
        const H = 1 << 5;
        /// Carry Flag
        const C = 1 << 4;

        const _ = !0;
    }
}

impl Default for CpuFlags {
    fn default() -> Self {
        Self::from_bits(0xb0).unwrap()
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct Interrupt: u8 {
        const Joypad = 1 << 4;
        const Serial = 1 << 3;
        const Timer = 1 << 2;
        const LcdStat = 1 << 1;
        const VBlank = 1;

        const _ = !0;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct JoypadInput: u8 {
        const Select_Buttons = 1 << 5;
        const Select_Dpad = 1 << 4;
        const Start_Down = 1 << 3;
        const Select_Up = 1 << 2;
        const B_Left = 1 << 1;
        const A_Right = 1;

        const _ = !0;
    }
}
