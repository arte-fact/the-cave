use super::super::*;

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
