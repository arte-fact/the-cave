/// Every named sprite in `items.png` (11 cols x 26 rows, 32x32 cells).
/// Positions from `32rogues/items.txt`.
use super::{Sheet, SpriteRef};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ItemSprite {
    // Row 0 (1) — swords
    Dagger,
    ShortSword,
    ShortSword2,
    LongSword,
    BastardSword,
    Zweihander,
    SanguineDagger,
    MagicDagger,
    CrystalSword,
    EvilSword,
    FlameSword,

    // Row 1 (2) — wide swords and rapiers
    WideShortSword,
    WideLongSword,
    Rapier,
    LongRapier,
    Flamberge,
    LargeFlamberge,
    GreatSword,

    // Row 2 (3) — curved blades
    Shotel,
    Scimitar,
    LargeScimitar,
    GreatScimitar,
    Kukri,

    // Row 3 (4) — axes
    HandAxe,
    BattleAxe,
    Halberd,
    GreatAxe,
    GiantAxe,
    Hatchet,
    WoodcuttersAxe,

    // Row 4 (5) — hammers
    BlacksmithsHammer,
    ShortWarhammer,
    LongWarhammer,
    Hammer,
    GreatHammer,

    // Row 5 (6) — maces
    Mace1,
    Mace2,
    GreatMace,
    SpikedBat,

    // Row 6 (7) — spears
    Spear,
    ShortSpear,
    Pitchfork,
    Trident,
    MagicSpear,

    // Row 7 (8) — flails
    Flail1,
    Flail2,
    Flail3,

    // Row 8 (9) — clubs
    Club,
    SpikedClub,
    GreatClub,
    ClubWithNails,

    // Row 9 (10) — ranged weapons
    Crossbow,
    ShortBow,
    LongBow,
    LongBow2,
    LargeCrossbow,

    // Row 10 (11) — staves
    CrystalStaff,
    HolyStaff,
    DruidStaff,
    BlueStaff,
    GoldenStaff,
    RedCrystalStaff,
    FlameStaff,
    BlueCrystalStaff,
    CrossStaff,
    SaintsStaff,

    // Row 11 (12) — shields
    Buckler,
    KiteShield,
    CrossShield,
    DarkShield,
    RoundShield,
    Buckler2,
    LargeShield,

    // Row 12 (13) — body armor
    ClothArmor,
    LeatherArmor,
    Robe,
    ChainMail,
    ScaleMail,
    ChestPlate,

    // Row 13 (14) — gloves
    ClothGloves,
    LeatherGloves,
    BlueClothGloves,
    Gauntlets,

    // Row 14 (15) — boots
    Shoes,
    LeatherBoots,
    HighBlueBoots,
    Greaves,

    // Row 15 (16) — helmets
    ClothHood,
    LeatherHelm,
    WideBrimmedHat,
    ChainMailCoif,
    Helm,
    HelmWithChainMail,
    PlateHelm1,
    PlateHelm2,

    // Row 16 (17) — pendants
    RedPendant,
    MetalPendant,
    CrystalPendant,
    DiscPendant,
    CrossPendant,
    StonePendant,
    Ankh,

    // Row 17 (18) — rings set 1
    GoldEmeraldRing,
    GoldBandRing,
    GreenSignetRing,
    RubyRing,
    SapphireRing,
    OnyxRing,

    // Row 18 (19) — rings set 2
    GoldSignetRing,
    SilverSignetRing,
    JadeRing,
    SilverSignetRing2,
    TwistedGoldRing,
    TwistedMetalRing,

    // Row 19 (20) — potions set 1
    PurplePotion,
    RedPotion,
    BrownVial,
    LargeDarkPotion,
    GreenPotion,

    // Row 20 (21) — potions set 2
    BlackPotion,
    BrightGreenPotion,
    PinkVial,
    BluePotion,
    OrangePotion,

    // Row 21 (22) — scrolls and books
    Scroll,
    Book,
    RedBook,
    DarkTome,
    Tome,
    Tome2,
    Scroll2,
    Page,

    // Row 22 (23) — keys
    GoldKey,
    OrnateKey,
    MetalKey,
    PrimitiveKey,

    // Row 23 (24) — ammunition
    Arrow,
    Arrows,
    Bolt,
    Bolts,

    // Row 24 (25) — coins
    Coin,
    SmallStacksOfCoins,
    LargeStacksOfCoins,
    CoinPurse,

    // Row 25 (26) — food and drink
    Cheese,
    Bread,
    Apple,
    BottleOfBeer,
    /// Listed as 25.e in items.txt (likely a typo for 26.e).
    BottleOfWater,
}

