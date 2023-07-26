use std::{ops::Index, hint};

use super::blockface::{BlockFace, Normal};

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
            Normal::South => { return &self.south; }
            Normal::West => { return &self.west; }
            Normal::Down => { return &self.down; }
            Normal::North => { return &self.north; }
            Normal::East => { return &self.east; }
            Normal::Up => { return &self.up; }
            Normal::Unaligned => { hint::unreachable_unchecked() }
        }
    } }
}