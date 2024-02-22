use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct TimerControl: u8 {
        const Enable = 1 << 2;
        const ClockSelect1 = 1 << 1;
        const ClockSelect0 = 1 << 0;
        const _ = !0;
    }
}

impl TimerControl {
    pub fn get_speed(&self) -> u16 {
        match self.bits() & 0b11 {
            0b00 => 1 << 9,
            0b11 => 1 << 7,
            0b10 => 1 << 5,
            0b01 => 1 << 3,
            _ => unreachable!(),
        }
    }
}
