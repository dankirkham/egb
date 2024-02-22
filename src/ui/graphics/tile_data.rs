use egui::*;
use image::{DynamicImage, EncodableLayout, RgbImage};

use crate::memory::Memory;
use crate::ui::graphics::{generate_tile, TILE_SIZE};
use crate::ui::*;

pub struct TileData<'a, 'b> {
    image: &'a mut Option<Image<'b>>,
    mem: &'a Memory,
    texture: &'a mut Option<TextureHandle>,
}

impl<'a, 'b> TileData<'a, 'b> {
    pub fn new(
        mem: &'a Memory,
        image: &'a mut Option<Image<'b>>,
        texture: &'a mut Option<TextureHandle>,
    ) -> Self {
        Self {
            mem,
            image,
            texture,
        }
    }

    fn generate_tiles(&mut self, ui: &mut Ui, addr: u16) {
        const COLS: usize = 16;
        const ROWS: usize = 8 * 3;
        assert_eq!(COLS * ROWS, 128 * 3);

        let mut image = RgbImage::new((COLS * TILE_SIZE) as u32, (ROWS * TILE_SIZE) as u32);
        for row in 0..ROWS {
            for col in 0..COLS {
                let tile_idx = row * COLS + col;
                generate_tile(self.mem, &mut image, addr, tile_idx, row, col);
            }
        }

        let image = DynamicImage::from(image);

        let color_image = match &image {
            DynamicImage::ImageRgb8(image) => {
                // common case optimization
                egui::ColorImage::from_rgb(
                    [image.width() as usize, image.height() as usize],
                    image.as_bytes(),
                )
            }
            other => {
                let image = other.to_rgba8();
                egui::ColorImage::from_rgba_unmultiplied(
                    [image.width() as usize, image.height() as usize],
                    image.as_bytes(),
                )
            }
        };
        // you must keep the handle, if the handle is destroyed so the texture will be destroyed as well
        let handle =
            ui.ctx()
                .load_texture("tile_data", color_image.clone(), TextureOptions::default());
        let sized_image = egui::load::SizedTexture::new(
            handle.id(),
            egui::vec2(color_image.size[0] as f32, color_image.size[1] as f32),
        );
        let image = egui::Image::from_texture(sized_image).maintain_aspect_ratio(true);
        let mut image = Some(image);
        let mut texture = Some(handle);
        std::mem::swap(self.texture, &mut texture);
        std::mem::swap(self.image, &mut image);
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        self.generate_tiles(ui, 0x8000);
        ui.vertical(|ui| {
            ui.label(title(ui, "tile data"));
            if let Some(image) = self.image {
                let size = ui.available_size();
                let image = image.clone().fit_to_exact_size(size);
                ui.add(image);
            }
        });
    }
}
