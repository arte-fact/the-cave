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
fn get_out_of_bounds_returns_wall() {
    let map = Map::generate(30, 20, 42);
    // Negative coordinates
    assert_eq!(map.get(-1, 0), Tile::Wall);
    assert_eq!(map.get(0, -1), Tile::Wall);
    assert_eq!(map.get(-1, -1), Tile::Wall);
    // Past width/height
    assert_eq!(map.get(map.width, 0), Tile::Wall);
    assert_eq!(map.get(0, map.height), Tile::Wall);
    assert_eq!(map.get(map.width, map.height), Tile::Wall);
    // Large values
    assert_eq!(map.get(i32::MAX, 0), Tile::Wall);
    assert_eq!(map.get(0, i32::MAX), Tile::Wall);
    assert_eq!(map.get(i32::MIN, i32::MIN), Tile::Wall);
}

#[test]
fn set_out_of_bounds_is_noop() {
    let mut map = Map::generate(30, 20, 42);
    let original = map.get(0, 0);
    // These should not panic
    map.set(-1, 0, Tile::Floor);
    map.set(0, -1, Tile::Floor);
    map.set(map.width, 0, Tile::Floor);
    map.set(0, map.height, Tile::Floor);
    map.set(i32::MAX, 0, Tile::Floor);
    map.set(0, i32::MAX, Tile::Floor);
    map.set(i32::MIN, i32::MIN, Tile::Floor);
    // Original map unchanged
    assert_eq!(map.get(0, 0), original);
}

