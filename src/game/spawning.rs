use crate::map::Tile;
use super::types::*;
use super::{Game, xorshift64};

impl Game {
    /// Spawn forest creatures on the overworld with expanded variety.
    pub fn spawn_enemies(&mut self, seed: u64) {
        let map = self.world.current_map();
        let mut rng = seed;
        for y in 2..map.height - 2 {
            for x in 2..map.width - 2 {
                if !map.is_walkable(x, y) {
                    continue;
                }
                if x == self.player_x && y == self.player_y {
                    continue;
                }
                rng = xorshift64(rng);
                // Configurable chance per walkable tile (forest is sparse)
                if rng % 100 < self.config.spawn.overworld_enemy_pct {
                    rng = xorshift64(rng);
                    let enemy = roll_overworld_enemy(x, y, rng);
                    self.enemies.push(enemy);
                }
            }
        }
    }

    /// Spawn enemies appropriate for a dungeon level with expanded variety.
    pub(super) fn spawn_dungeon_enemies(&mut self, dungeon_index: usize, level: usize) {
        let total_levels = self.world.dungeons[dungeon_index].levels.len();
        let is_cave = total_levels == 4 && level == 3;

        let map = self.world.current_map();
        let seed = (dungeon_index as u64)
            .wrapping_mul(31)
            .wrapping_add(level as u64)
            .wrapping_mul(6364136223846793005);
        let mut rng = seed;
        for y in 1..map.height - 1 {
            for x in 1..map.width - 1 {
                if !map.is_walkable(x, y) {
                    continue;
                }
                if x == self.player_x && y == self.player_y {
                    continue;
                }
                let tile = map.get(x, y);
                if tile == Tile::StairsUp || tile == Tile::StairsDown {
                    continue;
                }
                rng = xorshift64(rng);
                let spawn_chance = if is_cave {
                    self.config.spawn.cave_enemy_pct
                } else {
                    self.config.spawn.dungeon_enemy_pct
                };
                if rng % 100 < spawn_chance {
                    rng = xorshift64(rng);
                    let enemy = if is_cave {
                        roll_cave_enemy(x, y, rng)
                    } else {
                        roll_dungeon_enemy(x, y, level, rng)
                    };
                    self.enemies.push(enemy);
                }
            }
        }

        // Place unique dragon boss only in the cave level
        if is_cave {
            self.place_dragon_boss();
        }
    }

    /// Place the dragon boss in the cave level, far from the player.
    fn place_dragon_boss(&mut self) {
        let map = self.world.current_map();
        for y in (1..map.height - 1).rev() {
            for x in (1..map.width - 1).rev() {
                if map.is_walkable(x, y)
                    && map.get(x, y) == Tile::Floor
                    && (x - self.player_x).abs() + (y - self.player_y).abs() > 5
                    && !self.enemies.iter().any(|e| e.x == x && e.y == y)
                {
                    self.enemies.push(Enemy {
                        x, y, hp: 40, attack: 10, defense: 6, glyph: 'D', name: "Dragon", facing_left: false, is_ranged: false,
                    });
                    return;
                }
            }
        }
    }
}

/// Roll a random overworld enemy based on the rng value.
fn roll_overworld_enemy(x: i32, y: i32, rng: u64) -> Enemy {
    let roll = rng % 100;
    // (hp, atk, def, glyph, name)
    let (hp, attack, def, glyph, name) = if roll < 6 {
        (3, 1, 0, 'r', "Giant Rat")
    } else if roll < 11 {
        (4, 2, 0, 'a', "Giant Bat")
    } else if roll < 16 {
        (4, 2, 1, 'f', "Fox")
    } else if roll < 20 {
        (4, 2, 0, 'q', "Buzzard")
    } else if roll < 25 {
        (5, 3, 0, 'n', "Viper")
    } else if roll < 28 {
        (5, 3, 0, 'v', "Black Mamba")
    } else if roll < 35 {
        (5, 2, 1, 'w', "Wolf")
    } else if roll < 39 {
        (5, 2, 1, 'y', "Coyote")
    } else if roll < 43 {
        (5, 3, 1, 'x', "Hyena")
    } else if roll < 48 {
        (6, 3, 0, 'i', "Giant Spider")
    } else if roll < 52 {
        (5, 3, 1, 'j', "Badger")
    } else if roll < 55 {
        (6, 3, 2, 'J', "Honey Badger")
    } else if roll < 59 {
        (5, 2, 0, '1', "Dryad")
    } else if roll < 62 {
        (4, 1, 1, '2', "Forest Spirit")
    } else if roll < 68 {
        (8, 2, 2, 'b', "Boar")
    } else if roll < 73 {
        (9, 4, 2, 'h', "Cougar")
    } else if roll < 77 {
        (10, 3, 3, 'Z', "Alligator")
    } else if roll < 80 {
        (10, 4, 2, '9', "Centaur")
    } else if roll < 86 {
        (12, 4, 2, 'B', "Bear")
    } else if roll < 91 {
        (14, 5, 3, 'L', "Lycanthrope")
    } else if roll < 96 {
        (16, 6, 2, 'F', "Male Lion")
    } else {
        (12, 5, 1, '0', "Wendigo")
    };
    Enemy { x, y, hp, attack, defense: def, glyph, name, facing_left: false, is_ranged: false }
}

