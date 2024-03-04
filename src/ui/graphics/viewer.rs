use egui::*;

use super::ViewerContext;

pub struct Viewer<'a, 'b> {
    context: &'a mut ViewerContext<'b>,
}

impl<'a, 'b> Viewer<'a, 'b> {
    pub fn new(context: &'a mut ViewerContext<'b>) -> Self {
        Self { context }
    }

    fn generate(&mut self, ui: &mut Ui, id: &str, color_image: ColorImage) {
        let width = color_image.size[0] as f32;
        let height = color_image.size[1] as f32;
        let handle = ui
            .ctx()
            .load_texture(id, color_image, TextureOptions::default());
        let sized_image = egui::load::SizedTexture::new(handle.id(), egui::vec2(width, height));
        let image = egui::Image::from_texture(sized_image).maintain_aspect_ratio(true);
        let mut image = Some(image);
        let mut texture = Some(handle);
        std::mem::swap(&mut self.context.texture, &mut texture);
        std::mem::swap(&mut self.context.image, &mut image);
    }

    pub fn ui(&mut self, ui: &mut Ui, id: &str, image: ColorImage) {
        self.generate(ui, id, image);
        ui.vertical(|ui| {
            if let Some(ref image) = self.context.image {
                let size = ui.available_size();
                let image = image.clone().fit_to_exact_size(size);
                ui.add(image);
            }
        });
    }
}
