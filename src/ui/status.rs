use egui::*;

use crate::ui::monospace;

pub struct Status {
    speed: f64,
}

impl Status {
    pub fn new(speed: f64) -> Self {
        Self { speed }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
            ui.label(monospace(format!("⚡ {:.2}x", self.speed)));
        });
    }
}
