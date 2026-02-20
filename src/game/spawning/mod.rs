mod overworld;
mod dungeon_tables;

use crate::map::Tile;
use super::types::*;
use super::{Game, xorshift64};

impl Game {
    /// Spawn creatures on the overworld, using biome-appropriate tables.
    pub fn spawn_enemies(&mut self, seed: u64) {
        let map = self.world.current_map();
        let map_height = map.height;
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
                if rng % 100 < self.config.spawn.overworld_enemy_pct {
                    rng = xorshift64(rng);
                    let enemy = overworld::roll_overworld_enemy(x, y, rng, map_height);
                    self.enemies.push(enemy);
                }
            }
        }
    }

    /// Spawn enemies appropriate for a dungeon level, using the dungeon's biome.
    pub(super) fn spawn_dungeon_enemies(&mut self, dungeon_index: usize, level: usize) {
        let total_levels = self.world.dungeons[dungeon_index].levels.len();
        let is_cave = total_levels == 4 && level == 3;
        let biome = self.world.dungeons[dungeon_index].biome;

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
                        dungeon_tables::roll_cave_enemy(x, y, rng)
                    } else {
                        dungeon_tables::roll_biome_enemy(x, y, biome, level, rng)
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
