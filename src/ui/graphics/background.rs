use egui::*;

use crate::ppu::Ppu;
use crate::ui::*;

pub struct Background<'a, 'b> {
    ppu: &'a Ppu,
    image: &'a mut Option<Image<'b>>,
    texture: &'a mut Option<TextureHandle>,
}

impl<'a, 'b> Background<'a, 'b> {
    pub fn new(
        ppu: &'a Ppu,
        image: &'a mut Option<Image<'b>>,
        texture: &'a mut Option<TextureHandle>,
    ) -> Self {
        Self {
            ppu,
            image,
            texture,
        }
    }

    fn generate_background(&mut self, ui: &mut Ui) {
        let color_image = self.ppu.get_background();
        let width = color_image.size[0] as f32;
        let height = color_image.size[1] as f32;
        let handle = ui
            .ctx()
            .load_texture("background", color_image, TextureOptions::default());
        let sized_image = egui::load::SizedTexture::new(handle.id(), egui::vec2(width, height));
        let image = egui::Image::from_texture(sized_image).maintain_aspect_ratio(true);
        let mut image = Some(image);
        let mut texture = Some(handle);
        std::mem::swap(self.texture, &mut texture);
        std::mem::swap(self.image, &mut image);
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        self.generate_background(ui);
        ui.vertical(|ui| {
            ui.label(title(ui, "background"));
            if let Some(image) = self.image {
                let size = ui.available_size();
                let image = image.clone().fit_to_exact_size(size);
                ui.add(image);
            }
        });
    }
}
