use crate::block::normal::Normal;

#[repr(C, align(8))]
#[derive(Debug)]
pub struct GpuQuad {
    pub rel_x: u8,
    pub rel_y: u8,
    pub rel_z: u8,
    pub normal: Normal,
    pub width: u8,
    pub height: u8,
    pub texture: u16,
}