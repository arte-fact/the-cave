/// Game-specific sprite mappings — maps game entities to sprite sheet positions.
/// Pure logic, no web_sys dependency.
///
/// The canonical sprite catalog lives in `crate::sprite_atlas`. This module
/// re-exports `Sheet` and `SpriteRef` for convenience, and provides the
/// game-specific mapping functions (`tile_sprite`, `player_sprite`, etc.).
pub use crate::sprite_atlas::{Sheet, SpriteRef};

use crate::map::{DungeonStyle, Tile};
use crate::sprite_atlas::animals::AnimalSprite;
use crate::sprite_atlas::items::ItemSprite;
use crate::sprite_atlas::monsters::MonsterSprite;
use crate::sprite_atlas::rogues::RogueSprite;
use crate::sprite_atlas::tiles::TileSprite;

// === Floor variant arrays per dungeon style ===

const FLOOR_STONE: [TileSprite; 3] = [
    TileSprite::FloorStone1,
    TileSprite::FloorStone2,
    TileSprite::FloorStone3,
];

const FLOOR_STONE_ALT: [TileSprite; 3] = [
    TileSprite::StoneFloor1,
    TileSprite::StoneFloor2,
    TileSprite::StoneFloor3,
];

const FLOOR_BONE: [TileSprite; 3] = [
    TileSprite::BoneFloor1,
    TileSprite::BoneFloor2,
    TileSprite::BoneFloor3,
];

const FLOOR_RED: [TileSprite; 3] = [
    TileSprite::RedStoneFloor1,
    TileSprite::RedStoneFloor2,
    TileSprite::RedStoneFloor3,
];

const FLOOR_BLUE: [TileSprite; 3] = [
    TileSprite::BlueStoneFloor1,
    TileSprite::BlueStoneFloor2,
    TileSprite::BlueStoneFloor3,
];

const FLOOR_GREEN_DIRT: [TileSprite; 3] = [
    TileSprite::GreenDirt1,
    TileSprite::GreenDirt2,
    TileSprite::GreenDirt3,
];

const FLOOR_DARK_BONES: [TileSprite; 3] = [
    TileSprite::DarkBrownBones1,
    TileSprite::DarkBrownBones2,
    TileSprite::DarkBrownBones3,
];

const GRASS_VARIANTS: [TileSprite; 3] = [
    TileSprite::Grass1,
    TileSprite::Grass2,
    TileSprite::Grass3,
];

const ROAD_VARIANTS: [TileSprite; 3] = [
    TileSprite::Dirt1,
    TileSprite::Dirt2,
    TileSprite::Dirt3,
];

/// Pick a tile sprite. `x, y` are world coordinates used for variation hashing.
/// `wall_face` should be true when the wall has a non-wall tile directly below
/// (so we show the "side/face" variant instead of the "top" variant).
/// `dungeon_style` determines which wall/floor theme to use (None = overworld).
pub fn tile_sprite(tile: Tile, x: i32, y: i32, wall_face: bool, dungeon_style: Option<DungeonStyle>) -> SpriteRef {
    let variation = (x.wrapping_mul(31).wrapping_add(y.wrapping_mul(17))).unsigned_abs() % 3;
    match tile {
        Tile::Wall => {
            let (top, side) = match dungeon_style {
                Some(DungeonStyle::DirtCaves) | Some(DungeonStyle::MossyTunnel) =>
                    (TileSprite::DirtWallTop, TileSprite::DirtWallSide),
                Some(DungeonStyle::StoneBrick) | None =>
                    (TileSprite::StoneBrickWallTop, TileSprite::StoneBrickWallSide1),
                Some(DungeonStyle::Igneous) | Some(DungeonStyle::RedCavern) | Some(DungeonStyle::BlueTemple) =>
                    (TileSprite::IgneousWallTop, TileSprite::IgneousWallSide),
                Some(DungeonStyle::LargeStone) =>
                    (TileSprite::LargeStoneWallTop, TileSprite::LargeStoneWallSide),
                Some(DungeonStyle::Catacombs) | Some(DungeonStyle::BoneCrypt) =>
                    (TileSprite::CatacombsWallTop, TileSprite::CatacombsWallSide),
                Some(DungeonStyle::MossyCavern) | Some(DungeonStyle::BoneCave) =>
                    (TileSprite::RoughStoneWallTop, TileSprite::RoughStoneWallSide),
            };
            if wall_face { side.sprite_ref() } else { top.sprite_ref() }
        }
        Tile::Floor => {
            let variants = match dungeon_style {
                Some(DungeonStyle::DirtCaves) => &FLOOR_STONE,
                Some(DungeonStyle::StoneBrick) | None => &FLOOR_STONE,
                Some(DungeonStyle::Igneous) => &FLOOR_STONE_ALT,
                Some(DungeonStyle::LargeStone) => &FLOOR_STONE_ALT,
                Some(DungeonStyle::Catacombs) => &FLOOR_BONE,
                Some(DungeonStyle::RedCavern) => &FLOOR_RED,
                Some(DungeonStyle::MossyCavern) | Some(DungeonStyle::MossyTunnel) => &FLOOR_GREEN_DIRT,
                Some(DungeonStyle::BoneCave) | Some(DungeonStyle::BoneCrypt) => &FLOOR_DARK_BONES,
                Some(DungeonStyle::BlueTemple) => &FLOOR_BLUE,
            };
            variants[variation as usize].sprite_ref()
        }
        Tile::Grass => GRASS_VARIANTS[variation as usize].sprite_ref(),
        Tile::Road => ROAD_VARIANTS[variation as usize].sprite_ref(),
        Tile::Tree => TileSprite::Tree.sprite_ref(),
        Tile::DungeonEntrance => TileSprite::Door1.sprite_ref(),
        Tile::StairsDown => TileSprite::StaircaseDown.sprite_ref(),
        Tile::StairsUp => TileSprite::StaircaseUp.sprite_ref(),
    }
}

