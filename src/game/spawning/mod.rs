mod enemies;
mod overworld;
mod dungeon_tables;

pub(super) use overworld::is_rare_monster;

use crate::map::{DungeonBiome, Tile};
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
        let dungeon = &self.world.dungeons[dungeon_index];
        let biome = dungeon.biome;
        let total_levels = dungeon.levels.len();
        let is_cave = biome == DungeonBiome::DragonLair && level == total_levels - 1;
        let is_deepest = level == total_levels - 1;

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
                    let enemy = dungeon_tables::roll_biome_enemy(x, y, biome, level, rng);
                    self.enemies.push(enemy);
                }
            }
        }

        // Place unique dragon boss only in the cave level
        if is_cave {
            self.place_dragon_boss();
        }
        // Place guaranteed biome boss on deepest non-cave level
        else if is_deepest && biome != DungeonBiome::DragonLair {
            self.place_dungeon_boss(biome);
        }
    }

    /// Place a guaranteed biome boss on the deepest level, far from the player.
    fn place_dungeon_boss(&mut self, biome: DungeonBiome) {
        let stats = dungeon_tables::boss_for_biome(biome);
        let (hp, attack, def, glyph, name, ranged, behavior) = stats;
        let map = self.world.current_map();
        for y in (1..map.height - 1).rev() {
            for x in (1..map.width - 1).rev() {
                if map.is_walkable(x, y)
                    && map.get(x, y) == Tile::Floor
                    && (x - self.player_x).abs() + (y - self.player_y).abs() > 5
                    && !self.enemies.iter().any(|e| e.x == x && e.y == y)
                {
                    self.enemies.push(Enemy {
                        x, y, hp, attack, defense: def, glyph, name,
                        facing_left: false, is_ranged: ranged, behavior,
                        spawn_x: x, spawn_y: y, provoked: false, is_boss: true,
                    });
                    return;
                }
            }
        }
    }

    /// Place the dragon boss in the cave level, far from the player.
    fn place_dragon_boss(&mut self) {
        let c = &self.config.combat;
        let map = self.world.current_map();
        for y in (1..map.height - 1).rev() {
            for x in (1..map.width - 1).rev() {
                if map.is_walkable(x, y)
                    && map.get(x, y) == Tile::Floor
                    && (x - self.player_x).abs() + (y - self.player_y).abs() > c.dragon_min_distance
                    && !self.enemies.iter().any(|e| e.x == x && e.y == y)
                {
                    self.enemies.push(Enemy {
                        x, y, hp: c.dragon_hp, attack: c.dragon_attack, defense: c.dragon_defense,
                        glyph: 'D', name: "Dragon", facing_left: false, is_ranged: false,
                        behavior: crate::config::EnemyBehavior::Aggressive,
                        spawn_x: x, spawn_y: y, provoked: false, is_boss: true,
                    });
                    return;
                }
            }
        }
    }
}
