use super::{blockmodel::BlockModel, block::Block, blockface::{BlockFace, HalfFaceType::*, QuarterFaceType::*}};
use crate::block::normal::Normal::*;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct BlockState {
    pub model: BlockModel,
}

pub static BLOCKS: [BlockState; 5] = [
    // air
    BlockState {
        model: BlockModel::none(),
    },
    // full
    BlockState {
        model: BlockModel::full(1)
            .set_texture(North, 1)
            .set_texture(West, 2)
            .set_texture(Down, 3)
            .set_texture(South, 4)
            .set_texture(East, 5)
            .set_texture(Up, 6),
    },
    // stair
    BlockState {
        model: BlockModel {
            south: BlockFace::half(South, Bottom, 2),
            west: BlockFace::half(West, Bottom, 2),
            down: BlockFace::full(Down, 2),
            north: BlockFace::full(North, 2),
            east: BlockFace::half(East, Bottom, 2),
            up: BlockFace::half(Up, Right, 2),
            extra_south: &[BlockFace::half(South, Top, 2).set_depth(8)],
            extra_west: &[BlockFace::quarter(West, TopRight, 2)],
            extra_down: &[],
            extra_north: &[],
            extra_east: &[BlockFace::quarter(East, TopRight, 2)],
            extra_up: &[BlockFace::half(Up, Left, 2).set_depth(7)],
            transparent_south: &[],
            transparent_west: &[],
            transparent_down: &[],
            transparent_north: &[],
            transparent_east: &[],
            transparent_up: &[],
            unaligned: &[],
        },
    },
    // tall grass
    BlockState {
        model: BlockModel {
            unaligned: &[BlockFace::full(Diagonal, 4), BlockFace::full(OtherDiagonal, 4)],
            ..BlockModel::none()
        },
    },
    // glass
    BlockState {
        model: BlockModel {
            transparent_south: &[BlockFace::full(South, 1)],
            transparent_west: &[BlockFace::full(West, 1)],
            transparent_down: &[BlockFace::full(Down, 1)],
            transparent_north: &[BlockFace::full(North, 1)],
            transparent_east: &[BlockFace::full(East, 1)],
            transparent_up: &[BlockFace::full(Up, 1)],
            ..BlockModel::none()
        },
    }
];