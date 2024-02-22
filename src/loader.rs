use std::fs::File;
use std::io::Read;

use crate::debugger::Debugger;
use crate::gameboy::Gameboy;
use crate::memory::Memory;
use crate::rom::{Rom, Data};
use crate::symbols::Symbols;

#[derive(Default)]
pub struct Loader {
    pub rom: Rom,
    rom_path: Option<String>,
    symbols: Option<Symbols>,
}

impl Loader {
    pub fn new(rom: Rom, rom_path: Option<String>) -> Self {
        Self {
            rom,
            rom_path,
            symbols: None,
        }
    }

    pub fn with_symbols(mut self, symbols: Option<Symbols>) -> Self {
        self.symbols = symbols;
        self
    }

    pub fn load_rom(&self) -> Result<Gameboy, std::io::Error> {
        let data = match self.rom {
            Rom::File => {
                let mut f = File::open(self.rom_path.as_ref().unwrap())?;
                let mut data: Vec<u8> = Vec::with_capacity(0x8000);
                f.read_to_end(&mut data)?;
                data
            },
            Rom::Game2048 => {
                let file = Data::get("games/2048.gb").unwrap();
                let data = file.data;
                let data = data.into_owned();
                data
            },
            Rom::TestCpuInstr => {
                let file = Data::get("tests/cpu_instrs.gb").unwrap();
                let data = file.data;
                let data = data.into_owned();
                data
            },
        };
        let mem = Memory::from(data);
        let mut gameboy = Gameboy::new(mem);
        let debugger = Debugger::new(self.symbols.clone());
        gameboy.attach_debugger(Some(debugger));
        Ok(gameboy)
    }

    pub fn reset_gameboy(&self, gameboy: &mut Gameboy) -> Result<(), std::io::Error> {
        let debugger = gameboy.detach_debugger();
        let data = match self.rom {
            Rom::File => {
                let mut f = File::open(self.rom_path.as_ref().unwrap())?;
                let mut data: Vec<u8> = Vec::with_capacity(0x8000);
                f.read_to_end(&mut data)?;
                data
            },
            Rom::Game2048 => {
                let file = Data::get("games/2048.gb").unwrap();
                let data = file.data;
                let data = data.into_owned();
                data
            },
            Rom::TestCpuInstr => {
                let file = Data::get("tests/cpu_instrs.gb").unwrap();
                let data = file.data;
                let data = data.into_owned();
                data
            },
        };
        let mem = Memory::from(data);
        let mut new_gameboy = Gameboy::new(mem);
        new_gameboy.attach_debugger(debugger);
        std::mem::swap(&mut new_gameboy, gameboy);
        Ok(())
    }
}
