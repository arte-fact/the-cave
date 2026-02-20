/// Sprites from `tiles.png` (17 cols x 26 rows, 32x32 cells).
/// Positions from `32rogues/tiles.txt`.
use super::{Sheet, SpriteRef};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileSprite {
    // === Walls (top + side variants per biome) ===
    DirtWallTop,
    DirtWallSide,

    RoughStoneWallTop,
    RoughStoneWallSide,

    StoneBrickWallTop,
    StoneBrickWallSide1,
    StoneBrickWallSide2,

    IgneousWallTop,
    IgneousWallSide,

    LargeStoneWallTop,
    LargeStoneWallSide,

    CatacombsWallTop,
    CatacombsWallSide,

    // === Floors ===
    BlankFloorDark,
    FloorStone1,
    FloorStone2,
    FloorStone3,

    // === Grass ===
    BlankFloorGrass,
    Grass1,
    Grass2,
    Grass3,

    // === Dirt / Road ===
    Dirt1,
    Dirt2,
    Dirt3,

    // === Stone floor (dungeon) ===
    StoneFloor1,
    StoneFloor2,
    StoneFloor3,

    // === Bone floor (catacombs) ===
    BoneFloor1,
    BoneFloor2,
    BoneFloor3,

    // === Red stone floor (hell/deep) ===
    BlankRedFloor,
    RedStoneFloor1,
    RedStoneFloor2,
    RedStoneFloor3,

    // === Blue stone floor ===
    BlankBlueFloor,
    BlueStoneFloor1,
    BlueStoneFloor2,
    BlueStoneFloor3,

    // === Green dirt ===
    GreenDirt1,
    GreenDirt2,
    GreenDirt3,

    // === Green grass ===
    GreenGrass1,
    GreenGrass2,
    GreenGrass3,

    // === Dark brown bones ===
    DarkBrownBones1,
    DarkBrownBones2,
    DarkBrownBones3,

    // === Doors, stairs, traps ===
    Door1,
    Door2,
    FramedDoorShut,
    FramedDoorOpen,
    GratedDoor,
    StaircaseDown,
    StaircaseUp,
    PressurePlateUp,
    PressurePlateDown,
    Pentagram,

    // === Containers ===
    ChestClosed,
    ChestOpen,
    JarClosed,
    JarOpen,
    Barrel,

    // === Rocks ===
    LargeRock1,
    LargeRock2,

    // === Crops ===
    Buckwheat,
    Flax,
    Rice,
    Wheat,
    MaizeCorn,
    Amaranth,
    Quinoa,
    BitterVetch,
    Sorghum,
    RedSpinach,

    // === Mushrooms ===
    SmallMushrooms,
    LargeMushroom,

    // === Corpses / blood ===
    Corpse1,
    Corpse2,
    BloodSpatter1,
    BloodSpatter2,

    // === Coffins ===
    CoffinClosed,
    CoffinAjar,
    SarcophagusClosed,
    SarcophagusOpen,

    // === Trees ===
    Sapling,
    SmallTree,
    Tree,
}

