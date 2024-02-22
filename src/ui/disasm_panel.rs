use std::mem;

use egui::*;
use egui_notify::Toasts;

use crate::dasm::Disassembly;
use crate::memory::Memory;
use crate::symbols::Symbols;
use crate::ui::*;

#[derive(Default, PartialEq)]
pub enum DisasmPanelSetting {
    #[default]
    PC,
    Manual,
}

#[derive(Default)]
pub struct DisasmPanelState {
    setting: DisasmPanelSetting,
    address: u16,
    edit_string: String,
    symbols: Option<Symbols>,
}

impl DisasmPanelState {
    pub fn new(symbols: Option<Symbols>) -> Self {
        Self {
            symbols,
            ..Default::default()
        }
    }
}

pub struct DisasmPanel<'a> {
    disassembly: &'a mut Option<Disassembly>,
    mem: &'a Memory,
    pc: u16,
    state: &'a mut DisasmPanelState,
    toasts: &'a mut Toasts,
}

impl<'a> DisasmPanel<'a> {
    pub fn new(
        disassembly: &'a mut Option<Disassembly>,
        mem: &'a Memory,
        pc: u16,
        state: &'a mut DisasmPanelState,
        toasts: &'a mut Toasts,
    ) -> Self {
        Self {
            disassembly,
            mem,
            pc,
            state,
            toasts,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(title(ui, "disassembly"));
            ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                if ui.button(monospace("↻")).clicked() {
                    let mut disassembly = None;
                    mem::swap(self.disassembly, &mut disassembly);
                }
            });
        });

        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.state.setting, DisasmPanelSetting::PC, "PC");
            ui.selectable_value(
                &mut self.state.setting,
                DisasmPanelSetting::Manual,
                monospace(format!("0x{:04x}", self.state.address)),
            );
            ui.separator();
            let mut input = AddressInput::new(self.toasts, &mut self.state.edit_string);
            if let Some(addr) = input.ui(ui) {
                self.state.address = addr;
                self.state.setting = DisasmPanelSetting::Manual;
            }
        });

        let addr = match self.state.setting {
            DisasmPanelSetting::PC => self.pc,
            DisasmPanelSetting::Manual => self.state.address,
        };

        let disassembly = if let Some(ref disassembly) = self.disassembly {
            disassembly
        } else {
            let mut disassembly = Some(Disassembly::from_bytes(
                self.mem.into(),
                self.state.symbols.as_ref(),
            ));
            mem::swap(self.disassembly, &mut disassembly);
            self.disassembly.as_ref().unwrap()
        };
        if let Some(instructions) = disassembly.get_instructions_near(addr, -5..10) {
            let count = instructions.len();
            assert!(count >= 10);
            for instr in instructions.into_iter() {
                let mut rt = monospace(format!("{}", instr));
                if instr.addr == self.pc {
                    rt = rt.color(highlight(ui));
                }
                ui.label(rt);
            }
            for _ in 0..(15 - count) {
                ui.label("");
            }
        } else {
            for _ in 0..15 {
                ui.label("");
            }
        }
    }
}
