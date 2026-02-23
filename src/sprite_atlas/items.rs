/// Sprites from `items.png` (11 cols x 26 rows, 32x32 cells).
/// Positions from `32rogues/items.txt`.
use super::{Sheet, SpriteRef};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemSprite {
    // === Swords (rows 1-2) ===
    Dagger,
    ShortSword,
    ShortSword2,
    LongSword,
    BastardSword,
    Zweihander,
    MagicDagger,
    CrystalSword,
    EvilSword,
    FlameSword,

    // === Wide swords / rapiers (rows 2-3) ===
    WideShortSword,
    WideLongSword,
    Rapier,
    LongRapier,
    Flamberge,
    GreatSword,

    // === Curved swords (row 3) ===
    Scimitar,
    LargeScimitar,
    GreatScimitar,
    Kukri,

    // === Axes (row 4) ===
    HandAxe,
    BattleAxe,
    Halberd,
    GreatAxe,
    GiantAxe,
    Hatchet,
    WoodcuttersAxe,

    // === Hammers (row 5) ===
    BlacksmithHammer,
    ShortWarhammer,
    LongWarhammer,
    Hammer,
    GreatHammer,

    // === Maces (row 6) ===
    Mace1,
    Mace2,
    GreatMace,
    SpikedBat,

    // === Spears (row 7) ===
    Spear,
    ShortSpear,
    Pitchfork,
    Trident,
    MagicSpear,

    // === Flails (row 8) ===
    Flail1,
    Flail2,
    Flail3,

    // === Clubs (row 9) ===
    Club,
    SpikedClub,
    GreatClub,

    // === Ranged weapons (row 10) ===
    Crossbow,
    ShortBow,
    LongBow,
    LongBow2,
    LargeCrossbow,

    // === Staves (row 11) ===
    CrystalStaff,
    HolyStaff,
    DruidStaff,
    BlueStaff,
    GoldenStaff,
    RedCrystalStaff,
    FlameStaff,

    // === Shields (row 12) ===
    Buckler,
    KiteShield,
    CrossShield,
    DarkShield,
    RoundShield,
    LargeShield,

    // === Body armor (row 13) ===
    ClothArmor,
    LeatherArmor,
    Robe,
    ChainMail,
    ScaleMail,
    ChestPlate,

    // === Gloves (row 14) ===
    ClothGloves,
    LeatherGloves,
    Gauntlets,

    // === Boots (row 15) ===
    Shoes,
    LeatherBoots,
    HighBlueBoots,
    Greaves,

    // === Helmets (row 16) ===
    ClothHood,
    LeatherHelm,
    WideBrimmedHat,
    ChainMailCoif,
    Helm,
    HelmChainMail,
    PlateHelm1,
    PlateHelm2,

    // === Pendants (row 17) ===
    RedPendant,
    MetalPendant,
    CrystalPendant,

    // === Rings (rows 18-19) ===
    GoldEmeraldRing,
    GoldBandRing,
    GreenSignetRing,
    RubyRing,
    SapphireRing,
    OnyxRing,
    GoldSignetRing,
    SilverSignetRing,
    JadeRing,
    TwistedGoldRing,
    TwistedMetalRing,

    // === Potions (rows 20-21) ===
    PurplePotion,
    RedPotion,
    BrownVial,
    LargeDarkPotion,
    GreenPotion,
    BlackPotion,
    BrightGreenPotion,
    PinkVial,
    BluePotion,
    OrangePotion,

    // === Scrolls & books (row 22) ===
    Scroll,
    Book,
    RedBook,
    DarkTome,
    Tome,
    Tome2,
    Scroll2,

    // === Keys (row 23) ===
    GoldKey,
    OrnateKey,
    MetalKey,

    // === Ammo (row 24) ===
    Arrow,
    Arrows,
    Bolt,
    Bolts,

    // === Coins (row 25) ===
    Coin,
    SmallCoins,
    LargeCoins,
    CoinPurse,

    // === Food & drink (row 26) ===
    Cheese,
    Bread,
    Apple,
    BottleOfBeer,
    BottleOfWater,
}

