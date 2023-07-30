use std::hint;

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
    pub extra_south: &'static [BlockFace],
    pub extra_west: &'static [BlockFace],
    pub extra_down: &'static [BlockFace],
    pub extra_north: &'static [BlockFace],
    pub extra_east: &'static [BlockFace],
    pub extra_up: &'static [BlockFace],
    pub unaligned: &'static [BlockFace],
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
            _ => { hint::unreachable_unchecked() }
        }
    } }

    pub const fn full(texture: u16) -> BlockModel {
        return BlockModel {
            south: BlockFace::full(South, texture),
            west: BlockFace::full(West, texture),
            down: BlockFace::full(Down, texture),
            north: BlockFace::full(North, texture),
            east: BlockFace::full(East, texture),
            up: BlockFace::full(Up, texture),
            extra_south: &[],
            extra_west: &[],
            extra_down: &[],
            extra_north: &[],
            extra_east: &[],
            extra_up: &[],
            unaligned: &[],
        }
    }

    pub const fn set_texture(mut self, normal: Normal, texture: u16) -> BlockModel { unsafe {
        match normal {
            South => {
                self.south = self.south.set_texture(texture);
            }
            West => {
                self.west = self.west.set_texture(texture);
            }
            Down => {
                self.down = self.down.set_texture(texture);
            }
            North => {
                self.north = self.north.set_texture(texture);
            }
            East => {
                self.east = self.east.set_texture(texture);
            }
            Up => {
                self.up = self.up.set_texture(texture);
            }
            _ => { hint::unreachable_unchecked(); },
        }
        return self;
    } }

    pub const fn none() -> BlockModel {
        return BlockModel {
            south: BlockFace::none(),
            west: BlockFace::none(),
            down: BlockFace::none(),
            north: BlockFace::none(),
            east: BlockFace::none(),
            up: BlockFace::none(),
            extra_south: &[],
            extra_west: &[],
            extra_down: &[],
            extra_north: &[],
            extra_east: &[],
            extra_up: &[],
            unaligned: &[]
        };
    }
}