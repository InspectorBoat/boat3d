use super::{blockmodel::BlockModel, block::Block, blockface::{BlockFace, HalfFaceType::*, QuarterFaceType::*}};
use crate::block::normal::Normal::*;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct BlockState {
    pub model: BlockModel,
}

pub static mut BLOCKS: [BlockState; 5] = [
    // air
    BlockState {
        model: BlockModel::none(),
    },
    // full
    BlockState {
        model: BlockModel::full(1)
            .set_texture(South, 1)
            .set_texture(East, 2)
            .set_texture(Down, 3)
            .set_texture(North, 4)
            .set_texture(West, 5)
            .set_texture(Up, 6),
    },
    // stair
    BlockState {
        model: BlockModel {
            north: BlockFace::half(North, Bottom, 2),
            east: BlockFace::half(East, Bottom, 2),
            down: BlockFace::full(Down, 2),
            south: BlockFace::full(South, 2),
            west: BlockFace::half(West, Bottom, 2),
            up: BlockFace::half(Up, Right, 2),
            extra_north: &[BlockFace::half(North, Top, 2).set_depth(8)],
            extra_east: &[BlockFace::quarter(East, TopRight, 2)],
            extra_down: &[],
            extra_south: &[],
            extra_west: &[BlockFace::quarter(West, TopRight, 2)],
            extra_up: &[BlockFace::half(Up, Left, 2).set_depth(7)],
            trans_north: &[],
            trans_east: &[],
            trans_down: &[],
            trans_south: &[],
            trans_west: &[],
            trans_up: &[],
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
            trans_north: &[BlockFace::full(North, 1)],
            trans_east: &[BlockFace::full(East, 1)],
            trans_down: &[BlockFace::full(Down, 1)],
            trans_south: &[BlockFace::full(South, 1)],
            trans_west: &[BlockFace::full(West, 1)],
            trans_up: &[BlockFace::full(Up, 1)],
            ..BlockModel::none()
        },
    }
];