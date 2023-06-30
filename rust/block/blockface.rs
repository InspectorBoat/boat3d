use std::{mem, hint::unreachable_unchecked};


/*
 * |---------------|
 * |         top   |
 * |    |-----|rig |
 * |    |     |    |
 * | lef|-----|    |
 * |   bot         |
 * |---------------|
 * 
 */

#[derive(Clone, Debug)]
#[repr(C)]
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
    pub fn compare_is_culled(a: &BlockFace, b: &BlockFace) -> (bool, bool) {
        if a.dep != 0 || b.dep != 15 { return (a.tex == u16::MAX, b.tex == u16::MAX) }
        // if a.as_u32() == b.as_u32() { return (true, true) }
        let diff = a.as_u32() + 0x10101010 - b.as_u32();
        return (diff >= 0x10101010, diff <= 0x10101010)
    }
    
    pub fn as_u64(&self) -> u64 {
        unsafe {
            *(&raw const *self as *const u64)
        }
    }
    pub fn as_u32(&self) -> u32 {
        unsafe {
            *(&raw const *self as *const u32)
        }
    }
    pub fn as_i32(&self) -> i32 {
        unsafe {
            *(&raw const *self as *const i32)
        }
    }
    pub fn copy(&self) -> BlockFace {
        return unsafe { mem::transmute_copy(self) };
    }
    pub const NONE: BlockFace = BlockFace {
        lef: 0x0f, bot: 0x0f, dep: 0, nor: Normal::NONE,
        rig: 0x0f, top: 0x0f, tex: u16::MAX
    };
}

impl PartialEq for BlockFace {
    fn eq(&self, other: &Self) -> bool {
        return self.top == other.top && self.lef == other.lef && self.bot == other.bot && self.dep == other.dep && self.nor == other.nor && self.rig == other.rig && self.top == other.top;
    }
}

#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Normal(pub u8);

#[allow(dead_code)]
#[allow(unused_parens)]
impl Normal {
    pub fn reverse(&self) -> Normal {
        match *self {
            Normal::SOUTH => { return Normal::NORTH; }
            Normal::WEST => { return Normal::EAST; }
            Normal::DOWN => { return Normal::UP; }
            Normal::NORTH => { return Normal::SOUTH; }
            Normal::EAST => { return Normal::WEST; }
            Normal::UP => { return Normal::DOWN; }
            _ => unsafe { unreachable_unchecked(); }
        }
    }
    pub const SOUTH: Normal = Normal(0);
    pub const WEST:  Normal = Normal(1);
    pub const DOWN:  Normal = Normal(2);
    pub const NORTH: Normal = Normal(3);
    pub const EAST:  Normal = Normal(4);
    pub const UP:    Normal = Normal(5);

    pub const NONE:  Normal = Normal(u8::MAX);
}

impl Into<u8> for Normal {
    fn into(self) -> u8 {
        return self.0;
    }
}
impl Into<usize> for Normal {
    fn into(self) -> usize {
        return self.0 as usize;
    }
}
// impl<T: Into<u8>> From<T> for Normal {
    // fn from(val: T) -> Normal {
        // return Normal(val.into());
    // }
// }