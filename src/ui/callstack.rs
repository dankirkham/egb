// use egui::scroll_area::ScrollBarVisibility;
use egui::*;

use crate::debugger::Debugger;
use crate::ui::*;

pub struct Callstack<'a> {
    debugger: &'a mut Option<Debugger>,
}

impl<'a> Callstack<'a> {
    pub fn new(debugger: &'a mut Option<Debugger>) -> Self {
        Self { debugger }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.label(title(ui, "callstack"));
        if let Some(debugger) = self.debugger.as_mut() {
            egui::ScrollArea::vertical()
                // .stick_to_bottom(true)
                // .max_height(50.)
                // .auto_shrink(true)
                // .scroll_bar_visibility(ScrollBarVisibility::AlwaysVisible)
                .show(ui, |ui| {
                    for (idx, call) in debugger.get_callstack().iter().enumerate() {
                        ui.label(monospace(format!("{}: {}", idx, call)));
                    }
                });
        }
    }
}
