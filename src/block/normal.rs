use std::{marker::ConstParamTy, hint::unreachable_unchecked};
use self::Normal::*;

#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq, ConstParamTy)]
#[repr(u8)]
pub enum Normal {
    North = 0,
    East = 1,
    Down = 2,
    South = 3,
    West = 4,
    Up = 5,

    Diagonal = 6,
    OtherDiagonal = 7,
    Unaligned = u8::MAX
}

impl Normal {
    pub const fn reverse(&self) -> Normal { unsafe {
        match self {
            North => { return South; }
            East  => { return West; }
            Down  => { return Up; }
            South => { return North; }
            West  => { return East; }
            Up    => { return Down; }
            _     => { unreachable_unchecked(); }
        }
    } }
}