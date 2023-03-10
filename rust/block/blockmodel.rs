use std::ops::Index;

use super::blockface::{BlockFace, Norm, N};

#[derive(Clone, Debug)]
pub struct BlockModel(pub [BlockFace; 6]);

impl Index<Norm> for BlockModel {
    type Output = BlockFace;

    fn index(&self, index: Norm) -> &Self::Output {
        return unsafe {
            &self.0.get_unchecked(index.0 as usize)
        }
    }
}

impl Index<N> for BlockModel {
    type Output = BlockFace;

    fn index(&self, index: N) -> &Self::Output {
        return unsafe {
            &self.0.get_unchecked(index.index() as usize)
        }
    }
}