impl ItemSprite {
    pub const fn sprite_ref(self) -> SpriteRef {
        let (row, col) = match self {
            // Swords
            Self::Dagger => (0, 0),
            Self::ShortSword => (0, 1),
            Self::ShortSword2 => (0, 2),
            Self::LongSword => (0, 3),
            Self::BastardSword => (0, 4),
            Self::Zweihander => (0, 5),
            Self::MagicDagger => (0, 7),
            Self::CrystalSword => (0, 8),
            Self::EvilSword => (0, 9),
            Self::FlameSword => (0, 10),

            // Wide swords / rapiers
            Self::WideShortSword => (1, 0),
            Self::WideLongSword => (1, 1),
            Self::Rapier => (1, 2),
            Self::LongRapier => (1, 3),
            Self::Flamberge => (1, 4),
            Self::GreatSword => (1, 6),

            // Curved swords
            Self::Scimitar => (2, 1),
            Self::LargeScimitar => (2, 2),
            Self::GreatScimitar => (2, 3),
            Self::Kukri => (2, 4),

            // Axes
            Self::HandAxe => (3, 0),
            Self::BattleAxe => (3, 1),
            Self::Halberd => (3, 2),
            Self::GreatAxe => (3, 3),
            Self::GiantAxe => (3, 4),
            Self::Hatchet => (3, 5),
            Self::WoodcuttersAxe => (3, 6),

            // Hammers
            Self::BlacksmithHammer => (4, 0),
            Self::ShortWarhammer => (4, 1),
            Self::LongWarhammer => (4, 2),
            Self::Hammer => (4, 3),
            Self::GreatHammer => (4, 4),

            // Maces
            Self::Mace1 => (5, 0),
            Self::Mace2 => (5, 1),
            Self::GreatMace => (5, 2),
            Self::SpikedBat => (5, 3),

            // Spears
            Self::Spear => (6, 0),
            Self::ShortSpear => (6, 1),
            Self::Pitchfork => (6, 2),
            Self::Trident => (6, 3),
            Self::MagicSpear => (6, 4),

            // Flails
            Self::Flail1 => (7, 0),
            Self::Flail2 => (7, 1),
            Self::Flail3 => (7, 2),

            // Clubs
            Self::Club => (8, 0),
            Self::SpikedClub => (8, 1),
            Self::GreatClub => (8, 2),

            // Ranged weapons
            Self::Crossbow => (9, 0),
            Self::ShortBow => (9, 1),
            Self::LongBow => (9, 2),
            Self::LongBow2 => (9, 3),
            Self::LargeCrossbow => (9, 4),

            // Staves
            Self::CrystalStaff => (10, 0),
            Self::HolyStaff => (10, 1),
            Self::DruidStaff => (10, 2),
            Self::BlueStaff => (10, 3),
            Self::GoldenStaff => (10, 4),
            Self::RedCrystalStaff => (10, 5),
            Self::FlameStaff => (10, 6),

            // Shields
            Self::Buckler => (11, 0),
            Self::KiteShield => (11, 1),
            Self::CrossShield => (11, 2),
            Self::DarkShield => (11, 3),
            Self::RoundShield => (11, 4),
            Self::LargeShield => (11, 6),

            // Body armor
            Self::ClothArmor => (12, 0),
            Self::LeatherArmor => (12, 1),
            Self::Robe => (12, 2),
            Self::ChainMail => (12, 3),
            Self::ScaleMail => (12, 4),
            Self::ChestPlate => (12, 5),

            // Gloves
            Self::ClothGloves => (13, 0),
            Self::LeatherGloves => (13, 1),
            Self::Gauntlets => (13, 3),

            // Boots
            Self::Shoes => (14, 0),
            Self::LeatherBoots => (14, 1),
            Self::HighBlueBoots => (14, 2),
            Self::Greaves => (14, 3),

            // Helmets
            Self::ClothHood => (15, 0),
            Self::LeatherHelm => (15, 1),
            Self::WideBrimmedHat => (15, 2),
            Self::ChainMailCoif => (15, 3),
            Self::Helm => (15, 4),
            Self::HelmChainMail => (15, 5),
            Self::PlateHelm1 => (15, 6),
            Self::PlateHelm2 => (15, 7),

            // Pendants
            Self::RedPendant => (16, 0),
            Self::MetalPendant => (16, 1),
            Self::CrystalPendant => (16, 2),

            // Rings
            Self::GoldEmeraldRing => (17, 0),
            Self::GoldBandRing => (17, 1),
            Self::GreenSignetRing => (17, 2),
            Self::RubyRing => (17, 3),
            Self::SapphireRing => (17, 4),
            Self::OnyxRing => (17, 5),
            Self::GoldSignetRing => (18, 0),
            Self::SilverSignetRing => (18, 1),
            Self::JadeRing => (18, 2),
            Self::TwistedGoldRing => (18, 4),
            Self::TwistedMetalRing => (18, 5),

            // Potions
            Self::PurplePotion => (19, 0),
            Self::RedPotion => (19, 1),
            Self::BrownVial => (19, 2),
            Self::LargeDarkPotion => (19, 3),
            Self::GreenPotion => (19, 4),
            Self::BlackPotion => (20, 0),
            Self::BrightGreenPotion => (20, 1),
            Self::PinkVial => (20, 2),
            Self::BluePotion => (20, 3),
            Self::OrangePotion => (20, 4),

            // Scrolls & books
            Self::Scroll => (21, 0),
            Self::Book => (21, 1),
            Self::RedBook => (21, 2),
            Self::DarkTome => (21, 3),
            Self::Tome => (21, 4),
            Self::Tome2 => (21, 5),
            Self::Scroll2 => (21, 6),

            // Keys
            Self::GoldKey => (22, 0),
            Self::OrnateKey => (22, 1),
            Self::MetalKey => (22, 2),

            // Ammo
            Self::Arrow => (23, 0),
            Self::Arrows => (23, 1),
            Self::Bolt => (23, 2),
            Self::Bolts => (23, 3),

            // Coins
            Self::Coin => (24, 0),
            Self::SmallCoins => (24, 1),
            Self::LargeCoins => (24, 2),
            Self::CoinPurse => (24, 3),

            // Food & drink
            Self::Cheese => (25, 0),
            Self::Bread => (25, 1),
            Self::Apple => (25, 2),
            Self::BottleOfBeer => (25, 3),
            Self::BottleOfWater => (25, 4),
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
            ItemSprite::Dagger, ItemSprite::ShortSword, ItemSprite::ShortSword2, ItemSprite::LongSword,
            ItemSprite::BastardSword, ItemSprite::Zweihander, ItemSprite::MagicDagger,
            ItemSprite::CrystalSword, ItemSprite::EvilSword, ItemSprite::FlameSword,
            ItemSprite::WideShortSword, ItemSprite::WideLongSword, ItemSprite::Rapier, ItemSprite::LongRapier,
            ItemSprite::Flamberge, ItemSprite::GreatSword,
            ItemSprite::Scimitar, ItemSprite::LargeScimitar, ItemSprite::GreatScimitar, ItemSprite::Kukri,
            ItemSprite::HandAxe, ItemSprite::BattleAxe, ItemSprite::Halberd, ItemSprite::GreatAxe,
            ItemSprite::GiantAxe, ItemSprite::Hatchet, ItemSprite::WoodcuttersAxe,
            ItemSprite::BlacksmithHammer, ItemSprite::ShortWarhammer, ItemSprite::LongWarhammer,
            ItemSprite::Hammer, ItemSprite::GreatHammer,
            ItemSprite::Mace1, ItemSprite::Mace2, ItemSprite::GreatMace, ItemSprite::SpikedBat,
            ItemSprite::Spear, ItemSprite::ShortSpear, ItemSprite::Pitchfork, ItemSprite::Trident, ItemSprite::MagicSpear,
            ItemSprite::Flail1, ItemSprite::Flail2, ItemSprite::Flail3,
            ItemSprite::Club, ItemSprite::SpikedClub, ItemSprite::GreatClub,
            ItemSprite::Crossbow, ItemSprite::ShortBow, ItemSprite::LongBow, ItemSprite::LongBow2, ItemSprite::LargeCrossbow,
            ItemSprite::CrystalStaff, ItemSprite::HolyStaff, ItemSprite::DruidStaff, ItemSprite::BlueStaff,
            ItemSprite::GoldenStaff, ItemSprite::RedCrystalStaff, ItemSprite::FlameStaff,
            ItemSprite::Buckler, ItemSprite::KiteShield, ItemSprite::CrossShield, ItemSprite::DarkShield,
            ItemSprite::RoundShield, ItemSprite::LargeShield,
            ItemSprite::ClothArmor, ItemSprite::LeatherArmor, ItemSprite::Robe, ItemSprite::ChainMail,
            ItemSprite::ScaleMail, ItemSprite::ChestPlate,
            ItemSprite::ClothGloves, ItemSprite::LeatherGloves, ItemSprite::Gauntlets,
            ItemSprite::Shoes, ItemSprite::LeatherBoots, ItemSprite::HighBlueBoots, ItemSprite::Greaves,
            ItemSprite::ClothHood, ItemSprite::LeatherHelm, ItemSprite::WideBrimmedHat, ItemSprite::ChainMailCoif,
            ItemSprite::Helm, ItemSprite::HelmChainMail, ItemSprite::PlateHelm1, ItemSprite::PlateHelm2,
            ItemSprite::RedPendant, ItemSprite::MetalPendant, ItemSprite::CrystalPendant,
            ItemSprite::GoldEmeraldRing, ItemSprite::GoldBandRing, ItemSprite::GreenSignetRing,
            ItemSprite::RubyRing, ItemSprite::SapphireRing, ItemSprite::OnyxRing,
            ItemSprite::GoldSignetRing, ItemSprite::SilverSignetRing, ItemSprite::JadeRing,
            ItemSprite::TwistedGoldRing, ItemSprite::TwistedMetalRing,
            ItemSprite::PurplePotion, ItemSprite::RedPotion, ItemSprite::BrownVial,
            ItemSprite::LargeDarkPotion, ItemSprite::GreenPotion,
            ItemSprite::BlackPotion, ItemSprite::BrightGreenPotion, ItemSprite::PinkVial,
            ItemSprite::BluePotion, ItemSprite::OrangePotion,
            ItemSprite::Scroll, ItemSprite::Book, ItemSprite::RedBook, ItemSprite::DarkTome,
            ItemSprite::Tome, ItemSprite::Tome2, ItemSprite::Scroll2,
            ItemSprite::GoldKey, ItemSprite::OrnateKey, ItemSprite::MetalKey,
            ItemSprite::Arrow, ItemSprite::Arrows, ItemSprite::Bolt, ItemSprite::Bolts,
            ItemSprite::Coin, ItemSprite::SmallCoins, ItemSprite::LargeCoins, ItemSprite::CoinPurse,
            ItemSprite::Cheese, ItemSprite::Bread, ItemSprite::Apple, ItemSprite::BottleOfBeer, ItemSprite::BottleOfWater,
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
