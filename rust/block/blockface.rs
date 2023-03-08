use std::ptr;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct BlockFace {
    pub u: u8,
    pub v: u8,
    pub d: u8,
    pub n: Normal,

    pub w: u8,
    pub h: u8,
    
    pub t: u16,
}

impl BlockFace {
    pub fn compare(first: &BlockFace, second: &BlockFace) -> (bool, bool) {
        if first.is_none() {
            return (true, second.is_none())
        } else if second.is_none() {
            return (false, true)
        }
        let x_overlap = (first.u + first.w).min(second.u + second.w) - first.u.min(second.u);
        let y_overlap = (first.v + first.h).min(second.v + second.h) - first.v.min(second.v);
        return (
            (x_overlap == first.w - first.u && y_overlap == first.h - first.v),
            (x_overlap == second.w - second.u && y_overlap == second.h - second.v)
        )
    }
    pub fn not_culled_by(&self, other: &BlockFace) -> bool {
        return if other.is_none() { true } else {
            self.u < other.u ||
            self.v < other.v ||
            self.u + self.w > other.u + other.w ||
            self.v + self.h > other.v + other.h
        }
    }
    pub fn culled_by(&self, other: &BlockFace) -> bool {
        return if other.is_none() { false } else {
            self.u >= other.u &&
            self.v >= other.v &&
            self.u + self.w <= other.u + other.w &&
            self.v + self.h <= other.v + other.h
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

    pub fn is_some(&self) -> bool {
        return self.t != u16::MAX;
    }
    pub const NONE: BlockFace = BlockFace {
        u: 1, v: 2, d: 3, n: Normal::NONE, w: 5, h: 6, t: u16::MAX
    };
}

impl PartialEq for BlockFace {
    fn eq(&self, other: &Self) -> bool {
        return self.t == other.t && self.u == other.u && self.v == other.v && self.d == other.d && self.n == other.n && self.w == other.w && self.h == other.h;
    }
}

#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Normal(pub u8/* , pub isize */);

#[allow(dead_code)]
#[allow(unused_parens)]
impl Normal {
    fn reverse(&self) -> Normal {
        return Normal(6 - self.0/* , -self.1 */);
    }
    pub const SOUTH: Normal = Normal(0/* , -0x001 */);
    pub const NORTH: Normal = Normal(5/* ,  0x001 */);

    pub const WEST:  Normal = Normal(1/* , -0x100 */);
    pub const EAST:  Normal = Normal(4/* ,  0x100 */);

    pub const DOWN:  Normal = Normal(2/* , -0x010 */);
    pub const UP:    Normal = Normal(3/* ,  0x010 */);

    const NONE:      Normal = Normal(255/* ,    0 */);
}

impl Into<u8> for Normal {
    fn into(self) -> u8 {
        return self.0;
    }
}

#[derive(PartialEq, Eq)]
pub enum N {
    SOUTH,
    NORTH,
    WEST,
    EAST,
    DOWN,
    UP,
}

impl N {
    pub const fn index(&self) -> i32 {
        match self {
            N::SOUTH => 0,
            N::NORTH => 5,
            N::WEST => 1,
            N::EAST => 4,
            N::DOWN => 2,
            N::UP => 3,
        }
    }

    pub const fn opposite(&self) -> N {
        match self {
            N::SOUTH => N::NORTH,
            N::NORTH => N::SOUTH,
            N::WEST => N::EAST,
            N::EAST => N::WEST,
            N::DOWN => N::UP,
            N::UP => N::DOWN,
        }
    }

    pub const fn offset(&self) -> isize {
        match self {
            N::SOUTH => -0x001,
            N::NORTH =>  0x001,
            N::WEST  => -0x100,
            N::EAST  =>  0x100,
            N::DOWN  => -0x010,
            N::UP    =>  0x010,
        }
    }

    pub const fn offset_mask(&self) -> (usize, usize) {
        match self {
            N::SOUTH =>  (0x00f, 0),
            N::NORTH =>  (0x00f, 15),
            N::WEST  =>  (0xf00, 0),
            N::EAST  =>  (0xf00, 15),
            N::DOWN  =>  (0x0f0, 0),
            N::UP    =>  (0x0f0, 15),
        }
    }
}