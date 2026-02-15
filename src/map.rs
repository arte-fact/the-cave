#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    Wall,
    Floor,
    Tree,
    Grass,
    Road,
    DungeonEntrance,
    StairsDown,
    StairsUp,
}

impl Tile {
    pub fn is_walkable(self) -> bool {
        matches!(self, Tile::Floor | Tile::Grass | Tile::Road | Tile::DungeonEntrance | Tile::StairsDown | Tile::StairsUp)
    }

    pub fn glyph(self) -> char {
        match self {
            Tile::Wall => '#',
            Tile::Floor => '.',
            Tile::Tree => 'T',
            Tile::Grass => '.',
            Tile::Road => '=',
            Tile::DungeonEntrance => '>',
            Tile::StairsDown => '>',
            Tile::StairsUp => '<',
        }
    }

    pub fn color(self) -> &'static str {
        match self {
            Tile::Wall => "#333",
            Tile::Floor => "#111",
            Tile::Tree => "#050",
            Tile::Grass => "#141",
            Tile::Road => "#543",
            Tile::DungeonEntrance => "#a70",
            Tile::StairsDown => "#88f",
            Tile::StairsUp => "#88f",
        }
    }
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

    /// Create an empty map filled with a single tile type.
    pub fn new_filled(width: i32, height: i32, tile: Tile) -> Self {
        Self {
            width,
            height,
            tiles: vec![tile; (width * height) as usize],
        }
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

        let mut map = Self { width, height, tiles };

        // Step 3: ensure connectivity — keep only the largest grass region
        map.fill_isolated_grass();

        map
    }

    /// Flood-fill to find the largest connected grass region.
    /// Fill all other grass regions with trees.
    fn fill_isolated_grass(&mut self) {
        let len = (self.width * self.height) as usize;
        let mut region_id = vec![0u32; len];
        let mut region_sizes: Vec<usize> = vec![0]; // index 0 unused
        let mut current_id = 0u32;

        for y in 0..self.height {
            for x in 0..self.width {
                let i = (y * self.width + x) as usize;
                if self.tiles[i] == Tile::Grass && region_id[i] == 0 {
                    current_id += 1;
                    let size = self.flood_fill_region(x, y, current_id, &mut region_id);
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

        // Fill non-largest grass with trees
        for i in 0..len {
            if self.tiles[i] == Tile::Grass && region_id[i] != largest {
                self.tiles[i] = Tile::Tree;
            }
        }
    }

    fn flood_fill_region(&self, sx: i32, sy: i32, id: u32, region_id: &mut [u32]) -> usize {
        let mut stack = vec![(sx, sy)];
        let mut count = 0;
        while let Some((x, y)) = stack.pop() {
            let i = (y * self.width + x) as usize;
            if region_id[i] != 0 {
                continue;
            }
            if self.tiles[i] != Tile::Grass {
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

    /// Place dungeon footprints on a forest map using BSP zone partitioning.
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

            // Place a 15×15 dungeon footprint centered in the zone
            let fw = 15.min(zone.2 - 4);
            let fh = 15.min(zone.3 - 4);
            if fw < 8 || fh < 8 {
                continue;
            }
            let fx = zone.0 + (zone.2 - fw) / 2;
            let fy = zone.1 + (zone.3 - fh) / 2;

            // Carve the footprint as Floor (dungeon interior on overworld)
            for y in fy..fy + fh {
                for x in fx..fx + fw {
                    self.set(x, y, Tile::Floor);
                }
            }
            // Surround with walls
            for x in (fx - 1)..=(fx + fw) {
                if x >= 0 && x < self.width {
                    if fy - 1 >= 0 {
                        self.set(x, fy - 1, Tile::Wall);
                    }
                    if fy + fh < self.height {
                        self.set(x, fy + fh, Tile::Wall);
                    }
                }
            }
            for y in (fy - 1)..=(fy + fh) {
                if y >= 0 && y < self.height {
                    if fx - 1 >= 0 {
                        self.set(fx - 1, y, Tile::Wall);
                    }
                    if fx + fw < self.width {
                        self.set(fx + fw, y, Tile::Wall);
                    }
                }
            }

            // Place entrance on the south edge of the footprint, centered
            let ex = fx + fw / 2;
            let ey = fy + fh; // one tile below the footprint
            if ey < self.height - 1 {
                self.set(ex, ey, Tile::DungeonEntrance);
                // Ensure the tile below entrance is walkable (grass) for road connection
                if ey + 1 < self.height - 1 {
                    self.set(ex, ey + 1, Tile::Grass);
                }
                entrances.push((ex, ey));
            }
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

    pub fn get(&self, x: i32, y: i32) -> Tile {
        self.tiles[(y * self.width + x) as usize]
    }

    pub fn set(&mut self, x: i32, y: i32, tile: Tile) {
        self.tiles[(y * self.width + x) as usize] = tile;
    }

    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width && y < self.height && self.get(x, y).is_walkable()
    }

    /// A* pathfinding. Returns the path from `start` to `goal` (inclusive of both),
    /// or empty vec if unreachable. Moves in 4 cardinal directions only.
    pub fn find_path(&self, start: (i32, i32), goal: (i32, i32)) -> Vec<(i32, i32)> {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;

        if !self.is_walkable(goal.0, goal.1) {
            return Vec::new();
        }
        if start == goal {
            return vec![start];
        }

        let idx = |x: i32, y: i32| (y * self.width + x) as usize;
        let len = (self.width * self.height) as usize;
        let mut g_score = vec![i32::MAX; len];
        let mut came_from: Vec<(i32, i32)> = vec![(-1, -1); len];
        let heuristic = |x: i32, y: i32| (x - goal.0).abs() + (y - goal.1).abs();

        g_score[idx(start.0, start.1)] = 0;
        // (f_score, g, x, y) — Reverse for min-heap
        let mut open = BinaryHeap::new();
        open.push(Reverse((heuristic(start.0, start.1), 0, start.0, start.1)));

        while let Some(Reverse((_f, g, x, y))) = open.pop() {
            if (x, y) == goal {
                // Reconstruct path
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
                continue; // stale entry
            }

            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let (nx, ny) = (x + dx, y + dy);
                if !self.is_walkable(nx, ny) {
                    continue;
                }
                let ng = g + 1;
                let ni = idx(nx, ny);
                if ng < g_score[ni] {
                    g_score[ni] = ng;
                    came_from[ni] = (x, y);
                    open.push(Reverse((ng + heuristic(nx, ny), ng, nx, ny)));
                }
            }
        }

        Vec::new() // unreachable
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

/// A dungeon complex with multiple levels, each a self-contained Map.
pub struct Dungeon {
    pub levels: Vec<Map>,
    pub entrance: (i32, i32), // overworld position
}

impl Dungeon {
    /// Generate a dungeon with `depth` levels.
    /// Level 1 = 40×30, level 2 = 50×35, level 3+ = 60×40.
    pub fn generate(entrance: (i32, i32), depth: usize, seed: u64) -> Self {
        let mut levels = Vec::new();
        let mut rng = seed;

        for level in 0..depth {
            let (w, h) = match level {
                0 => (40, 30),
                1 => (50, 35),
                _ => (60, 40),
            };
            rng = xorshift64(rng);
            let map = Map::generate_bsp_dungeon(w, h, rng, level, depth);
            levels.push(map);
        }

        Dungeon { levels, entrance }
    }
}

impl Map {
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
    fn map_walkability_matches_tile_walkability() {
        let map = Map::generate(30, 20, 42);
        for y in 0..map.height {
            for x in 0..map.width {
                let tile = map.get(x, y);
                assert_eq!(map.is_walkable(x, y), tile.is_walkable(),
                    "walkability mismatch at ({x},{y}) for {tile:?}");
            }
        }
    }

    #[test]
    fn tile_walkability_truth_table() {
        // Exhaustive check: every tile type has the expected walkability
        assert!(!Tile::Wall.is_walkable());
        assert!(Tile::Floor.is_walkable());
        assert!(!Tile::Tree.is_walkable());
        assert!(Tile::Grass.is_walkable());
        assert!(Tile::Road.is_walkable());
        assert!(Tile::DungeonEntrance.is_walkable());
        assert!(Tile::StairsDown.is_walkable());
        assert!(Tile::StairsUp.is_walkable());
    }

    #[test]
    fn tile_glyph_and_color_defined() {
        // Every tile type must have a non-empty glyph and color
        let all = [Tile::Wall, Tile::Floor, Tile::Tree, Tile::Grass,
                   Tile::Road, Tile::DungeonEntrance, Tile::StairsDown, Tile::StairsUp];
        for tile in all {
            assert!(tile.glyph() != '\0', "{tile:?} has null glyph");
            assert!(!tile.color().is_empty(), "{tile:?} has empty color");
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

    // === A* pathfinding ===

    #[test]
    fn path_to_self_is_single_tile() {
        let map = Map::generate(30, 20, 42);
        let (sx, sy) = map.find_spawn();
        let path = map.find_path((sx, sy), (sx, sy));
        assert_eq!(path, vec![(sx, sy)]);
    }

    #[test]
    fn path_to_adjacent_floor() {
        let map = Map::generate(30, 20, 42);
        let (sx, sy) = map.find_spawn();
        // Find an adjacent walkable tile
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in dirs {
            let (nx, ny) = (sx + dx, sy + dy);
            if map.is_walkable(nx, ny) {
                let path = map.find_path((sx, sy), (nx, ny));
                assert_eq!(path.len(), 2);
                assert_eq!(path[0], (sx, sy));
                assert_eq!(path[1], (nx, ny));
                return;
            }
        }
        panic!("spawn has no adjacent floor");
    }

    #[test]
    fn path_to_wall_is_empty() {
        let map = Map::generate(30, 20, 42);
        let (sx, sy) = map.find_spawn();
        // Border is always wall
        let path = map.find_path((sx, sy), (0, 0));
        assert!(path.is_empty(), "path to wall should be empty");
    }

    #[test]
    fn path_all_tiles_walkable() {
        let map = Map::generate(30, 20, 42);
        let (sx, sy) = map.find_spawn();
        // Find a distant floor tile
        for y in (1..map.height - 1).rev() {
            for x in (1..map.width - 1).rev() {
                if map.is_walkable(x, y) && (x - sx).abs() + (y - sy).abs() > 5 {
                    let path = map.find_path((sx, sy), (x, y));
                    if path.is_empty() {
                        continue; // might be unreachable
                    }
                    for &(px, py) in &path {
                        assert!(map.is_walkable(px, py), "path tile ({px},{py}) not walkable");
                    }
                    // Each step should be adjacent (manhattan dist 1)
                    for w in path.windows(2) {
                        let dist = (w[0].0 - w[1].0).abs() + (w[0].1 - w[1].1).abs();
                        assert_eq!(dist, 1, "steps must be adjacent");
                    }
                    return;
                }
            }
        }
    }

    #[test]
    fn path_starts_and_ends_correctly() {
        let map = Map::generate(30, 20, 42);
        let (sx, sy) = map.find_spawn();
        for y in 1..map.height - 1 {
            for x in 1..map.width - 1 {
                if map.is_walkable(x, y) && (x != sx || y != sy) {
                    let path = map.find_path((sx, sy), (x, y));
                    if !path.is_empty() {
                        assert_eq!(*path.first().unwrap(), (sx, sy));
                        assert_eq!(*path.last().unwrap(), (x, y));
                        return;
                    }
                }
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

    // === Forest generation ===

    #[test]
    fn forest_border_is_trees() {
        let map = Map::generate_forest(200, 200, 42);
        for x in 0..map.width {
            assert_eq!(map.get(x, 0), Tile::Tree, "top border at x={x}");
            assert_eq!(map.get(x, map.height - 1), Tile::Tree, "bottom border at x={x}");
        }
        for y in 0..map.height {
            assert_eq!(map.get(0, y), Tile::Tree, "left border at y={y}");
            assert_eq!(map.get(map.width - 1, y), Tile::Tree, "right border at y={y}");
        }
    }

    #[test]
    fn forest_has_enough_grass() {
        let map = Map::generate_forest(200, 200, 42);
        let grass_count = (0..map.height)
            .flat_map(|y| (0..map.width).map(move |x| (x, y)))
            .filter(|&(x, y)| map.get(x, y) == Tile::Grass)
            .count();
        let total = (map.width * map.height) as usize;
        assert!(
            grass_count > total * 30 / 100,
            "too few grass: {grass_count} out of {total} ({:.1}%)",
            grass_count as f64 / total as f64 * 100.0
        );
    }

    #[test]
    fn forest_all_grass_reachable() {
        let map = Map::generate_forest(200, 200, 42);
        let (sx, sy) = map.find_spawn();
        assert_eq!(map.get(sx, sy), Tile::Grass, "spawn should be on grass");

        // BFS from spawn to all reachable grass
        let len = (map.width * map.height) as usize;
        let mut visited = vec![false; len];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((sx, sy));
        visited[(sy * map.width + sx) as usize] = true;
        let mut reachable = 0;

        while let Some((x, y)) = queue.pop_front() {
            reachable += 1;
            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let (nx, ny) = (x + dx, y + dy);
                if nx >= 0 && ny >= 0 && nx < map.width && ny < map.height {
                    let ni = (ny * map.width + nx) as usize;
                    if !visited[ni] && map.get(nx, ny) == Tile::Grass {
                        visited[ni] = true;
                        queue.push_back((nx, ny));
                    }
                }
            }
        }

        // Count total grass
        let total_grass = (0..map.height)
            .flat_map(|y| (0..map.width).map(move |x| (x, y)))
            .filter(|&(x, y)| map.get(x, y) == Tile::Grass)
            .count();

        assert_eq!(reachable, total_grass,
            "not all grass reachable: {reachable} reachable out of {total_grass} total");
    }

    #[test]
    fn forest_deterministic() {
        let a = Map::generate_forest(100, 100, 77);
        let b = Map::generate_forest(100, 100, 77);
        for y in 0..a.height {
            for x in 0..a.width {
                assert_eq!(a.get(x, y), b.get(x, y), "forest mismatch at ({x},{y})");
            }
        }
    }

    #[test]
    fn forest_only_tree_and_grass() {
        let map = Map::generate_forest(200, 200, 42);
        for y in 0..map.height {
            for x in 0..map.width {
                let t = map.get(x, y);
                assert!(t == Tile::Tree || t == Tile::Grass,
                    "unexpected tile {t:?} at ({x},{y})");
            }
        }
    }

    // === Dungeon placement ===

    fn test_overworld() -> (Map, Vec<(i32, i32)>) {
        let mut map = Map::generate_forest(200, 200, 42);
        let entrances = map.place_dungeons(42);
        (map, entrances)
    }

    #[test]
    fn at_least_three_dungeons() {
        let (_, entrances) = test_overworld();
        assert!(entrances.len() >= 3,
            "need at least 3 dungeons, got {}", entrances.len());
    }

    #[test]
    fn dungeon_entrances_are_entrance_tiles() {
        let (map, entrances) = test_overworld();
        for &(x, y) in &entrances {
            assert_eq!(map.get(x, y), Tile::DungeonEntrance,
                "entrance at ({x},{y}) is not DungeonEntrance");
        }
    }

    #[test]
    fn dungeon_entrance_adjacent_to_walkable() {
        let (map, entrances) = test_overworld();
        for &(ex, ey) in &entrances {
            let has_walkable = [(0, 1), (0, -1), (1, 0), (-1, 0)].iter().any(|&(dx, dy)| {
                let nx = ex + dx;
                let ny = ey + dy;
                nx >= 0 && ny >= 0 && nx < map.width && ny < map.height && {
                    let t = map.get(nx, ny);
                    t == Tile::Grass || t == Tile::Road || t == Tile::Floor
                }
            });
            assert!(has_walkable,
                "entrance at ({ex},{ey}) has no adjacent walkable tile");
        }
    }

    #[test]
    fn dungeon_footprints_dont_overlap() {
        let (_, entrances) = test_overworld();
        // Entrances should be at least 15 tiles apart (footprint size)
        for i in 0..entrances.len() {
            for j in (i + 1)..entrances.len() {
                let dx = (entrances[i].0 - entrances[j].0).abs();
                let dy = (entrances[i].1 - entrances[j].1).abs();
                assert!(dx > 5 || dy > 5,
                    "dungeons too close: {:?} and {:?}", entrances[i], entrances[j]);
            }
        }
    }

    // === Road generation ===

    fn test_overworld_with_roads() -> (Map, Vec<(i32, i32)>) {
        let mut map = Map::generate_forest(200, 200, 42);
        let entrances = map.place_dungeons(42);
        map.build_roads(&entrances);
        (map, entrances)
    }

    #[test]
    fn roads_exist_after_generation() {
        let (map, _) = test_overworld_with_roads();
        let road_count = (0..map.height)
            .flat_map(|y| (0..map.width).map(move |x| (x, y)))
            .filter(|&(x, y)| map.get(x, y) == Tile::Road)
            .count();
        assert!(road_count > 50, "too few roads: {road_count}");
        assert!(road_count < (map.width * map.height) as usize / 4,
            "too many roads: {road_count}");
    }

    #[test]
    fn no_road_on_border() {
        let (map, _) = test_overworld_with_roads();
        for x in 0..map.width {
            assert_ne!(map.get(x, 0), Tile::Road, "road on top border");
            assert_ne!(map.get(x, map.height - 1), Tile::Road, "road on bottom border");
        }
        for y in 0..map.height {
            assert_ne!(map.get(0, y), Tile::Road, "road on left border");
            assert_ne!(map.get(map.width - 1, y), Tile::Road, "road on right border");
        }
    }

    // === BSP dungeon interiors ===

    #[test]
    fn dungeon_has_correct_level_count() {
        let d = Dungeon::generate((50, 50), 3, 42);
        assert_eq!(d.levels.len(), 3);
    }

    #[test]
    fn dungeon_level_sizes_scale_with_depth() {
        let d = Dungeon::generate((50, 50), 3, 42);
        assert_eq!((d.levels[0].width, d.levels[0].height), (40, 30));
        assert_eq!((d.levels[1].width, d.levels[1].height), (50, 35));
        assert_eq!((d.levels[2].width, d.levels[2].height), (60, 40));
    }

    #[test]
    fn dungeon_levels_have_stairs() {
        let d = Dungeon::generate((50, 50), 3, 42);
        for (i, level) in d.levels.iter().enumerate() {
            let has_up = (0..level.height)
                .flat_map(|y| (0..level.width).map(move |x| (x, y)))
                .any(|(x, y)| level.get(x, y) == Tile::StairsUp);
            assert!(has_up, "level {i} missing StairsUp");

            if i < d.levels.len() - 1 {
                let has_down = (0..level.height)
                    .flat_map(|y| (0..level.width).map(move |x| (x, y)))
                    .any(|(x, y)| level.get(x, y) == Tile::StairsDown);
                assert!(has_down, "level {i} missing StairsDown");
            }
        }
    }

    #[test]
    fn dungeon_deepest_level_has_no_stairs_down() {
        let d = Dungeon::generate((50, 50), 3, 42);
        let last = &d.levels[2];
        let has_down = (0..last.height)
            .flat_map(|y| (0..last.width).map(move |x| (x, y)))
            .any(|(x, y)| last.get(x, y) == Tile::StairsDown);
        assert!(!has_down, "deepest level should have no StairsDown");
    }

    #[test]
    fn dungeon_rooms_reachable_from_stairs() {
        let d = Dungeon::generate((50, 50), 3, 42);
        for (i, level) in d.levels.iter().enumerate() {
            // Find StairsUp as the starting point
            let stairs_up = (0..level.height)
                .flat_map(|y| (0..level.width).map(move |x| (x, y)))
                .find(|&(x, y)| level.get(x, y) == Tile::StairsUp);
            let (sx, sy) = stairs_up.expect(&format!("level {i} has no StairsUp"));

            // BFS from stairs to all walkable tiles
            let len = (level.width * level.height) as usize;
            let mut visited = vec![false; len];
            let mut queue = std::collections::VecDeque::new();
            queue.push_back((sx, sy));
            visited[(sy * level.width + sx) as usize] = true;
            let mut reachable = 0;

            while let Some((x, y)) = queue.pop_front() {
                reachable += 1;
                for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let (nx, ny) = (x + dx, y + dy);
                    if level.is_walkable(nx, ny) {
                        let ni = (ny * level.width + nx) as usize;
                        if !visited[ni] {
                            visited[ni] = true;
                            queue.push_back((nx, ny));
                        }
                    }
                }
            }

            let total_walkable = (0..level.height)
                .flat_map(|y| (0..level.width).map(move |x| (x, y)))
                .filter(|&(x, y)| level.is_walkable(x, y))
                .count();

            assert_eq!(reachable, total_walkable,
                "level {i}: {reachable} reachable out of {total_walkable} walkable");
        }
    }

    #[test]
    fn dungeon_bsp_produces_valid_rooms() {
        let d = Dungeon::generate((50, 50), 1, 42);
        let level = &d.levels[0];
        let floor_count = (0..level.height)
            .flat_map(|y| (0..level.width).map(move |x| (x, y)))
            .filter(|&(x, y)| level.get(x, y) == Tile::Floor)
            .count();
        // Should have a reasonable number of floor tiles (at least 15% of interior)
        let interior = ((level.width - 2) * (level.height - 2)) as usize;
        assert!(floor_count > interior / 7,
            "too few floor tiles: {floor_count} out of {interior}");
    }

    #[test]
    fn all_entrances_reachable_from_spawn() {
        let (map, entrances) = test_overworld_with_roads();
        let (sx, sy) = map.find_road_spawn();
        assert!(map.is_walkable(sx, sy), "spawn not walkable");

        for &(ex, ey) in &entrances {
            let path = map.find_path((sx, sy), (ex, ey));
            assert!(!path.is_empty(),
                "entrance ({ex},{ey}) unreachable from spawn ({sx},{sy})");
        }
    }
}
