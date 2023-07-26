use std::{marker::ConstParamTy, hint::unreachable_unchecked};
use self::Normal::*;

#[derive(Clone, Copy, Debug)]
#[derive(PartialEq, Eq, ConstParamTy)]
#[repr(u8)]
pub enum Normal {
    South = 0,
    West = 1,
    Down = 2,
    North = 3,
    East = 4,
    Up = 5,
    Unaligned = u8::MAX
}

impl Normal {
    pub const fn reverse(&self) -> Normal { unsafe {
        match self {
            South => { return North; }
            West  => { return East; }
            Down  => { return Up; }
            North => { return South; }
            East  => { return West; }
            Up    => { return Down; }
            _     => { unreachable_unchecked(); }
        }
    } }
}