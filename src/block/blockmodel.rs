use std::{ops::Index, hint};

use super::{blockface::BlockFace, normal::Normal::{self, *}};

#[derive(Clone, Debug)]
#[repr(C)]
pub struct BlockModel {
    pub south: BlockFace,
    pub west: BlockFace,
    pub down: BlockFace,
    pub north: BlockFace,
    pub east: BlockFace,
    pub up: BlockFace,
}

impl BlockModel {
    pub fn get_face(&self, normal: Normal) -> &BlockFace { unsafe {
        match normal {
            South => { return &self.south; }
            West => { return &self.west; }
            Down => { return &self.down; }
            North => { return &self.north; }
            East => { return &self.east; }
            Up => { return &self.up; }
            Unaligned => { hint::unreachable_unchecked() }
        }
    } }
}