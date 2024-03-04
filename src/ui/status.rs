use egui::*;
use egui_notify::Toasts;

use crate::governor::Governor;
use crate::ui::*;

pub struct Status<'a> {
    toasts: &'a mut Toasts,
    governor: &'a mut Governor,
}

impl<'a> Status<'a> {
    pub fn new(toasts: &'a mut Toasts, governor: &'a mut Governor) -> Self {
        Self { toasts, governor }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(monospace(format!(
                "⚡ actual speed {:.2}x",
                self.governor.average_speed()
            )));
            ui.separator();
            let mut input = SpeedInput::new(self.toasts, self.governor.edit_string());
            if let Some(speed) = input.ui(ui) {
                self.governor.set_speed(speed);
            }
        });
    }
}
