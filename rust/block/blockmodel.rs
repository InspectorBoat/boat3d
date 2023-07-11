use std::ops::Index;

use super::blockface::{BlockFace, Normal};

#[derive(Clone, Debug)]
pub struct BlockModel(pub [BlockFace; 6]);

impl BlockModel {
    pub fn get_face(&self, normal: Normal) -> &BlockFace { unsafe {
        return self.0.get_unchecked(normal.0 as usize);
    } }
}