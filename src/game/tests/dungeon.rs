use super::*;
use super::overworld_game;

#[test]
fn enter_dungeon_changes_location() {
    let mut g = overworld_game();
    let entrance = g.world.dungeon_entrances[0];
    // Teleport player to dungeon entrance
    g.player_x = entrance.0;
    g.player_y = entrance.1;
    g.enter_dungeon(0);
    assert_eq!(g.world.location, Location::Dungeon { index: 0, level: 0 });
}

#[test]
fn enter_dungeon_places_player_at_stairs_up() {
    let mut g = overworld_game();
    g.enter_dungeon(0);
    let map = g.current_map();
    assert_eq!(map.get(g.player_x, g.player_y), Tile::StairsUp);
}

#[test]
fn enter_dungeon_saves_overworld_pos() {
    let mut g = overworld_game();
    let (ox, oy) = (g.player_x, g.player_y);
    g.enter_dungeon(0);
    assert_eq!(g.world.saved_overworld_pos, (ox, oy));
}

#[test]
fn exit_dungeon_restores_overworld() {
    let mut g = overworld_game();
    let (ox, oy) = (g.player_x, g.player_y);
    let enemy_count_before = g.enemies.len();
    g.enter_dungeon(0);
    // Now we're in dungeon
    assert_ne!(g.enemies.len(), enemy_count_before);
    g.exit_dungeon();
    assert_eq!(g.world.location, Location::Overworld);
    assert_eq!((g.player_x, g.player_y), (ox, oy));
    assert_eq!(g.enemies.len(), enemy_count_before);
}

#[test]
fn descend_changes_level() {
    let mut g = overworld_game();
    g.enter_dungeon(0);
    assert_eq!(g.world.location, Location::Dungeon { index: 0, level: 0 });
    g.descend(0, 0);
    assert_eq!(g.world.location, Location::Dungeon { index: 0, level: 1 });
}

#[test]
fn ascend_changes_level() {
    let mut g = overworld_game();
    g.enter_dungeon(0);
    g.descend(0, 0);
    assert_eq!(g.world.location, Location::Dungeon { index: 0, level: 1 });
    g.ascend(0, 1);
    assert_eq!(g.world.location, Location::Dungeon { index: 0, level: 0 });
}

#[test]
fn round_trip_dungeon_preserves_overworld_position() {
    let mut g = overworld_game();
    let (ox, oy) = (g.player_x, g.player_y);
    // Enter dungeon
    g.enter_dungeon(0);
    // Descend to level 2
    g.descend(0, 0);
    g.descend(0, 1);
    assert_eq!(g.world.location, Location::Dungeon { index: 0, level: 2 });
    // Ascend back
    g.ascend(0, 2);
    g.ascend(0, 1);
    assert_eq!(g.world.location, Location::Dungeon { index: 0, level: 0 });
    // Exit to overworld
    g.exit_dungeon();
    assert_eq!(g.world.location, Location::Overworld);
    assert_eq!((g.player_x, g.player_y), (ox, oy));
}

#[test]
fn stairs_connect_correct_levels() {
    let mut g = overworld_game();
    g.enter_dungeon(0);
    // On level 0, player should be at StairsUp
    assert_eq!(g.current_map().get(g.player_x, g.player_y), Tile::StairsUp);
    // Descend
    g.descend(0, 0);
    // On level 1, player should be at StairsUp
    assert_eq!(g.current_map().get(g.player_x, g.player_y), Tile::StairsUp);
}

#[test]
fn dungeon_enemies_spawn_on_walkable() {
    let mut g = overworld_game();
    g.enter_dungeon(0);
    for e in &g.enemies {
        assert!(g.current_map().is_walkable(e.x, e.y),
            "{} at ({},{}) not walkable", e.name, e.x, e.y);
    }
}

#[test]
fn dungeon_has_classic_enemies() {
    let mut g = overworld_game();
    g.enter_dungeon(0);
    // Level 0: rats, kobolds, slimes, goblins, skeletons
    let l0_glyphs = ['r', 'c', 'S', 'g', 's'];
    for e in &g.enemies {
        assert!(
            l0_glyphs.contains(&e.glyph),
            "unexpected dungeon L0 enemy: {} ('{}')", e.name, e.glyph
        );
    }
}

#[test]
fn no_dragon_on_shallow_levels() {
    let mut g = overworld_game();
    // Check level 0 of the first dungeon — no dragon
    g.enter_dungeon(0);
    assert!(
        !g.enemies.iter().any(|e| e.glyph == 'D'),
        "level 0 should not have a dragon"
    );
    // Check level 1 — no dragon
    g.descend(0, 0);
    assert!(
        !g.enemies.iter().any(|e| e.glyph == 'D'),
        "level 1 should not have a dragon"
    );
}

#[test]
fn deeper_dungeon_enemies_are_stronger() {
    let mut g = overworld_game();
    g.enter_dungeon(0);
    let l0_max_hp = g.enemies.iter().filter(|e| e.glyph != 'D').map(|e| e.hp).max().unwrap_or(0);

    g.descend(0, 0);
    g.descend(0, 1);
    let l2_max_hp = g.enemies.iter().filter(|e| e.glyph != 'D').map(|e| e.hp).max().unwrap_or(0);

    assert!(l2_max_hp > l0_max_hp,
        "deeper enemies should be stronger: l0_max={l0_max_hp}, l2_max={l2_max_hp}");
}

#[test]
fn transition_message_on_enter_dungeon() {
    let mut g = overworld_game();
    g.enter_dungeon(0);
    assert!(g.messages.iter().any(|m| m.contains("descend")));
}

#[test]
fn transition_message_on_exit_dungeon() {
    let mut g = overworld_game();
    g.enter_dungeon(0);
    g.exit_dungeon();
    assert!(g.messages.iter().any(|m| m.contains("overworld")));
}
