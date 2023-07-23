use super::{blockmodel::BlockModel, block::Block};

#[derive(Clone, Debug)]
pub struct BlockState {
    pub model: BlockModel,
    pub block: Block,
    pub otherFaces: [u16; 6],
}

impl BlockState {
    pub const NONE: [u16; 6] = [0xffff, 0xffff, 0xffff, 0xffff, 0xffff, 0xffff];
}