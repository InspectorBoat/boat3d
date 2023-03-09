use std::ptr;

#[derive(Clone, Debug)]
pub struct BlockFace {
    pub lef: u8,
    pub bot: u8,
    pub rig: u8,
    pub top: u8,

    pub dep: u8,

    pub nor: Normal,
    
    pub tex: u16,
}

impl BlockFace {
    pub fn compare(a: &BlockFace, b: &BlockFace) -> (bool, bool) {
        let (lef, bot) = (u8::max(a.lef, b.lef), u8::max(a.bot, b.bot));
        let (rig, top) = (u8::max(a.rig, b.rig), u8::max(a.top, b.top));
        return (
            (a.lef == lef && a.bot == bot && a.rig == rig && a.top == top),
            (b.lef == lef && b.bot == bot && b.rig == rig && b.top == top)
        )
    }

    pub fn not_culled_by(&self, other: &BlockFace) -> bool {
        return if other.is_none() { true } else {
            self.lef < other.lef ||
            self.bot < other.bot ||
            self.lef + self.rig > other.lef + other.rig ||
            self.bot + self.top > other.bot + other.top
        }
    }
    pub fn culled_by(&self, other: &BlockFace) -> bool {
        return if other.is_none() { false } else {
            self.lef >= other.lef &&
            self.bot >= other.bot &&
            self.lef + self.rig <= other.lef + other.rig &&
            self.bot + self.top <= other.bot + other.top
        }
    }
    pub fn as_u64(&self) -> u64 {
        unsafe {
            *(ptr::addr_of!(*self) as *const u64)
        }
    }

    pub fn is_none(&self) -> bool {
        return self.tex == u16::MAX
    }

    pub fn is_some(&self) -> bool {
        return self.tex != u16::MAX;
    }
    pub const NONE: BlockFace = BlockFace {
        lef: u8::MAX, bot: u8::MAX, dep: u8::MAX, nor: Normal::NONE, rig: u8::MAX, top: u8::MAX, tex: u16::MAX
    };
}

impl PartialEq for BlockFace {
    fn eq(&self, other: &Self) -> bool {
        return self.top == other.top && self.lef == other.lef && self.bot == other.bot && self.dep == other.dep && self.nor == other.nor && self.rig == other.rig && self.top == other.top;
    }
}

#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Normal(pub u8/* , pub isize */);

#[allow(dead_code)]
#[allow(unused_parens)]
impl Normal {
    fn reverse(&self) -> Normal {
        return Normal(6 - self.0);
    }
    pub const SOUTH: Normal = Normal(0);
    pub const NORTH: Normal = Normal(5);

    pub const WEST:  Normal = Normal(1);
    pub const EAST:  Normal = Normal(4);

    pub const DOWN:  Normal = Normal(2);
    pub const UP:    Normal = Normal(3);

    pub const NONE:  Normal = Normal(u8::MAX);
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

pub struct Face {
    pub left: u8,
    pub bottom: u8,
    pub right: u8,
    pub top: u8,
    pub depth: u8,
    pub normal: u8,
    pub texture: u16,
}

impl Face {
    fn as_u64(&self) -> u64 {
        unsafe {
            *(ptr::addr_of!(*self) as *const u64)
        }
    }
    fn as_u32(&self) -> u32 {
        unsafe {
            *(ptr::addr_of!(*self) as *const u32)
        }
    }
    fn as_u16(&self) -> u16 {
        unsafe {
            *(ptr::addr_of!(*self) as *const u16)
        }
    }
}