/// Every named sprite in `tiles.png` (17 cols x 26 rows, 32x32 cells).
/// Positions from `32rogues/tiles.txt`.
use super::{Sheet, SpriteRef};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileSprite {
    // Row 0 (1) — dirt walls
    DirtWallTop,
    DirtWallSide,
    InnerWall,

    // Row 1 (2) — rough stone walls
    RoughStoneWallTop,
    RoughStoneWallSide,

    // Row 2 (3) — stone brick walls
    StoneBrickWallTop,
    StoneBrickWallSide1,
    StoneBrickWallSide2,

    // Row 3 (4) — igneous walls
    IgneousWallTop,
    IgneousWallSide,

    // Row 4 (5) — large stone walls
    LargeStoneWallTop,
    LargeStoneWallSide,

    // Row 5 (6) — catacombs / skull walls
    CatacombsSkullWallTop,
    CatacombsSkullWallSide,

    // Row 6 (7) — stone floor
    BlankFloorDarkGrey,
    FloorStone1,
    FloorStone2,
    FloorStone3,
    FloorStone1NoBg,
    FloorStone2NoBg,
    FloorStone3NoBg,

    // Row 7 (8) — grass
    BlankFloorDarkPurple,
    Grass1,
    Grass2,
    Grass3,
    Grass1NoBg,
    Grass2NoBg,
    Grass3NoBg,

    // Row 8 (9) — dirt
    Dirt1,
    Dirt2,
    Dirt3,
    Dirt1NoBg,
    Dirt2NoBg,
    Dirt3NoBg,

    // Row 9 (10) — stone floor alt
    StoneFloor1,
    StoneFloor2,
    StoneFloor3,
    StoneFloor1NoBg,
    StoneFloor2NoBg,
    StoneFloor3NoBg,

    // Row 10 (11) — bone floor
    Bone1,
    Bone2,
    Bone3,
    Bone1NoBg,
    Bone2NoBg,
    Bone3NoBg,

    // Row 11 (12) — red stone floor
    BlankRedFloor,
    RedStoneFloor1,
    RedStoneFloor2,
    RedStoneFloor3,
    RedStoneFloor1NoBg,
    RedStoneFloor2NoBg,
    RedStoneFloor3NoBg,

    // Row 12 (13) — blue stone floor
    BlankBlueFloor,
    BlueStoneFloor1,
    BlueStoneFloor2,
    BlueStoneFloor3,

    // Row 13 (14) — dirt on green bg
    BlankGreenFloor,
    DirtGreenBg1,
    DirtGreenBg2,
    DirtGreenBg3,

    // Row 14 (15) — grass on green bg
    GrassGreenBg1,
    GrassGreenBg2,
    GrassGreenBg3,

    // Row 15 (16) — bones on dark brown bg
    DarkBrownBg,
    BonesDarkBrown1,
    BonesDarkBrown2,
    BonesDarkBrown3,

    // Row 16 (17) — doors, stairs, traps
    Door1,
    Door2,
    FramedDoor1Shut,
    FramedDoor1Open,
    FramedDoor2Shut,
    FramedDoor2Open,
    GratedDoor,
    StaircaseDown,
    StaircaseUp,
    PressurePlateUp,
    PressurePlateDown,
    Chute,
    Pit,
    TrapDoor,
    Pentagram,
    SpikesDown,
    SpikesUp,

    // Row 17 (18) — containers
    ChestClosed,
    ChestOpen,
    JarClosed,
    JarOpen,
    Barrel,
    OreSack,
    LogPile,

    // Row 18 (19) — rocks
    LargeRock1,
    LargeRock2,

    // Row 19 (20) — crops
    Buckwheat,
    Flax,
    PapyrusSedge,
    Kenaf,
    Ramie,
    Jute,
    Rice,
    Wheat,
    MaizeCorn,
    Amaranth,
    Quinoa,
    BitterVetch,
    Sorghum,
    RedSpinach,
    Cotton,
    Alfalfa,

    // Row 20 (21) — mushrooms
    SmallMushrooms,
    LargeMushroom,

    // Row 21 (22) — corpses
    CorpseBones1,
    CorpseBones2,

    // Row 22 (23) — blood and slime
    BloodSpatter1,
    BloodSpatter2,
    SlimeSmall,
    SlimeLarge,

    // Row 23 (24) — coffins and sarcophagi
    CoffinClosed,
    CoffinAjar,
    CoffinOpen,
    SarcophagusClosed,
    SarcophagusAjar,
    SarcophagusOpen,

    // Row 25 (26) — trees
    Sapling,
    SmallTree,
    Tree,
    TwoTileTree,
}