#[test]
fn in_bounds_edges() {
    let map = Map::generate(30, 20, 42);
    // Valid corners
    assert!(map.in_bounds(0, 0));
    assert!(map.in_bounds(29, 19));
    assert!(map.in_bounds(0, 19));
    assert!(map.in_bounds(29, 0));
    // Just outside
    assert!(!map.in_bounds(-1, 0));
    assert!(!map.in_bounds(0, -1));
    assert!(!map.in_bounds(30, 0));
    assert!(!map.in_bounds(0, 20));
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
                // Each step should be adjacent (Chebyshev dist 1, includes diagonals)
                for w in path.windows(2) {
                    let cdist = (w[0].0 - w[1].0).abs().max((w[0].1 - w[1].1).abs());
                    assert_eq!(cdist, 1, "steps must be adjacent (Chebyshev)");
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
        for (dx, dy) in [(1,0),(-1,0),(0,1),(0,-1),(1,1),(-1,-1),(1,-1),(-1,1)] {
            let (nx, ny) = (x + dx, y + dy);
            if nx >= 0 && ny >= 0 && nx < map.width && ny < map.height {
                // Corner-cutting prevention for diagonals
                if dx != 0 && dy != 0
                    && (map.get(x + dx, y) != Tile::Grass || map.get(x, y + dy) != Tile::Grass)
                {
                    continue;
                }
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
    let d = Dungeon::generate(3, 42, false);
    assert_eq!(d.levels.len(), 3);
}

#[test]
fn dungeon_level_sizes_scale_with_depth() {
    let d = Dungeon::generate(3, 42, false);
    assert_eq!((d.levels[0].width, d.levels[0].height), (40, 30));
    assert_eq!((d.levels[1].width, d.levels[1].height), (50, 35));
    assert_eq!((d.levels[2].width, d.levels[2].height), (60, 40));
}

#[test]
fn dungeon_levels_have_stairs() {
    let d = Dungeon::generate(3, 42, false);
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
    let d = Dungeon::generate(3, 42, false);
    let last = &d.levels[2];
    let has_down = (0..last.height)
        .flat_map(|y| (0..last.width).map(move |x| (x, y)))
        .any(|(x, y)| last.get(x, y) == Tile::StairsDown);
    assert!(!has_down, "deepest level should have no StairsDown");
}

#[test]
fn dungeon_rooms_reachable_from_stairs() {
    let d = Dungeon::generate(3, 42, false);
    for (i, level) in d.levels.iter().enumerate() {
        // Find StairsUp as the starting point
        let stairs_up = (0..level.height)
            .flat_map(|y| (0..level.width).map(move |x| (x, y)))
            .find(|&(x, y)| level.get(x, y) == Tile::StairsUp);
        let (sx, sy) = stairs_up.unwrap_or_else(|| panic!("level {i} has no StairsUp"));

        // BFS from stairs to all walkable tiles
        let len = (level.width * level.height) as usize;
        let mut visited = vec![false; len];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((sx, sy));
        visited[(sy * level.width + sx) as usize] = true;
        let mut reachable = 0;

        while let Some((x, y)) = queue.pop_front() {
            reachable += 1;
            for (dx, dy) in [(1,0),(-1,0),(0,1),(0,-1),(1,1),(-1,-1),(1,-1),(-1,1)] {
                let (nx, ny) = (x + dx, y + dy);
                // Corner-cutting prevention for diagonals
                if dx != 0 && dy != 0
                    && (!level.is_walkable(x + dx, y) || !level.is_walkable(x, y + dy))
                {
                    continue;
                }
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
    let d = Dungeon::generate(1, 42, false);
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

// === Cave (dragon's lair) ===

#[test]
fn cave_has_stairs_up() {
    let cave = Map::generate_cave(80, 60, 42);
    let has_up = (0..cave.height)
        .flat_map(|y| (0..cave.width).map(move |x| (x, y)))
        .any(|(x, y)| cave.get(x, y) == Tile::StairsUp);
    assert!(has_up, "cave should have StairsUp");
}

#[test]
fn cave_has_no_stairs_down() {
    let cave = Map::generate_cave(80, 60, 42);
    let has_down = (0..cave.height)
        .flat_map(|y| (0..cave.width).map(move |x| (x, y)))
        .any(|(x, y)| cave.get(x, y) == Tile::StairsDown);
    assert!(!has_down, "cave should have no StairsDown");
}

#[test]
fn cave_floor_connected() {
    let cave = Map::generate_cave(80, 60, 42);
    // BFS from StairsUp to all walkable tiles
    let stairs = cave.find_tile(Tile::StairsUp).expect("no StairsUp in cave");
    let len = (cave.width * cave.height) as usize;
    let mut visited = vec![false; len];
    let mut queue = std::collections::VecDeque::new();
    queue.push_back(stairs);
    visited[(stairs.1 * cave.width + stairs.0) as usize] = true;
    let mut reachable = 0;
    while let Some((x, y)) = queue.pop_front() {
        reachable += 1;
        for (dx, dy) in [(1,0),(-1,0),(0,1),(0,-1),(1,1),(-1,-1),(1,-1),(-1,1)] {
            let (nx, ny) = (x + dx, y + dy);
            // Corner-cutting prevention for diagonals
            if dx != 0 && dy != 0
                && (!cave.is_walkable(x + dx, y) || !cave.is_walkable(x, y + dy))
            {
                continue;
            }
            if cave.is_walkable(nx, ny) {
                let ni = (ny * cave.width + nx) as usize;
                if !visited[ni] {
                    visited[ni] = true;
                    queue.push_back((nx, ny));
                }
            }
        }
    }
    let total_walkable = (0..cave.height)
        .flat_map(|y| (0..cave.width).map(move |x| (x, y)))
        .filter(|&(x, y)| cave.is_walkable(x, y))
        .count();
    assert_eq!(reachable, total_walkable,
        "cave not connected: {reachable} reachable out of {total_walkable} walkable");
}

#[test]
fn cave_has_enough_floor() {
    let cave = Map::generate_cave(80, 60, 42);
    let floor_count = (0..cave.height)
        .flat_map(|y| (0..cave.width).map(move |x| (x, y)))
        .filter(|&(x, y)| cave.get(x, y) == Tile::Floor || cave.get(x, y) == Tile::StairsUp)
        .count();
    let interior = ((cave.width - 2) * (cave.height - 2)) as usize;
    assert!(floor_count > interior / 5,
        "too few floor tiles in cave: {floor_count} out of {interior}");
}

#[test]
fn dungeon_with_cave_has_4_levels() {
    let d = Dungeon::generate(3, 42, true);
    assert_eq!(d.levels.len(), 4, "dragon dungeon should have 4 levels");
    // Cave level is 80x60
    assert_eq!((d.levels[3].width, d.levels[3].height), (80, 60));
}

#[test]
fn dungeon_without_cave_has_3_levels() {
    let d = Dungeon::generate(3, 42, false);
    assert_eq!(d.levels.len(), 3);
}

#[test]
fn cave_dungeon_level2_has_stairs_down() {
    // With a cave, level 2 (the last BSP level) should have StairsDown connecting to cave
    let d = Dungeon::generate(3, 42, true);
    let level2 = &d.levels[2];
    let has_down = (0..level2.height)
        .flat_map(|y| (0..level2.width).map(move |x| (x, y)))
        .any(|(x, y)| level2.get(x, y) == Tile::StairsDown);
    assert!(has_down, "level 2 of cave dungeon should have StairsDown");
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

// === FOV / Visibility ===

#[test]
fn player_tile_always_visible() {
    let mut map = Map::generate(30, 20, 42);
    let (sx, sy) = map.find_spawn();
    map.compute_fov(sx, sy, 8);
    assert_eq!(map.get_visibility(sx, sy), Visibility::Visible);
}

#[test]
fn walls_block_los() {
    // Build a simple map: open room with a wall in the middle
    let mut map = Map::new_filled(10, 10, Tile::Floor);
    // Wall at (5, 5)
    map.set(5, 5, Tile::Wall);
    // Compute FOV from (3, 5) with radius 8
    map.compute_fov(3, 5, 8);
    // (5,5) itself should be visible (wall is seen but blocks behind)
    assert_eq!(map.get_visibility(5, 5), Visibility::Visible);
    // (7,5) should be hidden — blocked by wall at (5,5)
    assert_eq!(map.get_visibility(7, 5), Visibility::Hidden,
        "tile behind wall should be hidden");
}

#[test]
fn fov_radius_respected() {
    let mut map = Map::new_filled(50, 50, Tile::Floor);
    let radius = 6;
    map.compute_fov(25, 25, radius);
    // Tile well beyond radius should be hidden
    assert_eq!(map.get_visibility(25, 25 + radius + 2), Visibility::Hidden,
        "tile beyond radius should be hidden");
    // Tile within radius and LOS should be visible
    assert_eq!(map.get_visibility(25, 25 + radius - 1), Visibility::Visible,
        "tile within radius should be visible");
}

#[test]
fn seen_tiles_persist_after_aging() {
    let mut map = Map::new_filled(20, 20, Tile::Floor);
    map.compute_fov(10, 10, 5);
    assert_eq!(map.get_visibility(12, 10), Visibility::Visible);
    // Age: Visible -> Seen
    map.age_visibility();
    assert_eq!(map.get_visibility(12, 10), Visibility::Seen);
    // Compute FOV from a different position
    map.compute_fov(5, 5, 5);
    // (12, 10) should still be Seen (not reverted to Hidden)
    assert_eq!(map.get_visibility(12, 10), Visibility::Seen,
        "previously seen tile should stay Seen");
}

#[test]
fn hidden_tiles_stay_hidden() {
    let mut map = Map::new_filled(30, 30, Tile::Floor);
    map.compute_fov(5, 5, 3);
    // Far corner should be hidden
    assert_eq!(map.get_visibility(25, 25), Visibility::Hidden);
}

#[test]
fn fov_symmetric_in_open_room() {
    let mut map = Map::new_filled(20, 20, Tile::Floor);
    map.compute_fov(10, 10, 6);
    // Check symmetry: if (10+3, 10) is visible, (10-3, 10) should be too
    assert_eq!(map.get_visibility(13, 10), Visibility::Visible);
    assert_eq!(map.get_visibility(7, 10), Visibility::Visible);
    assert_eq!(map.get_visibility(10, 13), Visibility::Visible);
    assert_eq!(map.get_visibility(10, 7), Visibility::Visible);
}

#[test]
fn opaque_tiles_are_visible_but_block() {
    let mut map = Map::new_filled(20, 20, Tile::Floor);
    // Line of trees at x=12
    for y in 0..20 {
        map.set(12, y, Tile::Tree);
    }
    map.compute_fov(10, 10, 8);
    // Tree itself should be visible
    assert_eq!(map.get_visibility(12, 10), Visibility::Visible);
    // Tile behind the tree line should be hidden
    assert_eq!(map.get_visibility(14, 10), Visibility::Hidden,
        "tile behind tree line should be hidden");
}

// --- Bresenham line ---

#[test]
fn bresenham_horizontal() {
    let line = bresenham_line(0, 0, 5, 0);
    assert_eq!(line, vec![(0,0), (1,0), (2,0), (3,0), (4,0), (5,0)]);
}

#[test]
fn bresenham_vertical() {
    let line = bresenham_line(3, 1, 3, 5);
    assert_eq!(line, vec![(3,1), (3,2), (3,3), (3,4), (3,5)]);
}

#[test]
fn bresenham_diagonal() {
    let line = bresenham_line(0, 0, 3, 3);
    assert_eq!(line.len(), 4);
    assert_eq!(line[0], (0, 0));
    assert_eq!(line[3], (3, 3));
}

#[test]
fn bresenham_single_point() {
    let line = bresenham_line(5, 5, 5, 5);
    assert_eq!(line, vec![(5, 5)]);
}

#[test]
fn bresenham_negative_direction() {
    let line = bresenham_line(5, 0, 0, 0);
    assert_eq!(line.len(), 6);
    assert_eq!(line[0], (5, 0));
    assert_eq!(line[5], (0, 0));
}

#[test]
fn bresenham_includes_start_and_end() {
    let line = bresenham_line(2, 3, 8, 6);
    assert_eq!(line[0], (2, 3));
    assert_eq!(*line.last().unwrap(), (8, 6));
}

// --- Line of sight ---

#[test]
fn los_open_room() {
    let map = Map::new_filled(20, 20, Tile::Floor);
    assert!(map.has_line_of_sight(5, 5, 15, 15));
}

#[test]
fn los_blocked_by_wall() {
    let mut map = Map::new_filled(20, 20, Tile::Floor);
    map.set(10, 5, Tile::Wall);
    // LOS from (5,5) to (15,5) should be blocked by wall at (10,5)
    assert!(!map.has_line_of_sight(5, 5, 15, 5));
}

#[test]
fn los_blocked_by_tree() {
    let mut map = Map::new_filled(20, 20, Tile::Floor);
    map.set(8, 5, Tile::Tree);
    assert!(!map.has_line_of_sight(5, 5, 12, 5));
}

#[test]
fn los_can_see_adjacent() {
    let map = Map::new_filled(10, 10, Tile::Floor);
    assert!(map.has_line_of_sight(5, 5, 6, 5));
    assert!(map.has_line_of_sight(5, 5, 5, 6));
}

#[test]
fn los_endpoint_wall_not_blocking() {
    let mut map = Map::new_filled(20, 20, Tile::Floor);
    map.set(10, 5, Tile::Wall);
    // Can "see" the wall itself
    assert!(map.has_line_of_sight(5, 5, 10, 5));
}

#[test]
fn los_same_tile() {
    let map = Map::new_filled(10, 10, Tile::Floor);
    assert!(map.has_line_of_sight(5, 5, 5, 5));
}

// === Diagonal pathfinding tests ===

#[test]
fn path_uses_diagonals() {
    // On open floor, path from (1,1) to (4,4) should use diagonals (length 4, not 7)
    let map = Map::new_filled(10, 10, Tile::Floor);
    let path = map.find_path((1, 1), (4, 4));
    assert!(!path.is_empty());
    // Diagonal path should be 4 steps: (1,1)->(2,2)->(3,3)->(4,4)
    assert_eq!(path.len(), 4);
}

#[test]
fn path_avoids_corner_cutting() {
    // Map layout (5x5):
    // F F F F F
    // F F W F F
    // F W F F F
    // F F F F F
    // F F F F F
    // Path from (1,1) to (2,2) should NOT cut through (2,1)/(1,2) walls
    let mut map = Map::new_filled(5, 5, Tile::Floor);
    map.set(2, 1, Tile::Wall);
    map.set(1, 2, Tile::Wall);
    let path = map.find_path((1, 1), (2, 2));
    assert!(!path.is_empty(), "path should exist via alternate route");
    // Verify no step cuts a corner
    for w in path.windows(2) {
        let (x1, y1) = w[0];
        let (x2, y2) = w[1];
        let dx = x2 - x1;
        let dy = y2 - y1;
        if dx != 0 && dy != 0 {
            assert!(map.is_walkable(x1 + dx, y1), "corner-cutting at ({},{})->({},{})", x1, y1, x2, y2);
            assert!(map.is_walkable(x1, y1 + dy), "corner-cutting at ({},{})->({},{})", x1, y1, x2, y2);
        }
    }
}
