mod constants;
mod gb_image;
mod pixel;

use egui::ColorImage;

use crate::memory::PpuMemory;
use crate::memory_map::MemoryMap;
use crate::registers::graphics::*;
use crate::registers::Interrupt;

use self::constants::*;
use self::gb_image::GbImage;
use self::pixel::Pixel;

struct Object {
    y: u8,
    x: u8,
    tile: u8,
    attributes: ObjectAttributes,
}

#[derive(Default, PartialEq)]
enum PpuMode {
    Mode0,
    Mode1,
    #[default]
    Mode2,
    Mode3,
}

pub struct Ppu {
    mode: PpuMode,
    dot: u32,
    background: GbImage,
    screen: GbImage,
    other_background: GbImage,
    other_screen: GbImage,
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            mode: PpuMode::default(),
            dot: 0,
            background: GbImage::background(),
            screen: GbImage::screen(),
            other_background: GbImage::background(),
            other_screen: GbImage::screen(),
        }
    }
}

pub fn draw_tile(
    mem: &impl PpuMemory,
    image: &mut GbImage,
    lcdc4: bool,
    tile_idx: u32,
    row: u32,
    col: u32,
) {
    let tile_start = tile_idx * BYTES_PER_TILE;
    for tile_y in 0..TILE_SIZE {
        let line_start = (tile_start + (tile_y * BYTES_PER_TILE_LINE)) as u16;
        let (low, high) = if lcdc4 {
            let ptr = (0x8000_u16).wrapping_add(line_start);
            let low = mem.get_u8(ptr);
            let high = mem.get_u8(ptr + 1);
            (low, high)
        } else {
            let ptr = if line_start >= 0x800 {
                0x8000_u16.wrapping_add(line_start)
            } else {
                0x9000_u16.wrapping_add_signed(line_start as i16)
            };
            let low = mem.get_u8(ptr);
            let high = mem.get_u8(ptr + 1);
            (low, high)
        };
        let y = row * TILE_SIZE + tile_y;
        for tile_x in 0..TILE_SIZE {
            let dot = (((high >> tile_x) & 1) << 1) | ((low >> tile_x) & 1);
            let x = col * TILE_SIZE + (TILE_SIZE - tile_x - 1);
            image.put_pixel(x, y, dot.into());
        }
    }
}

impl Ppu {
    fn draw_background(&mut self, mem: &mut impl PpuMemory) {
        let lcdc = mem.get_reg::<LcdControl>(MemoryMap::LCDC);
        let lcdc4 = lcdc.contains(LcdControl::BgWindowTileDataArea);
        let map_addr = if lcdc.contains(LcdControl::BgTileMapArea) {
            0x9c00
        } else {
            0x9800
        };
        for row in 0..BACKGROUND_ROWS {
            for col in 0..BACKGROUND_COLS {
                let map_idx = row * BACKGROUND_COLS + col;
                let tile_idx = mem.get_u8(map_addr + map_idx as u16) as u32;
                draw_tile(mem, &mut self.background, lcdc4, tile_idx, row, col);
            }
        }
    }

    fn get_objects(&self, mem: &mut impl PpuMemory, ly: u8) -> Vec<Object> {
        let lcdc = mem.get_reg::<LcdControl>(MemoryMap::LCDC);
        let mut objects = Vec::with_capacity(10);

        let base_address = 0xfe00;
        for i in 0..40 {
            let y = mem.get_u8(base_address + i * BYTES_PER_OAM);
            if y == 0 {
                continue;
            }

            let height = lcdc.object_height();
            if y - height <= ly || y - 16 > ly {
                continue;
            }

            let x = mem.get_u8(base_address + i * BYTES_PER_OAM + 1);
            let tile = mem.get_u8(base_address + i * BYTES_PER_OAM + 2);
            let attributes = mem.get_u8(base_address + i * BYTES_PER_OAM + 3);
            let attributes = ObjectAttributes::from_bits_retain(attributes);

            let object = Object {
                y,
                x,
                tile,
                attributes,
            };
            objects.push(object);

            if objects.len() >= MAX_OBJECTS_PER_LINE {
                break;
            }
        }

        objects.sort_by(|a, b| b.x.cmp(&a.x));

        objects
    }

