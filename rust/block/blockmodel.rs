use std::ops::Index;

use super::blockface::{BlockFace, Normal, N};

#[derive(Clone, Debug)]
pub struct BlockModel(pub [BlockFace; 6]);

impl Index<Normal> for BlockModel {
    type Output = BlockFace;

    fn index(&self, index: Normal) -> &Self::Output {
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