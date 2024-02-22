use std::time::Duration;

use egui::*;
use egui_notify::Toasts;

use crate::ui::*;

pub struct AddressInput<'a> {
    toasts: &'a mut Toasts,
    edit_string: &'a mut String,
}

impl<'a> AddressInput<'a> {
    pub fn new(toasts: &'a mut Toasts, edit_string: &'a mut String) -> Self {
        Self {
            toasts,
            edit_string,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) -> Option<u16> {
        ui.horizontal(|ui| {
            ui.label(monospace("0x"));
            let tb_response = ui.add(TextEdit::singleline(self.edit_string).desired_width(30.));
            let tb_submit =
                tb_response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter));
            let button_response = ui.button(monospace("⮩"));
            let button_submit = button_response.clicked();
            if tb_submit || button_submit {
                let without_prefix = self.edit_string.trim_start_matches("0x");
                match u16::from_str_radix(without_prefix, 16) {
                    Ok(address) => {
                        self.edit_string.clear();
                        return Some(address);
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