/// Player sprite: rogue character.
pub fn player_sprite() -> SpriteRef {
    RogueSprite::Rogue.sprite_ref()
}

/// Enemy sprite based on glyph character.
pub fn enemy_sprite(glyph: char) -> SpriteRef {
    match glyph {
        // Forest beasts (animals sheet)
        'r' => MonsterSprite::GiantRat.sprite_ref(),
        'a' => MonsterSprite::GiantBat.sprite_ref(),
        'w' => AnimalSprite::Wolf.sprite_ref(),
        'i' => MonsterSprite::GiantSpider.sprite_ref(),
        'b' => AnimalSprite::Boar.sprite_ref(),
        'B' => AnimalSprite::GrizzlyBear.sprite_ref(),
        'L' => MonsterSprite::Lycanthrope.sprite_ref(),
        'f' => AnimalSprite::Fox.sprite_ref(),
        'n' => AnimalSprite::Cobra.sprite_ref(),
        'h' => AnimalSprite::Cougar.sprite_ref(),
        'j' => AnimalSprite::Badger.sprite_ref(),
        // Forest extra animals
        'q' => AnimalSprite::Buzzard.sprite_ref(),
        'v' => AnimalSprite::BlackMamba.sprite_ref(),
        'y' => AnimalSprite::Coyote.sprite_ref(),
        'x' => AnimalSprite::Hyena.sprite_ref(),
        'J' => AnimalSprite::Honeybadger.sprite_ref(),
        'Z' => AnimalSprite::Alligator.sprite_ref(),
        'F' => AnimalSprite::MaleLion.sprite_ref(),
        // Forest monsters
        '9' => MonsterSprite::Centaur.sprite_ref(),
        '0' => MonsterSprite::Wendigo.sprite_ref(),
        // Dungeon — shallow (L0)
        'c' => MonsterSprite::SmallKoboldCanine.sprite_ref(),
        'S' => MonsterSprite::SmallSlime.sprite_ref(),
        'g' => MonsterSprite::Goblin.sprite_ref(),
        's' => MonsterSprite::Skeleton.sprite_ref(),
        'e' => MonsterSprite::GiantCentipede.sprite_ref(),
        'p' => MonsterSprite::SmallMyconid.sprite_ref(),
        't' => MonsterSprite::LargeMyconid.sprite_ref(),
        '1' => MonsterSprite::Dryad.sprite_ref(),
        '2' => MonsterSprite::ForestSpirit.sprite_ref(),
        // Dungeon — mid (L1)
        'G' => MonsterSprite::GoblinArcher.sprite_ref(),
        'z' => MonsterSprite::Zombie.sprite_ref(),
        'k' => MonsterSprite::SkeletonArcher.sprite_ref(),
        'm' => MonsterSprite::BigSlime.sprite_ref(),
        'o' => MonsterSprite::Orc.sprite_ref(),
        'A' => MonsterSprite::GiantAnt.sprite_ref(),
        'M' => MonsterSprite::GoblinMage.sprite_ref(),
        'H' => MonsterSprite::HagWitch.sprite_ref(),
        '3' => MonsterSprite::GoblinBrute.sprite_ref(),
        '4' => MonsterSprite::Satyr.sprite_ref(),
        '5' => MonsterSprite::OrcWarchief.sprite_ref(),
        // Dungeon — deep (L2)
        'u' => MonsterSprite::Ghoul.sprite_ref(),
        'O' => MonsterSprite::OrcBlademaster.sprite_ref(),
        'W' => MonsterSprite::Wraith.sprite_ref(),
        'N' => MonsterSprite::Naga.sprite_ref(),
        'T' => MonsterSprite::Troll.sprite_ref(),
        'E' => MonsterSprite::Ettin.sprite_ref(),
        'R' => MonsterSprite::RockGolem.sprite_ref(),
        'Y' => MonsterSprite::Minotaur.sprite_ref(),
        'P' => MonsterSprite::GorgonMedusa.sprite_ref(),
        'Q' => MonsterSprite::Banshee.sprite_ref(),
        '6' => MonsterSprite::FacelessMonk.sprite_ref(),
        '7' => MonsterSprite::UnholyCardinal.sprite_ref(),
        '8' => MonsterSprite::LargeWrithingMass.sprite_ref(),
        // Cave — boss floor
        'K' => MonsterSprite::DeathKnight.sprite_ref(),
        'l' => MonsterSprite::Lich.sprite_ref(),
        'D' => MonsterSprite::Dragon.sprite_ref(),
        'd' => MonsterSprite::Drake.sprite_ref(),
        'C' => MonsterSprite::Basilisk.sprite_ref(),
        'I' => MonsterSprite::ImpDevil.sprite_ref(),
        'X' => MonsterSprite::Manticore.sprite_ref(),
        'V' => MonsterSprite::Reaper.sprite_ref(),
        // Biome-specific — previously unused sprites
        '$' => MonsterSprite::Harpy.sprite_ref(),
        '~' => MonsterSprite::Cockatrice.sprite_ref(),
        '>' => MonsterSprite::LizardfolkKobold.sprite_ref(),
        '{' => MonsterSprite::KoboldCanine.sprite_ref(),
        '}' => MonsterSprite::OrcWizard.sprite_ref(),
        '^' => MonsterSprite::TwoHeadedEttin.sprite_ref(),
        '[' => MonsterSprite::Lampreymander.sprite_ref(),
        ']' => MonsterSprite::GiantEarthworm.sprite_ref(),
        '<' => MonsterSprite::LesserGiantSpider.sprite_ref(),
        'U' => MonsterSprite::WargDireWolf.sprite_ref(),
        '!' => MonsterSprite::Cultist.sprite_ref(),
        '(' => MonsterSprite::SmallWrithingMass.sprite_ref(),
        ')' => MonsterSprite::WrithingHumanoid.sprite_ref(),
        _   => MonsterSprite::Goblin.sprite_ref(),
    }
}

