use crate::game::{Enemy, GroundItem};
use crate::map::{Dungeon, DungeonStyle, Map};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Location {
    Overworld,
    Dungeon { index: usize, level: usize },
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
}

impl World {
    pub fn new(overworld: Map, dungeon_entrances: Vec<(i32, i32)>, seed: u64) -> Self {
        let mut dungeons = Vec::new();
        let mut rng = seed;
        // Exactly one dungeon gets a cave level (the dragon's lair)
        let cave_index = if dungeon_entrances.is_empty() {
            0
        } else {
            (seed % dungeon_entrances.len() as u64) as usize
        };
        for i in 0..dungeon_entrances.len() {
            rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
            let depth = 3;
            let has_cave = i == cave_index;
            dungeons.push(Dungeon::generate(depth, rng, has_cave));
        }
        Self {
            overworld,
            dungeons,
            dungeon_entrances,
            location: Location::Overworld,
            saved_overworld_pos: (0, 0),
            saved_overworld_enemies: Vec::new(),
            saved_overworld_items: Vec::new(),
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
mod tests {
    use super::*;

    fn test_world() -> World {
        let mut map = Map::generate_forest(200, 200, 42);
        let entrances = map.place_dungeons(42);
        map.build_roads(&entrances);
        World::new(map, entrances, 99)
    }

    #[test]
    fn world_starts_on_overworld() {
        let w = test_world();
        assert_eq!(w.location, Location::Overworld);
    }

    #[test]
    fn current_map_is_overworld_by_default() {
        let w = test_world();
        assert_eq!(w.current_map().width, 200);
    }

    #[test]
    fn dungeon_at_finds_entrance() {
        let w = test_world();
        let (x, y) = w.dungeon_entrances[0];
        assert_eq!(w.dungeon_at(x, y), Some(0));
    }

    #[test]
    fn dungeon_at_returns_none_for_non_entrance() {
        let w = test_world();
        assert_eq!(w.dungeon_at(0, 0), None);
    }

    #[test]
    fn dungeons_generated_for_each_entrance() {
        let w = test_world();
        assert_eq!(w.dungeons.len(), w.dungeon_entrances.len());
    }

    #[test]
    fn exactly_one_dungeon_has_cave() {
        let w = test_world();
        let cave_count = w.dungeons.iter().filter(|d| d.levels.len() == 4).count();
        let normal_count = w.dungeons.iter().filter(|d| d.levels.len() == 3).count();
        assert_eq!(cave_count, 1, "exactly one dungeon should have a cave level");
        assert_eq!(cave_count + normal_count, w.dungeons.len(),
            "all dungeons should have 3 or 4 levels");
    }

    #[test]
    fn current_map_changes_with_location() {
        let mut w = test_world();
        let ow_width = w.overworld.width;
        w.location = Location::Dungeon { index: 0, level: 0 };
        let dw = w.current_map().width;
        // Dungeon level 0 is 40 wide, overworld is 200
        assert_ne!(ow_width, dw);
        assert_eq!(dw, 40);
    }

    #[test]
    fn from_single_map_has_no_dungeons() {
        let map = Map::generate(30, 20, 42);
        let w = World::from_single_map(map);
        assert!(w.dungeons.is_empty());
        assert!(w.dungeon_entrances.is_empty());
        assert_eq!(w.location, Location::Overworld);
    }
}
