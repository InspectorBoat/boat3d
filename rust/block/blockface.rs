use std::{mem, hint::unreachable_unchecked, marker::ConstParamTy, simd::{Simd, SimdPartialOrd, SimdUint}};


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
#[repr(C, align(8))]
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
    pub fn should_cull(a: &BlockFace, b: &BlockFace) -> (bool, bool) {
        if a.dep != 0 || b.dep != 15 { return (a.tex == u16::MAX, b.tex == u16::MAX) }

        let a = Simd::from_array([a.lef, a.bot, a.rig, a.top]);
        let b = Simd::from_array([b.lef, b.bot, b.rig, b.top]);

        let cull_a = a.simd_ge(b).all();
        let cull_b = b.simd_ge(a).all();

        return (cull_a, cull_b);
    }
    
    pub fn as_u64(&self) -> u64 { unsafe {
        return *(&raw const *self as *const u64);
    } }
    pub fn as_u32(&self) -> u32 { unsafe {
        return *(&raw const *self as *const u32);
    } }
    pub const NONE: BlockFace = BlockFace {
        lef: 0x0f, bot: 0x0f, dep: 0, nor: Normal::NONE,
        rig: 0x0f, top: 0x0f, tex: u16::MAX
    };
}

#[repr(C, align(8))]
#[derive(Debug)]
pub struct GpuQuad {
    pub ure: u8,
    pub ven: u8,
    pub dep: u8,
    pub nor: Normal,
    pub wid: u8,
    pub hei: u8,
    pub tex: u16,
}


impl PartialEq for BlockFace {
    fn eq(&self, other: &Self) -> bool {
        return self.top == other.top && self.lef == other.lef && self.bot == other.bot && self.dep == other.dep && self.nor == other.nor && self.rig == other.rig && self.top == other.top;
    }
}

#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq, ConstParamTy)]
#[repr(u8)]
pub enum Normal {
    SOUTH = 0,
    WEST = 1,
    DOWN = 2,
    NORTH = 3,
    EAST = 4,
    UP = 5,
    NONE = u8::MAX
}

impl Normal {
    pub fn reverse(&self) -> Normal {
        match self {
            Normal::SOUTH => { return Normal::NORTH; }
            Normal::WEST => { return Normal::EAST; }
            Normal::DOWN => { return Normal::UP; }
            Normal::NORTH => { return Normal::SOUTH; }
            Normal::EAST => { return Normal::WEST; }
            Normal::UP => { return Normal::DOWN; }
            _ => unsafe { unreachable_unchecked(); }
        }
    }
}