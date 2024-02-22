mod background;
mod screen;
mod tile_data;

use image::RgbImage;

use crate::gameboy::color;
use crate::memory::{CpuMemory, Memory};

pub use background::Background;
pub use screen::Screen;
pub use tile_data::TileData;

pub const TILE_SIZE: usize = 8;
pub const BYTES_PER_TILE: usize = 16;
pub const BYTES_PER_TILE_LINE: usize = 2;

pub fn generate_tile(
    mem: &Memory,
    image: &mut RgbImage,
    addr: u16,
    tile_idx: usize,
    row: usize,
    col: usize,
) {
    for tile_y in 0..TILE_SIZE {
        let tile_start = tile_idx * BYTES_PER_TILE;
        let line_start = tile_start + (tile_y * BYTES_PER_TILE_LINE);
        let low = mem.get_u8(addr + line_start as u16);
        let high = mem.get_u8(addr + (line_start + 1) as u16);
        let y = (row * TILE_SIZE + tile_y) as u32;
        for tile_x in 0..TILE_SIZE {
            let dot = (((high >> tile_x) & 1) << 1) | ((low >> tile_x) & 1);
            let c = color(dot);
            let x = (col * TILE_SIZE + (TILE_SIZE - tile_x - 1)) as u32;
            image.put_pixel(x, y, c);
        }
    }
}
