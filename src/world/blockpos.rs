use std::ops::{Deref, Sub, Add};
use std::fmt::Debug;


#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, PartialOrd, Ord, Eq)]
pub struct BlockPos {
    pub index: usize
}

impl BlockPos {
    #[inline(always)]
    pub fn x(&self) -> usize {
        return (self.index & 0xf00) >> 8;
    }
    #[inline(always)]
    pub fn y(&self) -> usize {
        return (self.index & 0x0f0) >> 4;
    }
    #[inline(always)]
    pub fn z(&self) -> usize {
        return (self.index & 0x00f) >> 0;
    }
    #[inline(always)]
    pub fn set_x(&self, x: usize) -> BlockPos {
        return BlockPos { index: (self.index & 0x0ff) | (x << 8) };
    }
    #[inline(always)]
    pub fn set_y(&self, y: usize) -> BlockPos {
        return BlockPos { index: (self.index & 0xf0f) | (y << 4) };
    }
    #[inline(always)]
    pub fn set_z(&self, z: usize) -> BlockPos {
        return BlockPos { index: (self.index & 0xff0) | (z << 0) };
    }
    #[inline(always)]
    pub fn new<T: TryInto<usize>>(x: T, y: T, z: T) -> BlockPos { unsafe {
        return BlockPos {
            index: (x.try_into().unwrap_unchecked() << 8) |
                (y.try_into().unwrap_unchecked() << 4) |
                (z.try_into().unwrap_unchecked() << 0)
        };
    } }
}

impl Sub for BlockPos {
    type Output = BlockPos;

    fn sub(self, rhs: Self) -> Self::Output {
        return BlockPos { index: self.index - rhs.index };
    }
}

impl Add for BlockPos {
    type Output = BlockPos;

    fn add(self, rhs: Self) -> Self::Output {
        return BlockPos { index: self.index + rhs.index };
    }
}

impl From<BlockPos> for usize {
    fn from(value: BlockPos) -> usize {
        return value.index;
    }
}

impl Debug for BlockPos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BlockPos")
            .field("x", &self.x())
            .field("y", &self.y())
            .field("z", &self.z())
            .finish()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RelBlockPos {
    pub rel_x: u8,
    pub rel_y: u8,
    pub rel_z: u8,
}

impl RelBlockPos {
    #[inline(always)]
    pub fn new<T: TryInto<u8>>(x: T, y: T, z: T) -> RelBlockPos { unsafe {
        return RelBlockPos {
            rel_x: x.try_into().unwrap_unchecked(),
            rel_y: y.try_into().unwrap_unchecked(),
            rel_z: z.try_into().unwrap_unchecked()
        };
    } }
}

impl Sub for RelBlockPos {
    type Output = RelBlockPos;

    fn sub(self, rhs: Self) -> Self::Output {
        return RelBlockPos {
            rel_x: self.rel_x - rhs.rel_x,
            rel_y: self.rel_y - rhs.rel_y,
            rel_z: self.rel_z - rhs.rel_z,
        };
    }
}

impl Add for RelBlockPos {
    type Output = RelBlockPos;

    fn add(self, rhs: Self) -> Self::Output {
        return RelBlockPos {
            rel_x: self.rel_x + rhs.rel_x,
            rel_y: self.rel_y + rhs.rel_y,
            rel_z: self.rel_z + rhs.rel_z,
        };
    }
}