use crate::block::normal::Normal;

#[repr(C, align(8))]
#[derive(Debug)]
pub struct GpuQuad {
    pub x: u8,
    pub y: u8,
    pub z: u8,
    pub normal: Normal,
    pub width: u8,
    pub height: u8,
    pub texture: u16,
}