impl TileSprite {
    pub const fn sprite_ref(self) -> SpriteRef {
        let (row, col) = match self {
            Self::DirtWallTop => (0, 0),
            Self::DirtWallSide => (0, 1),
            Self::InnerWall => (0, 2),

            Self::RoughStoneWallTop => (1, 0),
            Self::RoughStoneWallSide => (1, 1),

            Self::StoneBrickWallTop => (2, 0),
            Self::StoneBrickWallSide1 => (2, 1),
            Self::StoneBrickWallSide2 => (2, 2),

            Self::IgneousWallTop => (3, 0),
            Self::IgneousWallSide => (3, 1),

            Self::LargeStoneWallTop => (4, 0),
            Self::LargeStoneWallSide => (4, 1),

            Self::CatacombsSkullWallTop => (5, 0),
            Self::CatacombsSkullWallSide => (5, 1),

            Self::BlankFloorDarkGrey => (6, 0),
            Self::FloorStone1 => (6, 1),
            Self::FloorStone2 => (6, 2),
            Self::FloorStone3 => (6, 3),
            Self::FloorStone1NoBg => (6, 4),
            Self::FloorStone2NoBg => (6, 5),
            Self::FloorStone3NoBg => (6, 6),

            Self::BlankFloorDarkPurple => (7, 0),
            Self::Grass1 => (7, 1),
            Self::Grass2 => (7, 2),
            Self::Grass3 => (7, 3),
            Self::Grass1NoBg => (7, 4),
            Self::Grass2NoBg => (7, 5),
            Self::Grass3NoBg => (7, 6),

            Self::Dirt1 => (8, 1),
            Self::Dirt2 => (8, 2),
            Self::Dirt3 => (8, 3),
            Self::Dirt1NoBg => (8, 4),
            Self::Dirt2NoBg => (8, 5),
            Self::Dirt3NoBg => (8, 6),

            Self::StoneFloor1 => (9, 1),
            Self::StoneFloor2 => (9, 2),
            Self::StoneFloor3 => (9, 3),
            Self::StoneFloor1NoBg => (9, 4),
            Self::StoneFloor2NoBg => (9, 5),
            Self::StoneFloor3NoBg => (9, 6),

            Self::Bone1 => (10, 1),
            Self::Bone2 => (10, 2),
            Self::Bone3 => (10, 3),
            Self::Bone1NoBg => (10, 4),
            Self::Bone2NoBg => (10, 5),
            Self::Bone3NoBg => (10, 6),

            Self::BlankRedFloor => (11, 0),
            Self::RedStoneFloor1 => (11, 1),
            Self::RedStoneFloor2 => (11, 2),
            Self::RedStoneFloor3 => (11, 3),
            Self::RedStoneFloor1NoBg => (11, 4),
            Self::RedStoneFloor2NoBg => (11, 5),
            Self::RedStoneFloor3NoBg => (11, 6),

            Self::BlankBlueFloor => (12, 0),
            Self::BlueStoneFloor1 => (12, 1),
            Self::BlueStoneFloor2 => (12, 2),
            Self::BlueStoneFloor3 => (12, 3),

            Self::BlankGreenFloor => (13, 0),
            Self::DirtGreenBg1 => (13, 1),
            Self::DirtGreenBg2 => (13, 2),
            Self::DirtGreenBg3 => (13, 3),

            Self::GrassGreenBg1 => (14, 1),
            Self::GrassGreenBg2 => (14, 2),
            Self::GrassGreenBg3 => (14, 3),

            Self::DarkBrownBg => (15, 0),
            Self::BonesDarkBrown1 => (15, 1),
            Self::BonesDarkBrown2 => (15, 2),
            Self::BonesDarkBrown3 => (15, 3),

            Self::Door1 => (16, 0),
            Self::Door2 => (16, 1),
            Self::FramedDoor1Shut => (16, 2),
            Self::FramedDoor1Open => (16, 3),
            Self::FramedDoor2Shut => (16, 4),
            Self::FramedDoor2Open => (16, 5),
            Self::GratedDoor => (16, 6),
            Self::StaircaseDown => (16, 7),
            Self::StaircaseUp => (16, 8),
            Self::PressurePlateUp => (16, 9),
            Self::PressurePlateDown => (16, 10),
            Self::Chute => (16, 11),
            Self::Pit => (16, 12),
            Self::TrapDoor => (16, 13),
            Self::Pentagram => (16, 14),
            Self::SpikesDown => (16, 15),
            Self::SpikesUp => (16, 16),

            Self::ChestClosed => (17, 0),
            Self::ChestOpen => (17, 1),
            Self::JarClosed => (17, 2),
            Self::JarOpen => (17, 3),
            Self::Barrel => (17, 4),
            Self::OreSack => (17, 5),
            Self::LogPile => (17, 6),

            Self::LargeRock1 => (18, 0),
            Self::LargeRock2 => (18, 1),

            Self::Buckwheat => (19, 0),
            Self::Flax => (19, 1),
            Self::PapyrusSedge => (19, 2),
            Self::Kenaf => (19, 3),
            Self::Ramie => (19, 4),
            Self::Jute => (19, 5),
            Self::Rice => (19, 6),
            Self::Wheat => (19, 7),
            Self::MaizeCorn => (19, 8),
            Self::Amaranth => (19, 9),
            Self::Quinoa => (19, 10),
            Self::BitterVetch => (19, 11),
            Self::Sorghum => (19, 12),
            Self::RedSpinach => (19, 13),
            Self::Cotton => (19, 14),
            Self::Alfalfa => (19, 15),

            Self::SmallMushrooms => (20, 0),
            Self::LargeMushroom => (20, 1),

            Self::CorpseBones1 => (21, 0),
            Self::CorpseBones2 => (21, 1),

            Self::BloodSpatter1 => (22, 0),
            Self::BloodSpatter2 => (22, 1),
            Self::SlimeSmall => (22, 2),
            Self::SlimeLarge => (22, 3),

            Self::CoffinClosed => (23, 0),
            Self::CoffinAjar => (23, 1),
            Self::CoffinOpen => (23, 2),
            Self::SarcophagusClosed => (23, 3),
            Self::SarcophagusAjar => (23, 4),
            Self::SarcophagusOpen => (23, 5),

            Self::Sapling => (25, 0),
            Self::SmallTree => (25, 1),
            Self::Tree => (25, 2),
            Self::TwoTileTree => (25, 3),
        };
        SpriteRef::new(Sheet::Tiles, row, col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_tile_sprites_within_sheet_bounds() {
        // tiles.png: 17 cols x 26 rows
        let all = [
            TileSprite::DirtWallTop, TileSprite::DirtWallSide, TileSprite::InnerWall,
            TileSprite::RoughStoneWallTop, TileSprite::RoughStoneWallSide,
            TileSprite::StoneBrickWallTop, TileSprite::StoneBrickWallSide1, TileSprite::StoneBrickWallSide2,
            TileSprite::IgneousWallTop, TileSprite::IgneousWallSide,
            TileSprite::LargeStoneWallTop, TileSprite::LargeStoneWallSide,
            TileSprite::CatacombsSkullWallTop, TileSprite::CatacombsSkullWallSide,
            TileSprite::BlankFloorDarkGrey, TileSprite::FloorStone1, TileSprite::FloorStone2, TileSprite::FloorStone3,
            TileSprite::FloorStone1NoBg, TileSprite::FloorStone2NoBg, TileSprite::FloorStone3NoBg,
            TileSprite::BlankFloorDarkPurple, TileSprite::Grass1, TileSprite::Grass2, TileSprite::Grass3,
            TileSprite::Grass1NoBg, TileSprite::Grass2NoBg, TileSprite::Grass3NoBg,
            TileSprite::Dirt1, TileSprite::Dirt2, TileSprite::Dirt3,
            TileSprite::Dirt1NoBg, TileSprite::Dirt2NoBg, TileSprite::Dirt3NoBg,
            TileSprite::StoneFloor1, TileSprite::StoneFloor2, TileSprite::StoneFloor3,
            TileSprite::StoneFloor1NoBg, TileSprite::StoneFloor2NoBg, TileSprite::StoneFloor3NoBg,
            TileSprite::Bone1, TileSprite::Bone2, TileSprite::Bone3,
            TileSprite::Bone1NoBg, TileSprite::Bone2NoBg, TileSprite::Bone3NoBg,
            TileSprite::BlankRedFloor, TileSprite::RedStoneFloor1, TileSprite::RedStoneFloor2, TileSprite::RedStoneFloor3,
            TileSprite::RedStoneFloor1NoBg, TileSprite::RedStoneFloor2NoBg, TileSprite::RedStoneFloor3NoBg,
            TileSprite::BlankBlueFloor, TileSprite::BlueStoneFloor1, TileSprite::BlueStoneFloor2, TileSprite::BlueStoneFloor3,
            TileSprite::BlankGreenFloor, TileSprite::DirtGreenBg1, TileSprite::DirtGreenBg2, TileSprite::DirtGreenBg3,
            TileSprite::GrassGreenBg1, TileSprite::GrassGreenBg2, TileSprite::GrassGreenBg3,
            TileSprite::DarkBrownBg, TileSprite::BonesDarkBrown1, TileSprite::BonesDarkBrown2, TileSprite::BonesDarkBrown3,
            TileSprite::Door1, TileSprite::Door2,
            TileSprite::FramedDoor1Shut, TileSprite::FramedDoor1Open,
            TileSprite::FramedDoor2Shut, TileSprite::FramedDoor2Open,
            TileSprite::GratedDoor, TileSprite::StaircaseDown, TileSprite::StaircaseUp,
            TileSprite::PressurePlateUp, TileSprite::PressurePlateDown,
            TileSprite::Chute, TileSprite::Pit, TileSprite::TrapDoor,
            TileSprite::Pentagram, TileSprite::SpikesDown, TileSprite::SpikesUp,
            TileSprite::ChestClosed, TileSprite::ChestOpen,
            TileSprite::JarClosed, TileSprite::JarOpen,
            TileSprite::Barrel, TileSprite::OreSack, TileSprite::LogPile,
            TileSprite::LargeRock1, TileSprite::LargeRock2,
            TileSprite::Buckwheat, TileSprite::Flax, TileSprite::PapyrusSedge,
            TileSprite::Kenaf, TileSprite::Ramie, TileSprite::Jute, TileSprite::Rice,
            TileSprite::Wheat, TileSprite::MaizeCorn, TileSprite::Amaranth,
            TileSprite::Quinoa, TileSprite::BitterVetch, TileSprite::Sorghum,
            TileSprite::RedSpinach, TileSprite::Cotton, TileSprite::Alfalfa,
            TileSprite::SmallMushrooms, TileSprite::LargeMushroom,
            TileSprite::CorpseBones1, TileSprite::CorpseBones2,
            TileSprite::BloodSpatter1, TileSprite::BloodSpatter2,
            TileSprite::SlimeSmall, TileSprite::SlimeLarge,
            TileSprite::CoffinClosed, TileSprite::CoffinAjar, TileSprite::CoffinOpen,
            TileSprite::SarcophagusClosed, TileSprite::SarcophagusAjar, TileSprite::SarcophagusOpen,
            TileSprite::Sapling, TileSprite::SmallTree, TileSprite::Tree, TileSprite::TwoTileTree,
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
        // Verify atlas positions match existing sprites.rs mappings
        assert_eq!(TileSprite::StaircaseDown.sprite_ref(), SpriteRef::new(Sheet::Tiles, 16, 7));
        assert_eq!(TileSprite::StaircaseUp.sprite_ref(), SpriteRef::new(Sheet::Tiles, 16, 8));
        assert_eq!(TileSprite::StoneBrickWallTop.sprite_ref(), SpriteRef::new(Sheet::Tiles, 2, 0));
        assert_eq!(TileSprite::StoneBrickWallSide1.sprite_ref(), SpriteRef::new(Sheet::Tiles, 2, 1));
        assert_eq!(TileSprite::Tree.sprite_ref(), SpriteRef::new(Sheet::Tiles, 25, 2));
        assert_eq!(TileSprite::Door1.sprite_ref(), SpriteRef::new(Sheet::Tiles, 16, 0));
    }
}