    fn draw_object(&mut self, mem: &mut impl PpuMemory, ly: u8, object: Object) {
        let tile_start = object.tile as u32 * BYTES_PER_TILE;
        let lcdc = mem.get_reg::<LcdControl>(MemoryMap::LCDC);
        let height = lcdc.object_height();
        assert_eq!(height, 8);
        let tile_y = if object.attributes.contains(ObjectAttributes::YFlip) {
            height - (ly + 16 - object.y) - 1
        } else {
            ly + 16 - object.y
        };
        assert!((tile_y as u32) < TILE_SIZE);
        let line_start = (tile_start + (tile_y as u32 * BYTES_PER_TILE_LINE)) as u16;
        let ptr = 0x8000 + line_start;
        let low = mem.get_u8(ptr);
        let high = mem.get_u8(ptr + 1);
        let priority = !object.attributes.contains(ObjectAttributes::Priority);
        if !object.attributes.contains(ObjectAttributes::XFlip) {
            for tile_x in 0..TILE_SIZE {
                let dot = (((high >> tile_x) & 1) << 1) | ((low >> tile_x) & 1);
                let dot: Pixel = dot.into();
                if priority && dot == Pixel::Lighter {
                    continue;
                }
                let x = object.x as u32 + (TILE_SIZE - tile_x);
                self.screen.put_pixel(x - 8, ly as u32, dot);
            }
        } else {
            for tile_x in 0..TILE_SIZE {
                let dot = (((high >> tile_x) & 1) << 1) | ((low >> tile_x) & 1);
                let dot: Pixel = dot.into();
                if priority && dot == Pixel::Lighter {
                    continue;
                }
                let x = object.x as u32 + tile_x;
                self.screen.put_pixel(x - 8, ly as u32 - 16, dot);
            }
        }
    }

    fn draw_line(&mut self, mem: &mut impl PpuMemory) {
        let scy = mem.get_u8(MemoryMap::SCY) as u32;
        let scx = mem.get_u8(MemoryMap::SCX) as u32;
        let y = mem.get_u8(MemoryMap::LY) as u32;

        for x in 0..(LCD_WIDTH as u32) {
            let pixel = self
                .background
                .get_pixel_wrapping(x.wrapping_add(scx), y.wrapping_add(scy));
            self.screen.put_pixel(x, y, pixel);
        }

        let lcdc = mem.get_reg::<LcdControl>(MemoryMap::LCDC);
        if lcdc.contains(LcdControl::ObjEnable) {
            let objects = self.get_objects(mem, y as u8);
            for object in objects {
                self.draw_object(mem, y as u8, object);
            }
        }
    }

    pub fn get_background(&self) -> ColorImage {
        (&self.other_background).into()
    }

    pub fn get_screen(&self) -> ColorImage {
        (&self.other_screen).into()
    }

    pub fn tick(&mut self, mem: &mut impl PpuMemory) {
        let lcdc = mem.get_reg::<LcdControl>(MemoryMap::LCDC);

        if !lcdc.contains(LcdControl::LcdPpuEnable) {
            // PPU is disabled. Reset.
            self.dot = 0;
            self.mode = PpuMode::Mode2;
            mem.set_u8(MemoryMap::LY, 0);
            return;
        }

        self.dot += 1;

        let mut stat = mem.get_reg::<LcdStatus>(MemoryMap::STAT);

        match self.mode {
            PpuMode::Mode0 => {
                if self.dot > MODE_0_DOTS {
                    mem.inc_u8(MemoryMap::LY);
                    self.dot = 0;

                    if mem.get_u8(MemoryMap::LY) >= LCD_HEIGHT {
                        self.mode = PpuMode::Mode1;
                        mem.set_reg_flag(MemoryMap::IF, Interrupt::VBlank);
                        if stat.contains(LcdStatus::Mode1Interrupt) {
                            mem.set_reg_flag(MemoryMap::IF, Interrupt::LcdStat);
                        }
                    } else {
                        self.mode = PpuMode::Mode2;
                        if stat.contains(LcdStatus::Mode2Interrupt) {
                            mem.set_reg_flag(MemoryMap::IF, Interrupt::LcdStat);
                        }
                    }
                }
            }
            PpuMode::Mode1 => {
                if self.dot > MODE_1_DOTS_PER_LINE {
                    mem.inc_u8(MemoryMap::LY);
                    self.dot = 0;

                    if mem.get_u8(MemoryMap::LY) > SCAN_LINES {
                        self.draw_background(mem);
                        std::mem::swap(&mut self.background, &mut self.other_background);
                        std::mem::swap(&mut self.screen, &mut self.other_screen);
                        self.mode = PpuMode::Mode2;
                        mem.set_u8(MemoryMap::LY, 0);
                        if stat.contains(LcdStatus::Mode2Interrupt) {
                            mem.set_reg_flag(MemoryMap::IF, Interrupt::LcdStat);
                        }
                    }
                }
            }
            PpuMode::Mode2 => {
                if self.dot > MODE_2_DOTS {
                    self.dot = 0;
                    self.mode = PpuMode::Mode3;
                }
            }
            PpuMode::Mode3 => {
                if self.dot > MODE_3_DOTS {
                    self.draw_line(mem);
                    self.dot = 0;
                    self.mode = PpuMode::Mode0;
                    if stat.contains(LcdStatus::Mode1Interrupt) {
                        mem.set_reg_flag(MemoryMap::IF, Interrupt::LcdStat);
                    }
                }
            }
        }

        let lyc_eq_ly = mem.get_u8(MemoryMap::LY) == mem.get_u8(MemoryMap::LYC);
        if lyc_eq_ly && stat.contains(LcdStatus::LycInterrupt) {
            mem.set_reg_flag(MemoryMap::IF, Interrupt::LcdStat);
        }
        stat.set(LcdStatus::LycEqLy, lyc_eq_ly);
        // TODO: Set PPU mode
        mem.set_reg(MemoryMap::STAT, stat);
    }
}
