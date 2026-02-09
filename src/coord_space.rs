#[derive(Copy, Clone, Debug)]
pub struct TermCoords(pub f32, pub f32);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TwoxelCoords(pub i16, pub i16);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct OctadCoords(pub i16, pub i16);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct TetrisBlockCoords(pub i16, pub i16);

impl From<TwoxelCoords> for TermCoords {
    fn from(coords: TwoxelCoords) -> Self {
        TermCoords(coords.0 as f32, coords.1 as f32 * 0.5)
    }
}

impl From<OctadCoords> for TermCoords {
    fn from(coords: OctadCoords) -> Self {
        TermCoords(coords.0 as f32 / 2.0, coords.1 as f32 / 4.0)
    }
}

impl From<TetrisBlockCoords> for TermCoords {
    fn from(coords: TetrisBlockCoords) -> Self {
        TermCoords(coords.0 as f32 * 2.0, coords.1 as f32)
    }
}
