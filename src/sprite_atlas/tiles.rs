/// Sprites from `tiles.png` (17 cols x 26 rows, 32x32 cells).
/// Positions from `32rogues/tiles.txt`.
use super::{Sheet, SpriteRef};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileSprite {
    // Stone brick walls
    StoneBrickWallTop,
    StoneBrickWallSide1,

    // Stone floor
    FloorStone1,
    FloorStone2,
    FloorStone3,

    // Grass
    Grass1,
    Grass2,
    Grass3,

    // Dirt
    Dirt1,
    Dirt2,
    Dirt3,

    // Doors, stairs
    Door1,
    StaircaseDown,
    StaircaseUp,

    // Crops
    Buckwheat,
    Rice,
    Wheat,
    MaizeCorn,
    Amaranth,
    Quinoa,
    BitterVetch,
    Sorghum,
    RedSpinach,

    // Mushrooms
    SmallMushrooms,

    // Trees
    Tree,
}

impl TileSprite {
    pub const fn sprite_ref(self) -> SpriteRef {
        let (row, col) = match self {
            Self::StoneBrickWallTop => (2, 0),
            Self::StoneBrickWallSide1 => (2, 1),

            Self::FloorStone1 => (6, 1),
            Self::FloorStone2 => (6, 2),
            Self::FloorStone3 => (6, 3),

            Self::Grass1 => (7, 1),
            Self::Grass2 => (7, 2),
            Self::Grass3 => (7, 3),

            Self::Dirt1 => (8, 1),
            Self::Dirt2 => (8, 2),
            Self::Dirt3 => (8, 3),

            Self::Door1 => (16, 0),
            Self::StaircaseDown => (16, 7),
            Self::StaircaseUp => (16, 8),

            Self::Buckwheat => (19, 0),
            Self::Rice => (19, 6),
            Self::Wheat => (19, 7),
            Self::MaizeCorn => (19, 8),
            Self::Amaranth => (19, 9),
            Self::Quinoa => (19, 10),
            Self::BitterVetch => (19, 11),
            Self::Sorghum => (19, 12),
            Self::RedSpinach => (19, 13),

            Self::SmallMushrooms => (20, 0),

            Self::Tree => (25, 2),
        };
        SpriteRef::new(Sheet::Tiles, row, col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_tile_sprites_within_sheet_bounds() {
        let all = [
            TileSprite::StoneBrickWallTop, TileSprite::StoneBrickWallSide1,
            TileSprite::FloorStone1, TileSprite::FloorStone2, TileSprite::FloorStone3,
            TileSprite::Grass1, TileSprite::Grass2, TileSprite::Grass3,
            TileSprite::Dirt1, TileSprite::Dirt2, TileSprite::Dirt3,
            TileSprite::Door1, TileSprite::StaircaseDown, TileSprite::StaircaseUp,
            TileSprite::Buckwheat, TileSprite::Rice, TileSprite::Wheat,
            TileSprite::MaizeCorn, TileSprite::Amaranth, TileSprite::Quinoa,
            TileSprite::BitterVetch, TileSprite::Sorghum, TileSprite::RedSpinach,
            TileSprite::SmallMushrooms,
            TileSprite::Tree,
        ];
        for sprite in all {
            let r = sprite.sprite_ref();
            assert_eq!(r.sheet, Sheet::Tiles);
            assert!(r.row < 26, "{:?} row {} >= 26", sprite, r.row);
            assert!(r.col < 17, "{:?} col {} >= 17", sprite, r.col);
        }
    }

    #[test]
    fn staircase_positions_match_legacy() {
        assert_eq!(TileSprite::StaircaseDown.sprite_ref(), SpriteRef::new(Sheet::Tiles, 16, 7));
        assert_eq!(TileSprite::StaircaseUp.sprite_ref(), SpriteRef::new(Sheet::Tiles, 16, 8));
        assert_eq!(TileSprite::StoneBrickWallTop.sprite_ref(), SpriteRef::new(Sheet::Tiles, 2, 0));
        assert_eq!(TileSprite::StoneBrickWallSide1.sprite_ref(), SpriteRef::new(Sheet::Tiles, 2, 1));
        assert_eq!(TileSprite::Tree.sprite_ref(), SpriteRef::new(Sheet::Tiles, 25, 2));
        assert_eq!(TileSprite::Door1.sprite_ref(), SpriteRef::new(Sheet::Tiles, 16, 0));
    }
}
