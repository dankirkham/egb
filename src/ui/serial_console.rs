use egui::*;

use crate::ui::*;

const LINES: usize = 12;

pub struct SerialConsole<'a> {
    console: &'a mut String,
}

impl<'a> SerialConsole<'a> {
    pub fn new(console: &'a mut String) -> Self {
        Self { console }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        let lines = self.console.split('\n').collect::<Vec<_>>();
        if lines.len() > LINES {
            let mut new = String::default();
            for (idx, line) in lines.into_iter().rev().take(LINES).rev().enumerate() {
                new.push_str(line);
                if idx < LINES - 1 {
                    new.push('\n');
                }
            }
            std::mem::swap(self.console, &mut new);
        }
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(title(ui, "serial monitor"));
                ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                    if ui.button(monospace("🗋")).clicked() {
                        self.console.clear()
                    }
                });
            });

            let te = TextEdit::multiline(self.console)
                .code_editor()
                .frame(true)
                .vertical_align(Align::Max)
                .interactive(false);
            ui.add_sized(ui.available_size(), te);
        });
    }
}
