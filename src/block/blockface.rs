use std::{simd::{Simd, SimdPartialOrd}, hint::{unreachable_unchecked, self}};
use super::normal::Normal;
use Normal::*;
use QuarterFaceType::*;


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

#[derive(Debug)]
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
    pub fn pair_culled(a: &BlockFace, b: &BlockFace) -> (bool, bool) {
        if a.dep != 0 || b.dep != 15 { return (a.tex == u16::MAX, b.tex == u16::MAX); }

        let a = a.as_u32();
        let b = b.as_u32();

        let diff = (a + 0x10101010 - b);

        // nibble subtraction:
        // a > b : 1x
        // a = b : 10
        // a < b : 0x
        // where 1 <= x <= 15

        // all of a's fields >= b's fields
        let cull_a = diff & 0x10101010 == 0x10101010;

        // subtract 0x01010101 so that:
        // a > b : 1x
        // a = b : 0f
        // a < b : 0x
        // where 0 <= x <= 14

        // all of a's fields <= b's fields
        let cull_b = ((diff - 0x01010101) & 0x10101010) == 0x00000000;
        
        return (cull_a, cull_b);
    }
    
    pub fn culled_by(&self, b: &BlockFace, normal: Normal) -> bool { unsafe {
        match normal {
            North | West | Down => {
                if self.dep != 0 || b.dep != 15 { return self.tex == u16::MAX; }
            },
            South | East | Up => {
                if self.dep != 15 || b.dep != 0 { return self.tex == u16::MAX; }
            }
            _ => { panic!(); }
        }

        let a = self.as_u32();
        let b = b.as_u32();

        return ((a + 0x10101010 - b) & 0x10101010 == 0x10101010);

    } }

    pub fn as_u64(&self) -> u64 { unsafe {
        return *(&raw const *self as *const u64);
    } }
    pub fn as_u32(&self) -> u32 { unsafe {
        return *(&raw const *self as *const u32);
    } }
    
    pub const fn full(normal: Normal, texture: u16) -> BlockFace { unsafe {
        let depth = match normal {
            North | West | Down => { 0 }
            South | East | Up => { 15 }
            _ => { 0 }
        };
        return BlockFace {
            lef: 0,
            bot: 0,
            rig: 0,
            top: 0,
            dep: depth,
            nor: normal,
            tex: texture,
        }
    } }

    pub const fn half(normal: Normal, half: HalfFaceType, texture: u16) -> BlockFace { unsafe {
        let depth = match normal {
            North | West | Down => { 0 },
            South | East | Up => { 15 },
            _ => { panic!(); }
        };
        match half {
            HalfFaceType::Left => {
                return BlockFace {
                    lef: 0,
                    bot: 0,
                    rig: 8,
                    top: 0,
                    dep: depth,
                    nor: normal,
                    tex: texture,
                };
            },
            HalfFaceType::Right => {
                return BlockFace {
                    lef: 8,
                    bot: 0,
                    rig: 0,
                    top: 0,
                    dep: depth,
                    nor: normal,
                    tex: texture,
                };

            },
            HalfFaceType::Top => {
                return BlockFace {
                    lef: 0,
                    bot: 8,
                    rig: 0,
                    top: 0,
                    dep: depth,
                    nor: normal,
                    tex: texture,
                };


            },
            HalfFaceType::Bottom => {
                return BlockFace {
                    lef: 0,
                    bot: 0,
                    rig: 0,
                    top: 8,
                    dep: depth,
                    nor: normal,
                    tex: texture,
                };
            },
        }
    } }
    
    pub const fn quarter(normal: Normal, quarter: QuarterFaceType, texture: u16) -> BlockFace { unsafe {
        let depth = match normal {
            North | West | Down => { 0 },
            South | East | Up => { 15 },
            _ => { panic!() }
        };
        match quarter {
            TopLeft => {
                return BlockFace {
                    lef: 0,
                    bot: 8,
                    rig: 8,
                    top: 0,
                    dep: depth,
                    nor: normal,
                    tex: texture,
                };
            },
            TopRight => {
                return BlockFace {
                    lef: 8,
                    bot: 8,
                    rig: 0,
                    top: 0,
                    dep: depth,
                    nor: normal,
                    tex: texture,
                };

            },
            BottomLeft => {
                return BlockFace {
                    lef: 0,
                    bot: 0,
                    rig: 8,
                    top: 8,
                    dep: depth,
                    nor: normal,
                    tex: texture,
                };


            },
            BottomRight => {
                return BlockFace {
                    lef: 8,
                    bot: 0,
                    rig: 0,
                    top: 8,
                    dep: depth,
                    nor: normal,
                    tex: texture,
                };
            },
        }
    } }

    pub const fn none(normal: Normal) -> BlockFace {
        let depth = match normal {
            North | West | Down => { 0 },
            South | East | Up => { 15 },
            _ => { panic!(); }
        };
        return BlockFace {
            lef: 0x0f, bot: 0x0f, dep: depth, nor: normal,
            rig: 0x0f, top: 0x0f, tex: u16::MAX
        };
    }

    pub const fn set_depth(self, depth: u8) -> BlockFace {
        return BlockFace { 
            dep: depth,
            ..self
        };
    }

    pub const fn set_texture(self, texture: u16) -> BlockFace {
        return BlockFace { 
            tex: texture,
            ..self
        };
    }
}

impl Clone for BlockFace {
    fn clone(&self) -> BlockFace {
        return BlockFace {
            lef: self.lef,
            bot: self.bot,
            rig: self.rig,
            top: self.top,
            dep: self.dep,
            nor: self.nor,
            tex: self.tex 
        };
    }
}

pub enum HalfFaceType {
    Left, Right, Top, Bottom
}

pub enum QuarterFaceType {
    TopLeft, TopRight, BottomLeft, BottomRight
}

impl PartialEq for BlockFace {
    fn eq(&self, other: &Self) -> bool {
        return self.top == other.top && self.lef == other.lef && self.bot == other.bot && self.dep == other.dep && self.nor == other.nor && self.rig == other.rig && self.top == other.top;
    }
}