/// Item sprite based on item name. Each tier gets a distinct sprite.
pub fn item_sprite(name: &str) -> SpriteRef {
    match name {
        // === Potions ===
        "Health Potion"          => ItemSprite::RedPotion.sprite_ref(),
        "Greater Health Potion"  => ItemSprite::LargeDarkPotion.sprite_ref(),
        "Superior Health Potion" => ItemSprite::GreenPotion.sprite_ref(),
        "Antidote"               => ItemSprite::BrightGreenPotion.sprite_ref(),
        "Stamina Potion"         => ItemSprite::BluePotion.sprite_ref(),
        "Elixir of Power"        => ItemSprite::OrangePotion.sprite_ref(),
        "Poison Vial"            => ItemSprite::PurplePotion.sprite_ref(),

        // === Scrolls ===
        "Scroll of Fire"      => ItemSprite::RedBook.sprite_ref(),
        "Scroll of Lightning" => ItemSprite::DarkTome.sprite_ref(),
        "Scroll of Storm"     => ItemSprite::Tome2.sprite_ref(),
        "Scroll of Ice"       => ItemSprite::Book.sprite_ref(),
        "Scroll of Wrath"     => ItemSprite::Tome.sprite_ref(),

        // === Melee weapons — Tier 0 (shallow/overworld) ===
        "Rusty Sword"     => ItemSprite::ShortSword.sprite_ref(),
        "Iron Dagger"     => ItemSprite::Dagger.sprite_ref(),
        "Wooden Club"     => ItemSprite::Club.sprite_ref(),
        "Hand Axe"        => ItemSprite::HandAxe.sprite_ref(),
        "Wooden Spear"    => ItemSprite::ShortSpear.sprite_ref(),
        "Kukri"           => ItemSprite::Kukri.sprite_ref(),

        // === Melee weapons — Tier 1 (mid) ===
        "Iron Sword"      => ItemSprite::LongSword.sprite_ref(),
        "Battle Axe"      => ItemSprite::BattleAxe.sprite_ref(),
        "War Hammer"      => ItemSprite::Hammer.sprite_ref(),
        "Scimitar"        => ItemSprite::Scimitar.sprite_ref(),
        "Mace"            => ItemSprite::Mace1.sprite_ref(),
        "Spear"           => ItemSprite::Spear.sprite_ref(),
        "Flail"           => ItemSprite::Flail1.sprite_ref(),
        "Rapier"          => ItemSprite::Rapier.sprite_ref(),
        "Spiked Club"     => ItemSprite::SpikedClub.sprite_ref(),

        // === Melee weapons — Tier 2 (deep) ===
        "Enchanted Blade" => ItemSprite::CrystalSword.sprite_ref(),
        "Crystal Staff"   => ItemSprite::CrystalStaff.sprite_ref(),
        "Flame Sword"     => ItemSprite::FlameSword.sprite_ref(),
        "Great Axe"       => ItemSprite::GreatAxe.sprite_ref(),
        "Great Hammer"    => ItemSprite::GreatHammer.sprite_ref(),
        "Trident"         => ItemSprite::Trident.sprite_ref(),
        "Bastard Sword"   => ItemSprite::BastardSword.sprite_ref(),
        "Evil Blade"      => ItemSprite::EvilSword.sprite_ref(),
        "Halberd"         => ItemSprite::Halberd.sprite_ref(),
        "Great Scimitar"  => ItemSprite::GreatScimitar.sprite_ref(),
        "Flamberge"       => ItemSprite::Flamberge.sprite_ref(),
        "Great Mace"      => ItemSprite::GreatMace.sprite_ref(),
        "Magic Spear"     => ItemSprite::MagicSpear.sprite_ref(),
        "Holy Staff"      => ItemSprite::HolyStaff.sprite_ref(),
        "Flame Staff"     => ItemSprite::FlameStaff.sprite_ref(),

        // === Ranged weapons ===
        "Short Bow"       => ItemSprite::ShortBow.sprite_ref(),
        "Crossbow"        => ItemSprite::Crossbow.sprite_ref(),
        "Long Bow"        => ItemSprite::LongBow.sprite_ref(),
        "Heavy Crossbow"  => ItemSprite::LargeCrossbow.sprite_ref(),
        "Elven Bow"       => ItemSprite::LongBow2.sprite_ref(),

        // === Body armor ===
        "Cloth Armor"    => ItemSprite::ClothArmor.sprite_ref(),
        "Leather Armor"  => ItemSprite::LeatherArmor.sprite_ref(),
        "Chain Mail"     => ItemSprite::ChainMail.sprite_ref(),
        "Scale Mail"     => ItemSprite::ScaleMail.sprite_ref(),
        "Dragon Scale"   => ItemSprite::ChestPlate.sprite_ref(),
        "Robe"           => ItemSprite::Robe.sprite_ref(),

        // === Shields ===
        "Wooden Shield"  => ItemSprite::Buckler.sprite_ref(),
        "Iron Shield"    => ItemSprite::KiteShield.sprite_ref(),
        "Cross Shield"   => ItemSprite::CrossShield.sprite_ref(),
        "Dark Shield"    => ItemSprite::DarkShield.sprite_ref(),
        "Round Shield"   => ItemSprite::RoundShield.sprite_ref(),
        "Tower Shield"   => ItemSprite::LargeShield.sprite_ref(),

        // === Helmets ===
        "Cloth Hood"     => ItemSprite::ClothHood.sprite_ref(),
        "Leather Cap"    => ItemSprite::LeatherHelm.sprite_ref(),
        "Iron Helmet"    => ItemSprite::Helm.sprite_ref(),
        "Chain Coif"     => ItemSprite::ChainMailCoif.sprite_ref(),
        "Mithril Helm"   => ItemSprite::PlateHelm1.sprite_ref(),
        "Plate Helm"     => ItemSprite::PlateHelm2.sprite_ref(),

        // === Boots ===
        "Leather Boots"  => ItemSprite::LeatherBoots.sprite_ref(),
        "Chain Boots"    => ItemSprite::HighBlueBoots.sprite_ref(),
        "Plate Boots"    => ItemSprite::Greaves.sprite_ref(),
        "Shoes"          => ItemSprite::Shoes.sprite_ref(),

        // === Rings ===
        "Copper Ring"   => ItemSprite::GoldBandRing.sprite_ref(),
        "Silver Ring"   => ItemSprite::SilverSignetRing.sprite_ref(),
        "Ruby Ring"     => ItemSprite::RubyRing.sprite_ref(),
        "Gold Ring"     => ItemSprite::TwistedGoldRing.sprite_ref(),
        "Diamond Ring"  => ItemSprite::SapphireRing.sprite_ref(),
        "Jade Ring"     => ItemSprite::JadeRing.sprite_ref(),
        "Emerald Ring"  => ItemSprite::GoldEmeraldRing.sprite_ref(),
        "Onyx Ring"     => ItemSprite::OnyxRing.sprite_ref(),

        // === Food — forageables ===
        "Wild Berries"   => ItemSprite::Apple.sprite_ref(),
        "Wild Mushrooms" => TileSprite::SmallMushrooms.sprite_ref(),
        "Clean Water"    => ItemSprite::BottleOfWater.sprite_ref(),
        // Food — foraged plants (tile sprites)
        "Wild Wheat"     => TileSprite::Wheat.sprite_ref(),
        "Wild Rice"      => TileSprite::Rice.sprite_ref(),
        "Wild Corn"      => TileSprite::MaizeCorn.sprite_ref(),
        "Quinoa Seeds"   => TileSprite::Quinoa.sprite_ref(),
        "Amaranth"       => TileSprite::Amaranth.sprite_ref(),
        "Red Spinach"    => TileSprite::RedSpinach.sprite_ref(),
        "Bitter Vetch"   => TileSprite::BitterVetch.sprite_ref(),
        "Sorghum"        => TileSprite::Sorghum.sprite_ref(),
        "Buckwheat"      => TileSprite::Buckwheat.sprite_ref(),
        // Food — meats
        "Rat Meat"       => ItemSprite::Bread.sprite_ref(),
        "Wolf Meat"      => ItemSprite::Bread.sprite_ref(),
        "Boar Meat"      => ItemSprite::Bread.sprite_ref(),
        "Bear Meat"      => ItemSprite::Bread.sprite_ref(),
        "Fox Meat"       => ItemSprite::Bread.sprite_ref(),
        "Venison"        => ItemSprite::Bread.sprite_ref(),
        "Snake Meat"     => ItemSprite::Cheese.sprite_ref(),
        "Gator Meat"     => ItemSprite::Bread.sprite_ref(),
        "Lion Meat"      => ItemSprite::Bread.sprite_ref(),
        "Fowl Meat"      => ItemSprite::Bread.sprite_ref(),
        "Stolen Rations" => ItemSprite::Bread.sprite_ref(),
        // Food — dungeon provisions
        "Stale Bread"    => ItemSprite::Bread.sprite_ref(),
        "Waterskin"      => ItemSprite::BottleOfWater.sprite_ref(),
        "Cheese Wedge"   => ItemSprite::Cheese.sprite_ref(),
        "Dried Rations"  => ItemSprite::Bread.sprite_ref(),
        "Dwarven Ale"    => ItemSprite::BottleOfBeer.sprite_ref(),
        "Elven Waybread" => ItemSprite::Bread.sprite_ref(),
        "Honey Mead"     => ItemSprite::BottleOfBeer.sprite_ref(),
        // Default fallback
        _ => ItemSprite::RedPotion.sprite_ref(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- SpriteRef pixel offsets ---

    #[test]
    fn sprite_ref_pixel_coords() {
        let s = SpriteRef::new(Sheet::Tiles, 3, 5);
        assert_eq!(s.src_x(), 160.0); // 5 * 32
        assert_eq!(s.src_y(), 96.0);  // 3 * 32
    }

    // --- Tile sprite mappings ---

    #[test]
    fn every_tile_variant_returns_valid_sprite() {
        let tiles = [
            Tile::Wall, Tile::Floor, Tile::Grass, Tile::Road,
            Tile::Tree, Tile::DungeonEntrance, Tile::StairsDown, Tile::StairsUp,
        ];
        for tile in tiles {
            let s = tile_sprite(tile, 10, 20, false, None);
            assert_eq!(s.sheet, Sheet::Tiles, "tile {:?} should use Tiles sheet", tile);
        }
    }

    #[test]
    fn wall_top_vs_face() {
        let top = tile_sprite(Tile::Wall, 0, 0, false, None);
        let face = tile_sprite(Tile::Wall, 0, 0, true, None);
        assert_eq!(top, TileSprite::StoneBrickWallTop.sprite_ref());
        assert_eq!(face, TileSprite::StoneBrickWallSide1.sprite_ref());
    }

    #[test]
    fn floor_variation_produces_all_variants() {
        let mut seen = [false; 3];
        for x in 0..100 {
            let s = tile_sprite(Tile::Floor, x, 0, false, None);
            assert_eq!(s.row, TileSprite::FloorStone1.sprite_ref().row);
            let idx = (s.col - TileSprite::FloorStone1.sprite_ref().col) as usize;
            assert!(idx < 3, "floor variant index {} out of range", idx);
            seen[idx] = true;
        }
        assert!(seen.iter().all(|&v| v), "should produce all 3 floor variants");
    }

    #[test]
    fn grass_variation_produces_all_variants() {
        let mut seen = [false; 3];
        for x in 0..100 {
            let s = tile_sprite(Tile::Grass, x, 0, false, None);
            assert_eq!(s.row, TileSprite::Grass1.sprite_ref().row);
            let idx = (s.col - TileSprite::Grass1.sprite_ref().col) as usize;
            assert!(idx < 3);
            seen[idx] = true;
        }
        assert!(seen.iter().all(|&v| v), "should produce all 3 grass variants");
    }

    #[test]
    fn road_variation_produces_all_variants() {
        let mut seen = [false; 3];
        for x in 0..100 {
            let s = tile_sprite(Tile::Road, x, 0, false, None);
            assert_eq!(s.row, TileSprite::Dirt1.sprite_ref().row);
            let idx = (s.col - TileSprite::Dirt1.sprite_ref().col) as usize;
            assert!(idx < 3);
            seen[idx] = true;
        }
        assert!(seen.iter().all(|&v| v), "should produce all 3 road variants");
    }

    #[test]
    fn tree_sprite_fixed() {
        let s = tile_sprite(Tile::Tree, 42, 99, false, None);
        assert_eq!(s, TileSprite::Tree.sprite_ref());
    }

    #[test]
    fn dungeon_entrance_sprite() {
        let s = tile_sprite(Tile::DungeonEntrance, 0, 0, false, None);
        assert_eq!(s, TileSprite::Door1.sprite_ref());
    }

    #[test]
    fn stairs_sprites() {
        let down = tile_sprite(Tile::StairsDown, 0, 0, false, None);
        assert_eq!(down, TileSprite::StaircaseDown.sprite_ref());
        let up = tile_sprite(Tile::StairsUp, 0, 0, false, None);
        assert_eq!(up, TileSprite::StaircaseUp.sprite_ref());
    }

    // --- Dungeon styles produce different wall sprites ---

    #[test]
    fn dungeon_styles_give_different_walls() {
        let styles = [
            DungeonStyle::DirtCaves, DungeonStyle::StoneBrick,
            DungeonStyle::Igneous, DungeonStyle::LargeStone,
            DungeonStyle::Catacombs, DungeonStyle::RedCavern,
        ];
        let mut wall_tops = std::collections::HashSet::new();
        for style in styles {
            let s = tile_sprite(Tile::Wall, 0, 0, false, Some(style));
            wall_tops.insert((s.row, s.col));
        }
        assert!(wall_tops.len() >= 4, "should have at least 4 distinct wall styles");
    }

    #[test]
    fn dungeon_styles_give_different_floors() {
        let catacombs = tile_sprite(Tile::Floor, 0, 0, false, Some(DungeonStyle::Catacombs));
        let stone = tile_sprite(Tile::Floor, 0, 0, false, Some(DungeonStyle::StoneBrick));
        let dragon = tile_sprite(Tile::Floor, 0, 0, false, Some(DungeonStyle::RedCavern));
        assert_ne!(catacombs, dragon);
        assert_ne!(catacombs, stone);
    }

    // --- Entity sprites ---

    #[test]
    fn player_sprite_is_rogue() {
        let s = player_sprite();
        assert_eq!(s, RogueSprite::Rogue.sprite_ref());
    }

    #[test]
    fn enemy_sprite_goblin() {
        assert_eq!(enemy_sprite('g'), MonsterSprite::Goblin.sprite_ref());
    }

    #[test]
    fn enemy_sprite_orc() {
        assert_eq!(enemy_sprite('o'), MonsterSprite::Orc.sprite_ref());
    }

    #[test]
    fn enemy_sprite_skeleton() {
        assert_eq!(enemy_sprite('s'), MonsterSprite::Skeleton.sprite_ref());
    }

    #[test]
    fn enemy_sprite_dragon() {
        assert_eq!(enemy_sprite('D'), MonsterSprite::Dragon.sprite_ref());
    }

    #[test]
    fn enemy_sprite_wolf() {
        assert_eq!(enemy_sprite('w'), AnimalSprite::Wolf.sprite_ref());
    }

    #[test]
    fn enemy_sprite_boar() {
        assert_eq!(enemy_sprite('b'), AnimalSprite::Boar.sprite_ref());
    }

    #[test]
    fn enemy_sprite_bear() {
        assert_eq!(enemy_sprite('B'), AnimalSprite::GrizzlyBear.sprite_ref());
    }

    #[test]
    fn enemy_sprite_slime() {
        assert_eq!(enemy_sprite('S'), MonsterSprite::SmallSlime.sprite_ref());
    }

    #[test]
    fn enemy_sprite_unknown_defaults_to_goblin() {
        assert_eq!(enemy_sprite('?'), MonsterSprite::Goblin.sprite_ref());
    }

    #[test]
    fn enemy_sprite_new_enemies_within_bounds() {
        // Forest animals (Animals sheet)
        for g in ['f', 'n', 'h', 'j', 'q', 'v', 'y', 'x', 'J', 'Z', 'F'] {
            let s = enemy_sprite(g);
            assert!(s.row < 17, "glyph '{}' row {} out of bounds", g, s.row);
        }
        // Forest + dungeon monsters (Monsters sheet)
        for g in ['1', '2', '9', '0', 't', '3', '4', '5', '6', '7', '8',
                   'e', 'p', 'A', 'M', 'H', 'E', 'R', 'Y', 'P', 'Q', 'd', 'C', 'I', 'X', 'V'] {
            let s = enemy_sprite(g);
            assert!(s.row < 13, "glyph '{}' row {} out of bounds", g, s.row);
        }
    }

    // --- Variation determinism ---

    #[test]
    fn variation_is_deterministic() {
        let a = tile_sprite(Tile::Floor, 10, 20, false, None);
        let b = tile_sprite(Tile::Floor, 10, 20, false, None);
        assert_eq!(a, b, "same coords should always give same sprite");
    }

    #[test]
    fn variation_changes_with_position() {
        let a = tile_sprite(Tile::Floor, 0, 0, false, None);
        let b = tile_sprite(Tile::Floor, 1, 0, false, None);
        assert_ne!(a.col, b.col, "different positions should vary");
    }

    // --- Bounds check: sprites within sheet dimensions ---

    #[test]
    fn tile_sprites_within_sheet_bounds() {
        let tiles = [
            Tile::Wall, Tile::Floor, Tile::Grass, Tile::Road,
            Tile::Tree, Tile::DungeonEntrance, Tile::StairsDown, Tile::StairsUp,
        ];
        let styles = [
            None,
            Some(DungeonStyle::DirtCaves),
            Some(DungeonStyle::StoneBrick),
            Some(DungeonStyle::Igneous),
            Some(DungeonStyle::LargeStone),
            Some(DungeonStyle::Catacombs),
            Some(DungeonStyle::RedCavern),
            Some(DungeonStyle::MossyCavern),
            Some(DungeonStyle::BoneCave),
            Some(DungeonStyle::BlueTemple),
            Some(DungeonStyle::MossyTunnel),
            Some(DungeonStyle::BoneCrypt),
        ];
        for tile in tiles {
            for &style in &styles {
                for x in 0..10 {
                    for y in 0..10 {
                        let s = tile_sprite(tile, x, y, false, style);
                        assert!(s.row < 26, "{:?} {:?} row {} >= 26", tile, style, s.row);
                        assert!(s.col < 17, "{:?} {:?} col {} >= 17", tile, style, s.col);
                        let s2 = tile_sprite(tile, x, y, true, style);
                        assert!(s2.row < 26, "{:?} wall_face {:?} row {} >= 26", tile, style, s2.row);
                        assert!(s2.col < 17, "{:?} wall_face {:?} col {} >= 17", tile, style, s2.col);
                    }
                }
            }
        }
    }

    #[test]
    fn monster_sprites_within_sheet_bounds() {
        let monster_glyphs = [
            // Forest animals
            'r', 'a', 'w', 'i', 'b', 'B', 'L',
            'f', 'n', 'h', 'j', 'q', 'v', 'y', 'x', 'J', 'Z', 'F',
            // Forest monsters
            '1', '2', '9', '0',
            // Dungeon — shallow
            'c', 'S', 'g', 's', 'e', 'p', 't',
            // Dungeon — mid
            'G', 'z', 'k', 'm', 'o', 'A', 'M', 'H', '3', '4', '5',
            // Dungeon — deep
            'u', 'O', 'W', 'N', 'T', 'E', 'R', 'Y', 'P', 'Q', '6', '7', '8',
            // Cave — boss
            'K', 'l', 'D', 'd', 'C', 'I', 'X', 'V',
            // Biome-specific (previously unused sprites)
            '$', '~', '>', '{', '}', '^', '[', ']', '<', 'U', '!', '(', ')',
            '?',
        ];
        for glyph in monster_glyphs {
            let s = enemy_sprite(glyph);
            match s.sheet {
                Sheet::Monsters => {
                    assert!(s.row < 13, "glyph '{}' row {} >= 13", glyph, s.row);
                    assert!(s.col < 12, "glyph '{}' col {} >= 12", glyph, s.col);
                }
                Sheet::Animals => {
                    assert!(s.row < 17, "glyph '{}' row {} >= 17", glyph, s.row);
                    assert!(s.col < 9, "glyph '{}' col {} >= 9", glyph, s.col);
                }
                _ => panic!("glyph '{}' uses unexpected sheet {:?}", glyph, s.sheet),
            }
        }
    }

    #[test]
    fn player_sprite_within_sheet_bounds() {
        let s = player_sprite();
        assert!(s.row < 8, "player row {} >= 8", s.row);
        assert!(s.col < 7, "player col {} >= 7", s.col);
    }

    // --- Item sprites ---

    #[test]
    fn item_sprites_within_sheet_bounds() {
        let names = [
            "Health Potion", "Greater Health Potion", "Superior Health Potion",
            "Antidote", "Stamina Potion", "Elixir of Power", "Poison Vial",
            "Scroll of Fire", "Scroll of Lightning", "Scroll of Storm",
            "Scroll of Ice", "Scroll of Wrath",
            "Rusty Sword", "Iron Sword", "Enchanted Blade",
            "Hand Axe", "Battle Axe", "Great Axe",
            "Mace", "Great Mace", "Trident", "Halberd",
            "Leather Armor", "Chain Mail", "Scale Mail", "Dragon Scale",
            "unknown_fallback",
        ];
        for name in names {
            let s = item_sprite(name);
            match s.sheet {
                Sheet::Items => {
                    assert!(s.row < 26, "{name} row {} >= 26", s.row);
                    assert!(s.col < 11, "{name} col {} >= 11", s.col);
                }
                Sheet::Tiles => {
                    assert!(s.row < 26, "{name} Tiles row {} >= 26", s.row);
                    assert!(s.col < 17, "{name} Tiles col {} >= 17", s.col);
                }
                _ => panic!("{name} unexpected sheet {:?}", s.sheet),
            }
        }
    }

    #[test]
    fn item_sprite_potions_use_potion_sprites() {
        let hp = item_sprite("Health Potion");
        let ghp = item_sprite("Greater Health Potion");
        let shp = item_sprite("Superior Health Potion");
        assert_eq!(hp, ItemSprite::RedPotion.sprite_ref());
        assert_eq!(ghp, ItemSprite::LargeDarkPotion.sprite_ref());
        assert_eq!(shp, ItemSprite::GreenPotion.sprite_ref());
    }

    #[test]
    fn item_sprite_scrolls_use_book_sprites() {
        let fire = item_sprite("Scroll of Fire");
        let lightning = item_sprite("Scroll of Lightning");
        let storm = item_sprite("Scroll of Storm");
        assert_eq!(fire, ItemSprite::RedBook.sprite_ref());
        assert_eq!(lightning, ItemSprite::DarkTome.sprite_ref());
        assert_eq!(storm, ItemSprite::Tome2.sprite_ref());
    }

    #[test]
    fn item_sprite_weapons_are_correct() {
        assert_eq!(item_sprite("Rusty Sword"), ItemSprite::ShortSword.sprite_ref());
        assert_eq!(item_sprite("Iron Sword"), ItemSprite::LongSword.sprite_ref());
        assert_eq!(item_sprite("Enchanted Blade"), ItemSprite::CrystalSword.sprite_ref());
        assert_eq!(item_sprite("Iron Dagger"), ItemSprite::Dagger.sprite_ref());
        assert_eq!(item_sprite("Battle Axe"), ItemSprite::BattleAxe.sprite_ref());
        assert_eq!(item_sprite("War Hammer"), ItemSprite::Hammer.sprite_ref());
        assert_eq!(item_sprite("Wooden Club"), ItemSprite::Club.sprite_ref());
        assert_eq!(item_sprite("Crystal Staff"), ItemSprite::CrystalStaff.sprite_ref());
        assert_eq!(item_sprite("Flame Sword"), ItemSprite::FlameSword.sprite_ref());
    }

    #[test]
    fn item_sprite_tiers_differ() {
        assert_ne!(item_sprite("Health Potion"), item_sprite("Greater Health Potion"));
        assert_ne!(item_sprite("Rusty Sword"), item_sprite("Iron Sword"));
        assert_ne!(item_sprite("Leather Armor"), item_sprite("Chain Mail"));
    }

    #[test]
    fn item_sprite_ranged_weapons() {
        assert_eq!(item_sprite("Short Bow"), ItemSprite::ShortBow.sprite_ref());
        assert_eq!(item_sprite("Crossbow"), ItemSprite::Crossbow.sprite_ref());
        assert_eq!(item_sprite("Long Bow"), ItemSprite::LongBow.sprite_ref());
        assert_eq!(item_sprite("Heavy Crossbow"), ItemSprite::LargeCrossbow.sprite_ref());
        assert_eq!(item_sprite("Elven Bow"), ItemSprite::LongBow2.sprite_ref());
    }
}
