use std::str::FromStr;

use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "roms/"]
pub struct Data;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum Rom {
    File,
    #[default]
    Game2048,
    TestCpuInstr,
}

impl FromStr for Rom {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "file" => Ok(Self::File),
            "2048" => Ok(Self::Game2048),
            "cpu_instr" => Ok(Self::TestCpuInstr),
            _ => Err(()),
        }
    }
}
