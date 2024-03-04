use std::time::Duration;

use egui::*;
use egui_notify::Toasts;

use crate::ui::*;

pub struct SpeedInput<'a> {
    toasts: &'a mut Toasts,
    edit_string: &'a mut String,
}

impl<'a> SpeedInput<'a> {
    pub fn new(toasts: &'a mut Toasts, edit_string: &'a mut String) -> Self {
        Self {
            toasts,
            edit_string,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) -> Option<f64> {
        ui.horizontal(|ui| {
            ui.label(monospace("🎯 target speed"));
            let tb_response = ui.add(TextEdit::singleline(self.edit_string).desired_width(30.));
            let tb_submit =
                tb_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
            let button_response = ui.button(monospace("⮩"));
            let button_submit = button_response.clicked();
            if tb_submit || button_submit {
                match str::parse::<f64>(&self.edit_string) {
                    Ok(value) => {
                        if value > 0. {
                            return Some(value);
                        }
                        self.toasts
                            .error("Error: Speed must be > 0.")
                            .set_duration(Some(Duration::from_secs(5)));
                    }
                    Err(e) => {
                        self.toasts
                            .error(format!("Error: {}", e))
                            .set_duration(Some(Duration::from_secs(5)));
                    }
                }
            }
            None
        })
        .inner
    }
}
