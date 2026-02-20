use super::super::*;

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
