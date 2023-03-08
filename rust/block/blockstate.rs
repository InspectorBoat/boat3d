use super::{blockmodel::BlockModel, block::Block};

#[derive(Clone, Debug)]
pub struct BlockState {
    pub model: BlockModel,
    pub block: Block,
}