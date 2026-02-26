use crate::game::{Enemy, GroundItem, ItemKind};
use crate::map::{Dungeon, DungeonBiome, DungeonStyle, Map};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Location {
    Overworld,
    Dungeon { index: usize, level: usize },
}

/// Per-dungeon saved state for enemy/item persistence across level transitions.
pub struct DungeonState {
    /// Per-level saved enemies. `None` = unvisited (spawn fresh).
    pub enemies: Vec<Option<Vec<Enemy>>>,
    /// Per-level saved ground items. `None` = unvisited (spawn fresh).
    pub items: Vec<Option<Vec<GroundItem>>>,
}

pub struct World {
    pub overworld: Map,
    pub dungeons: Vec<Dungeon>,
    pub dungeon_entrances: Vec<(i32, i32)>,
    pub location: Location,
    /// Player position saved when entering a dungeon.
    pub saved_overworld_pos: (i32, i32),
    /// Overworld enemies saved when entering a dungeon.
    pub saved_overworld_enemies: Vec<Enemy>,
    /// Overworld ground items saved when entering a dungeon.
    pub saved_overworld_items: Vec<GroundItem>,
    /// Legendary equipment slot assigned to each regular (non-DragonLair) dungeon.
    /// Index matches dungeon index. `None` for the DragonLair dungeon.
    pub legendary_slots: Vec<Option<ItemKind>>,
    /// Per-dungeon level persistence (enemies & items survive transitions).
    pub dungeon_states: Vec<DungeonState>,
}

impl World {
    pub fn new(overworld: Map, dungeon_entrances: Vec<(i32, i32)>, seed: u64, mapgen: &crate::config::MapGenConfig) -> Self {
        let mut dungeons = Vec::new();
        let mut rng = seed;
        // Exactly one dungeon gets a cave level (the dragon's lair)
        let cave_index = if dungeon_entrances.is_empty() {
            0
        } else {
            (seed % dungeon_entrances.len() as u64) as usize
        };

        // Pick unique biomes for the regular (non-DragonLair) dungeons
        let regular_count = if dungeon_entrances.is_empty() { 0 } else { dungeon_entrances.len() - 1 };
        let unique_biomes = DungeonBiome::select_unique(regular_count, seed);
        let mut biome_iter = unique_biomes.iter();

        // Legendary slots: Helmet, Armor, Shield, Boots — one per regular dungeon
        let legendary_kinds = [ItemKind::Helmet, ItemKind::Armor, ItemKind::Shield, ItemKind::Boots];
        let mut legendary_slots = Vec::new();
        let mut legend_idx = 0;

        for (i, &(_ex, _ey)) in dungeon_entrances.iter().enumerate() {
            rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
            let depth = mapgen.dungeon_depth;
            let has_cave = i == cave_index;
            let biome = if has_cave {
                DungeonBiome::DragonLair
            } else {
                *biome_iter.next().unwrap_or(&DungeonBiome::GoblinWarren)
            };
            dungeons.push(Dungeon::generate(depth, rng, has_cave, biome, mapgen));

            if has_cave {
                legendary_slots.push(None);
            } else {
                let slot = legendary_kinds.get(legend_idx).cloned();
                legendary_slots.push(slot);
                legend_idx += 1;
            }
        }

        let dungeon_states = dungeons.iter().map(|d| DungeonState {
            enemies: vec![None; d.levels.len()],
            items: vec![None; d.levels.len()],
        }).collect();

        Self {
            overworld,
            dungeons,
            dungeon_entrances,
            location: Location::Overworld,
            saved_overworld_pos: (0, 0),
            saved_overworld_enemies: Vec::new(),
            saved_overworld_items: Vec::new(),
            legendary_slots,
            dungeon_states,
        }
    }

    /// Minimal world for tests: just a single map, no dungeons.
    #[cfg(test)]
    pub fn from_single_map(map: Map) -> Self {
        Self {
            overworld: map,
            dungeons: Vec::new(),
            dungeon_entrances: Vec::new(),
            location: Location::Overworld,
            saved_overworld_pos: (0, 0),
            saved_overworld_enemies: Vec::new(),
            saved_overworld_items: Vec::new(),
            legendary_slots: Vec::new(),
            dungeon_states: Vec::new(),
        }
    }

    pub fn current_map(&self) -> &Map {
        match &self.location {
            Location::Overworld => &self.overworld,
            Location::Dungeon { index, level } => &self.dungeons[*index].levels[*level],
        }
    }

    pub fn current_map_mut(&mut self) -> &mut Map {
        match self.location.clone() {
            Location::Overworld => &mut self.overworld,
            Location::Dungeon { index, level } => &mut self.dungeons[index].levels[level],
        }
    }

    /// Save enemies and items for a dungeon level (persistence across transitions).
    pub fn save_level(&mut self, dungeon_index: usize, level: usize, enemies: Vec<Enemy>, items: Vec<GroundItem>) {
        let state = &mut self.dungeon_states[dungeon_index];
        state.enemies[level] = Some(enemies);
        state.items[level] = Some(items);
    }

    /// Take saved state for a dungeon level, leaving `None` (so next visit spawns fresh if not re-saved).
    pub fn take_level(&mut self, dungeon_index: usize, level: usize) -> Option<(Vec<Enemy>, Vec<GroundItem>)> {
        let state = &mut self.dungeon_states[dungeon_index];
        let enemies = state.enemies[level].take()?;
        let items = state.items[level].take()?;
        Some((enemies, items))
    }

    /// Find dungeon index for an overworld entrance position.
    pub fn dungeon_at(&self, x: i32, y: i32) -> Option<usize> {
        self.dungeon_entrances
            .iter()
            .position(|&(ex, ey)| ex == x && ey == y)
    }

    /// Get the visual style of the current dungeon level.
    /// Returns None on the overworld.
    pub fn current_dungeon_style(&self) -> Option<DungeonStyle> {
        match &self.location {
            Location::Overworld => None,
            Location::Dungeon { index, level } => {
                self.dungeons.get(*index)
                    .and_then(|d| d.styles.get(*level))
                    .copied()
            }
        }
    }
}

#[cfg(test)]
mod tests;
