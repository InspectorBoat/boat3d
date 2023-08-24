use std::{simd::{Simd, SimdPartialOrd, SimdPartialEq}, hint::{unreachable_unchecked, self}};
use crate::world::merged_quad::RelPos;

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
    pub left: u8,
    pub bottom: u8,
    pub right: u8,
    pub top: u8,
    pub depth: u8,
    pub normal: Normal,
    pub texture: u16,
}

impl BlockFace {
    pub fn should_cull_pair_simd(a: &BlockFace, b: &BlockFace) -> (bool, bool) {
        if a.depth != 0 || b.depth != 15 { return (a.texture == u16::MAX, b.texture == u16::MAX); }

        let a = Simd::from_array([a.left, a.bottom, a.right, a.top]);
        let b = Simd::from_array([b.left, b.bottom, b.right, b.top]);

        let cull_a = a.simd_ge(b).all();
        let cull_b = b.simd_ge(a).all();

        return (cull_a, cull_b);
    }
    
    pub fn should_cull_pair(a: &BlockFace, b: &BlockFace) -> (bool, bool) {
        if a.depth != 0 || b.depth != 15 { return (a.texture == u16::MAX, b.texture == u16::MAX); }
        
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
        let depth_a = Simd::from_array(row_a.clone().map(|face| face.depth)).simd_eq(Simd::splat(0));
        let depth_b = Simd::from_array(row_b.clone().map(|face| face.depth)).simd_eq(Simd::splat(15));
        let tex_a = Simd::from_array(row_a.clone().map(|face| face.texture)).simd_eq(Simd::splat(u16::MAX));
        let tex_b = Simd::from_array(row_b.clone().map(|face| face.texture)).simd_eq(Simd::splat(u16::MAX));
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

    pub fn culled_by(&self, b: &BlockFace, normal: Normal) -> bool { unsafe {
        match normal {
            North | East | Down => {
                if self.depth != 0 || b.depth != 15 { return false; }
            },
            South | West | Up => {
                if self.depth != 15 || b.depth != 0 { return false; }
            }
            _ => { hint::unreachable_unchecked(); }
        }
        let a = Simd::from_array([self.left, self.bottom, self.right, self.top]);
        let b = Simd::from_array([b.left, b.bottom, b.right, b.top]);

        return a.simd_ge(b).all();
    } }

    pub fn as_u64(&self) -> u64 { unsafe {
        return *(&raw const *self as *const u64);
    } }
    pub fn as_u32(&self) -> u32 { unsafe {
        return *(&raw const *self as *const u32);
    } }
    
    pub fn min(&self, rel_pos: RelPos) -> RelPos {
        return RelPos {
            layer:  rel_pos.layer  * 16 + self.depth,
            row:    rel_pos.row    * 16 + self.bottom,
            column: rel_pos.column * 16 + self.left,
        }
    }

    pub const fn full(normal: Normal, texture: u16) -> BlockFace { unsafe {
        let depth = match normal {
            North | West | Down => { 0 }
            South | East | Up => { 15 }
            _ => { 0 }
        };
        return BlockFace {
            left: 0,
            bottom: 0,
            right: 0,
            top: 0,
            depth,
            normal,
            texture,
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
                    left: 0,
                    bottom: 0,
                    right: 8,
                    top: 0,
                    depth,
                    normal,
                    texture,
                };
            },
            HalfFaceType::Right => {
                return BlockFace {
                    left: 8,
                    bottom: 0,
                    right: 0,
                    top: 0,
                    depth,
                    normal,
                    texture,
                };

            },
            HalfFaceType::Top => {
                return BlockFace {
                    left: 0,
                    bottom: 8,
                    right: 0,
                    top: 0,
                    depth,
                    normal,
                    texture,
                };


            },
            HalfFaceType::Bottom => {
                return BlockFace {
                    left: 0,
                    bottom: 0,
                    right: 0,
                    top: 8,
                    depth,
                    normal,
                    texture,
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
                    left: 0,
                    bottom: 8,
                    right: 8,
                    top: 0,
                    depth,
                    normal,
                    texture,
                };
            },
            TopRight => {
                return BlockFace {
                    left: 8,
                    bottom: 8,
                    right: 0,
                    top: 0,
                    depth,
                    normal,
                    texture,
                };

            },
            BottomLeft => {
                return BlockFace {
                    left: 0,
                    bottom: 0,
                    right: 8,
                    top: 8,
                    depth,
                    normal,
                    texture,
                };


            },
            BottomRight => {
                return BlockFace {
                    left: 8,
                    bottom: 0,
                    right: 0,
                    top: 8,
                    depth,
                    normal,
                    texture,
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
            left: 0x0f, bottom: 0x0f, depth, normal: Unaligned,
            right: 0x0f, top: 0x0f, texture: u16::MAX
        };
    }

    pub const fn set_depth(self, depth: u8) -> BlockFace {
        return BlockFace { 
            depth,
            ..self
        };
    }

    pub const fn set_texture(self, texture: u16) -> BlockFace {
        return BlockFace { 
            texture,
            ..self
        };
    }
}

impl Clone for BlockFace {
    fn clone(&self) -> BlockFace {
        return BlockFace {
            left: self.left,
            bottom: self.bottom,
            right: self.right,
            top: self.top,
            depth: self.depth,
            normal: self.normal,
            texture: self.texture 
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
        return self.top == other.top && self.left == other.left && self.bottom == other.bottom && self.depth == other.depth && self.normal == other.normal && self.right == other.right && self.top == other.top;
    }
}