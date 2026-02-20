mod cave;
mod forest;
mod dungeon;
mod roads;

pub use dungeon::{Dungeon, DungeonStyle};

use super::{Map, Tile};

// --- Shared helpers ---

pub(super) fn xorshift64(mut state: u64) -> u64 {
    state ^= state << 13;
    state ^= state >> 7;
    state ^= state << 17;
    state
}

fn count_neighbors_of(tiles: &[Tile], width: i32, x: i32, y: i32, target: Tile) -> i32 {
    let height = tiles.len() as i32 / width;
    let mut count = 0;
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            let nx = x + dx;
            let ny = y + dy;
            if nx < 0 || ny < 0 || nx >= width || ny >= height {
                count += 1; // out-of-bounds counts as match
            } else if tiles[(ny * width + nx) as usize] == target {
                count += 1;
            }
        }
    }
    count
}

// --- Region connectivity helpers (shared by cave and forest) ---

impl Map {
    /// Flood-fill to find the largest connected grass region.
    /// Fill all other grass regions with trees.
    fn fill_isolated_grass(&mut self) {
        self.fill_isolated_tile(Tile::Grass, Tile::Tree);
    }

    /// Flood-fill to find the largest connected floor region.
    /// Fill all other floor regions with walls.
    fn fill_isolated_floors(&mut self) {
        self.fill_isolated_tile(Tile::Floor, Tile::Wall);
    }

    /// Generic: keep only the largest connected region of `target` tile,
    /// fill all smaller regions with `fill` tile.
    fn fill_isolated_tile(&mut self, target: Tile, fill: Tile) {
        let len = (self.width * self.height) as usize;
        let mut region_id = vec![0u32; len];
        let mut region_sizes: Vec<usize> = vec![0]; // index 0 unused
        let mut current_id = 0u32;

        for y in 0..self.height {
            for x in 0..self.width {
                let i = (y * self.width + x) as usize;
                if self.tiles[i] == target && region_id[i] == 0 {
                    current_id += 1;
                    let size = self.flood_fill_region_of(x, y, current_id, &mut region_id, target);
                    region_sizes.push(size);
                }
            }
        }

        if current_id == 0 {
            return;
        }

        // Find the largest region
        let largest = region_sizes
            .iter()
            .enumerate()
            .skip(1)
            .max_by_key(|&(_, &s)| s)
            .map(|(id, _)| id as u32)
            .unwrap_or(1);

        // Fill non-largest regions
        for (i, rid) in region_id.iter().enumerate().take(len) {
            if self.tiles[i] == target && *rid != largest {
                self.tiles[i] = fill;
            }
        }
    }

    fn flood_fill_region_of(&self, sx: i32, sy: i32, id: u32, region_id: &mut [u32], target: Tile) -> usize {
        let mut stack = vec![(sx, sy)];
        let mut count = 0;
        while let Some((x, y)) = stack.pop() {
            let i = (y * self.width + x) as usize;
            if region_id[i] != 0 {
                continue;
            }
            if self.tiles[i] != target {
                continue;
            }
            region_id[i] = id;
            count += 1;
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let (nx, ny) = (x + dx, y + dy);
                if nx >= 0 && ny >= 0 && nx < self.width && ny < self.height {
                    stack.push((nx, ny));
                }
            }
        }
        count
    }
}
