use std::ops::{Deref, Sub, Add};
use std::fmt::Debug;


#[repr(transparent)]
#[derive(Clone, Copy, Debug)]
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
    pub fn new<T: TryInto<usize> + Debug>(x: T, y: T, z: T) -> BlockPos where <T as TryInto<usize>>::Error: Debug { unsafe {
        return BlockPos {
            index: (x.try_into().unwrap() << 8) |
                (y.try_into().unwrap() << 4) |
                (z.try_into().unwrap() << 0)
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

impl Deref for BlockPos {
    type Target = usize;

    fn deref(&self) -> &usize {
        return &self.index;
    }
}