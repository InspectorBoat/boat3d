use crate::block::normal::Normal;

#[repr(C, align(8))]
#[derive(Debug)]
pub struct BufferQuad {
    pub block_x_left: u8,
    pub block_y_bottom: u8,
    pub block_z_right: u8,
    pub block_width_top: u8,
    pub block_height_depth: u8,
    pub normal: Normal,
    pub texture: u16
}

impl BufferQuad {
    pub fn block_x(&self) -> u8 {
        return self.block_x_left >> 4;
    }
    pub fn block_y(&self) -> u8 {
        return self.block_y_bottom >> 4;
    }
    pub fn block_z(&self) -> u8 {
        return self.block_z_right >> 4;
    }

    pub fn left(&self) -> u8 {
        return self.block_x_left & 0xf;
    }
    pub fn bottom(&self) -> u8 {
        return self.block_y_bottom & 0xf;
    }
    pub fn right(&self) -> u8 {
        return self.block_z_right & 0xf;
    }
    pub fn top(&self) -> u8 {
        return self.block_width_top & 0xf;
    }
    
    pub fn depth(&self) -> u8 {
        return self.block_height_depth & 0xf;
    }
    
    pub fn block_width(&self) -> u8 {
        return self.block_width_top >> 4;
    }
    pub fn block_height(&self) -> u8 {
        return self.block_width_top >> 4;
    }

    pub fn rel_x(&self) -> u8 {
        return self.block_x_left;
    }
    pub fn rel_y(&self) -> u8 {
        return self.block_y_bottom;
    }
    pub fn rel_z(&self) -> u8 {
        return (self.block_z_right & 0xf0) | (self.block_height_depth & 0x0f);
    }
    pub fn normal(&self) -> Normal {
        return self.normal;
    }
    pub fn width(&self) -> u8 {
        return (self.block_width_top & 0xf0) | (0xf - (self.block_z_right & 0x0f) - (self.block_x_left & 0x0f));
    }
    pub fn height(&self) -> u8 {
        return (self.block_height_depth & 0xf0) | (0xf - (self.block_width_top & 0x0f) - (self.block_y_bottom & 0x0f));
    }
    pub fn texture(&self) -> u16 {
        return self.texture;
    }
}

