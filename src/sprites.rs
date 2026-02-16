/// Game-specific sprite mappings — maps game entities to sprite sheet positions.
/// Pure logic, no web_sys dependency.
///
/// The canonical sprite catalog lives in `crate::sprite_atlas`. This module
/// re-exports `Sheet` and `SpriteRef` for convenience, and provides the
/// game-specific mapping functions (`tile_sprite`, `player_sprite`, etc.).

pub use crate::sprite_atlas::{Sheet, SpriteRef};

use crate::map::Tile;
use crate::sprite_atlas::items::ItemSprite;
use crate::sprite_atlas::monsters::MonsterSprite;
use crate::sprite_atlas::rogues::RogueSprite;
use crate::sprite_atlas::tiles::TileSprite;

const FLOOR_VARIANTS: [TileSprite; 3] = [
    TileSprite::FloorStone1,
    TileSprite::FloorStone2,
    TileSprite::FloorStone3,
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
pub fn tile_sprite(tile: Tile, x: i32, y: i32, wall_face: bool) -> SpriteRef {
    let variation = (x.wrapping_mul(31).wrapping_add(y.wrapping_mul(17))).unsigned_abs() % 3;
    match tile {
        Tile::Wall => {
            if wall_face {
                TileSprite::StoneBrickWallSide1.sprite_ref()
            } else {
                TileSprite::StoneBrickWallTop.sprite_ref()
            }
        }
        Tile::Floor => FLOOR_VARIANTS[variation as usize].sprite_ref(),
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
        // Forest beasts
        'r' => MonsterSprite::GiantRat.sprite_ref(),
        'a' => MonsterSprite::GiantBat.sprite_ref(),
        'w' => MonsterSprite::WargDireWolf.sprite_ref(),
        'i' => MonsterSprite::GiantSpider.sprite_ref(),
        'b' => MonsterSprite::Manticore.sprite_ref(),
        'B' => MonsterSprite::Wendigo.sprite_ref(),
        'L' => MonsterSprite::Lycanthrope.sprite_ref(),
        // Dungeon — shallow
        'c' => MonsterSprite::SmallKoboldCanine.sprite_ref(),
        'S' => MonsterSprite::SmallSlime.sprite_ref(),
        'g' => MonsterSprite::Goblin.sprite_ref(),
        's' => MonsterSprite::Skeleton.sprite_ref(),
        // Dungeon — mid
        'G' => MonsterSprite::GoblinArcher.sprite_ref(),
        'z' => MonsterSprite::Zombie.sprite_ref(),
        'k' => MonsterSprite::SkeletonArcher.sprite_ref(),
        'm' => MonsterSprite::BigSlime.sprite_ref(),
        'o' => MonsterSprite::Orc.sprite_ref(),
        // Dungeon — deep
        'u' => MonsterSprite::Ghoul.sprite_ref(),
        'O' => MonsterSprite::OrcBlademaster.sprite_ref(),
        'W' => MonsterSprite::Wraith.sprite_ref(),
        'N' => MonsterSprite::Naga.sprite_ref(),
        'T' => MonsterSprite::Troll.sprite_ref(),
        // Cave — boss floor
        'K' => MonsterSprite::DeathKnight.sprite_ref(),
        'l' => MonsterSprite::Lich.sprite_ref(),
        'D' => MonsterSprite::Dragon.sprite_ref(),
        _   => MonsterSprite::Goblin.sprite_ref(),
    }
}

/// Item sprite based on item name. Each tier gets a distinct sprite.
pub fn item_sprite(name: &str) -> SpriteRef {
    match name {
        // Potions
        "Health Potion"          => ItemSprite::RedPotion.sprite_ref(),
        "Greater Health Potion"  => ItemSprite::LargeDarkPotion.sprite_ref(),
        "Superior Health Potion" => ItemSprite::GreenPotion.sprite_ref(),
        // Scrolls
        "Scroll of Fire"      => ItemSprite::RedBook.sprite_ref(),
        "Scroll of Lightning" => ItemSprite::DarkTome.sprite_ref(),
        "Scroll of Storm"     => ItemSprite::Tome2.sprite_ref(),
        // Weapons
        "Rusty Sword"     => ItemSprite::ShortSword.sprite_ref(),
        "Iron Sword"      => ItemSprite::LongSword.sprite_ref(),
        "Enchanted Blade" => ItemSprite::CrystalSword.sprite_ref(),
        "Iron Dagger"     => ItemSprite::Dagger.sprite_ref(),
        "Battle Axe"      => ItemSprite::BattleAxe.sprite_ref(),
        "War Hammer"      => ItemSprite::Hammer.sprite_ref(),
        "Wooden Club"     => ItemSprite::Club.sprite_ref(),
        "Crystal Staff"   => ItemSprite::CrystalStaff.sprite_ref(),
        "Flame Sword"     => ItemSprite::FlameSword.sprite_ref(),
        // Body armor
        "Leather Armor"  => ItemSprite::LeatherArmor.sprite_ref(),
        "Chain Mail"     => ItemSprite::ChainMail.sprite_ref(),
        "Dragon Scale"   => ItemSprite::ChestPlate.sprite_ref(),
        // Shields
        "Wooden Shield"  => ItemSprite::Buckler.sprite_ref(),
        "Iron Shield"    => ItemSprite::KiteShield.sprite_ref(),
        // Helmets
        "Leather Cap"    => ItemSprite::LeatherHelm.sprite_ref(),
        "Iron Helmet"    => ItemSprite::Helm.sprite_ref(),
        "Mithril Helm"   => ItemSprite::PlateHelm1.sprite_ref(),
        // Boots
        "Plate Boots"    => ItemSprite::Greaves.sprite_ref(),
        // Rings
        "Copper Ring"   => ItemSprite::GoldBandRing.sprite_ref(),
        "Silver Ring"   => ItemSprite::SilverSignetRing.sprite_ref(),
        "Ruby Ring"     => ItemSprite::RubyRing.sprite_ref(),
        "Gold Ring"     => ItemSprite::TwistedGoldRing.sprite_ref(),
        "Diamond Ring"  => ItemSprite::SapphireRing.sprite_ref(),
        // Food
        "Wild Berries"  => ItemSprite::Apple.sprite_ref(),
        "Mushrooms"     => ItemSprite::Cheese.sprite_ref(),
        "Wolf Meat"     => ItemSprite::Bread.sprite_ref(),
        "Boar Meat"     => ItemSprite::Bread.sprite_ref(),
        "Bear Meat"     => ItemSprite::Cheese.sprite_ref(),
        "Dried Rations" => ItemSprite::Bread.sprite_ref(),
        // Default fallback
        _ => ItemSprite::RedPotion.sprite_ref(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sprite_atlas::monsters::MonsterSprite;
    use crate::sprite_atlas::items::ItemSprite;

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
            let s = tile_sprite(tile, 10, 20, false);
            assert_eq!(s.sheet, Sheet::Tiles, "tile {:?} should use Tiles sheet", tile);
        }
    }

    #[test]
    fn wall_top_vs_face() {
        let top = tile_sprite(Tile::Wall, 0, 0, false);
        let face = tile_sprite(Tile::Wall, 0, 0, true);
        assert_eq!(top, TileSprite::StoneBrickWallTop.sprite_ref());
        assert_eq!(face, TileSprite::StoneBrickWallSide1.sprite_ref());
    }

    #[test]
    fn floor_variation_produces_all_variants() {
        let mut seen = [false; 3];
        for x in 0..100 {
            let s = tile_sprite(Tile::Floor, x, 0, false);
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
            let s = tile_sprite(Tile::Grass, x, 0, false);
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
            let s = tile_sprite(Tile::Road, x, 0, false);
            assert_eq!(s.row, TileSprite::Dirt1.sprite_ref().row);
            let idx = (s.col - TileSprite::Dirt1.sprite_ref().col) as usize;
            assert!(idx < 3);
            seen[idx] = true;
        }
        assert!(seen.iter().all(|&v| v), "should produce all 3 road variants");
    }

    #[test]
    fn tree_sprite_fixed() {
        let s = tile_sprite(Tile::Tree, 42, 99, false);
        assert_eq!(s, TileSprite::Tree.sprite_ref());
    }

    #[test]
    fn dungeon_entrance_sprite() {
        let s = tile_sprite(Tile::DungeonEntrance, 0, 0, false);
        assert_eq!(s, TileSprite::Door1.sprite_ref());
    }

    #[test]
    fn stairs_sprites() {
        let down = tile_sprite(Tile::StairsDown, 0, 0, false);
        assert_eq!(down, TileSprite::StaircaseDown.sprite_ref());
        let up = tile_sprite(Tile::StairsUp, 0, 0, false);
        assert_eq!(up, TileSprite::StaircaseUp.sprite_ref());
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
        assert_eq!(enemy_sprite('w'), MonsterSprite::WargDireWolf.sprite_ref());
    }

    #[test]
    fn enemy_sprite_boar() {
        assert_eq!(enemy_sprite('b'), MonsterSprite::Manticore.sprite_ref());
    }

    #[test]
    fn enemy_sprite_bear() {
        assert_eq!(enemy_sprite('B'), MonsterSprite::Wendigo.sprite_ref());
    }

    #[test]
    fn enemy_sprite_slime() {
        assert_eq!(enemy_sprite('S'), MonsterSprite::SmallSlime.sprite_ref());
    }

    #[test]
    fn enemy_sprite_unknown_defaults_to_goblin() {
        assert_eq!(enemy_sprite('?'), MonsterSprite::Goblin.sprite_ref());
    }

    // --- Variation determinism ---

    #[test]
    fn variation_is_deterministic() {
        let a = tile_sprite(Tile::Floor, 10, 20, false);
        let b = tile_sprite(Tile::Floor, 10, 20, false);
        assert_eq!(a, b, "same coords should always give same sprite");
    }

    #[test]
    fn variation_changes_with_position() {
        let a = tile_sprite(Tile::Floor, 0, 0, false);
        let b = tile_sprite(Tile::Floor, 1, 0, false);
        assert_ne!(a.col, b.col, "different positions should vary");
    }

    // --- Bounds check: sprites within sheet dimensions ---

    #[test]
    fn tile_sprites_within_sheet_bounds() {
        let tiles = [
            Tile::Wall, Tile::Floor, Tile::Grass, Tile::Road,
            Tile::Tree, Tile::DungeonEntrance, Tile::StairsDown, Tile::StairsUp,
        ];
        for tile in tiles {
            for x in 0..10 {
                for y in 0..10 {
                    let s = tile_sprite(tile, x, y, false);
                    assert!(s.row < 26, "{:?} row {} >= 26", tile, s.row);
                    assert!(s.col < 17, "{:?} col {} >= 17", tile, s.col);
                    let s2 = tile_sprite(tile, x, y, true);
                    assert!(s2.row < 26, "{:?} wall_face row {} >= 26", tile, s2.row);
                    assert!(s2.col < 17, "{:?} wall_face col {} >= 17", tile, s2.col);
                }
            }
        }
    }

    #[test]
    fn monster_sprites_within_sheet_bounds() {
        let all_glyphs = [
            // Forest
            'r', 'a', 'w', 'i', 'b', 'B', 'L',
            // Dungeon shallow
            'c', 'S', 'g', 's',
            // Dungeon mid
            'G', 'z', 'k', 'm', 'o',
            // Dungeon deep
            'u', 'O', 'W', 'N', 'T',
            // Cave boss
            'K', 'l', 'D',
            // Unknown fallback
            '?',
        ];
        for glyph in all_glyphs {
            let s = enemy_sprite(glyph);
            assert!(s.row < 13, "glyph '{}' row {} >= 13", glyph, s.row);
            assert!(s.col < 12, "glyph '{}' col {} >= 12", glyph, s.col);
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
            "Scroll of Fire", "Scroll of Lightning", "Scroll of Storm",
            "Rusty Sword", "Iron Sword", "Enchanted Blade",
            "Leather Armor", "Chain Mail", "Dragon Scale",
            "unknown_fallback",
        ];
        for name in names {
            let s = item_sprite(name);
            assert_eq!(s.sheet, Sheet::Items, "{name} should use Items sheet");
            assert!(s.row < 26, "{name} row {} >= 26", s.row);
            assert!(s.col < 11, "{name} col {} >= 11", s.col);
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
    fn item_sprite_armor_uses_armor_sprites() {
        assert_eq!(item_sprite("Leather Armor"), ItemSprite::LeatherArmor.sprite_ref());
        assert_eq!(item_sprite("Chain Mail"), ItemSprite::ChainMail.sprite_ref());
        assert_eq!(item_sprite("Dragon Scale"), ItemSprite::ChestPlate.sprite_ref());
    }

    #[test]
    fn item_sprite_shields() {
        assert_eq!(item_sprite("Wooden Shield"), ItemSprite::Buckler.sprite_ref());
        assert_eq!(item_sprite("Iron Shield"), ItemSprite::KiteShield.sprite_ref());
    }

    #[test]
    fn item_sprite_helmets() {
        assert_eq!(item_sprite("Leather Cap"), ItemSprite::LeatherHelm.sprite_ref());
        assert_eq!(item_sprite("Iron Helmet"), ItemSprite::Helm.sprite_ref());
        assert_eq!(item_sprite("Mithril Helm"), ItemSprite::PlateHelm1.sprite_ref());
    }

    #[test]
    fn item_sprite_boots() {
        assert_eq!(item_sprite("Plate Boots"), ItemSprite::Greaves.sprite_ref());
    }

    #[test]
    fn item_sprite_food_uses_food_sprites() {
        let food_names = ["Wild Berries", "Mushrooms", "Wolf Meat", "Boar Meat", "Bear Meat", "Dried Rations"];
        for name in food_names {
            let s = item_sprite(name);
            assert_eq!(s.sheet, Sheet::Items);
            assert_eq!(s.row, 25, "{name} should be on food row 25");
        }
    }

    #[test]
    fn food_sprites_within_sheet_bounds() {
        let names = ["Wild Berries", "Mushrooms", "Wolf Meat", "Boar Meat", "Bear Meat", "Dried Rations"];
        for name in names {
            let s = item_sprite(name);
            assert_eq!(s.sheet, Sheet::Items, "{name} should use Items sheet");
            assert!(s.row < 26, "{name} row {} >= 26", s.row);
            assert!(s.col < 11, "{name} col {} >= 11", s.col);
        }
    }

    #[test]
    fn item_sprite_new_weapons_within_bounds() {
        let names = ["Iron Dagger", "Battle Axe", "War Hammer", "Wooden Club", "Crystal Staff", "Flame Sword"];
        for name in names {
            let s = item_sprite(name);
            assert_eq!(s.sheet, Sheet::Items, "{name} should use Items sheet");
            assert!(s.row < 26, "{name} row {} >= 26", s.row);
            assert!(s.col < 11, "{name} col {} >= 11", s.col);
        }
    }

    #[test]
    fn item_sprite_new_armor_within_bounds() {
        let names = ["Wooden Shield", "Iron Shield", "Leather Cap", "Iron Helmet", "Mithril Helm", "Plate Boots"];
        for name in names {
            let s = item_sprite(name);
            assert_eq!(s.sheet, Sheet::Items, "{name} should use Items sheet");
            assert!(s.row < 26, "{name} row {} >= 26", s.row);
            assert!(s.col < 11, "{name} col {} >= 11", s.col);
        }
    }

    #[test]
    fn item_sprite_rings_use_ring_sprites() {
        let names = ["Copper Ring", "Silver Ring", "Ruby Ring", "Gold Ring", "Diamond Ring"];
        for name in names {
            let s = item_sprite(name);
            assert_eq!(s.sheet, Sheet::Items, "{name} should use Items sheet");
            assert!(s.row == 17 || s.row == 18, "{name} should be on ring row 17 or 18, got {}", s.row);
        }
    }

    #[test]
    fn item_sprite_tiers_differ() {
        assert_ne!(item_sprite("Health Potion"), item_sprite("Greater Health Potion"));
        assert_ne!(item_sprite("Rusty Sword"), item_sprite("Iron Sword"));
        assert_ne!(item_sprite("Leather Armor"), item_sprite("Chain Mail"));
    }
}
