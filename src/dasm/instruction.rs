use std::fmt::Display;

#[derive(Clone)]
pub struct Instruction {
    pub addr: u16,
    repr: String,
    symbol: Option<String>,
}

impl Instruction {
    pub fn new(addr: u16, repr: String, symbol: Option<String>) -> Self {
        Self { addr, repr, symbol }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(ref symbol) = self.symbol {
            write!(f, "{}: 0x{:04x}: {}", symbol, self.addr, self.repr)
        } else {
            write!(f, "0x{:04x}: {}", self.addr, self.repr)
        }
    }
}
