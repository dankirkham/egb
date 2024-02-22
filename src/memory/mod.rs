mod boot_rom;

use bitflags::Flags;
use bytes::BytesMut;

use crate::buttons::Buttons;
use crate::mbc::{Mbc, Mbc1};
use crate::memory_map::MemoryMap;

use self::boot_rom::BOOT_ROM;

macro_rules! memory_port {
    ($name:ident) => {
        pub trait $name {
            fn set_u8(&mut self, address: impl Into<u16>, value: u8);
            fn get_u8(&self, address: impl Into<u16>) -> u8;

            fn set_u16(&mut self, address: impl Into<u16>, value: u16) {
                let address = address.into();
                self.set_u8(address, value as u8);
                self.set_u8(address.wrapping_add(1), (value >> 8) as u8);
            }

            fn get_u16(&self, address: impl Into<u16>) -> u16 {
                let address = address.into();
                let l = self.get_u8(address);
                let h = self.get_u8(address.wrapping_add(1));
                ((h as u16) << 8) | (l as u16)
            }

            fn get_reg<T: Flags<Bits = u8>>(&self, address: impl Into<u16>) -> T {
                T::from_bits(self.get_u8(address)).unwrap()
            }

            fn set_reg<T: Flags<Bits = u8>>(&mut self, address: impl Into<u16>, val: T) {
                self.set_u8(address, val.bits());
            }

            fn set_reg_flag<T: Flags<Bits = u8>>(&mut self, address: impl Into<u16>, val: T) {
                let address = address.into();
                let mut reg = self.get_reg::<T>(address);
                reg.set(val, true);
                self.set_reg(address, reg);
            }

            fn clear_reg_flag<T: Flags<Bits = u8>>(&mut self, address: impl Into<u16>, val: T) {
                let address = address.into();
                let mut reg = self.get_reg::<T>(address);
                reg.set(val, false);
                self.set_reg(address, reg);
            }

            /// Increments u8 in memory. Returns true if a carry occurs.
            fn inc_u8(&mut self, address: impl Into<u16>) -> bool {
                let address = address.into();
                let val = self.get_u8(address);

                match val.checked_add(1) {
                    Some(new_val) => {
                        self.set_u8(address, new_val);
                        false
                    }
                    None => {
                        self.set_u8(address, 0);
                        true
                    }
                }
            }
        }
    };
}
memory_port!(CpuMemory);
memory_port!(PpuMemory);
memory_port!(PrivilegedMemory);

pub struct Memory {
    pub mbc: Mbc1,
    pub buttons: Buttons,
}

impl From<&Memory> for BytesMut {
    fn from(val: &Memory) -> Self {
        val.mbc.dump()
    }
}

impl From<Vec<u8>> for Memory {
    fn from(data: Vec<u8>) -> Self {
        let mbc = Mbc1::new(&data);

        Self {
            mbc,
            buttons: Buttons::default(),
        }
    }
}

impl CpuMemory for Memory {
    fn set_u8(&mut self, address: impl Into<u16>, value: u8) {
        let address = address.into();
        match address {
            0xff46 => {
                // oam dma
                let start_addr = (value as u16) << 8;
                for i in 0..0x9f {
                    let val = self.mbc.get_u8(start_addr + i);
                    self.mbc.set_u8(0xfe00 + i, val);
                }
            }
            0xff00 => {
                self.buttons.write(value);
            }
            _ => self.mbc.set_u8(address, value),
        }
    }

    fn get_u8(&self, address: impl Into<u16>) -> u8 {
        let address = address.into();
        match address {
            0x0000..=0x00FF => {
                if self.mbc.get_u8(MemoryMap::BootRomDisable) == 0 {
                    BOOT_ROM[address as usize]
                } else {
                    self.mbc.get_u8(address)
                }
            }
            0xff00 => self.buttons.read(),
            // Unconnected IO registers always return 0xff
            0xff03
            | 0xff08
            | 0xff09
            | 0xff27..=0xff29
            | 0xff4c..=0xff4e
            | 0xff56..=0xff67
            | 0xff6c..=0xff6f
            | 0xff71..=0xff7f => 0xff,
            _ => self.mbc.get_u8(address),
        }
    }
}

impl PpuMemory for Memory {
    fn set_u8(&mut self, address: impl Into<u16>, value: u8) {
        self.mbc.set_u8(address, value);
    }

    fn get_u8(&self, address: impl Into<u16>) -> u8 {
        self.mbc.get_u8(address)
    }
}

impl PrivilegedMemory for Memory {
    fn set_u8(&mut self, address: impl Into<u16>, value: u8) {
        self.mbc.set_u8(address, value);
    }

    fn get_u8(&self, address: impl Into<u16>) -> u8 {
        let address = address.into();
        match address {
            0xff00 => self.buttons.read(),
            _ => self.mbc.get_u8(address),
        }
    }
}
