use super::super::*;
use crate::config::MapGenConfig;

#[test]
fn forest_border_is_trees() {
    let cfg = MapGenConfig::normal();
    let map = Map::generate_forest(200, 200, 42, &cfg);
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
    let cfg = MapGenConfig::normal();
    let map = Map::generate_forest(200, 200, 42, &cfg);
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
    let cfg = MapGenConfig::normal();
    let map = Map::generate_forest(200, 200, 42, &cfg);
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
    let cfg = MapGenConfig::normal();
    let a = Map::generate_forest(100, 100, 77, &cfg);
    let b = Map::generate_forest(100, 100, 77, &cfg);
    for y in 0..a.height {
        for x in 0..a.width {
            assert_eq!(a.get(x, y), b.get(x, y), "forest mismatch at ({x},{y})");
        }
    }
}

#[test]
fn forest_only_tree_and_grass() {
    let cfg = MapGenConfig::normal();
    let map = Map::generate_forest(200, 200, 42, &cfg);
    for y in 0..map.height {
        for x in 0..map.width {
            let t = map.get(x, y);
            assert!(t == Tile::Tree || t == Tile::Grass,
                "unexpected tile {t:?} at ({x},{y})");
        }
    }
}
