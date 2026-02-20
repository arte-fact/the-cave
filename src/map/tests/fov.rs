use super::super::*;

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
