use crate::memory::Memory;
use crate::memory_map::MemoryMap;
use crate::registers::SerialControl;

#[derive(Default)]
pub struct Serial;

impl Serial {
    pub fn tick(&mut self, mem: &mut Memory) -> Option<u8> {
        let mut sc = mem.get_reg::<SerialControl>(MemoryMap::SC);
        if sc.contains(SerialControl::TransferEnable) && sc.contains(SerialControl::MasterClock) {
            let data_in = mem.get_u8(MemoryMap::SB);
            mem.set_u8(MemoryMap::SB, 0xff); // Unconnected is pulled high
            sc.set(SerialControl::TransferEnable, false);
            mem.set_reg(MemoryMap::SC, sc);

            let c = data_in as char;
            if c.is_ascii_graphic() || c.is_whitespace() {
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
