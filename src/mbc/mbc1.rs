use bytes::BytesMut;

use crate::mbc::Mbc;
use crate::memory_map::MemoryMap;

pub struct Mbc1 {
    fixed_rom: [u8; 0x4000],
    switchable_rom: Vec<[u8; 0x4000]>,
    vram: [u8; 0x2000],
    external_ram: Vec<[u8; 0x2000]>,
    wram1: [u8; 0x1000],
    wram2: [u8; 0x1000],
    upper_ram: [u8; 0x0200],
    advanced_banking_mode: bool,
    rom_size: usize,
    ram_enable: bool,
    primary_banking: usize,
    secondary_banking: usize,
}

impl Mbc1 {
    pub fn new(data: &[u8]) -> Self {
        assert!(data.len() >= 0x8000);
        let fixed_rom = data[0x0000..0x4000].try_into().unwrap();

        assert!(matches!(
            data[MemoryMap::HeaderCartridgeType as usize],
            0x00..=0x03
        ));

        let rom_size = data[MemoryMap::HeaderRomSize as usize];
        let rom_size = match rom_size {
            0 => 2,
            1 => 4,
            2 => 8,
            3 => 16,
            4 => 32,
            5 => 64,
            6 => 128,
            7 => 256,
            _ => panic!("Invalid ROM size for MBC1"),
        };

        let ram_size = data[MemoryMap::HeaderRamSize as usize];
        let ram_size = match ram_size {
            0x00 => 1,
            0x01 => 1,
            0x02 => 1,
            0x03 => 4,
            0x04 => panic!("How are these addressed"),
            0x05 => panic!("How are these addressed"),
            _ => panic!("Invalid RAM size"),
        };

        let remaining_length = data.len() - 0x4000;
        assert_eq!(remaining_length % 0x4000, 0);
        let pages = remaining_length / 0x4000;
        assert_eq!(pages, rom_size - 1);
        let mut switchable_rom = Vec::with_capacity(pages);
        for page in 0..pages {
            let start = 0x4000 + page * 0x4000;
            let end = start + 0x4000;
            let page_data: [u8; 0x4000] = data[start..end].try_into().unwrap();
            switchable_rom.push(page_data);
        }

        let external_ram = vec![[0; 0x2000]; ram_size];

        Self {
            fixed_rom,
            switchable_rom,
            vram: [0; 0x2000],
            external_ram,
            wram1: [0; 0x1000],
            wram2: [0; 0x1000],
            upper_ram: [0; 0x0200],
            advanced_banking_mode: false,
            rom_size,
            ram_enable: false,
            primary_banking: 1,
            secondary_banking: 0,
        }
    }
}

impl Mbc for Mbc1 {
    fn set_u8(&mut self, address: impl Into<u16>, value: u8) {
        let address = address.into();
        match address {
            0x0000..=0x1fff => self.ram_enable = value & 0x0f == 0xa,
            0x2000..=0x3fff => {
                let value = value & (self.rom_size as u8 - 1);
                let bank = if value == 0 { 1 } else { value as usize };
                self.primary_banking = bank;
            }
            0x4000..=0x5fff => {
                let value = value & 0x03;
                self.secondary_banking = value as usize;
            }
            0x6000..=0x7fff => {
                self.advanced_banking_mode = value & 0x01 != 0;
            }
            0x8000..=0x9fff => self.vram[address as usize - 0x8000] = value,
            0xa000..=0xbfff => {
                if self.ram_enable {
                    if self.advanced_banking_mode {
                        self.external_ram[self.secondary_banking][address as usize - 0xa000] = value
                    } else {
                        self.external_ram[0][address as usize - 0xa000] = value
                    }
                }
            }
            0xc000..=0xcfff => self.wram1[address as usize - 0xc000] = value,
            0xd000..=0xdfff => self.wram2[address as usize - 0xd000] = value,
            0xe000..=0xefff => self.wram1[address as usize - 0xe000] = value,
            0xf000..=0xfdff => self.wram2[address as usize - 0xf000] = value,
            0xfe00..=0xffff => self.upper_ram[address as usize - 0xfe00] = value,
        }
    }

    fn get_u8(&self, address: impl Into<u16>) -> u8 {
        let address = address.into();
        match address {
            0x0000..=0x3fff => self.fixed_rom[address as usize],
            0x4000..=0x7fff => {
                if self.advanced_banking_mode {
                    let bank = ((self.secondary_banking & 0x3) << 4) | (self.primary_banking & 0xf);
                    self.switchable_rom[bank - 1][address as usize - 0x4000]
                } else {
                    self.switchable_rom[self.primary_banking - 1][address as usize - 0x4000]
                }
            }
            0x8000..=0x9fff => self.vram[address as usize - 0x8000],
            0xa000..=0xbfff => {
                if self.ram_enable {
                    if self.advanced_banking_mode {
                        self.external_ram[self.secondary_banking][address as usize - 0xa000]
                    } else {
                        self.external_ram[0][address as usize - 0xa000]
                    }
                } else {
                    0xff
                }
            }
            0xc000..=0xcfff => self.wram1[address as usize - 0xc000],
            0xd000..=0xdfff => self.wram2[address as usize - 0xd000],
            0xe000..=0xefff => self.wram1[address as usize - 0xe000],
            0xf000..=0xfdff => self.wram2[address as usize - 0xf000],
            0xfe00..=0xffff => self.upper_ram[address as usize - 0xfe00],
        }
    }

    fn dump(&self) -> bytes::BytesMut {
        let rom = if self.advanced_banking_mode {
            let bank = ((self.secondary_banking & 0x3) << 4) | (self.primary_banking & 0xf);
            &self.switchable_rom[bank - 1]
        } else {
            &self.switchable_rom[self.primary_banking - 1]
        };
        let ram = if self.advanced_banking_mode {
            self.external_ram[self.secondary_banking]
        } else {
            self.external_ram[0]
        };
        let iter = self
            .fixed_rom
            .iter()
            .chain(rom.iter())
            .chain(self.vram.iter())
            .chain(ram.iter())
            .chain(self.wram1.iter())
            .chain(self.wram2.iter())
            .chain(self.wram1.iter())
            .chain(self.upper_ram.iter());

        BytesMut::from_iter(iter)
    }
}
