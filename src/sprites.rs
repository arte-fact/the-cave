/// Sprite atlas — maps game entities to (sheet, row, col) in the 32×32 sprite sheets.
/// Pure logic, no web_sys dependency.

/// Which sprite sheet PNG to source from.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Sheet {
    Tiles,
    Monsters,
    Rogues,
    Items,
}

/// A reference to a single 32×32 sprite in a sheet.
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

use crate::map::Tile;

/// Pick a tile sprite. `x, y` are world coordinates used for variation hashing.
/// `wall_face` should be true when the wall has a non-wall tile directly below
/// (so we show the "side/face" variant instead of the "top" variant).
pub fn tile_sprite(tile: Tile, x: i32, y: i32, wall_face: bool) -> SpriteRef {
    let variation = ((x.wrapping_mul(31).wrapping_add(y.wrapping_mul(17))).unsigned_abs() % 3) as u16;
    match tile {
        Tile::Wall => {
            if wall_face {
                // Stone brick wall side — row 2, col 1 (3.b)
                SpriteRef::new(Sheet::Tiles, 2, 1)
            } else {
                // Stone brick wall top — row 2, col 0 (3.a)
                SpriteRef::new(Sheet::Tiles, 2, 0)
            }
        }
        Tile::Floor => {
            // Floor stone variants — row 6, cols 1-3 (7.b/c/d)
            SpriteRef::new(Sheet::Tiles, 6, 1 + variation)
        }
        Tile::Grass => {
            // Grass variants — row 7, cols 1-3 (8.b/c/d)
            SpriteRef::new(Sheet::Tiles, 7, 1 + variation)
        }
        Tile::Road => {
            // Dirt variants — row 8, cols 1-3 (9.b/c/d)
            SpriteRef::new(Sheet::Tiles, 8, 1 + variation)
        }
        Tile::Tree => {
            // Tree — row 25, col 2 (26.c)
            SpriteRef::new(Sheet::Tiles, 25, 2)
        }
        Tile::DungeonEntrance => {
            // Door — row 16, col 0 (17.a)
            SpriteRef::new(Sheet::Tiles, 16, 0)
        }
        Tile::StairsDown => {
            // Staircase down — row 16, col 7 (17.h)
            SpriteRef::new(Sheet::Tiles, 16, 7)
        }
        Tile::StairsUp => {
            // Staircase up — row 16, col 8 (17.i)
            SpriteRef::new(Sheet::Tiles, 16, 8)
        }
    }
}

/// Player sprite: rogue character from rogues.png row 0, col 3 (1.d).
pub fn player_sprite() -> SpriteRef {
    SpriteRef::new(Sheet::Rogues, 0, 3)
}

/// Enemy sprite based on glyph character.
pub fn enemy_sprite(glyph: char) -> SpriteRef {
    match glyph {
        // Forest animals
        'w' => SpriteRef::new(Sheet::Monsters, 7, 0),  // wolf (8.a)
        'b' => SpriteRef::new(Sheet::Monsters, 5, 0),  // boar (6.a)
        'B' => SpriteRef::new(Sheet::Monsters, 6, 0),  // bear (7.a)
        // Dungeon enemies
        'g' => SpriteRef::new(Sheet::Monsters, 0, 2),  // goblin (1.c)
        's' => SpriteRef::new(Sheet::Monsters, 4, 0),  // skeleton (5.a)
        'o' => SpriteRef::new(Sheet::Monsters, 0, 0),  // orc (1.a)
        'T' => SpriteRef::new(Sheet::Monsters, 1, 2),  // troll (2.c)
        'D' => SpriteRef::new(Sheet::Monsters, 8, 2),  // dragon boss (9.c)
        // Unused but mapped
        'S' => SpriteRef::new(Sheet::Monsters, 2, 0),  // slime (3.a)
        _   => SpriteRef::new(Sheet::Monsters, 0, 2),  // default: goblin
    }
}

