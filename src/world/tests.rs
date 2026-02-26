use super::*;
use crate::config::MapGenConfig;
use crate::game::ItemKind;

fn test_world() -> World {
    let cfg = MapGenConfig::normal();
    let mut map = Map::generate_forest(200, 200, 42, &cfg);
    let entrances = map.place_dungeons(42, &cfg);
    map.build_roads(&entrances, &cfg);
    World::new(map, entrances, 99, &cfg)
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

#[test]
fn five_dungeons_total() {
    let w = test_world();
    assert_eq!(w.dungeons.len(), 5);
}

#[test]
fn exactly_one_dragon_lair() {
    let w = test_world();
    let lair_count = w.dungeons.iter().filter(|d| d.biome == DungeonBiome::DragonLair).count();
    assert_eq!(lair_count, 1, "should have exactly one Dragon's Lair");
}

#[test]
fn all_regular_dungeons_unique_biomes() {
    let w = test_world();
    let regular: Vec<DungeonBiome> = w.dungeons.iter()
        .filter(|d| d.biome != DungeonBiome::DragonLair)
        .map(|d| d.biome)
        .collect();
    assert_eq!(regular.len(), 4, "should have 4 regular dungeons");
    let unique: std::collections::HashSet<_> = regular.iter().collect();
    assert_eq!(unique.len(), 4, "all regular dungeons should have unique biomes");
}

#[test]
fn legendary_slots_cover_all_four_types() {
    let w = test_world();
    let slots: Vec<ItemKind> = w.legendary_slots.iter().filter_map(|s| s.clone()).collect();
    assert_eq!(slots.len(), 4, "should have 4 legendary slots");
    assert!(slots.contains(&ItemKind::Helmet));
    assert!(slots.contains(&ItemKind::Armor));
    assert!(slots.contains(&ItemKind::Shield));
    assert!(slots.contains(&ItemKind::Boots));
}

#[test]
fn dragon_lair_has_no_legendary_slot() {
    let w = test_world();
    let cave_idx = w.dungeons.iter().position(|d| d.biome == DungeonBiome::DragonLair).unwrap();
    assert_eq!(w.legendary_slots[cave_idx], None, "Dragon's Lair should have no legendary slot");
}
