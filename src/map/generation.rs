use super::{Map, Tile, Visibility};

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
        for i in 0..len {
            if self.tiles[i] == target && region_id[i] != largest {
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

    /// Place small dungeon entrance structures on a forest map using BSP zone partitioning.
    /// Returns the list of dungeon entrance positions.
    pub fn place_dungeons(&mut self, seed: u64) -> Vec<(i32, i32)> {
        let mut rng = seed;
        let zones = bsp_subdivide(2, 2, self.width - 4, self.height - 4, 30, &mut rng);
        let mut entrances = Vec::new();

        for zone in &zones {
            rng = xorshift64(rng);
            // ~60% chance a zone gets a dungeon
            if rng % 100 >= 60 {
                continue;
            }

            // Place a small stone entrance structure (3x2) at zone center:
            //   W W W
            //   W > W
            let cx = zone.0 + zone.2 / 2;
            let cy = zone.1 + zone.3 / 2;

            if cx < 2 || cy < 2 || cx >= self.width - 2 || cy + 2 >= self.height - 1 {
                continue;
            }

            // Top row: 3 walls
            for dx in -1..=1 {
                self.set(cx + dx, cy, Tile::Wall);
            }
            // Bottom row: wall | entrance | wall
            self.set(cx - 1, cy + 1, Tile::Wall);
            self.set(cx, cy + 1, Tile::DungeonEntrance);
            self.set(cx + 1, cy + 1, Tile::Wall);

            // Small grass clearing below entrance for road connection
            for dx in -1..=1 {
                let ny = cy + 2;
                if self.get(cx + dx, ny) == Tile::Tree {
                    self.set(cx + dx, ny, Tile::Grass);
                }
            }

            entrances.push((cx, cy + 1));
        }

        // Guarantee at least 3 dungeons by retrying with offset seed
        if entrances.len() < 3 {
            let extra = self.place_dungeons(seed.wrapping_add(7));
            entrances.extend(extra);
        }

        entrances
    }

    /// Build roads connecting all dungeon entrances using MST + weighted A*.
    /// Cost: grass=1, tree=3, road=0 (reuse), wall/other=impassable.
    pub fn build_roads(&mut self, entrances: &[(i32, i32)]) {
        if entrances.len() < 2 {
            return;
        }

        // Build MST using Prim's algorithm on Euclidean distances
        let n = entrances.len();
        let mut in_tree = vec![false; n];
        let mut min_cost = vec![f64::MAX; n];
        let mut min_edge = vec![0usize; n]; // which tree node is closest
        in_tree[0] = true;

        for i in 1..n {
            let dx = (entrances[i].0 - entrances[0].0) as f64;
            let dy = (entrances[i].1 - entrances[0].1) as f64;
            min_cost[i] = (dx * dx + dy * dy).sqrt();
            min_edge[i] = 0;
        }

        let mut edges: Vec<(usize, usize)> = Vec::new();
        for _ in 1..n {
            // Find cheapest edge to non-tree node
            let mut best = usize::MAX;
            let mut best_cost = f64::MAX;
            for i in 0..n {
                if !in_tree[i] && min_cost[i] < best_cost {
                    best_cost = min_cost[i];
                    best = i;
                }
            }
            if best == usize::MAX {
                break;
            }
            in_tree[best] = true;
            edges.push((min_edge[best], best));

            // Update costs
            for i in 0..n {
                if !in_tree[i] {
                    let dx = (entrances[i].0 - entrances[best].0) as f64;
                    let dy = (entrances[i].1 - entrances[best].1) as f64;
                    let dist = (dx * dx + dy * dy).sqrt();
                    if dist < min_cost[i] {
                        min_cost[i] = dist;
                        min_edge[i] = best;
                    }
                }
            }
        }

        // For each MST edge, run weighted A* and carve the road
        for (a, b) in edges {
            let path = self.weighted_path(entrances[a], entrances[b]);
            for (x, y) in path {
                let tile = self.get(x, y);
                if tile == Tile::Grass || tile == Tile::Tree {
                    self.set(x, y, Tile::Road);
                }
            }
        }
    }

    /// Weighted A* for road carving. Grass=1, Tree=3, Road=0.5, Wall/border=impassable.
    /// Allows pathing through trees (to carve roads) and grass.
    fn weighted_path(&self, start: (i32, i32), goal: (i32, i32)) -> Vec<(i32, i32)> {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;

        let idx = |x: i32, y: i32| (y * self.width + x) as usize;
        let len = (self.width * self.height) as usize;
        let mut g_score = vec![i32::MAX; len];
        let mut came_from: Vec<(i32, i32)> = vec![(-1, -1); len];
        let heuristic = |x: i32, y: i32| (x - goal.0).abs() + (y - goal.1).abs();

        fn tile_cost(tile: Tile) -> Option<i32> {
            match tile {
                Tile::Grass => Some(2),
                Tile::Tree => Some(6),
                Tile::Road => Some(1),
                Tile::Floor => Some(2),
                Tile::DungeonEntrance => Some(1),
                _ => None, // Wall, etc = impassable
            }
        }

        g_score[idx(start.0, start.1)] = 0;
        let mut open = BinaryHeap::new();
        open.push(Reverse((heuristic(start.0, start.1), 0, start.0, start.1)));

        while let Some(Reverse((_f, g, x, y))) = open.pop() {
            if (x, y) == goal {
                let mut path = vec![goal];
                let mut cur = goal;
                while cur != start {
                    cur = came_from[idx(cur.0, cur.1)];
                    path.push(cur);
                }
                path.reverse();
                return path;
            }

            if g > g_score[idx(x, y)] {
                continue;
            }

            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let (nx, ny) = (x + dx, y + dy);
                if nx <= 0 || ny <= 0 || nx >= self.width - 1 || ny >= self.height - 1 {
                    continue; // stay off the border
                }
                let cost = match tile_cost(self.get(nx, ny)) {
                    Some(c) => c,
                    None => continue,
                };
                let ng = g + cost;
                let ni = idx(nx, ny);
                if ng < g_score[ni] {
                    g_score[ni] = ng;
                    came_from[ni] = (x, y);
                    open.push(Reverse((ng + heuristic(nx, ny), ng, nx, ny)));
                }
            }
        }

        Vec::new()
    }

    /// Find a walkable tile near the center for spawning.
    /// Prefers Road tiles, then any walkable tile.
    pub fn find_road_spawn(&self) -> (i32, i32) {
        let cx = self.width / 2;
        let cy = self.height / 2;
        let max_r = self.width.max(self.height);
        // First pass: look for Road near center
        for r in 0..max_r {
            for dy in -r..=r {
                for dx in -r..=r {
                    let x = cx + dx;
                    let y = cy + dy;
                    if x >= 0 && y >= 0 && x < self.width && y < self.height
                        && self.get(x, y) == Tile::Road
                    {
                        return (x, y);
                    }
                }
            }
        }
        // Fallback: any walkable tile
        self.find_spawn()
    }

    /// Generate a BSP dungeon level.
    /// Recursive BSP splits create rooms connected by L-shaped corridors.
    pub fn generate_bsp_dungeon(width: i32, height: i32, seed: u64, level: usize, total_levels: usize) -> Self {
        let mut map = Map::new_filled(width, height, Tile::Wall);
        let mut rng = seed;

        // BSP split into rooms
        let min_room = 5;
        let rooms = bsp_rooms(1, 1, width - 2, height - 2, min_room, &mut rng);

        // Carve rooms
        for &(rx, ry, rw, rh) in &rooms {
            for y in ry..ry + rh {
                for x in rx..rx + rw {
                    map.set(x, y, Tile::Floor);
                }
            }
        }

        // Connect rooms with L-shaped corridors between BSP siblings
        for i in 1..rooms.len() {
            let (ax, ay, aw, ah) = rooms[i - 1];
            let (bx, by, bw, bh) = rooms[i];
            let cx1 = ax + aw / 2;
            let cy1 = ay + ah / 2;
            let cx2 = bx + bw / 2;
            let cy2 = by + bh / 2;

            // Horizontal then vertical
            let xr = if cx1 < cx2 { cx1..=cx2 } else { cx2..=cx1 };
            for x in xr {
                if x > 0 && x < width - 1 && cy1 > 0 && cy1 < height - 1 {
                    map.set(x, cy1, Tile::Floor);
                }
            }
            let yr = if cy1 < cy2 { cy1..=cy2 } else { cy2..=cy1 };
            for y in yr {
                if cx2 > 0 && cx2 < width - 1 && y > 0 && y < height - 1 {
                    map.set(cx2, y, Tile::Floor);
                }
            }
        }

        // Place stairs
        if rooms.len() >= 2 {
            // StairsUp in the first room (connects to previous level or overworld)
            let (rx, ry, rw, rh) = rooms[0];
            map.set(rx + rw / 2, ry + rh / 2, Tile::StairsUp);

            // StairsDown in the last room (if not the deepest level)
            if level < total_levels - 1 {
                let (rx, ry, rw, rh) = rooms[rooms.len() - 1];
                map.set(rx + rw / 2, ry + rh / 2, Tile::StairsDown);
            }
        }

        map
    }
}

