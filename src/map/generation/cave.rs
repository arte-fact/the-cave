use super::super::{Map, Tile, Visibility};
use super::{xorshift64, count_neighbors_of};

impl Map {
    /// Build cave tiles via cellular automata: random fill + smoothing passes.
    /// Used by both `generate` (simple cave) and `generate_cave` (dragon lair).
    fn cellular_automata_cave(width: i32, height: i32, seed: u64) -> Vec<Tile> {
        let len = (width * height) as usize;
        let mut tiles = vec![Tile::Wall; len];
        let mut rng = seed;

        // Random fill — ~45% walls for interior cells
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                rng = xorshift64(rng);
                if (rng % 100) >= 45 {
                    tiles[(y * width + x) as usize] = Tile::Floor;
                }
            }
        }

        // Cellular automata smoothing (5 passes)
        for _ in 0..5 {
            let prev = tiles.clone();
            for y in 1..height - 1 {
                for x in 1..width - 1 {
                    let walls = count_neighbors_of(&prev, width, x, y, Tile::Wall);
                    tiles[(y * width + x) as usize] = if walls >= 5 {
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
        let tiles = Self::cellular_automata_cave(width, height, seed);
        let len = tiles.len();
        Self { width, height, tiles, visibility: vec![Visibility::Hidden; len] }
    }

    /// Generate a dragon's lair cave using cellular automata.
    /// Ensures floor connectivity and places StairsUp. No StairsDown.
    pub fn generate_cave(width: i32, height: i32, seed: u64) -> Self {
        let tiles = Self::cellular_automata_cave(width, height, seed);
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
