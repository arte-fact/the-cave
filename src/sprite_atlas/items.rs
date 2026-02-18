/// Sprites from `items.png` (11 cols x 26 rows, 32x32 cells).
/// Positions from `32rogues/items.txt`.
use super::{Sheet, SpriteRef};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemSprite {
    // Swords
    Dagger,
    ShortSword,
    LongSword,
    CrystalSword,
    FlameSword,

    // Axes
    BattleAxe,

    // Hammers
    Hammer,

    // Clubs
    Club,

    // Ranged weapons
    Crossbow,
    ShortBow,
    LongBow,
    LongBow2,
    LargeCrossbow,

    // Staves
    CrystalStaff,

    // Shields
    Buckler,
    KiteShield,
    LargeShield,

    // Body armor
    LeatherArmor,
    ChainMail,
    ChestPlate,

    // Boots
    LeatherBoots,
    HighBlueBoots,
    Greaves,

    // Helmets
    LeatherHelm,
    Helm,
    PlateHelm1,

    // Rings
    GoldBandRing,
    SilverSignetRing,
    RubyRing,
    TwistedGoldRing,
    SapphireRing,

    // Potions
    RedPotion,
    LargeDarkPotion,
    GreenPotion,

    // Scrolls and books
    RedBook,
    DarkTome,
    Tome2,

    // Food and drink
    Bread,
    Apple,
    BottleOfBeer,
    BottleOfWater,
}

impl ItemSprite {
    pub const fn sprite_ref(self) -> SpriteRef {
        let (row, col) = match self {
            Self::Dagger => (0, 0),
            Self::ShortSword => (0, 1),
            Self::LongSword => (0, 3),
            Self::CrystalSword => (0, 8),
            Self::FlameSword => (0, 10),

            Self::BattleAxe => (3, 1),

            Self::Hammer => (4, 3),

            Self::Club => (8, 0),

            Self::Crossbow => (9, 0),
            Self::ShortBow => (9, 1),
            Self::LongBow => (9, 2),
            Self::LongBow2 => (9, 3),
            Self::LargeCrossbow => (9, 4),

            Self::CrystalStaff => (10, 0),

            Self::Buckler => (11, 0),
            Self::KiteShield => (11, 1),
            Self::LargeShield => (11, 6),

            Self::LeatherArmor => (12, 1),
            Self::ChainMail => (12, 3),
            Self::ChestPlate => (12, 5),

            Self::LeatherBoots => (14, 1),
            Self::HighBlueBoots => (14, 2),
            Self::Greaves => (14, 3),

            Self::LeatherHelm => (15, 1),
            Self::Helm => (15, 4),
            Self::PlateHelm1 => (15, 6),

            Self::GoldBandRing => (17, 1),
            Self::RubyRing => (17, 3),
            Self::SapphireRing => (17, 4),

            Self::SilverSignetRing => (18, 1),
            Self::TwistedGoldRing => (18, 4),

            Self::RedPotion => (19, 1),
            Self::LargeDarkPotion => (19, 3),
            Self::GreenPotion => (19, 4),

            Self::RedBook => (21, 2),
            Self::DarkTome => (21, 3),
            Self::Tome2 => (21, 5),

            Self::Bread => (25, 1),
            Self::Apple => (25, 2),
            Self::BottleOfBeer => (25, 3),
            // txt says 25.e (row 24, col 4) — likely typo for 26.e
            Self::BottleOfWater => (24, 4),
        };
        SpriteRef::new(Sheet::Items, row, col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_item_sprites_within_sheet_bounds() {
        let all = [
            ItemSprite::Dagger, ItemSprite::ShortSword, ItemSprite::LongSword,
            ItemSprite::CrystalSword, ItemSprite::FlameSword,
            ItemSprite::BattleAxe,
            ItemSprite::Hammer,
            ItemSprite::Club,
            ItemSprite::Crossbow, ItemSprite::ShortBow, ItemSprite::LongBow,
            ItemSprite::LongBow2, ItemSprite::LargeCrossbow,
            ItemSprite::CrystalStaff,
            ItemSprite::Buckler, ItemSprite::KiteShield, ItemSprite::LargeShield,
            ItemSprite::LeatherArmor, ItemSprite::ChainMail, ItemSprite::ChestPlate,
            ItemSprite::LeatherBoots, ItemSprite::HighBlueBoots, ItemSprite::Greaves,
            ItemSprite::LeatherHelm, ItemSprite::Helm, ItemSprite::PlateHelm1,
            ItemSprite::GoldBandRing, ItemSprite::SilverSignetRing, ItemSprite::RubyRing,
            ItemSprite::TwistedGoldRing, ItemSprite::SapphireRing,
            ItemSprite::RedPotion, ItemSprite::LargeDarkPotion, ItemSprite::GreenPotion,
            ItemSprite::RedBook, ItemSprite::DarkTome, ItemSprite::Tome2,
            ItemSprite::Bread, ItemSprite::Apple, ItemSprite::BottleOfBeer,
            ItemSprite::BottleOfWater,
        ];
        for sprite in all {
            let r = sprite.sprite_ref();
            assert_eq!(r.sheet, Sheet::Items);
            assert!(r.row < 26, "{:?} row {} >= 26", sprite, r.row);
            assert!(r.col < 11, "{:?} col {} >= 11", sprite, r.col);
        }
    }

    #[test]
    fn flame_sword_position() {
        let r = ItemSprite::FlameSword.sprite_ref();
        assert_eq!(r.row, 0);
        assert_eq!(r.col, 10);
    }
}