/// A dungeon complex with multiple levels, each a self-contained Map.
pub struct Dungeon {
    pub levels: Vec<Map>,
    pub entrance: (i32, i32), // overworld position
}

impl Dungeon {
    /// Generate a dungeon with `depth` BSP levels.
    /// Level 0 = 40x30, level 1 = 50x35, level 2 = 60x40.
    /// If `has_cave` is true, appends a cellular automata cave (80x60)
    /// as the deepest level — the dragon's lair.
    pub fn generate(entrance: (i32, i32), depth: usize, seed: u64, has_cave: bool) -> Self {
        let mut levels = Vec::new();
        let mut rng = seed;

        // Total levels: BSP depth + optional cave
        let total = if has_cave { depth + 1 } else { depth };

        for level in 0..depth {
            let (w, h) = match level {
                0 => (40, 30),
                1 => (50, 35),
                _ => (60, 40),
            };
            rng = xorshift64(rng);
            let map = Map::generate_bsp_dungeon(w, h, rng, level, total);
            levels.push(map);
        }

        // Append cave level if this is the dragon's dungeon
        if has_cave {
            rng = xorshift64(rng);
            let cave = Map::generate_cave(80, 60, rng);
            levels.push(cave);
        }

        Dungeon { levels, entrance }
    }
}

