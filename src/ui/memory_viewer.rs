use egui::*;
use egui_notify::Toasts;

use crate::cpu::Cpu;
use crate::memory::{CpuMemory, Memory};
use crate::ui::*;

#[derive(Default, PartialEq)]
pub enum MemoryViewerSetting {
    #[default]
    PC,
    HL,
    SP,
    BC,
    DE,
    Manual,
}

#[derive(Default)]
pub struct MemoryViewerState {
    setting: MemoryViewerSetting,
    address: u16,
    edit_string: String,
}

pub struct MemoryViewer<'a> {
    mem: &'a Memory,
    cpu: &'a Cpu,
    toasts: &'a mut Toasts,
    state: &'a mut MemoryViewerState,
}

impl<'a> MemoryViewer<'a> {
    pub fn new(
        mem: &'a Memory,
        cpu: &'a Cpu,
        toasts: &'a mut Toasts,
        state: &'a mut MemoryViewerState,
    ) -> Self {
        Self {
            mem,
            cpu,
            toasts,
            state,
        }
    }

    fn memory_cell(&self, cell_addr: u16, addr: u16, ui: &mut Ui) {
        let mut text = monospace(format!("{:02x}", self.mem.get_u8(cell_addr)));
        if cell_addr == addr {
            text = text.color(highlight(ui));
        }
        ui.label(text);
    }

    fn memory_cell_ascii(&self, cell_addr: u16, addr: u16, ui: &mut Ui) {
        let c = self.mem.get_u8(cell_addr) as char;
        let c = if c.is_ascii_graphic() { c } else { '.' };
        let mut text = monospace(format!("{}", c));
        if cell_addr == addr {
            text = text.color(highlight(ui));
        }
        ui.label(text);
    }

    fn memory_row(&self, row_addr: u16, addr: u16, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(monospace(format!("0x{:04x}: ", row_addr)));

            for i in 0..8 {
                self.memory_cell(row_addr + i * 2, addr, ui);
                self.memory_cell(row_addr + i * 2 + 1, addr, ui);
                ui.label(monospace(" "));
            }

            for i in 0..16 {
                self.memory_cell_ascii(row_addr + i, addr, ui);
            }
        });
    }

    fn memory_at(&self, addr: u16, ui: &mut Ui) {
        let rem = addr as u32 % 0x10;
        let row_addr = addr as u32 - rem;

        let (start, end) = if row_addr < 0x40 {
            (0, 9 * 0x10)
        } else if row_addr >= 0xff70 {
            (0xff70, 0x10000)
        } else {
            let start = row_addr - 0x40;
            (start, start + 9 * 0x10)
        };

        ui.style_mut().spacing.item_spacing = vec2(0.0, 0.0);
        ui.vertical(|ui| {
            for row_addr in (start..end).step_by(0x10) {
                self.memory_row(row_addr as u16, addr, ui);
            }
        });
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.label(title(ui, "memory viewer"));
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.state.setting, MemoryViewerSetting::PC, "PC");
                ui.selectable_value(&mut self.state.setting, MemoryViewerSetting::HL, "HL");
                ui.selectable_value(&mut self.state.setting, MemoryViewerSetting::SP, "SP");
                ui.selectable_value(&mut self.state.setting, MemoryViewerSetting::BC, "BC");
                ui.selectable_value(&mut self.state.setting, MemoryViewerSetting::DE, "DE");
                ui.selectable_value(
                    &mut self.state.setting,
                    MemoryViewerSetting::Manual,
                    monospace(format!("0x{:04x}", self.state.address)),
                );
                ui.separator();
                let mut input = AddressInput::new(self.toasts, &mut self.state.edit_string);
                if let Some(addr) = input.ui(ui) {
                    self.state.address = addr;
                    self.state.setting = MemoryViewerSetting::Manual;
                }
            });

            let addr = match self.state.setting {
                MemoryViewerSetting::PC => self.cpu.pc,
                MemoryViewerSetting::HL => self.cpu.get_hl(),
                MemoryViewerSetting::SP => self.cpu.get_sp(),
                MemoryViewerSetting::BC => self.cpu.get_bc(),
                MemoryViewerSetting::DE => self.cpu.get_de(),
                MemoryViewerSetting::Manual => self.state.address,
            };
            self.memory_at(addr, ui);
        });
    }
}
