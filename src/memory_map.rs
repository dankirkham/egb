#[repr(u16)]
pub enum MemoryMap {
    InterruptVBlank = 0x0040,
    InterruptLcdStat = 0x0048,
    InterruptTimer = 0x0050,
    InterruptSerial = 0x0058,
    InterruptJoypad = 0x0060,
    HeaderCartridgeType = 0x0147,
    HeaderRomSize = 0x0148,
    HeaderRamSize = 0x0149,
    VRam = 0x8000,
    Joypad = 0xff00,
    SB = 0xff01,
    SC = 0xff02,
    DIV = 0xff04,
    TIMA = 0xff05,
    TMA = 0xff06,
    TAC = 0xff07,
    IF = 0xff0f,
    LCDC = 0xff40,
    STAT = 0xff41,
    SCY = 0xff42,
    SCX = 0xff43,
    LY = 0xff44,
    LYC = 0xff45,
    BootRomDisable = 0xff50,
    IE = 0xffff,
}

impl From<MemoryMap> for u16 {
    fn from(val: MemoryMap) -> Self {
        val as u16
    }
}
