mod mbc1;

use bytes::BytesMut;
pub use mbc1::Mbc1;

pub trait Mbc {
    fn set_u8(&mut self, address: impl Into<u16>, value: u8);
    fn get_u8(&self, address: impl Into<u16>) -> u8;

    fn dump(&self) -> BytesMut;
}
