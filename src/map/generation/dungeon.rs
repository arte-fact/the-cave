use super::super::{Map, Tile};
use super::xorshift64;

impl Map {
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
        connect_rooms_with_corridors(&mut map, &rooms, width, height);

        // Place stairs
        place_stairs(&mut map, &rooms, level, total_levels);

        map
    }
}

/// Connect rooms with L-shaped corridors between BSP siblings.
fn connect_rooms_with_corridors(map: &mut Map, rooms: &[(i32, i32, i32, i32)], width: i32, height: i32) {
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
}

/// Place StairsUp/StairsDown in the first and last rooms.
fn place_stairs(map: &mut Map, rooms: &[(i32, i32, i32, i32)], level: usize, total_levels: usize) {
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
}

/// Visual style of a dungeon, determining which wall/floor sprites are used.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DungeonStyle {
    /// Basic dirt/rough stone — shallow dungeons.
    DirtCaves,
    /// Stone brick — standard dungeons.
    StoneBrick,
    /// Dark igneous rock — mid-depth.
    Igneous,
    /// Massive stone blocks — deep dungeons.
    LargeStone,
    /// Skull-decorated catacombs.
    Catacombs,
    /// Open cave with red stone — dragon's lair.
    DragonLair,
    /// Rough stone walls + mossy green dirt floors — fungal grotto.
    FungalCave,
    /// Rough stone walls + dark brown bone floors — beast den.
    BeastDen,
    /// Igneous walls + blue stone floors — abyssal temple.
    AbyssalTemple,
    /// Dirt walls + green dirt floors — serpent pit.
    SerpentPit,
    /// Catacombs walls + dark brown bone floors — deep undead/beast.
    DarkBones,
}

/// A dungeon complex with multiple levels, each a self-contained Map.
pub struct Dungeon {
    pub levels: Vec<Map>,
    /// Visual style per level (determines wall/floor sprites).
    pub styles: Vec<DungeonStyle>,
    /// Thematic biome of this dungeon (determines enemies, visuals, boss).
    pub biome: super::biome::DungeonBiome,
}

impl Dungeon {
    /// Generate a dungeon with `depth` BSP levels and a specific biome.
    /// Level 0 = 40x30, level 1 = 50x35, level 2 = 60x40.
    /// If `has_cave` is true, appends a cellular automata cave (80x60)
    /// as the deepest level — the dragon's lair.
    pub fn generate(depth: usize, seed: u64, has_cave: bool, biome: super::biome::DungeonBiome) -> Self {
        let mut levels = Vec::new();
        let mut styles = Vec::new();
        let mut rng = seed;

        // Total levels: BSP depth + optional cave
        let total = if has_cave { depth + 1 } else { depth };

        for level in 0..depth {
            let (w, h) = dungeon_level_size(level);
            rng = xorshift64(rng);
            let map = Map::generate_bsp_dungeon(w, h, rng, level, total);
            levels.push(map);
            styles.push(biome.style_for_level(level, false));
        }

        // Append cave level if this is the dragon's dungeon
        if has_cave {
            rng = xorshift64(rng);
            let cave = Map::generate_cave(80, 60, rng);
            levels.push(cave);
            styles.push(DungeonStyle::DragonLair);
        }

        Dungeon { levels, styles, biome }
    }
}

/// Returns (width, height) for a dungeon level based on depth.
fn dungeon_level_size(level: usize) -> (i32, i32) {
    match level {
        0 => (40, 30),
        1 => (50, 35),
        _ => (60, 40),
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
        (*rng).is_multiple_of(2)
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
        (*rng).is_multiple_of(2)
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
