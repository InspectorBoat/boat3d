use std::ptr;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct BlockFace {
    pub lef: u8,
    pub bot: u8,
    pub rig: u8,
    pub top: u8,

    pub dep: u8,

    pub nor: Norm,
    
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
        lef: u8::MAX, bot: u8::MAX, dep: u8::MAX, nor: Norm::NONE, rig: u8::MAX, top: u8::MAX, tex: u16::MAX
    };
}

impl PartialEq for BlockFace {
    fn eq(&self, other: &Self) -> bool {
        return self.top == other.top && self.lef == other.lef && self.bot == other.bot && self.dep == other.dep && self.nor == other.nor && self.rig == other.rig && self.top == other.top;
    }
}

#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Norm(pub u8/* , pub isize */);

#[allow(dead_code)]
#[allow(unused_parens)]
impl Norm {
    fn reverse(&self) -> Norm {
        return Norm(6 - self.0);
    }
    pub const SOUTH: Norm = Norm(0);
    pub const NORTH: Norm = Norm(5);

    pub const WEST:  Norm = Norm(1);
    pub const EAST:  Norm = Norm(4);

    pub const DOWN:  Norm = Norm(2);
    pub const UP:    Norm = Norm(3);

    pub const NONE:  Norm = Norm(u8::MAX);
}

impl Into<u8> for Norm {
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