impl ItemSprite {
    pub const fn sprite_ref(self) -> SpriteRef {
        let (row, col) = match self {
            Self::Dagger => (0, 0),
            Self::ShortSword => (0, 1),
            Self::ShortSword2 => (0, 2),
            Self::LongSword => (0, 3),
            Self::BastardSword => (0, 4),
            Self::Zweihander => (0, 5),
            Self::SanguineDagger => (0, 6),
            Self::MagicDagger => (0, 7),
            Self::CrystalSword => (0, 8),
            Self::EvilSword => (0, 9),
            Self::FlameSword => (0, 10),

            Self::WideShortSword => (1, 0),
            Self::WideLongSword => (1, 1),
            Self::Rapier => (1, 2),
            Self::LongRapier => (1, 3),
            Self::Flamberge => (1, 4),
            Self::LargeFlamberge => (1, 5),
            Self::GreatSword => (1, 6),

            Self::Shotel => (2, 0),
            Self::Scimitar => (2, 1),
            Self::LargeScimitar => (2, 2),
            Self::GreatScimitar => (2, 3),
            Self::Kukri => (2, 4),

            Self::HandAxe => (3, 0),
            Self::BattleAxe => (3, 1),
            Self::Halberd => (3, 2),
            Self::GreatAxe => (3, 3),
            Self::GiantAxe => (3, 4),
            Self::Hatchet => (3, 5),
            Self::WoodcuttersAxe => (3, 6),

            Self::BlacksmithsHammer => (4, 0),
            Self::ShortWarhammer => (4, 1),
            Self::LongWarhammer => (4, 2),
            Self::Hammer => (4, 3),
            Self::GreatHammer => (4, 4),

            Self::Mace1 => (5, 0),
            Self::Mace2 => (5, 1),
            Self::GreatMace => (5, 2),
            Self::SpikedBat => (5, 3),

            Self::Spear => (6, 0),
            Self::ShortSpear => (6, 1),
            Self::Pitchfork => (6, 2),
            Self::Trident => (6, 3),
            Self::MagicSpear => (6, 4),

            Self::Flail1 => (7, 0),
            Self::Flail2 => (7, 1),
            Self::Flail3 => (7, 2),

            Self::Club => (8, 0),
            Self::SpikedClub => (8, 1),
            Self::GreatClub => (8, 2),
            Self::ClubWithNails => (8, 3),

            Self::Crossbow => (9, 0),
            Self::ShortBow => (9, 1),
            Self::LongBow => (9, 2),
            Self::LongBow2 => (9, 3),
            Self::LargeCrossbow => (9, 4),

            Self::CrystalStaff => (10, 0),
            Self::HolyStaff => (10, 1),
            Self::DruidStaff => (10, 2),
            Self::BlueStaff => (10, 3),
            Self::GoldenStaff => (10, 4),
            Self::RedCrystalStaff => (10, 5),
            Self::FlameStaff => (10, 6),
            Self::BlueCrystalStaff => (10, 7),
            Self::CrossStaff => (10, 8),
            Self::SaintsStaff => (10, 9),

            Self::Buckler => (11, 0),
            Self::KiteShield => (11, 1),
            Self::CrossShield => (11, 2),
            Self::DarkShield => (11, 3),
            Self::RoundShield => (11, 4),
            Self::Buckler2 => (11, 5),
            Self::LargeShield => (11, 6),

            Self::ClothArmor => (12, 0),
            Self::LeatherArmor => (12, 1),
            Self::Robe => (12, 2),
            Self::ChainMail => (12, 3),
            Self::ScaleMail => (12, 4),
            Self::ChestPlate => (12, 5),

            Self::ClothGloves => (13, 0),
            Self::LeatherGloves => (13, 1),
            Self::BlueClothGloves => (13, 2),
            Self::Gauntlets => (13, 3),

            Self::Shoes => (14, 0),
            Self::LeatherBoots => (14, 1),
            Self::HighBlueBoots => (14, 2),
            Self::Greaves => (14, 3),

            Self::ClothHood => (15, 0),
            Self::LeatherHelm => (15, 1),
            Self::WideBrimmedHat => (15, 2),
            Self::ChainMailCoif => (15, 3),
            Self::Helm => (15, 4),
            Self::HelmWithChainMail => (15, 5),
            Self::PlateHelm1 => (15, 6),
            Self::PlateHelm2 => (15, 7),

            Self::RedPendant => (16, 0),
            Self::MetalPendant => (16, 1),
            Self::CrystalPendant => (16, 2),
            Self::DiscPendant => (16, 3),
            Self::CrossPendant => (16, 4),
            Self::StonePendant => (16, 5),
            Self::Ankh => (16, 6),

            Self::GoldEmeraldRing => (17, 0),
            Self::GoldBandRing => (17, 1),
            Self::GreenSignetRing => (17, 2),
            Self::RubyRing => (17, 3),
            Self::SapphireRing => (17, 4),
            Self::OnyxRing => (17, 5),

            Self::GoldSignetRing => (18, 0),
            Self::SilverSignetRing => (18, 1),
            Self::JadeRing => (18, 2),
            Self::SilverSignetRing2 => (18, 3),
            Self::TwistedGoldRing => (18, 4),
            Self::TwistedMetalRing => (18, 5),

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

            Self::Scroll => (21, 0),
            Self::Book => (21, 1),
            Self::RedBook => (21, 2),
            Self::DarkTome => (21, 3),
            Self::Tome => (21, 4),
            Self::Tome2 => (21, 5),
            Self::Scroll2 => (21, 6),
            Self::Page => (21, 7),

            Self::GoldKey => (22, 0),
            Self::OrnateKey => (22, 1),
            Self::MetalKey => (22, 2),
            Self::PrimitiveKey => (22, 3),

            Self::Arrow => (23, 0),
            Self::Arrows => (23, 1),
            Self::Bolt => (23, 2),
            Self::Bolts => (23, 3),

            Self::Coin => (24, 0),
            Self::SmallStacksOfCoins => (24, 1),
            Self::LargeStacksOfCoins => (24, 2),
            Self::CoinPurse => (24, 3),

            Self::Cheese => (25, 0),
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
        // items.png: 11 cols x 26 rows
        let all = [
            ItemSprite::Dagger, ItemSprite::ShortSword, ItemSprite::ShortSword2,
            ItemSprite::LongSword, ItemSprite::BastardSword, ItemSprite::Zweihander,
            ItemSprite::SanguineDagger, ItemSprite::MagicDagger, ItemSprite::CrystalSword,
            ItemSprite::EvilSword, ItemSprite::FlameSword,
            ItemSprite::WideShortSword, ItemSprite::WideLongSword, ItemSprite::Rapier,
            ItemSprite::LongRapier, ItemSprite::Flamberge, ItemSprite::LargeFlamberge, ItemSprite::GreatSword,
            ItemSprite::Shotel, ItemSprite::Scimitar, ItemSprite::LargeScimitar,
            ItemSprite::GreatScimitar, ItemSprite::Kukri,
            ItemSprite::HandAxe, ItemSprite::BattleAxe, ItemSprite::Halberd,
            ItemSprite::GreatAxe, ItemSprite::GiantAxe, ItemSprite::Hatchet, ItemSprite::WoodcuttersAxe,
            ItemSprite::BlacksmithsHammer, ItemSprite::ShortWarhammer, ItemSprite::LongWarhammer,
            ItemSprite::Hammer, ItemSprite::GreatHammer,
            ItemSprite::Mace1, ItemSprite::Mace2, ItemSprite::GreatMace, ItemSprite::SpikedBat,
            ItemSprite::Spear, ItemSprite::ShortSpear, ItemSprite::Pitchfork,
            ItemSprite::Trident, ItemSprite::MagicSpear,
            ItemSprite::Flail1, ItemSprite::Flail2, ItemSprite::Flail3,
            ItemSprite::Club, ItemSprite::SpikedClub, ItemSprite::GreatClub, ItemSprite::ClubWithNails,
            ItemSprite::Crossbow, ItemSprite::ShortBow, ItemSprite::LongBow,
            ItemSprite::LongBow2, ItemSprite::LargeCrossbow,
            ItemSprite::CrystalStaff, ItemSprite::HolyStaff, ItemSprite::DruidStaff,
            ItemSprite::BlueStaff, ItemSprite::GoldenStaff, ItemSprite::RedCrystalStaff,
            ItemSprite::FlameStaff, ItemSprite::BlueCrystalStaff, ItemSprite::CrossStaff, ItemSprite::SaintsStaff,
            ItemSprite::Buckler, ItemSprite::KiteShield, ItemSprite::CrossShield,
            ItemSprite::DarkShield, ItemSprite::RoundShield, ItemSprite::Buckler2, ItemSprite::LargeShield,
            ItemSprite::ClothArmor, ItemSprite::LeatherArmor, ItemSprite::Robe,
            ItemSprite::ChainMail, ItemSprite::ScaleMail, ItemSprite::ChestPlate,
            ItemSprite::ClothGloves, ItemSprite::LeatherGloves, ItemSprite::BlueClothGloves, ItemSprite::Gauntlets,
            ItemSprite::Shoes, ItemSprite::LeatherBoots, ItemSprite::HighBlueBoots, ItemSprite::Greaves,
            ItemSprite::ClothHood, ItemSprite::LeatherHelm, ItemSprite::WideBrimmedHat,
            ItemSprite::ChainMailCoif, ItemSprite::Helm, ItemSprite::HelmWithChainMail,
            ItemSprite::PlateHelm1, ItemSprite::PlateHelm2,
            ItemSprite::RedPendant, ItemSprite::MetalPendant, ItemSprite::CrystalPendant,
            ItemSprite::DiscPendant, ItemSprite::CrossPendant, ItemSprite::StonePendant, ItemSprite::Ankh,
            ItemSprite::GoldEmeraldRing, ItemSprite::GoldBandRing, ItemSprite::GreenSignetRing,
            ItemSprite::RubyRing, ItemSprite::SapphireRing, ItemSprite::OnyxRing,
            ItemSprite::GoldSignetRing, ItemSprite::SilverSignetRing, ItemSprite::JadeRing,
            ItemSprite::SilverSignetRing2, ItemSprite::TwistedGoldRing, ItemSprite::TwistedMetalRing,
            ItemSprite::PurplePotion, ItemSprite::RedPotion, ItemSprite::BrownVial,
            ItemSprite::LargeDarkPotion, ItemSprite::GreenPotion,
            ItemSprite::BlackPotion, ItemSprite::BrightGreenPotion, ItemSprite::PinkVial,
            ItemSprite::BluePotion, ItemSprite::OrangePotion,
            ItemSprite::Scroll, ItemSprite::Book, ItemSprite::RedBook, ItemSprite::DarkTome,
            ItemSprite::Tome, ItemSprite::Tome2, ItemSprite::Scroll2, ItemSprite::Page,
            ItemSprite::GoldKey, ItemSprite::OrnateKey, ItemSprite::MetalKey, ItemSprite::PrimitiveKey,
            ItemSprite::Arrow, ItemSprite::Arrows, ItemSprite::Bolt, ItemSprite::Bolts,
            ItemSprite::Coin, ItemSprite::SmallStacksOfCoins, ItemSprite::LargeStacksOfCoins, ItemSprite::CoinPurse,
            ItemSprite::Cheese, ItemSprite::Bread, ItemSprite::Apple, ItemSprite::BottleOfBeer,
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

    #[test]
    fn potion_rows() {
        assert_eq!(ItemSprite::PurplePotion.sprite_ref().row, 19);
        assert_eq!(ItemSprite::BlackPotion.sprite_ref().row, 20);
    }
}