/// BSP room generation: recursively split a rectangle, returning leaf rooms.
fn bsp_rooms(x: i32, y: i32, w: i32, h: i32, min_room: i32, rng: &mut u64) -> Vec<(i32, i32, i32, i32)> {
    // Minimum room size with 1-tile padding for walls
    let min_split = min_room * 2 + 1;

    if w < min_split && h < min_split {
        // Leaf node — create a room with some random shrinkage
        *rng = xorshift64(*rng);
        let pad_x = if w > min_room + 2 { (*rng as i32 % 2).abs() } else { 0 };
        *rng = xorshift64(*rng);
        let pad_y = if h > min_room + 2 { (*rng as i32 % 2).abs() } else { 0 };
        let rw = (w - pad_x * 2).max(min_room);
        let rh = (h - pad_y * 2).max(min_room);
        return vec![(x + pad_x, y + pad_y, rw, rh)];
    }

    *rng = xorshift64(*rng);
    let split_h = if w < min_split {
        false
    } else if h < min_split {
        true
    } else {
        (*rng % 2) == 0
    };

    *rng = xorshift64(*rng);
    if split_h {
        let split = min_room + 1 + (*rng as i32 % (w - min_split + 1).max(1)).abs();
        let split = split.min(w - min_room - 1);
        let mut rooms = bsp_rooms(x, y, split, h, min_room, rng);
        rooms.extend(bsp_rooms(x + split, y, w - split, h, min_room, rng));
        rooms
    } else {
        let split = min_room + 1 + (*rng as i32 % (h - min_split + 1).max(1)).abs();
        let split = split.min(h - min_room - 1);
        let mut rooms = bsp_rooms(x, y, w, split, min_room, rng);
        rooms.extend(bsp_rooms(x, y + split, w, h - split, min_room, rng));
        rooms
    }
}

/// BSP subdivide a rectangle into zones of at least `min_size` in each dimension.
/// Returns a list of (x, y, w, h) leaf rectangles.
fn bsp_subdivide(x: i32, y: i32, w: i32, h: i32, min_size: i32, rng: &mut u64) -> Vec<(i32, i32, i32, i32)> {
    // Too small to split further
    if w < min_size * 2 && h < min_size * 2 {
        return vec![(x, y, w, h)];
    }

    *rng = xorshift64(*rng);
    // Prefer splitting the longer dimension
    let split_h = if w < min_size * 2 {
        false
    } else if h < min_size * 2 {
        true
    } else {
        (*rng % 2) == 0
    };

    *rng = xorshift64(*rng);
    if split_h {
        let split = min_size + (*rng as i32 % (w - min_size * 2 + 1)).abs();
        let mut result = bsp_subdivide(x, y, split, h, min_size, rng);
        result.extend(bsp_subdivide(x + split, y, w - split, h, min_size, rng));
        result
    } else {
        let split = min_size + (*rng as i32 % (h - min_size * 2 + 1)).abs();
        let mut result = bsp_subdivide(x, y, w, split, min_size, rng);
        result.extend(bsp_subdivide(x, y + split, w, h - split, min_size, rng));
        result
    }
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

fn xorshift64(mut state: u64) -> u64 {
    state ^= state << 13;
    state ^= state >> 7;
    state ^= state << 17;
    state
}
