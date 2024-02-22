use egui::*;
use egui_notify::Toasts;

use crate::debugger::{Command, Debugger};
use crate::ui::*;

pub struct Breakpoints<'a> {
    debugger: &'a mut Option<Debugger>,
    toasts: &'a mut Toasts,
    bp_string: &'a mut String,
}

impl<'a> Breakpoints<'a> {
    pub fn new(
        debugger: &'a mut Option<Debugger>,
        toasts: &'a mut Toasts,
        bp_string: &'a mut String,
    ) -> Self {
        Self {
            debugger,
            toasts,
            bp_string,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.label(title(ui, "breakpoints"));
        if let Some(debugger) = self.debugger.as_mut() {
            ui.horizontal(|ui| {
                let mut input = AddressInput::new(self.toasts, self.bp_string);
                if let Some(addr) = input.ui(ui) {
                    debugger.command(Command::InsertBreakpoint(addr));
                }
            });
            egui::ScrollArea::vertical().show(ui, |ui| {
                for bp in debugger.get_breakpoints().iter() {
                    ui.horizontal(|ui| {
                        if ui.button(monospace("➖")).clicked() {
                            debugger.command(Command::RemoveBreakpoint(*bp));
                        }
                        ui.label(monospace(format!("0x{:04x}", bp)));
                    });
                }
            });
        }
    }
}
