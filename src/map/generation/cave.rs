use crate::config::MapGenConfig;
use super::super::{Map, Tile, Visibility};
use super::{ChaCha8Rng, SeedableRng, Rng, count_neighbors_of};

impl Map {
    /// Build cave tiles via cellular automata: random fill + smoothing passes.
    /// Used by both `generate` (simple cave) and `generate_cave` (dragon lair).
    fn cellular_automata_cave(width: i32, height: i32, seed: u64, cfg: &MapGenConfig) -> Vec<Tile> {
        let len = (width * height) as usize;
        let mut tiles = vec![Tile::Wall; len];
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        // Random fill — wall_pct% walls for interior cells
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                if rng.gen_range(0u64..100) >= cfg.cave_wall_pct {
                    tiles[(y * width + x) as usize] = Tile::Floor;
                }
            }
        }

        // Cellular automata smoothing
        for _ in 0..cfg.cave_smooth_passes {
            let prev = tiles.clone();
            for y in 1..height - 1 {
                for x in 1..width - 1 {
                    let walls = count_neighbors_of(&prev, width, x, y, Tile::Wall);
                    tiles[(y * width + x) as usize] = if walls >= cfg.cave_neighbor_threshold {
                        Tile::Wall
                    } else {
                        Tile::Floor
                    };
                }
            }
        }

        tiles
    }

    /// Generate a cave using cellular automata.
    /// `seed` drives the initial random fill so caves are reproducible in tests.
    #[cfg(test)]
    pub fn generate(width: i32, height: i32, seed: u64) -> Self {
        let cfg = MapGenConfig::normal();
        let tiles = Self::cellular_automata_cave(width, height, seed, &cfg);
        let len = tiles.len();
        Self { width, height, tiles, visibility: vec![Visibility::Hidden; len] }
    }

    /// Generate a dragon's lair cave using cellular automata.
    /// Ensures floor connectivity and places StairsUp. No StairsDown.
    pub fn generate_cave(width: i32, height: i32, seed: u64, cfg: &MapGenConfig) -> Self {
        let tiles = Self::cellular_automata_cave(width, height, seed, cfg);
        let len = tiles.len();
        let mut map = Self { width, height, tiles, visibility: vec![Visibility::Hidden; len] };

        // Keep only the largest connected floor region
        map.fill_isolated_floors();

        // Place StairsUp on the first floor tile found (near top-left)
        'outer: for y in 1..height - 1 {
            for x in 1..width - 1 {
                if map.get(x, y) == Tile::Floor {
                    map.set(x, y, Tile::StairsUp);
                    break 'outer;
                }
            }
        }

        map
    }
}
