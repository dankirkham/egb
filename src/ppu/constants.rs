pub const TILE_SIZE: u32 = 8;
pub const BYTES_PER_TILE: u32 = 16;
pub const BYTES_PER_TILE_LINE: u32 = 2;
pub const BACKGROUND_COLS: u32 = 32;
pub const BACKGROUND_ROWS: u32 = 32;

pub const LCD_WIDTH: u8 = 160;
pub const LCD_HEIGHT: u8 = 144;
pub const SCAN_LINES: u8 = LCD_HEIGHT + 10;
pub const MODE_1_DOTS_PER_LINE: u32 = 456;
pub const MODE_2_DOTS: u32 = 80;
pub const MODE_30_DOTS: u32 = 376;
pub const MODE_3_DOTS: u32 = 220;
pub const MODE_0_DOTS: u32 = MODE_30_DOTS - MODE_3_DOTS;

pub const BYTES_PER_OAM: u16 = 4;
pub const MAX_OBJECTS_PER_LINE: usize = 10;
