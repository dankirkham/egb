mod constants;
mod gb_image;
mod pixel;
mod registers;

use egui::ColorImage;

use crate::memory::{Memory, UpperRam, VRam};
use crate::memory_map::MemoryMap;
use crate::registers::graphics::*;
use crate::registers::Interrupt;

use self::constants::*;
use self::gb_image::{Buffers, GbImage};
use self::pixel::Pixel;
use self::registers::Registers;

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
    background: Buffers,
    screen: Buffers,
    tiles: Buffers,
    objects: Buffers,
}

impl Default for Ppu {
    fn default() -> Self {
        Self {
            mode: PpuMode::default(),
            dot: 0,
            background: Buffers::background(),
            screen: Buffers::screen(),
            tiles: Buffers::tiles(),
            objects: Buffers::objects(),
        }
    }
}

pub fn draw_tile_for_tilemap(vram: &VRam, image: &mut GbImage, tile_idx: u32, row: u32, col: u32) {
    let tile_start = tile_idx * BYTES_PER_TILE;
    for tile_y in 0..TILE_SIZE {
        let line_start = (tile_start + (tile_y * BYTES_PER_TILE_LINE)) as u16;
        let ptr = (0x8000_u16).wrapping_add(line_start);
        let base: u16 = MemoryMap::VRam.into();
        let ptr = ptr - base;
        let low = vram[ptr as usize];
        let high = vram[ptr as usize + 1];
        let y = row * TILE_SIZE + tile_y;
        for tile_x in 0..TILE_SIZE {
            let dot: u8 = (((high >> tile_x) & 1) << 1) | ((low >> tile_x) & 1);
            let x = col * TILE_SIZE + (TILE_SIZE - tile_x - 1);
            image.put_pixel(x, y, dot.into());
        }
    }
}

pub fn draw_tile(vram: &VRam, image: &mut GbImage, lcdc4: bool, tile_idx: u32, row: u32, col: u32) {
    let tile_start = tile_idx * BYTES_PER_TILE;
    for tile_y in 0..TILE_SIZE {
        let line_start = (tile_start + (tile_y * BYTES_PER_TILE_LINE)) as u16;
        let ptr: u16 = if lcdc4 {
            let ptr = (0x8000_u16).wrapping_add(line_start);
            let base: u16 = MemoryMap::VRam.into();
            ptr - base
        } else {
            let ptr = if line_start >= 0x800 {
                0x8000_u16.wrapping_add(line_start)
            } else {
                0x9000_u16.wrapping_add_signed(line_start as i16)
            };
            let base: u16 = MemoryMap::VRam.into();
            ptr - base
        };
        let low = vram[ptr as usize];
        let high = vram[ptr as usize + 1];
        let y = row * TILE_SIZE + tile_y;
        for tile_x in 0..TILE_SIZE {
            let dot: u8 = (((high >> tile_x) & 1) << 1) | ((low >> tile_x) & 1);
            let x = col * TILE_SIZE + (TILE_SIZE - tile_x - 1);
            image.put_pixel(x, y, dot.into());
        }
    }
}

impl Ppu {
    fn draw_tiles(&mut self, vram: &VRam) {
        for row in 0..(8 * 3) {
            for col in 0..16 {
                let tile_idx = row * 16 + col;
                draw_tile_for_tilemap(vram, &mut self.tiles.draw, tile_idx, row, col);
            }
        }
    }

    fn draw_background(&mut self, vram: &VRam, regs: &Registers) {
        let lcdc4 = regs.lcdc.contains(LcdControl::BgWindowTileDataArea);
        let map_addr = if regs.lcdc.contains(LcdControl::BgTileMapArea) {
            0x9c00 - MemoryMap::VRam as usize
        } else {
            0x9800 - MemoryMap::VRam as usize
        };
        for row in 0..BACKGROUND_ROWS {
            for col in 0..BACKGROUND_COLS {
                let map_idx = row * BACKGROUND_COLS + col;
                let tile_idx = vram[map_addr + map_idx as usize] as u32;
                draw_tile(vram, &mut self.background.draw, lcdc4, tile_idx, row, col);
            }
        }
    }

