use egui::*;

use crate::ui::*;

pub struct Indicator<'a> {
    lit: bool,
    label: &'a str,
}

impl<'a> Indicator<'a> {
    pub fn new(lit: bool, label: &'a str) -> Self {
        Self { lit, label }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        if self.lit {
            let s = monospace(format!("🔴 {}", self.label)).color(highlight(ui));
            ui.label(s);
        } else {
            let s = monospace(format!("⭕ {}", self.label));
            ui.label(s);
        }
    }
}