impl TileSprite {
    pub const fn sprite_ref(self) -> SpriteRef {
        let (row, col) = match self {
            // Walls
            Self::DirtWallTop => (0, 0),
            Self::DirtWallSide => (0, 1),

            Self::RoughStoneWallTop => (1, 0),
            Self::RoughStoneWallSide => (1, 1),

            Self::StoneBrickWallTop => (2, 0),
            Self::StoneBrickWallSide1 => (2, 1),
            Self::StoneBrickWallSide2 => (2, 2),

            Self::IgneousWallTop => (3, 0),
            Self::IgneousWallSide => (3, 1),

            Self::LargeStoneWallTop => (4, 0),
            Self::LargeStoneWallSide => (4, 1),

            Self::CatacombsWallTop => (5, 0),
            Self::CatacombsWallSide => (5, 1),

            // Floors
            Self::BlankFloorDark => (6, 0),
            Self::FloorStone1 => (6, 1),
            Self::FloorStone2 => (6, 2),
            Self::FloorStone3 => (6, 3),

            // Grass
            Self::BlankFloorGrass => (7, 0),
            Self::Grass1 => (7, 1),
            Self::Grass2 => (7, 2),
            Self::Grass3 => (7, 3),

            // Dirt / Road
            Self::Dirt1 => (8, 1),
            Self::Dirt2 => (8, 2),
            Self::Dirt3 => (8, 3),

            // Stone floor
            Self::StoneFloor1 => (9, 1),
            Self::StoneFloor2 => (9, 2),
            Self::StoneFloor3 => (9, 3),

            // Bone floor
            Self::BoneFloor1 => (10, 1),
            Self::BoneFloor2 => (10, 2),
            Self::BoneFloor3 => (10, 3),

            // Red stone floor
            Self::BlankRedFloor => (11, 0),
            Self::RedStoneFloor1 => (11, 1),
            Self::RedStoneFloor2 => (11, 2),
            Self::RedStoneFloor3 => (11, 3),

            // Blue stone floor
            Self::BlankBlueFloor => (12, 0),
            Self::BlueStoneFloor1 => (12, 1),
            Self::BlueStoneFloor2 => (12, 2),
            Self::BlueStoneFloor3 => (12, 3),

            // Green dirt
            Self::GreenDirt1 => (13, 1),
            Self::GreenDirt2 => (13, 2),
            Self::GreenDirt3 => (13, 3),

            // Green grass
            Self::GreenGrass1 => (14, 1),
            Self::GreenGrass2 => (14, 2),
            Self::GreenGrass3 => (14, 3),

            // Dark brown bones
            Self::DarkBrownBones1 => (15, 1),
            Self::DarkBrownBones2 => (15, 2),
            Self::DarkBrownBones3 => (15, 3),

            // Doors, stairs, traps
            Self::Door1 => (16, 0),
            Self::Door2 => (16, 1),
            Self::FramedDoorShut => (16, 2),
            Self::FramedDoorOpen => (16, 3),
            Self::GratedDoor => (16, 6),
            Self::StaircaseDown => (16, 7),
            Self::StaircaseUp => (16, 8),
            Self::PressurePlateUp => (16, 9),
            Self::PressurePlateDown => (16, 10),
            Self::Pentagram => (16, 14),

            // Containers
            Self::ChestClosed => (17, 0),
            Self::ChestOpen => (17, 1),
            Self::JarClosed => (17, 2),
            Self::JarOpen => (17, 3),
            Self::Barrel => (17, 4),

            // Rocks
            Self::LargeRock1 => (18, 0),
            Self::LargeRock2 => (18, 1),

            // Crops
            Self::Buckwheat => (19, 0),
            Self::Flax => (19, 1),
            Self::Rice => (19, 6),
            Self::Wheat => (19, 7),
            Self::MaizeCorn => (19, 8),
            Self::Amaranth => (19, 9),
            Self::Quinoa => (19, 10),
            Self::BitterVetch => (19, 11),
            Self::Sorghum => (19, 12),
            Self::RedSpinach => (19, 13),

            // Mushrooms
            Self::SmallMushrooms => (20, 0),
            Self::LargeMushroom => (20, 1),

            // Corpses / blood
            Self::Corpse1 => (21, 0),
            Self::Corpse2 => (21, 1),
            Self::BloodSpatter1 => (22, 0),
            Self::BloodSpatter2 => (22, 1),

            // Coffins
            Self::CoffinClosed => (23, 0),
            Self::CoffinAjar => (23, 1),
            Self::SarcophagusClosed => (23, 3),
            Self::SarcophagusOpen => (23, 5),

            // Trees
            Self::Sapling => (25, 0),
            Self::SmallTree => (25, 1),
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
            TileSprite::DirtWallTop, TileSprite::DirtWallSide,
            TileSprite::RoughStoneWallTop, TileSprite::RoughStoneWallSide,
            TileSprite::StoneBrickWallTop, TileSprite::StoneBrickWallSide1, TileSprite::StoneBrickWallSide2,
            TileSprite::IgneousWallTop, TileSprite::IgneousWallSide,
            TileSprite::LargeStoneWallTop, TileSprite::LargeStoneWallSide,
            TileSprite::CatacombsWallTop, TileSprite::CatacombsWallSide,
            TileSprite::BlankFloorDark, TileSprite::FloorStone1, TileSprite::FloorStone2, TileSprite::FloorStone3,
            TileSprite::BlankFloorGrass, TileSprite::Grass1, TileSprite::Grass2, TileSprite::Grass3,
            TileSprite::Dirt1, TileSprite::Dirt2, TileSprite::Dirt3,
            TileSprite::StoneFloor1, TileSprite::StoneFloor2, TileSprite::StoneFloor3,
            TileSprite::BoneFloor1, TileSprite::BoneFloor2, TileSprite::BoneFloor3,
            TileSprite::BlankRedFloor, TileSprite::RedStoneFloor1, TileSprite::RedStoneFloor2, TileSprite::RedStoneFloor3,
            TileSprite::BlankBlueFloor, TileSprite::BlueStoneFloor1, TileSprite::BlueStoneFloor2, TileSprite::BlueStoneFloor3,
            TileSprite::GreenDirt1, TileSprite::GreenDirt2, TileSprite::GreenDirt3,
            TileSprite::GreenGrass1, TileSprite::GreenGrass2, TileSprite::GreenGrass3,
            TileSprite::DarkBrownBones1, TileSprite::DarkBrownBones2, TileSprite::DarkBrownBones3,
            TileSprite::Door1, TileSprite::Door2, TileSprite::FramedDoorShut, TileSprite::FramedDoorOpen,
            TileSprite::GratedDoor, TileSprite::StaircaseDown, TileSprite::StaircaseUp,
            TileSprite::PressurePlateUp, TileSprite::PressurePlateDown, TileSprite::Pentagram,
            TileSprite::ChestClosed, TileSprite::ChestOpen, TileSprite::JarClosed, TileSprite::JarOpen, TileSprite::Barrel,
            TileSprite::LargeRock1, TileSprite::LargeRock2,
            TileSprite::Buckwheat, TileSprite::Flax, TileSprite::Rice, TileSprite::Wheat,
            TileSprite::MaizeCorn, TileSprite::Amaranth, TileSprite::Quinoa,
            TileSprite::BitterVetch, TileSprite::Sorghum, TileSprite::RedSpinach,
            TileSprite::SmallMushrooms, TileSprite::LargeMushroom,
            TileSprite::Corpse1, TileSprite::Corpse2, TileSprite::BloodSpatter1, TileSprite::BloodSpatter2,
            TileSprite::CoffinClosed, TileSprite::CoffinAjar, TileSprite::SarcophagusClosed, TileSprite::SarcophagusOpen,
            TileSprite::Sapling, TileSprite::SmallTree, TileSprite::Tree,
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
