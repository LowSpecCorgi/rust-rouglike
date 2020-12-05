#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub blocked: bool,
    pub block_sight: bool,
}

impl Tile {
    /// Create an empty tile
    pub fn empty() -> Self {
        Tile {
            blocked: false,
            block_sight: false,
        }
    }

    /// Create a filled, wall tile
    pub fn wall() -> Self {
        Tile {
            blocked: true,
            block_sight: true,
        }
    }
}
