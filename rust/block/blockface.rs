use std::ptr;

#[derive(Clone, Debug)]
// #[repr(C)]
pub struct BlockFace {
    pub x: u8,
    pub y: u8,
    pub z: u8,
    
    pub n: Normal,
    
    pub w: u8,
    pub h: u8,
    
    pub t: u16,
}

impl BlockFace {
    pub fn not_culled_by_zyx(&self, other: &BlockFace) -> bool {
        return {
            self.x < other.x ||
            self.y < other.y ||
            self.x + self.w > other.x + other.w ||
            self.y + self.h > other.y + other.h
        }
    }
    pub fn culled_by_zyx(&self, other: &BlockFace) -> bool {
        return {
            self.x >= other.x &&
            self.y >= other.y &&
            self.x + self.w <= other.x + other.w &&
            self.y + self.h <= other.y + other.h
        }
    }
    
    pub fn not_culled_by_xyz(&self, other: &BlockFace) -> bool {
        return {
            self.z < other.z ||
            self.y < other.y ||
            self.z + self.w > other.z + other.w ||
            self.y + self.h > other.y + other.h
        }
    }
    pub fn culled_by_xyz(&self, other: &BlockFace) -> bool {
        return {
            self.z >= other.z &&
            self.y >= other.y &&
            self.z + self.w <= other.z + other.w &&
            self.y + self.h <= other.y + other.h
        }
    }

    pub fn match_right_zyx(&self, other: &BlockFace) -> bool {
        return {
            self.t == other.t &&
            self.x + self.w == 15 &&
            other.x == 0 &&
            self.y == other.y &&
            self.h == other.h
        }
    }
    pub fn match_right_xyz(&self, other: &BlockFace) -> bool {
        return {
            self.t == other.t &&
            self.z + self.w == 15 &&
            other.z == 0 &&
            self.y == other.y &&
            self.h == other.h
        }
    }
    pub fn match_right_yxz(&self, other: &BlockFace) -> bool {
        return {
            self.t == other.t &&
            self.z + self.h == 15 &&
            other.z == 0 &&
            self.x == other.x &&
            self.w == other.w
        }
    }
    
    pub fn as_u64(&self) -> u64 {
        unsafe {
            *(ptr::addr_of!(*self) as *const u64)
        }
    }

    pub fn is_none(&self) -> bool {
        return self.t == u16::MAX;
    }
    pub const NONE: BlockFace = BlockFace {
        x: 0, y: 0, z: 0, n: Normal::NONE, w: 0, h: 0, t: u16::MAX
    };
}

impl PartialEq for BlockFace {
    fn eq(&self, other: &Self) -> bool {
        return self.t == other.t && self.x == other.x && self.y == other.y && self.z == other.z && self.n == other.n && self.w == other.w && self.h == other.h;
    }
}

#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Normal(pub u8);

#[allow(dead_code)]
impl Normal {
    fn reverse(&self) -> Normal {
        return Normal(6 - self.0);
    }
    pub const SOUTH: Normal = Normal(0);
    pub const NORTH: Normal = Normal(5);

    pub const WEST: Normal = Normal(1);
    pub const EAST: Normal = Normal(4);

    pub const DOWN: Normal = Normal(2);
    pub const UP: Normal = Normal(3);

    const NONE: Normal = Normal(255);
}

impl Into<u8> for Normal {
    fn into(self) -> u8 {
        return self.0;
    }
}