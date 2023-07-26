use crate::block::blockface::Normal;

#[repr(C, align(8))]
#[derive(Debug)]
pub struct BufferQuad {
    pub block_rel_x_left: u8,
    pub block_rel_y_bottom: u8,
    pub block_rel_z_right: u8,
    pub block_width_top: u8,
    pub block_height_depth: u8,
    pub normal: Normal,
    pub texture: u16
}

impl BufferQuad {
    pub fn get_rel_x(&self) -> u8 {
        return self.block_rel_x_left;
    }
    pub fn get_rel_y(&self) -> u8 {
        return self.block_rel_y_bottom;
    }
    pub fn get_rel_z(&self) -> u8 {
        return (self.block_rel_z_right & 0xf0) | (self.block_height_depth & 0x0f);
    }
    pub fn get_normal(&self) -> Normal {
        return self.normal;
    }
    pub fn get_width(&self) -> u8 {
        return (self.block_width_top & 0xf0) | (0xf - (self.block_rel_z_right & 0x0f) - (self.block_rel_x_left & 0x0f));
    }
    pub fn get_height(&self) -> u8 {
        return (self.block_height_depth & 0xf0) | (0xf - (self.block_width_top & 0x0f) - (self.block_rel_y_bottom & 0x0f));
    }
    pub fn get_texture(&self) -> u16 {
        return self.texture;
    }
}

