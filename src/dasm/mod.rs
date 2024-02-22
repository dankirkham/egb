mod decoder;
mod instruction;

use std::collections::HashMap;
use std::ops::Range;

use bytes::BytesMut;

use crate::dasm::{decoder::Decoder, instruction::Instruction};
use crate::symbols::Symbols;

pub struct Disassembly {
    instructions: Vec<Instruction>,
    address_to_instruction: HashMap<u16, usize>,
}

impl Disassembly {
    pub fn from_bytes(bytes: BytesMut, symbols: Option<&Symbols>) -> Self {
        let decoder = Decoder::from_bytes(bytes, symbols);
        let mut instructions = Vec::new();
        let mut address_to_instruction = HashMap::new();
        let decoder = decoder.enumerate();

        for (idx, (addr, instr)) in decoder {
            instructions.push(instr);
            address_to_instruction.insert(addr, idx);
        }

        Self {
            instructions,
            address_to_instruction,
        }
    }

    pub fn get_instructions_near(&self, pc: u16, range: Range<isize>) -> Option<Vec<Instruction>> {
        let idx = if let Some(idx) = self.address_to_instruction.get(&pc) {
            *idx
        } else {
            return None;
        };

        let start = if (idx as isize) + range.start <= 0 {
            0
        } else {
            idx.wrapping_add_signed(range.start)
        };

        let end = if (idx as isize) + range.end > (self.instructions.len() - 1) as isize {
            self.instructions.len() - 1
        } else {
            idx.wrapping_add_signed(range.end)
        };

        Some(Vec::from(&self.instructions[start..end]))
    }
}
