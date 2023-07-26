use super::{blockmodel::BlockModel, block::Block};

#[derive(Clone, Debug)]
pub struct BlockState {
    pub model: BlockModel,
    pub block: Block,
    pub otherFaces: [u16; 6],
}

impl BlockState {
    pub const fn full(south_texture: u16, west_texture: u16, down_texture: u16, north_texture: u16, east_texture: u16, up_texture: u16) -> BlockState {
        return BlockState {
            model: BlockModel {
                south: todo!(),
                west: todo!(),
                down: todo!(),
                north: todo!(),
                east: todo!(),
                up: todo!(),
            },
            block: todo!(),
            otherFaces: todo!(),
        };
    }
    pub const NONE: [u16; 6] = [0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff];
}