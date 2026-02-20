use super::super::*;

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
