use egui::ColorImage;
use image::{DynamicImage, EncodableLayout, RgbImage};

use crate::ppu::constants;
use crate::ppu::pixel::Pixel;

pub struct Buffers {
    pub draw: GbImage,
    pub view: GbImage,
}

impl Buffers {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            draw: GbImage::new(width, height),
            view: GbImage::new(width, height),
        }
    }

    pub fn background() -> Self {
        let width = constants::BACKGROUND_COLS * constants::TILE_SIZE;
        let height = constants::BACKGROUND_ROWS * constants::TILE_SIZE;
        Self::new(width, height)
    }

    pub fn screen() -> Self {
        Self::new(constants::LCD_WIDTH as u32, constants::LCD_HEIGHT as u32)
    }

    pub fn tiles() -> Self {
        Self::new(16 * constants::TILE_SIZE, 8 * 3 * constants::TILE_SIZE)
    }

    pub fn swap(&mut self) {
        std::mem::swap(&mut self.draw, &mut self.view);
    }
}

pub struct GbImage {
    width: u32,
    height: u32,
    pixels: Vec<Pixel>,
}

impl GbImage {
    pub fn new(width: u32, height: u32) -> Self {
        let pixels = vec![Pixel::default(); (width * height) as usize];

        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, pixel: Pixel) {
        if x >= self.width || y >= self.height {
            return;
        }
        let idx = y * self.width + x;
        self.pixels[idx as usize] = pixel;
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Pixel {
        assert!(x < self.width);
        assert!(y < self.height);
        let idx = y * self.width + x;
        self.pixels[idx as usize]
    }

    pub fn get_pixel_wrapping(&self, x: u32, y: u32) -> Pixel {
        let x = x % self.width;
        let y = y % self.height;
        self.get_pixel(x, y)
    }
}

impl From<&GbImage> for RgbImage {
    fn from(val: &GbImage) -> Self {
        let mut image = RgbImage::new(val.width, val.height);
        for y in 0..val.height {
            for x in 0..val.width {
                let pixel = val.get_pixel(x, y);
                let rgb = pixel.into();
                image.put_pixel(x, y, rgb);
            }
        }
        image
    }
}

impl From<&GbImage> for ColorImage {
    fn from(val: &GbImage) -> Self {
        let rgb_image: RgbImage = val.into();
        let image = DynamicImage::from(rgb_image);

        match &image {
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
        }
    }
}
