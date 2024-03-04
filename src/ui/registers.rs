use bitflags::Flags;

use egui::*;

use crate::memory::Memory;
use crate::memory_map::MemoryMap;
use crate::registers::{graphics::*, timer::*, *};
use crate::ui::*;

pub struct Registers<'a> {
    mem: &'a Memory,
}

macro_rules! reg {
    ($self:ident, $ui:ident, $label:literal, $address:expr) => {
        let reg = $self.mem.get_u8($address);
        $ui.label(monospace(format!("{}: 0x{:02x}", $label, reg)));
    };
    ($self:ident, $ui:ident, $label:literal, $address:expr, $type:ident) => {
        egui::CollapsingHeader::new($label)
            .default_open(true)
            .show($ui, |ui| {
                let reg = $self.mem.get_reg::<$type>($address);
                for flag in $type::FLAGS.iter() {
                    if flag.is_named() {
                        let set = reg.contains(*flag.value());
                        Indicator::new(set, flag.name()).ui(ui);
                    }
                }
            });
    };
}

impl<'a> Registers<'a> {
    pub fn new(mem: &'a Memory) -> Self {
        Self { mem }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.label(title(ui, "registers"));

        egui::ScrollArea::vertical()
            .id_source("registers")
            .show(ui, |ui| {
                reg!(self, ui, "IE", MemoryMap::IE, Interrupt);
                reg!(self, ui, "IF", MemoryMap::IF, Interrupt);
                egui::CollapsingHeader::new("Graphics")
                    .default_open(true)
                    .show(ui, |ui| {
                        reg!(self, ui, "LCDC", MemoryMap::LCDC, LcdControl);
                        reg!(self, ui, "STAT", MemoryMap::STAT, LcdStatus);
                        reg!(self, ui, "LY", MemoryMap::LY);
                        reg!(self, ui, "LYC", MemoryMap::LYC);
                    });
                egui::CollapsingHeader::new("Timer")
                    .default_open(true)
                    .show(ui, |ui| {
                        reg!(self, ui, "TAC", MemoryMap::TAC, TimerControl);
                        reg!(self, ui, "DIV", MemoryMap::DIV);
                        reg!(self, ui, "TIMA", MemoryMap::TIMA);
                        reg!(self, ui, "TMA", MemoryMap::TMA);
                    });
                reg!(self, ui, "Joypad", MemoryMap::Joypad, JoypadInput);
                // reg!(self, ui, "BootRomDisable", MemoryMap::BootRomDisable);
            });
    }
}
