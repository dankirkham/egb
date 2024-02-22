use bitflags::bitflags;

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct LcdControl: u8 {
        const LcdPpuEnable = 1 << 7;
        const WindowTileMapArea = 1 << 6;
        const WindowEnable = 1 << 5;
        const BgWindowTileDataArea = 1 << 4;
        const BgTileMapArea = 1 << 3;
        const ObjSize = 1 << 2;
        const ObjEnable = 1 << 1;
        const BgWindowEnablePriority = 1;
    }
}

impl LcdControl {
    pub fn object_height(&self) -> u8 {
        if self.contains(LcdControl::ObjSize) {
            16
        } else {
            8
        }
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct LcdStatus: u8 {
        const LycInterrupt = 1 << 6;
        const Mode2Interrupt = 1 << 5;
        const Mode1Interrupt = 1 << 4;
        const Mode0Interrupt = 1 << 3;
        const LycEqLy = 1 << 2;
        const PpuMode1 = 1 << 1;
        const PpuMode0 = 1;
    }
}

bitflags! {
    #[derive(Clone, Copy, Debug)]
    pub struct ObjectAttributes: u8 {
        const Priority = 1 << 7;
        const YFlip = 1 << 6;
        const XFlip = 1 << 5;
        const DmgPalette = 1 << 4;

        const _ = !0;
    }
}
