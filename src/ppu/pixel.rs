use image::Rgb;

#[derive(Copy, Clone, Default, PartialEq)]
pub enum Pixel {
    #[default]
    Darker,
    Dark,
    Light,
    Lighter,
}

impl From<u8> for Pixel {
    fn from(val: u8) -> Self {
        match val {
            0x03 => Self::Darker,
            0x02 => Self::Dark,
            0x01 => Self::Light,
            0x00 => Self::Lighter,
            _ => panic!("invalid pixel"),
        }
    }
}

impl From<Pixel> for Rgb<u8> {
    fn from(val: Pixel) -> Self {
        match val {
            Pixel::Darker => Rgb([15, 56, 15]),
            Pixel::Dark => Rgb([48, 98, 48]),
            Pixel::Light => Rgb([139, 172, 15]),
            Pixel::Lighter => Rgb([155, 188, 15]),
        }
    }
}
