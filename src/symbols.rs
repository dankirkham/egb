use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Clone)]
pub struct Symbols {
    address_to_symbol: HashMap<u16, String>,
}

impl Symbols {
    pub fn get_symbol(&self, addr: u16) -> Option<String> {
        self.address_to_symbol.get(&addr).cloned()
    }
}

impl TryFrom<File> for Symbols {
    type Error = std::io::Error;

    fn try_from(f: File) -> Result<Self, Self::Error> {
        let mut address_to_symbol = HashMap::new();
        let reader = BufReader::new(f);

        for line in reader.lines() {
            let line = line.unwrap();
            let mut tokens = line.split(' ');
            let addr = if let Some(s) = tokens.next() {
                let without_prefix = s.trim_start_matches("0x");
                match u16::from_str_radix(without_prefix, 16) {
                    Ok(bp) => bp,
                    Err(_) => {
                        panic!("handle errors...");
                    }
                }
            } else {
                panic!("handle errors...");
            };

            let symbol = if let Some(s) = tokens.next() {
                s
            } else {
                panic!("handle errors...");
            };

            address_to_symbol.insert(addr, symbol.to_string());
        }

        Ok(Self { address_to_symbol })
    }
}
