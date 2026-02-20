use super::*;

fn test_world() -> World {
    let mut map = Map::generate_forest(200, 200, 42);
    let entrances = map.place_dungeons(42);
    map.build_roads(&entrances);
    World::new(map, entrances, 99)
}

#[test]
fn world_starts_on_overworld() {
    let w = test_world();
    assert_eq!(w.location, Location::Overworld);
}

#[test]
fn current_map_is_overworld_by_default() {
    let w = test_world();
    assert_eq!(w.current_map().width, 200);
}

#[test]
fn dungeon_at_finds_entrance() {
    let w = test_world();
    let (x, y) = w.dungeon_entrances[0];
    assert_eq!(w.dungeon_at(x, y), Some(0));
}

#[test]
fn dungeon_at_returns_none_for_non_entrance() {
    let w = test_world();
    assert_eq!(w.dungeon_at(0, 0), None);
}

#[test]
fn dungeons_generated_for_each_entrance() {
    let w = test_world();
    assert_eq!(w.dungeons.len(), w.dungeon_entrances.len());
}

#[test]
fn exactly_one_dungeon_has_cave() {
    let w = test_world();
    let cave_count = w.dungeons.iter().filter(|d| d.levels.len() == 4).count();
    let normal_count = w.dungeons.iter().filter(|d| d.levels.len() == 3).count();
    assert_eq!(cave_count, 1, "exactly one dungeon should have a cave level");
    assert_eq!(cave_count + normal_count, w.dungeons.len(),
        "all dungeons should have 3 or 4 levels");
}

#[test]
fn current_map_changes_with_location() {
    let mut w = test_world();
    let ow_width = w.overworld.width;
    w.location = Location::Dungeon { index: 0, level: 0 };
    let dw = w.current_map().width;
    // Dungeon level 0 is 40 wide, overworld is 200
    assert_ne!(ow_width, dw);
    assert_eq!(dw, 40);
}

#[test]
fn from_single_map_has_no_dungeons() {
    let map = Map::generate(30, 20, 42);
    let w = World::from_single_map(map);
    assert!(w.dungeons.is_empty());
    assert!(w.dungeon_entrances.is_empty());
    assert_eq!(w.location, Location::Overworld);
}
