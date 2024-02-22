use crate::registers::JoypadInput;

#[derive(Default)]
pub struct Buttons {
    pub start: bool,
    pub select: bool,
    pub b: bool,
    pub a: bool,
    pub down: bool,
    pub up: bool,
    pub left: bool,
    pub right: bool,

    select_buttons: bool,
    select_dpad: bool,
}

impl Buttons {
    pub fn write(&mut self, value: u8) {
        let command = JoypadInput::from_bits_retain(value);

        self.select_buttons = command.contains(JoypadInput::Select_Buttons);
        self.select_dpad = command.contains(JoypadInput::Select_Dpad);
    }

    pub fn read(&self) -> u8 {
        let buttons =
            (self.start as u8) << 3 | (self.select as u8) << 2 | (self.b as u8) << 1 | self.a as u8;

        let dpad = (self.down as u8) << 3
            | (self.up as u8) << 2
            | (self.left as u8) << 1
            | self.right as u8;

        let buttons = if self.select_buttons {
            buttons | 0xf
        } else {
            buttons
        };

        let dpad = if self.select_dpad { dpad | 0xf } else { dpad };

        let mut reg = buttons & dpad;
        reg |= (self.select_dpad as u8) << 4;
        reg |= (self.select_buttons as u8) << 5;

        reg
    }
}
