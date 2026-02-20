use super::super::*;

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

// === BSP dungeon interiors ===

#[test]
fn dungeon_has_correct_level_count() {
    let d = Dungeon::generate(3, 42, false, DungeonBiome::GoblinWarren);
    assert_eq!(d.levels.len(), 3);
}

#[test]
fn dungeon_level_sizes_scale_with_depth() {
    let d = Dungeon::generate(3, 42, false, DungeonBiome::GoblinWarren);
    assert_eq!((d.levels[0].width, d.levels[0].height), (40, 30));
    assert_eq!((d.levels[1].width, d.levels[1].height), (50, 35));
    assert_eq!((d.levels[2].width, d.levels[2].height), (60, 40));
}

#[test]
fn dungeon_levels_have_stairs() {
    let d = Dungeon::generate(3, 42, false, DungeonBiome::GoblinWarren);
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
    let d = Dungeon::generate(3, 42, false, DungeonBiome::GoblinWarren);
    let last = &d.levels[2];
    let has_down = (0..last.height)
        .flat_map(|y| (0..last.width).map(move |x| (x, y)))
        .any(|(x, y)| last.get(x, y) == Tile::StairsDown);
    assert!(!has_down, "deepest level should have no StairsDown");
}

#[test]
fn dungeon_rooms_reachable_from_stairs() {
    let d = Dungeon::generate(3, 42, false, DungeonBiome::GoblinWarren);
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
    let d = Dungeon::generate(1, 42, false, DungeonBiome::GoblinWarren);
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
