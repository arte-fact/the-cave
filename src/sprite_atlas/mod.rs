/// Sprite atlas — canonical catalog of every named sprite across all sprite sheets.
///
/// Each sub-module corresponds to one sprite sheet PNG and contains an enum with
/// a variant for every sprite listed in the artist's `.txt` manifest
/// (from 32rogues-0.5.0.zip). Positions are derived from the `row.col_letter`
/// notation in those files: row = (number - 1), col = (letter index a=0, b=1, ...).

pub mod animated_tiles;
pub mod animals;
pub mod items;
pub mod monsters;
pub mod rogues;
pub mod tiles;

/// Which sprite sheet PNG to source from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sheet {
    Tiles,
    Monsters,
    Rogues,
    Items,
    Animals,
    AnimatedTiles,
}

/// A reference to a single 32x32 sprite in a sheet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SpriteRef {
    pub sheet: Sheet,
    pub row: u16,
    pub col: u16,
}

impl SpriteRef {
    pub const fn new(sheet: Sheet, row: u16, col: u16) -> Self {
        Self { sheet, row, col }
    }

    /// Source x pixel in the sprite sheet (for drawImage).
    pub fn src_x(self) -> f64 {
        self.col as f64 * 32.0
    }

    /// Source y pixel in the sprite sheet (for drawImage).
    pub fn src_y(self) -> f64 {
        self.row as f64 * 32.0
    }
}
