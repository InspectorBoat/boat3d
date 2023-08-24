use std::{simd::{Simd, SimdPartialOrd, SimdPartialEq}, hint::{unreachable_unchecked, self}};
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
    pub fn should_cull_pair(a: &BlockFace, b: &BlockFace) -> (bool, bool) {
        if a.dep != 0 || b.dep != 15 { return (a.tex == u16::MAX, b.tex == u16::MAX); }

        let a = Simd::from_array([a.lef, a.bot, a.rig, a.top]);
        let b = Simd::from_array([b.lef, b.bot, b.rig, b.top]);

        let cull_a = a.simd_ge(b).all();
        let cull_b = b.simd_ge(a).all();

        return (cull_a, cull_b);
    }
    
    pub fn should_cull_pair_(a: &BlockFace, b: &BlockFace) -> (bool, bool) {
        if a.dep != 0 || b.dep != 15 { return (a.tex == u16::MAX, b.tex == u16::MAX); }
        
        let a = a.as_u32();
        let b = b.as_u32();

        let diff = a + 0x10101010 - b;
        // nibble subtraction:
        // a > b: 1x
        // a = b: 10
        // a < b: 0x
        // where 1 < x < 15
        
        // if all of a's fields were >= b
        let cull_a = diff & 0x10101010 == 0x10101010;
        
        let diff = diff - 0x01010101;

        // nibble subtraction again:
        // a > b: 1x
        // a = b: 0f
        // a < b: 0x
        // where 0 < x < 14

        // if all of b's fields were >= a
        // if all of a's fields were <= b
        // if none of a's fields were > b
        let cull_b = diff & 0x10101010 == 0x00000000;

        return (cull_a, cull_b);
    }

    pub fn should_cull_row_pair(row_a: &[&BlockFace; 16], row_b: &[&BlockFace; 16]) -> ([bool; 16], [bool; 16]) {
        let depth_a = Simd::from_array(row_a.clone().map(|face| face.dep)).simd_eq(Simd::splat(0));
        let depth_b = Simd::from_array(row_b.clone().map(|face| face.dep)).simd_eq(Simd::splat(15));
        let tex_a = Simd::from_array(row_a.clone().map(|face| face.tex)).simd_eq(Simd::splat(u16::MAX));
        let tex_b = Simd::from_array(row_b.clone().map(|face| face.tex)).simd_eq(Simd::splat(u16::MAX));
        let depth = depth_a | depth_b;

        let row_a = Simd::from_array(row_a.clone().map(|face| face.as_u32()));
        let row_b = Simd::from_array(row_b.clone().map(|face| face.as_u32()));

        let diff_row = row_a + Simd::splat(0x10101010) - row_b;

        let cull_row_a = (diff_row & Simd::splat(0x10101010)).simd_eq(Simd::splat(0x10101010));
        let cull_row_a = depth.select_mask(cull_row_a.into(), tex_a.into());

        let diff_row = diff_row - Simd::splat(1);

        let cull_row_b = (diff_row & Simd::splat(0x10101010)).simd_eq(Simd::splat(0x00000000));
        let cull_row_b = depth.select_mask(cull_row_b.into(), tex_b.into());

        return (cull_row_a.to_array(), cull_row_b.to_array());
    }

    pub fn culled_by<const N: Normal>(&self, b: &BlockFace) -> bool { unsafe {
        match N {
            North | East | Down => {
                if self.dep != 0 || b.dep != 15 { return false; }
            },
            South | West | Up => {
                if self.dep != 15 || b.dep != 0 { return false; }
            }
            _ => { hint::unreachable_unchecked(); }
        }
        let a = Simd::from_array([self.lef, self.bot, self.rig, self.top]);
        let b = Simd::from_array([b.lef, b.bot, b.rig, b.top]);

        return a.simd_ge(b).all();
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
            _ => { unreachable_unchecked() }
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
            _ => { unreachable_unchecked() }
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
            North | West | Down => { 0 }
            South | East | Up => { 15 }
            _ => { 0 }
        };
        return BlockFace {
            lef: 0x0f, bot: 0x0f, dep: depth, nor: Unaligned,
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