use crate::memory::PrivilegedMemory;
use crate::memory_map::MemoryMap;

#[derive(Default)]
pub struct Serial;

impl Serial {
    pub fn tick(&mut self, mem: &mut impl PrivilegedMemory) -> Option<u8> {
        if mem.get_u8(MemoryMap::SC) == 0x81 {
            let data_in = mem.get_u8(MemoryMap::SB);
            mem.set_u8(MemoryMap::SB, 0xff);
            mem.set_u8(MemoryMap::SC, 0x01);

            let c = data_in as char;
            if c.is_ascii_graphic() || c.is_whitespace() {
                print!("{}", c);
                Some(data_in)
            } else {
                None
            }
            // TODO: Serial interrupt
        } else {
            None
        }
    }
}
