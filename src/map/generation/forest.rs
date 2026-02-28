use crate::config::MapGenConfig;
use super::super::{Map, Tile, Visibility};
use super::{ChaCha8Rng, SeedableRng, Rng, count_neighbors_of};

impl Map {
    /// Generate a forest overworld using cellular automata.
    /// Produces Tree and Grass tiles with organic clearings.
    /// Border is always dense trees. Isolated grass pockets are filled.
    pub fn generate_forest(width: i32, height: i32, seed: u64, cfg: &MapGenConfig) -> Self {
        let len = (width * height) as usize;
        let mut tiles = vec![Tile::Tree; len];
        let mut rng = ChaCha8Rng::seed_from_u64(seed);

        // Step 1: random fill — tree_pct% trees for interior cells
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                if rng.gen_range(0u64..100) >= cfg.forest_tree_pct {
                    tiles[(y * width + x) as usize] = Tile::Grass;
                }
            }
        }

        // Step 2: cellular automata smoothing
        for _ in 0..cfg.forest_smooth_passes {
            let prev = tiles.clone();
            for y in 1..height - 1 {
                for x in 1..width - 1 {
                    let trees = count_neighbors_of(&prev, width, x, y, Tile::Tree);
                    tiles[(y * width + x) as usize] = if trees >= cfg.forest_neighbor_threshold {
                        Tile::Tree
                    } else {
                        Tile::Grass
                    };
                }
            }
        }

        let mut map = Self { width, height, visibility: vec![Visibility::Hidden; tiles.len()], tiles };

        // Step 3: ensure connectivity — keep only the largest grass region
        map.fill_isolated_grass();

        map
    }
}