/// Item sprite based on item name. Each tier gets a distinct sprite.
/// items.png: 11 cols × 26 rows, 32×32 cells.
pub fn item_sprite(name: &str) -> SpriteRef {
    match name {
        // Potions — row 15: red, blue, green
        "Health Potion"          => SpriteRef::new(Sheet::Items, 15, 2),
        "Greater Health Potion"  => SpriteRef::new(Sheet::Items, 15, 3),
        "Superior Health Potion" => SpriteRef::new(Sheet::Items, 15, 4),
        // Scrolls — row 16: red book, blue book, scroll
        "Scroll of Fire"      => SpriteRef::new(Sheet::Items, 16, 1),
        "Scroll of Lightning" => SpriteRef::new(Sheet::Items, 16, 3),
        "Scroll of Storm"     => SpriteRef::new(Sheet::Items, 16, 5),
        // Weapons — row 0: short sword, broad sword, cyan enchanted blade
        "Rusty Sword"     => SpriteRef::new(Sheet::Items, 0, 1),
        "Iron Sword"      => SpriteRef::new(Sheet::Items, 0, 3),
        "Enchanted Blade" => SpriteRef::new(Sheet::Items, 0, 10),
        // Armor — row 8: leather vest, chain tunic, plate armor
        "Leather Armor" => SpriteRef::new(Sheet::Items, 8, 0),
        "Chain Mail"    => SpriteRef::new(Sheet::Items, 8, 2),
        "Dragon Scale"  => SpriteRef::new(Sheet::Items, 8, 3),
        // Default fallback
        _ => SpriteRef::new(Sheet::Items, 15, 2),
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
            let s = tile_sprite(tile, 10, 20, false);
            assert_eq!(s.sheet, Sheet::Tiles, "tile {:?} should use Tiles sheet", tile);
        }
    }

    #[test]
    fn wall_top_vs_face() {
        let top = tile_sprite(Tile::Wall, 0, 0, false);
        let face = tile_sprite(Tile::Wall, 0, 0, true);
        assert_eq!(top.row, 2);
        assert_eq!(top.col, 0); // top variant
        assert_eq!(face.row, 2);
        assert_eq!(face.col, 1); // side/face variant
    }

    #[test]
    fn floor_variation_produces_cols_1_to_3() {
        let mut seen = [false; 3];
        for x in 0..100 {
            let s = tile_sprite(Tile::Floor, x, 0, false);
            assert_eq!(s.row, 6);
            assert!(s.col >= 1 && s.col <= 3, "floor col {} out of range", s.col);
            seen[(s.col - 1) as usize] = true;
        }
        assert!(seen.iter().all(|&v| v), "should produce all 3 floor variants");
    }

    #[test]
    fn grass_variation_produces_cols_1_to_3() {
        let mut seen = [false; 3];
        for x in 0..100 {
            let s = tile_sprite(Tile::Grass, x, 0, false);
            assert_eq!(s.row, 7);
            assert!(s.col >= 1 && s.col <= 3);
            seen[(s.col - 1) as usize] = true;
        }
        assert!(seen.iter().all(|&v| v), "should produce all 3 grass variants");
    }

    #[test]
    fn road_variation_produces_cols_1_to_3() {
        let mut seen = [false; 3];
        for x in 0..100 {
            let s = tile_sprite(Tile::Road, x, 0, false);
            assert_eq!(s.row, 8);
            assert!(s.col >= 1 && s.col <= 3);
            seen[(s.col - 1) as usize] = true;
        }
        assert!(seen.iter().all(|&v| v), "should produce all 3 road variants");
    }

    #[test]
    fn tree_sprite_fixed() {
        let s = tile_sprite(Tile::Tree, 42, 99, false);
        assert_eq!(s.row, 25);
        assert_eq!(s.col, 2);
    }

    #[test]
    fn dungeon_entrance_sprite() {
        let s = tile_sprite(Tile::DungeonEntrance, 0, 0, false);
        assert_eq!(s.row, 16);
        assert_eq!(s.col, 0);
    }

    #[test]
    fn stairs_sprites() {
        let down = tile_sprite(Tile::StairsDown, 0, 0, false);
        assert_eq!(down.row, 16);
        assert_eq!(down.col, 7);
        let up = tile_sprite(Tile::StairsUp, 0, 0, false);
        assert_eq!(up.row, 16);
        assert_eq!(up.col, 8);
    }

    // --- Entity sprites ---

    #[test]
    fn player_sprite_is_rogue() {
        let s = player_sprite();
        assert_eq!(s.sheet, Sheet::Rogues);
        assert_eq!(s.row, 0);
        assert_eq!(s.col, 3);
    }

    #[test]
    fn enemy_sprite_goblin() {
        let s = enemy_sprite('g');
        assert_eq!(s.sheet, Sheet::Monsters);
        assert_eq!(s.row, 0);
        assert_eq!(s.col, 2);
    }

    #[test]
    fn enemy_sprite_dragon() {
        let s = enemy_sprite('D');
        assert_eq!(s.sheet, Sheet::Monsters);
        assert_eq!(s.row, 8);
        assert_eq!(s.col, 2);
    }

    #[test]
    fn enemy_sprite_wolf() {
        let s = enemy_sprite('w');
        assert_eq!(s.sheet, Sheet::Monsters);
        assert_eq!(s.row, 7);
        assert_eq!(s.col, 0);
    }

    #[test]
    fn enemy_sprite_boar() {
        let s = enemy_sprite('b');
        assert_eq!(s.sheet, Sheet::Monsters);
        assert_eq!(s.row, 5);
        assert_eq!(s.col, 0);
    }

    #[test]
    fn enemy_sprite_bear() {
        let s = enemy_sprite('B');
        assert_eq!(s.sheet, Sheet::Monsters);
        assert_eq!(s.row, 6);
        assert_eq!(s.col, 0);
    }

    #[test]
    fn enemy_sprite_unknown_defaults_to_goblin() {
        let s = enemy_sprite('?');
        assert_eq!(s.sheet, Sheet::Monsters);
        assert_eq!(s.row, 0);
        assert_eq!(s.col, 2);
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
        // Not all positions can give different results, but (0,0) and (1,0)
        // should differ since (0*31+0*17)%3=0 and (1*31+0*17)%3=1
        let a = tile_sprite(Tile::Floor, 0, 0, false);
        let b = tile_sprite(Tile::Floor, 1, 0, false);
        assert_ne!(a.col, b.col, "different positions should vary");
    }

    // --- Bounds check: sprites within sheet dimensions ---

    #[test]
    fn tile_sprites_within_sheet_bounds() {
        // tiles.png: 17 cols x 26 rows
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
        // monsters.png: 12 cols x 13 rows
        for glyph in ['g', 'D', 'o', 's', 'S', 'T', 'w', 'b', 'B', '?'] {
            let s = enemy_sprite(glyph);
            assert!(s.row < 13, "glyph '{}' row {} >= 13", glyph, s.row);
            assert!(s.col < 12, "glyph '{}' col {} >= 12", glyph, s.col);
        }
    }

    #[test]
    fn player_sprite_within_sheet_bounds() {
        // rogues.png: 7 cols x 7 rows
        let s = player_sprite();
        assert!(s.row < 7, "player row {} >= 7", s.row);
        assert!(s.col < 7, "player col {} >= 7", s.col);
    }

    // --- Item sprites ---

    #[test]
    fn item_sprites_within_sheet_bounds() {
        // items.png: 11 cols x 26 rows
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
    fn item_sprite_potions_on_row_15() {
        assert_eq!(item_sprite("Health Potion").row, 15);
        assert_eq!(item_sprite("Greater Health Potion").row, 15);
        assert_eq!(item_sprite("Superior Health Potion").row, 15);
    }

    #[test]
    fn item_sprite_scrolls_on_row_16() {
        assert_eq!(item_sprite("Scroll of Fire").row, 16);
        assert_eq!(item_sprite("Scroll of Lightning").row, 16);
        assert_eq!(item_sprite("Scroll of Storm").row, 16);
    }

    #[test]
    fn item_sprite_weapons_on_row_0() {
        assert_eq!(item_sprite("Rusty Sword").row, 0);
        assert_eq!(item_sprite("Iron Sword").row, 0);
        assert_eq!(item_sprite("Enchanted Blade").row, 0);
    }

    #[test]
    fn item_sprite_armor_on_row_8() {
        assert_eq!(item_sprite("Leather Armor").row, 8);
        assert_eq!(item_sprite("Chain Mail").row, 8);
        assert_eq!(item_sprite("Dragon Scale").row, 8);
    }

    #[test]
    fn item_sprite_tiers_differ() {
        // Each tier should have a distinct sprite
        assert_ne!(item_sprite("Health Potion").col, item_sprite("Greater Health Potion").col);
        assert_ne!(item_sprite("Rusty Sword").col, item_sprite("Iron Sword").col);
        assert_ne!(item_sprite("Leather Armor").col, item_sprite("Chain Mail").col);
    }
}
