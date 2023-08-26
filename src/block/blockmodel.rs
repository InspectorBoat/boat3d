use std::hint;

use super::{blockface::BlockFace, normal::Normal::{self, *}};

#[derive(Clone, Debug)]
#[repr(C)]
pub struct BlockModel {
    pub north: BlockFace,
    pub east: BlockFace,
    pub down: BlockFace,
    pub south: BlockFace,
    pub west: BlockFace,
    pub up: BlockFace,
    
    pub extra_north: &'static [BlockFace],
    pub extra_east: &'static [BlockFace],
    pub extra_down: &'static [BlockFace],
    pub extra_south: &'static [BlockFace],
    pub extra_west: &'static [BlockFace],
    pub extra_up: &'static [BlockFace],

    pub trans_north: &'static [BlockFace],
    pub trans_east: &'static [BlockFace],
    pub trans_down: &'static [BlockFace],
    pub trans_south: &'static [BlockFace],
    pub trans_west: &'static [BlockFace],
    pub trans_up: &'static [BlockFace],
    
    pub unaligned: &'static [BlockFace],
}

impl BlockModel {
    pub fn get_face(&self, normal: Normal) -> &BlockFace { unsafe {
        match normal {
            North => { return &self.north; }
            East => { return &self.east; }
            Down => { return &self.down; }
            South => { return &self.south; }
            West => { return &self.west; }
            Up => { return &self.up; }
            _ => { panic!(); }
        }
    } }

    pub const fn full(texture: u16) -> BlockModel {
        return BlockModel {
            north: BlockFace::full(North, texture),
            east: BlockFace::full(East, texture),
            down: BlockFace::full(Down, texture),
            south: BlockFace::full(South, texture),
            west: BlockFace::full(West, texture),
            up: BlockFace::full(Up, texture),
            extra_north: &[],
            extra_east: &[],
            extra_down: &[],
            extra_south: &[],
            extra_west: &[],
            extra_up: &[],
            trans_north: &[],
            trans_east: &[],
            trans_down: &[],
            trans_south: &[],
            trans_west: &[],
            trans_up: &[],
            unaligned: &[],
        }
    }

    pub const fn set_texture(mut self, normal: Normal, texture: u16) -> BlockModel { unsafe {
        match normal {
            North => {
                self.north = self.north.set_texture(texture);
            }
            East => {
                self.east = self.east.set_texture(texture);
            }
            Down => {
                self.down = self.down.set_texture(texture);
            }
            South => {
                self.south = self.south.set_texture(texture);
            }
            West => {
                self.west = self.west.set_texture(texture);
            }
            Up => {
                self.up = self.up.set_texture(texture);
            }
            _ => { panic!(); },
        }
        return self;
    } }

    pub const fn none() -> BlockModel {
        return BlockModel {
            north: BlockFace::none(North),
            east: BlockFace::none(East),
            down: BlockFace::none(Down),
            south: BlockFace::none(South),
            west: BlockFace::none(West),
            up: BlockFace::none(Up),
            extra_north: &[],
            extra_east: &[],
            extra_down: &[],
            extra_south: &[],
            extra_west: &[],
            extra_up: &[],
            trans_north: &[],
            trans_east: &[],
            trans_down: &[],
            trans_south: &[],
            trans_west: &[],
            trans_up: &[],
            unaligned: &[]
        };
    }
}