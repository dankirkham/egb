use egui::*;

use crate::gameboy::Gameboy;
use crate::loader::Loader;
use crate::rom::Rom;
use crate::ui::*;

pub struct Toolbar<'a> {
    gameboy: &'a mut Gameboy,
    loader: &'a mut Loader,
    developer_mode: &'a mut bool,
}

impl<'a> Toolbar<'a> {
    pub fn new(gameboy: &'a mut Gameboy, loader: &'a mut Loader, developer_mode: &'a mut bool) -> Self {
        Self {
            gameboy,
            loader,
            developer_mode,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.menu_button("🎮 menu", |ui| {
                if ui
                    .checkbox(self.developer_mode, monospace("developer mode"))
                    .clicked()
                {
                    ui.close_menu();
                }
                ui.separator();
                ui.label(monospace("🖴 games"));
                if ui.radio_value(&mut self.loader.rom, Rom::Game2048, "2048").clicked() {
                    let _ = self.loader.reset_gameboy(self.gameboy);
                    ui.close_menu();
                }
                ui.separator();
                ui.label(monospace("🖴 tests"));
                if ui.radio_value(&mut self.loader.rom, Rom::TestCpuInstr, "cpu_instr").clicked() {
                    let _ = self.loader.reset_gameboy(self.gameboy);
                    ui.close_menu();
                }
                ui.separator();
                if ui.button("↺ reset").clicked() {
                    let _ = self.loader.reset_gameboy(self.gameboy);
                    ui.close_menu();
                }
            });
            if *self.developer_mode {
                ui.separator();
                DebuggerButtons::new(&mut self.gameboy.debugger).ui(ui);
            }
            ui.with_layout(egui::Layout::right_to_left(Align::Center), |ui| {
                global_dark_light_mode_switch(ui);
            });
        });
    }
}
