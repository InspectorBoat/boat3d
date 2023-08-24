pub struct MergedQuad {
    pub min: RelPos,
    pub max: RelPos,
}

pub struct RelPos {
    pub layer: u8,
    pub row: u8,
    pub column: u8,
}

impl RelPos {
    pub fn new(layer: u8, row: u8, column: u8) -> RelPos {
        return RelPos {
            layer: layer,
            row: row,
            column: column,
        }
    }
}