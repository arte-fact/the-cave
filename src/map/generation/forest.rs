use super::super::{Map, Tile, Visibility};
use super::{xorshift64, count_neighbors_of};

impl Map {
    /// Generate a forest overworld using cellular automata.
    /// Produces Tree and Grass tiles with organic clearings.
    /// Border is always dense trees. Isolated grass pockets are filled.
    pub fn generate_forest(width: i32, height: i32, seed: u64) -> Self {
        let len = (width * height) as usize;
        let mut tiles = vec![Tile::Tree; len];
        let mut rng = seed;

        // Step 1: random fill — ~55% trees for interior cells
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                rng = xorshift64(rng);
                if (rng % 100) >= 55 {
                    tiles[(y * width + x) as usize] = Tile::Grass;
                }
            }
        }

        // Step 2: cellular automata smoothing (4 passes)
        for _ in 0..4 {
            let prev = tiles.clone();
            for y in 1..height - 1 {
                for x in 1..width - 1 {
                    let trees = count_neighbors_of(&prev, width, x, y, Tile::Tree);
                    tiles[(y * width + x) as usize] = if trees >= 5 {
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
