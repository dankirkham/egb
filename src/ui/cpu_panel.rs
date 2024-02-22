use egui::*;

use crate::cpu::Cpu;
use crate::registers::CpuFlags;
use crate::ui::*;

pub struct CpuPanel<'a> {
    cpu: &'a Cpu,
}

impl<'a> CpuPanel<'a> {
    pub fn new(cpu: &'a Cpu) -> Self {
        Self { cpu }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.label(title(ui, "cpu"));
        ui.label(monospace(format!("{}", self.cpu)));

        Indicator::new(self.cpu.f.contains(CpuFlags::Z), "Z").ui(ui);
        Indicator::new(self.cpu.f.contains(CpuFlags::N), "N").ui(ui);
        Indicator::new(self.cpu.f.contains(CpuFlags::H), "H").ui(ui);
        Indicator::new(self.cpu.f.contains(CpuFlags::C), "C").ui(ui);
        Indicator::new(self.cpu.ie, "IME").ui(ui);
    }
}
