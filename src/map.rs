#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    Wall,
    Floor,
}

pub struct Map {
    pub width: i32,
    pub height: i32,
    tiles: Vec<Tile>,
}

impl Map {
    /// Generate a cave using cellular automata.
    /// `seed` drives the initial random fill so caves are reproducible in tests.
    pub fn generate(width: i32, height: i32, seed: u64) -> Self {
        let len = (width * height) as usize;
        let mut tiles = vec![Tile::Wall; len];
        let mut rng = seed;

        // Step 1: random fill — ~45% walls for interior cells
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                rng = xorshift64(rng);
                if (rng % 100) >= 45 {
                    tiles[(y * width + x) as usize] = Tile::Floor;
                }
            }
        }

        // Step 2: cellular automata smoothing (5 passes)
        for _ in 0..5 {
            let prev = tiles.clone();
            for y in 1..height - 1 {
                for x in 1..width - 1 {
                    let walls = count_neighbors(&prev, width, x, y);
                    tiles[(y * width + x) as usize] = if walls >= 5 {
                        Tile::Wall
                    } else {
                        Tile::Floor
                    };
                }
            }
        }

        Self { width, height, tiles }
    }

    pub fn get(&self, x: i32, y: i32) -> Tile {
        self.tiles[(y * self.width + x) as usize]
    }

    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width && y < self.height && self.get(x, y) == Tile::Floor
    }

    /// Find a floor tile to spawn the player on, searching from the center outward.
    pub fn find_spawn(&self) -> (i32, i32) {
        let cx = self.width / 2;
        let cy = self.height / 2;
        let max_r = self.width.max(self.height);
        for r in 0..max_r {
            for dy in -r..=r {
                for dx in -r..=r {
                    let x = cx + dx;
                    let y = cy + dy;
                    if self.is_walkable(x, y) {
                        return (x, y);
                    }
                }
            }
        }
        (cx, cy)
    }
}

fn xorshift64(mut state: u64) -> u64 {
    state ^= state << 13;
    state ^= state >> 7;
    state ^= state << 17;
    state
}

fn count_neighbors(tiles: &[Tile], width: i32, x: i32, y: i32) -> i32 {
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x + dx;
            let ny = y + dy;
            if nx < 0 || ny < 0 || nx >= width || ny >= (tiles.len() as i32 / width) {
                count += 1; // out-of-bounds counts as wall
            } else if tiles[(ny * width + nx) as usize] == Tile::Wall {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn borders_are_walls() {
        let map = Map::generate(30, 20, 42);
        // Top and bottom rows
        for x in 0..map.width {
            assert_eq!(map.get(x, 0), Tile::Wall, "top border at x={x}");
            assert_eq!(map.get(x, map.height - 1), Tile::Wall, "bottom border at x={x}");
        }
        // Left and right columns
        for y in 0..map.height {
            assert_eq!(map.get(0, y), Tile::Wall, "left border at y={y}");
            assert_eq!(map.get(map.width - 1, y), Tile::Wall, "right border at y={y}");
        }
    }

    #[test]
    fn has_floor_tiles() {
        let map = Map::generate(30, 20, 42);
        let floor_count = (0..map.height)
            .flat_map(|y| (0..map.width).map(move |x| (x, y)))
            .filter(|&(x, y)| map.get(x, y) == Tile::Floor)
            .count();
        // At least 20% of interior should be walkable
        let interior = ((map.width - 2) * (map.height - 2)) as usize;
        assert!(
            floor_count > interior / 5,
            "too few floors: {floor_count} out of {interior} interior tiles"
        );
    }

    #[test]
    fn spawn_is_on_floor() {
        let map = Map::generate(30, 20, 42);
        let (sx, sy) = map.find_spawn();
        assert_eq!(map.get(sx, sy), Tile::Floor);
    }

    #[test]
    fn out_of_bounds_not_walkable() {
        let map = Map::generate(30, 20, 42);
        assert!(!map.is_walkable(-1, 0));
        assert!(!map.is_walkable(0, -1));
        assert!(!map.is_walkable(map.width, 0));
        assert!(!map.is_walkable(0, map.height));
    }

    #[test]
    fn walls_not_walkable_floors_walkable() {
        let map = Map::generate(30, 20, 42);
        for y in 0..map.height {
            for x in 0..map.width {
                match map.get(x, y) {
                    Tile::Wall => assert!(!map.is_walkable(x, y)),
                    Tile::Floor => assert!(map.is_walkable(x, y)),
                }
            }
        }
    }

    #[test]
    fn deterministic_with_same_seed() {
        let a = Map::generate(30, 20, 99);
        let b = Map::generate(30, 20, 99);
        for y in 0..a.height {
            for x in 0..a.width {
                assert_eq!(a.get(x, y), b.get(x, y), "mismatch at ({x},{y})");
            }
        }
    }

    #[test]
    fn different_seed_different_map() {
        let a = Map::generate(30, 20, 1);
        let b = Map::generate(30, 20, 2);
        let diffs = (0..a.height)
            .flat_map(|y| (0..a.width).map(move |x| (x, y)))
            .filter(|&(x, y)| a.get(x, y) != b.get(x, y))
            .count();
        assert!(diffs > 0, "different seeds should produce different maps");
    }
}
