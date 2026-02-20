use super::super::*;

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
