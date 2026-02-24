/// Map generation parameters for forest, cave, BSP dungeon, and road systems.
#[derive(Clone, Debug)]
pub struct MapGenConfig {
    // --- Overworld ---
    pub overworld_width: i32,
    pub overworld_height: i32,

    // --- Forest cellular automata ---
    /// Initial percentage of tree tiles (0-100).
    pub forest_tree_pct: u64,
    /// Number of cellular automata smoothing passes.
    pub forest_smooth_passes: usize,
    /// Neighbor threshold: if tree neighbors >= this, cell becomes tree.
    pub forest_neighbor_threshold: i32,

    // --- Cave cellular automata ---
    /// Initial percentage of wall tiles (0-100).
    pub cave_wall_pct: u64,
    /// Number of cellular automata smoothing passes.
    pub cave_smooth_passes: usize,
    /// Neighbor threshold: if wall neighbors >= this, cell becomes wall.
    pub cave_neighbor_threshold: i32,
    /// Default cave dimensions (dragon's lair).
    pub cave_width: i32,
    pub cave_height: i32,

    // --- BSP dungeon placement on overworld ---
    /// Minimum zone size for BSP subdivision.
    pub bsp_min_zone: i32,
    /// Chance (0-100) that a BSP zone gets a dungeon entrance.
    pub dungeon_place_chance_pct: u64,
    /// Minimum number of dungeon entrances guaranteed.
    pub dungeon_min_count: usize,

    // --- BSP dungeon interior ---
    /// Minimum room dimension for BSP room generation.
    pub bsp_min_room: i32,
    /// Number of BSP dungeon levels per dungeon.
    pub dungeon_depth: usize,
    /// (width, height) for each dungeon level. Index = level.
    /// Levels beyond the array length use the last entry.
    pub dungeon_level_sizes: [(i32, i32); 3],

    // --- Road A* costs ---
    pub road_cost_grass: i32,
    pub road_cost_tree: i32,
    pub road_cost_road: i32,
    pub road_cost_floor: i32,
    pub road_cost_entrance: i32,
}

impl MapGenConfig {
    pub fn normal() -> Self {
        Self {
            overworld_width: 200,
            overworld_height: 200,
            forest_tree_pct: 55,
            forest_smooth_passes: 4,
            forest_neighbor_threshold: 5,
            cave_wall_pct: 45,
            cave_smooth_passes: 5,
            cave_neighbor_threshold: 5,
            cave_width: 80,
            cave_height: 60,
            bsp_min_zone: 30,
            dungeon_place_chance_pct: 60,
            dungeon_min_count: 3,
            bsp_min_room: 5,
            dungeon_depth: 3,
            dungeon_level_sizes: [(40, 30), (50, 35), (60, 40)],
            road_cost_grass: 2,
            road_cost_tree: 6,
            road_cost_road: 1,
            road_cost_floor: 2,
            road_cost_entrance: 1,
        }
    }
}
