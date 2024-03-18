use egui::*;

use crate::ppu::Ppu;
use crate::ui::*;

#[derive(Default, PartialEq)]
pub enum ViewerSelectSetting {
    #[default]
    Tiles,
    Background,
    Objects,
}

#[derive(Default)]
pub struct ViewerSelectState<'a> {
    setting: ViewerSelectSetting,
    background: ViewerContext<'a>,
    tiles: ViewerContext<'a>,
    objects: ViewerContext<'a>,
}

pub struct ViewerSelect<'a, 'b> {
    ppu: &'a Ppu,
    state: &'a mut ViewerSelectState<'b>,
}

impl<'a, 'b> ViewerSelect<'a, 'b> {
    pub fn new(
        ppu: &'a Ppu,
        state: &'a mut ViewerSelectState<'b>
    ) -> Self {
        Self {
            ppu,
            state,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.vertical(|ui| {
            ui.label(title(ui, "graphics viewer"));
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.state.setting, ViewerSelectSetting::Tiles, "tiles");
                ui.selectable_value(&mut self.state.setting, ViewerSelectSetting::Background, "background");
                ui.selectable_value(&mut self.state.setting, ViewerSelectSetting::Objects, "objects");
            });

            match self.state.setting {
                ViewerSelectSetting::Tiles => {
                    Viewer::new(&mut self.state.tiles).ui(
                        ui,
                        "tiles",
                        self.ppu.get_tiles(),
                    );
                },
                ViewerSelectSetting::Background => {
                    Viewer::new(&mut self.state.background).ui(
                        ui,
                        "background",
                        self.ppu.get_background(),
                    );
                },
                ViewerSelectSetting::Objects => {
                    Viewer::new(&mut self.state.objects).ui(
                        ui,
                        "objects",
                        self.ppu.get_objects(),
                    );
                },
            };
        });
    }
}