    fn get_objects_40(&self, ur: &UpperRam) -> Vec<Object> {
        let mut objects = Vec::with_capacity(10);

        for i in 0..40 {
            let y = ur[i * BYTES_PER_OAM];

            let x = ur[i * BYTES_PER_OAM + 1];
            let tile = ur[i * BYTES_PER_OAM + 2];
            let attributes = ur[i * BYTES_PER_OAM + 3];
            let attributes = ObjectAttributes::from_bits_retain(attributes);

            let object = Object {
                y,
                x,
                tile,
                attributes,
            };
            objects.push(object);
        }

        objects
    }

    fn get_objects_10(&self, ur: &UpperRam, regs: &Registers) -> Vec<Object> {
        let mut objects = Vec::with_capacity(10);

        for i in 0..40 {
            let y = ur[i * BYTES_PER_OAM];
            if y == 0 {
                continue;
            }

            let height = regs.lcdc.object_height();
            if y - height <= regs.ly || y - 16 > regs.ly {
                continue;
            }

            let x = ur[i * BYTES_PER_OAM + 1];
            let tile = ur[i * BYTES_PER_OAM + 2];
            let attributes = ur[i * BYTES_PER_OAM + 3];
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

    fn draw_object(&mut self, vram: &VRam, regs: &Registers, object: Object) {
        let tile_start = object.tile as u32 * BYTES_PER_TILE;
        assert_eq!(regs.lcdc.object_height(), 8);
        let tile_y = if object.attributes.contains(ObjectAttributes::YFlip) {
            regs.lcdc.object_height() - (regs.ly + 16 - object.y) - 1
        } else {
            regs.ly + 16 - object.y
        };
        assert!((tile_y as u32) < TILE_SIZE);
        let ptr = (tile_start + (tile_y as u32 * BYTES_PER_TILE_LINE)) as usize;
        let low = vram[ptr];
        let high = vram[ptr + 1];
        let priority = !object.attributes.contains(ObjectAttributes::Priority);
        if !object.attributes.contains(ObjectAttributes::XFlip) {
            for tile_x in 0..TILE_SIZE {
                let dot = (((high >> tile_x) & 1) << 1) | ((low >> tile_x) & 1);
                let dot: Pixel = dot.into();
                if priority && dot == Pixel::Lighter {
                    continue;
                }
                let x = object.x as u32 + (TILE_SIZE - tile_x);
                self.screen.draw.put_pixel(x - 8, regs.ly as u32, dot);
            }
        } else {
            for tile_x in 0..TILE_SIZE {
                let dot = (((high >> tile_x) & 1) << 1) | ((low >> tile_x) & 1);
                let dot: Pixel = dot.into();
                if priority && dot == Pixel::Lighter {
                    continue;
                }
                let x = object.x as u32 + tile_x;
                self.screen.draw.put_pixel(x - 8, regs.ly as u32 - 16, dot);
            }
        }
    }

    fn draw_object_raw(&mut self, vram: &VRam, regs: &Registers, object: Object, x: u32, y: u32) {
        let tile_start = object.tile as u32 * BYTES_PER_TILE;
        // assert_eq!(regs.lcdc.object_height(), 8);
        for tile_y in 0..TILE_SIZE {
            let tile_y = if object.attributes.contains(ObjectAttributes::YFlip) {
                regs.lcdc.object_height() as u32 - tile_y - 1
            } else {
                tile_y
            };
            let ptr = (tile_start + (tile_y as u32 * BYTES_PER_TILE_LINE)) as usize;
            let low = vram[ptr];
            let high = vram[ptr + 1];
            if !object.attributes.contains(ObjectAttributes::XFlip) {
                for tile_x in 0..TILE_SIZE {
                    let dot = (((high >> tile_x) & 1) << 1) | ((low >> tile_x) & 1);
                    let dot: Pixel = dot.into();
                    let x_local = x + (TILE_SIZE - tile_x);
                    self.objects.draw.put_pixel(x_local, y + tile_y, dot);
                }
            } else {
                for tile_x in 0..TILE_SIZE {
                    let dot = (((high >> tile_x) & 1) << 1) | ((low >> tile_x) & 1);
                    let dot: Pixel = dot.into();
                    let x_local = x + tile_x;
                    self.objects.draw.put_pixel(x_local, y + tile_y, dot);
                }
            }
        }
    }

    fn draw_line(&mut self, vram: &VRam, ur: &UpperRam, regs: &Registers) {
        for x in 0..(LCD_WIDTH as u32) {
            let pixel = self.background.view.get_pixel_wrapping(
                x.wrapping_add(regs.scx as u32),
                (regs.ly as u32).wrapping_add(regs.scy as u32),
            );
            self.screen.draw.put_pixel(x, regs.ly as u32, pixel);
        }

        if regs.lcdc.contains(LcdControl::ObjEnable) {
            let objects = self.get_objects_10(ur, regs);
            for object in objects {
                self.draw_object(vram, regs, object);
            }
        }
    }

    fn draw_objects(&mut self, vram: &VRam, ur: &UpperRam, regs: &Registers) {
        let objects = self.get_objects_40(ur);
        let mut objects = objects.into_iter();
        for y in 0..4 {
            for x in 0..10 {
                self.draw_object_raw(vram, regs, objects.next().unwrap(), x * 8, y * 16);
            }
        }
    }

    pub fn get_background(&self) -> ColorImage {
        (&self.background.view).into()
    }

    pub fn get_screen(&self) -> ColorImage {
        (&self.screen.view).into()
    }

    pub fn get_tiles(&self) -> ColorImage {
        (&self.tiles.view).into()
    }

    pub fn get_objects(&self) -> ColorImage {
        (&self.objects.view).into()
    }

    pub fn tick(&mut self, mem: &mut Memory) {
        let mut regs = Registers::read(mem);

        for _ in 0..4 {
            self.t_cycle(mem, &mut regs);
        }

        regs.write(mem);
    }

    fn t_cycle(&mut self, mem: &mut Memory, regs: &mut Registers) {
        let vram = mem.get_vram();
        if !regs.lcdc.contains(LcdControl::LcdPpuEnable) {
            // PPU is disabled. Reset.
            self.dot = 0;
            self.mode = PpuMode::Mode2;

            regs.ly = 0;
            return;
        }

        self.dot += 1;

        match self.mode {
            PpuMode::Mode0 => {
                if self.dot > MODE_0_DOTS {
                    regs.ly += 1;
                    self.dot = 0;

                    if regs.ly >= LCD_HEIGHT {
                        self.mode = PpuMode::Mode1;
                        regs.if_reg.set(Interrupt::VBlank, true);
                        if regs.stat.contains(LcdStatus::Mode1Interrupt) {
                            regs.if_reg.set(Interrupt::LcdStat, true);
                        }
                    } else {
                        self.mode = PpuMode::Mode2;
                        if regs.stat.contains(LcdStatus::Mode2Interrupt) {
                            regs.if_reg.set(Interrupt::LcdStat, true);
                        }
                    }
                }
            }
            PpuMode::Mode1 => {
                if self.dot > MODE_1_DOTS_PER_LINE {
                    regs.ly += 1;
                    self.dot = 0;

                    if regs.ly > SCAN_LINES {
                        self.draw_tiles(vram);
                        self.draw_background(vram, &regs);
                        self.draw_objects(vram, mem.get_upper_ram(), &regs);
                        self.tiles.swap();
                        self.background.swap();
                        self.objects.swap();
                        self.screen.swap();
                        self.mode = PpuMode::Mode2;
                        regs.ly = 0;
                        if regs.stat.contains(LcdStatus::Mode2Interrupt) {
                            regs.if_reg.set(Interrupt::LcdStat, true);
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
                    self.draw_line(vram, mem.get_upper_ram(), &regs);
                    self.dot = 0;
                    self.mode = PpuMode::Mode0;
                    if regs.stat.contains(LcdStatus::Mode1Interrupt) {
                        regs.if_reg.set(Interrupt::LcdStat, true);
                    }
                }
            }
        }

        if regs.ly == regs.lyc && regs.stat.contains(LcdStatus::LycInterrupt) {
            regs.if_reg.set(Interrupt::LcdStat, true);
        }
        regs.stat.set(LcdStatus::LycEqLy, regs.ly == regs.lyc);
    }
}
