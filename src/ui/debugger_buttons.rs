use egui::*;

use crate::debugger::{Command, Debugger};
use crate::ui::*;

macro_rules! button {
    ($ui:ident, $dbg:ident, $cmd:expr, $label:literal, $tf:literal) => {
        if $ui
            .add_enabled(
                $dbg.is_running() == $tf,
                egui::Button::new(monospace($label)),
            )
            .clicked()
        {
            $dbg.command($cmd);
        }
    };
}

pub struct DebuggerButtons<'a> {
    debugger: &'a mut Option<Debugger>,
}

impl<'a> DebuggerButtons<'a> {
    pub fn new(debugger: &'a mut Option<Debugger>) -> Self {
        Self { debugger }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(title(ui, "debugger"));
            if let Some(debugger) = self.debugger.as_mut() {
                button!(ui, debugger, Command::Pause, "⏸", true);
                button!(ui, debugger, Command::Continue, "▶", false);
                button!(ui, debugger, Command::Step, "⏭", false);
                button!(ui, debugger, Command::StepIn, "⮫", false);
                button!(ui, debugger, Command::StepOut, "⮩", false);
            }
        });
    }
}
