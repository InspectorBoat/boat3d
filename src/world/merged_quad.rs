pub struct MergedQuad {
    pub min: RelVec3,
    pub max: RelVec3,
}

pub struct RelVec3 {
    pub layer: u8,
    pub row: u8,
    pub column: u8,
}