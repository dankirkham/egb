mod mbc1;

use bytes::BytesMut;
pub use mbc1::Mbc1;

use crate::memory::{UpperRam, VRam};

pub trait Mbc
where
    for<'a> &'a Self: Into<BytesMut>,
{
    fn set_u8(&mut self, address: impl Into<u16>, value: u8);
    fn get_u8(&self, address: impl Into<u16>) -> u8;
    fn get_vram(&self) -> &VRam;
    fn get_upper_ram(&self) -> &UpperRam;
    fn get_upper_ram_mut(&mut self) -> &mut UpperRam;
}