/// Roll a random cave enemy based on the rng value.
fn roll_cave_enemy(x: i32, y: i32, rng: u64) -> Enemy {
    let roll = rng % 100;
    let (hp, attack, def, glyph, name, ranged) = if roll < 20 {
        (20, 7, 5, 'K', "Death Knight", false)
    } else if roll < 35 {
        (16, 5, 3, 'T', "Troll", false)
    } else if roll < 50 {
        (15, 8, 2, 'l', "Lich", false)
    } else if roll < 60 {
        (14, 6, 3, 'd', "Drake", false)
    } else if roll < 70 {
        (16, 7, 4, 'C', "Basilisk", false)
    } else if roll < 80 {
        (10, 6, 1, 'I', "Imp", false)
    } else if roll < 90 {
        (18, 7, 4, 'X', "Manticore", false)
    } else {
        (20, 9, 3, 'V', "Reaper", false)
    };
    Enemy { x, y, hp, attack, defense: def, glyph, name, facing_left: false, is_ranged: ranged }
}

/// Roll a random dungeon enemy based on level and rng value.
fn roll_dungeon_enemy(x: i32, y: i32, level: usize, rng: u64) -> Enemy {
    let roll = rng % 100;
    let (hp, attack, def, glyph, name, ranged) = match level {
        0 => roll_dungeon_level0(roll),
        1 => roll_dungeon_level1(roll),
        _ => roll_dungeon_deep(roll),
    };
    Enemy { x, y, hp, attack, defense: def, glyph, name, facing_left: false, is_ranged: ranged }
}

/// Enemy table for dungeon level 0 (shallow).
fn roll_dungeon_level0(roll: u64) -> (i32, i32, i32, char, &'static str, bool) {
    if roll < 12 {
        (3, 1, 0, 'r', "Giant Rat", false)
    } else if roll < 22 {
        (4, 2, 1, 'c', "Kobold", false)
    } else if roll < 30 {
        (4, 1, 0, 'S', "Small Slime", false)
    } else if roll < 42 {
        (5, 2, 1, 'g', "Goblin", false)
    } else if roll < 52 {
        (4, 2, 0, 'e', "Giant Centipede", false)
    } else if roll < 60 {
        (3, 1, 1, 'p', "Myconid", false)
    } else if roll < 68 {
        (4, 2, 1, 't', "Large Myconid", false)
    } else if roll < 78 {
        (5, 2, 0, '1', "Dryad", false)
    } else if roll < 88 {
        (4, 1, 1, '2', "Forest Spirit", false)
    } else {
        (6, 3, 2, 's', "Skeleton", false)
    }
}

/// Enemy table for dungeon level 1 (mid).
fn roll_dungeon_level1(roll: u64) -> (i32, i32, i32, char, &'static str, bool) {
    if roll < 10 {
        (6, 3, 1, 'G', "Goblin Archer", true)
    } else if roll < 20 {
        (10, 2, 1, 'z', "Zombie", false)
    } else if roll < 28 {
        (7, 4, 2, 'k', "Skeleton Archer", true)
    } else if roll < 36 {
        (10, 2, 0, 'm', "Big Slime", false)
    } else if roll < 46 {
        (10, 4, 3, 'o', "Orc", false)
    } else if roll < 54 {
        (8, 3, 2, 'A', "Giant Ant", false)
    } else if roll < 62 {
        (7, 5, 1, 'M', "Goblin Mage", true)
    } else if roll < 70 {
        (9, 4, 1, 'H', "Hag", false)
    } else if roll < 78 {
        (6, 3, 1, '3', "Goblin Brute", false)
    } else if roll < 88 {
        (7, 3, 0, '4', "Satyr", false)
    } else {
        (12, 5, 4, '5', "Orc Warchief", false)
    }
}

/// Enemy table for dungeon level 2+ (deep).
fn roll_dungeon_deep(roll: u64) -> (i32, i32, i32, char, &'static str, bool) {
    if roll < 8 {
        (10, 5, 2, 'u', "Ghoul", false)
    } else if roll < 16 {
        (14, 5, 4, 'O', "Orc Blademaster", false)
    } else if roll < 24 {
        (8, 6, 0, 'W', "Wraith", false)
    } else if roll < 32 {
        (12, 6, 3, 'N', "Naga", false)
    } else if roll < 40 {
        (16, 5, 3, 'T', "Troll", false)
    } else if roll < 48 {
        (18, 6, 4, 'E', "Ettin", false)
    } else if roll < 56 {
        (22, 5, 6, 'R', "Rock Golem", false)
    } else if roll < 64 {
        (16, 7, 3, 'Y', "Minotaur", false)
    } else if roll < 72 {
        (12, 7, 2, 'P', "Medusa", false)
    } else if roll < 80 {
        (10, 6, 1, 'Q', "Banshee", false)
    } else if roll < 86 {
        (11, 5, 2, '6', "Faceless Monk", false)
    } else if roll < 92 {
        (14, 7, 3, '7', "Unholy Cardinal", false)
    } else {
        (15, 6, 2, '8', "Writhing Mass", false)
    }
}
