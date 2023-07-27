use std::f32::consts::E;

use super::{blockmodel::BlockModel, block::Block, blockface::{BlockFace, HalfFaceType::*, QuarterFaceType::*}};
use crate::block::normal::Normal::*;

#[derive(Clone, Debug)]
pub struct BlockState {
    pub model: BlockModel,
    pub block: Block,
}

pub const BLOCKS: [BlockState; 4] = [
    BlockState {
        block: Block { name: "air" },
        model: BlockModel::none(),
    },
    BlockState {
        block: Block { name: "bricks" },
        model: BlockModel::full(1).set_texture(North, 1).set_texture(West, 2).set_texture(Down, 3).set_texture(South, 4).set_texture(East, 5).set_texture(Up, 6),
    },
    BlockState {
        block: Block { name: "brick_stairs" },
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
            unaligned: &[],
        },
    },
    BlockState {
        block: Block { name: "tall_grass" },
        model: BlockModel {
            unaligned: &[BlockFace::full(Diagonal, 4)],
            ..BlockModel::none()
        },
    },
];