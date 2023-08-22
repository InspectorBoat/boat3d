#[derive(Clone, Copy, Debug)]
#[repr(C, align(2))]
pub struct MergedQuad {
    pub min: RelPos,
    pub max: RelPos,
}

impl MergedQuad {
    pub fn new() -> MergedQuad {
        return MergedQuad {
            min: RelPos::new(255, 255, 255),
            max: RelPos::new(255, 255, 255),
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct RelBlockPos {
    pub layer: u8,
    pub row: u8,
    pub column: u8,
}

impl RelBlockPos {
    pub fn new<T: TryInto<u8>>(x: T, y: T, z: T) -> RelBlockPos { unsafe {
        return RelBlockPos {
            layer: x.try_into().unwrap_unchecked(),
            row: y.try_into().unwrap_unchecked(),
            column: z.try_into().unwrap_unchecked(),
        };
    } }
}

#[derive(Clone, Copy, Debug)]
pub struct RelPos {
    pub layer: u8,
    pub row: u8,
    pub column: u8,
}

impl RelPos {
    pub fn new<T: TryInto<u8>>(x: T, y: T, z: T) -> RelPos { unsafe {
        return RelPos {
            layer: x.try_into().unwrap_unchecked(),
            row: y.try_into().unwrap_unchecked(),
            column: z.try_into().unwrap_unchecked(),
        };
    } }
}
