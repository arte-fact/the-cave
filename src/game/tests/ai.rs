use super::*;

/// Helper to create an enemy with a specific behavior type.
fn enemy_with_behavior(x: i32, y: i32, hp: i32, attack: i32, name: &'static str, behavior: EnemyBehavior) -> Enemy {
    Enemy {
        x, y, hp, attack, defense: 0, glyph: name.chars().next().unwrap_or('?'),
        name, facing_left: false, is_ranged: false,
        behavior, spawn_x: x, spawn_y: y, provoked: false,
    }
}

// ── Passive ──────────────────────────────────────────────────────────

#[test]
fn passive_enemy_does_not_attack_adjacent_player() {
    let map = Map::new_filled(20, 20, Tile::Floor);
    let mut g = Game::new(map);
    g.player_x = 10;
    g.player_y = 10;
    let hp_before = g.player_hp;
    g.enemies.push(enemy_with_behavior(11, 10, 10, 5, "Buzzard", EnemyBehavior::Passive));
    g.enemy_turn();
    assert_eq!(g.player_hp, hp_before, "passive enemy should not attack");
}

#[test]
fn passive_enemy_flees_when_provoked() {
    let map = Map::new_filled(20, 20, Tile::Floor);
    let mut g = Game::new(map);
    g.player_x = 10;
    g.player_y = 10;
    let mut e = enemy_with_behavior(12, 10, 10, 5, "Buzzard", EnemyBehavior::Passive);
    e.provoked = true;
    g.enemies.push(e);
    let dist_before = (g.enemies[0].x - g.player_x).abs() + (g.enemies[0].y - g.player_y).abs();
    g.enemy_turn();
    let dist_after = (g.enemies[0].x - g.player_x).abs() + (g.enemies[0].y - g.player_y).abs();
    assert!(dist_after > dist_before,
        "provoked passive should flee: dist was {dist_before}, now {dist_after}");
}

#[test]
fn passive_enemy_does_nothing_when_not_provoked() {
    let map = Map::new_filled(20, 20, Tile::Floor);
    let mut g = Game::new(map);
    g.player_x = 10;
    g.player_y = 10;
    // Within passive_flee_range (4) but not provoked → no movement
    g.enemies.push(enemy_with_behavior(12, 10, 10, 5, "Buzzard", EnemyBehavior::Passive));
    let (ex, ey) = (g.enemies[0].x, g.enemies[0].y);
    g.enemy_turn();
    assert_eq!(g.enemies[0].x, ex, "unprovoked passive should stay still");
    assert_eq!(g.enemies[0].y, ey);
}

#[test]
fn player_attack_sets_provoked_flag() {
    let map = Map::new_filled(20, 20, Tile::Floor);
    let mut g = Game::new(map);
    g.player_x = 10;
    g.player_y = 10;
    g.enemies.push(enemy_with_behavior(11, 10, 20, 1, "Buzzard", EnemyBehavior::Passive));
    assert!(!g.enemies[0].provoked);
    g.attack_adjacent(11, 10);
    assert!(g.enemies[0].provoked, "attacking should set provoked flag");
}

// ── Timid ────────────────────────────────────────────────────────────

#[test]
fn timid_enemy_flees_when_player_close() {
    let map = Map::new_filled(20, 20, Tile::Floor);
    let mut g = Game::new(map);
    g.player_x = 10;
    g.player_y = 10;
    // Place 3 tiles away (within timid_flee_range of 5)
    g.enemies.push(enemy_with_behavior(13, 10, 10, 3, "Fox", EnemyBehavior::Timid));
    let dist_before = (g.enemies[0].x - g.player_x).abs() + (g.enemies[0].y - g.player_y).abs();
    g.enemy_turn();
    let dist_after = (g.enemies[0].x - g.player_x).abs() + (g.enemies[0].y - g.player_y).abs();
    assert!(dist_after > dist_before,
        "timid enemy should flee: dist was {dist_before}, now {dist_after}");
}

#[test]
fn timid_enemy_fights_when_cornered() {
    let map = Map::new_filled(20, 20, Tile::Floor);
    let mut g = Game::new(map);
    g.player_x = 10;
    g.player_y = 10;
    g.player_dexterity = 0; // no dodge
    let hp_before = g.player_hp;
    let mut e = enemy_with_behavior(11, 10, 20, 5, "Fox", EnemyBehavior::Timid);
    e.provoked = true; // has been hit
    g.enemies.push(e);
    g.enemy_turn();
    assert!(g.player_hp < hp_before,
        "cornered timid enemy (adjacent + provoked) should fight back");
}

#[test]
fn timid_enemy_does_not_engage_when_far() {
    let map = Map::new_filled(30, 30, Tile::Floor);
    let mut g = Game::new(map);
    g.player_x = 5;
    g.player_y = 5;
    // Distance 8 — outside timid_flee_range (5), within chase_range (8)
    g.enemies.push(enemy_with_behavior(13, 5, 10, 3, "Fox", EnemyBehavior::Timid));
    let (ex, ey) = (g.enemies[0].x, g.enemies[0].y);
    g.enemy_turn();
    // Should not chase; should stay put (not fleeing either since outside flee range)
    assert_eq!(g.enemies[0].x, ex, "timid should not chase when beyond flee range");
    assert_eq!(g.enemies[0].y, ey);
}

// ── Territorial ──────────────────────────────────────────────────────

#[test]
fn territorial_ignores_distant_player() {
    let map = Map::new_filled(30, 30, Tile::Floor);
    let mut g = Game::new(map);
    g.player_x = 5;
    g.player_y = 5;
    // Distance 7 (> territorial_alert_range 4, < chase_range 8)
    g.enemies.push(enemy_with_behavior(12, 5, 15, 4, "Wolf", EnemyBehavior::Territorial));
    let (ex, ey) = (g.enemies[0].x, g.enemies[0].y);
    g.enemy_turn();
    assert_eq!(g.enemies[0].x, ex, "territorial should ignore player beyond alert range");
    assert_eq!(g.enemies[0].y, ey);
}

#[test]
fn territorial_engages_when_close() {
    let map = Map::new_filled(20, 20, Tile::Floor);
    let mut g = Game::new(map);
    g.player_x = 10;
    g.player_y = 10;
    // Distance 3 (< territorial_alert_range 4)
    g.enemies.push(enemy_with_behavior(13, 10, 15, 4, "Wolf", EnemyBehavior::Territorial));
    let dist_before = (g.enemies[0].x - g.player_x).abs() + (g.enemies[0].y - g.player_y).abs();
    g.enemy_turn();
    let dist_after = (g.enemies[0].x - g.player_x).abs() + (g.enemies[0].y - g.player_y).abs();
    assert!(dist_after < dist_before, "territorial should engage within alert range");
}

#[test]
fn territorial_returns_to_spawn_beyond_leash() {
    let map = Map::new_filled(30, 30, Tile::Floor);
    let mut g = Game::new(map);
    g.player_x = 5;
    g.player_y = 5;
    // Enemy is at (15,5), spawn was at (25,5).
    // Distance from spawn: |15-25| = 10 > leash (8).
    // Even though provoked, should walk TOWARD spawn, not player.
    let mut e = enemy_with_behavior(15, 5, 15, 4, "Wolf", EnemyBehavior::Territorial);
    e.spawn_x = 25;
    e.spawn_y = 5;
    e.provoked = true;
    g.enemies.push(e);
    let ex_before = g.enemies[0].x;
    g.enemy_turn();
    assert!(g.enemies[0].x > ex_before,
        "territorial beyond leash should return to spawn (move from {} toward 25)", ex_before);
}

#[test]
fn provoked_territorial_ignores_alert_range() {
    let map = Map::new_filled(30, 30, Tile::Floor);
    let mut g = Game::new(map);
    g.player_x = 5;
    g.player_y = 5;
    // Distance 6 (> alert_range 4) but provoked, within leash (6 from spawn at 11)
    let mut e = enemy_with_behavior(11, 5, 15, 4, "Wolf", EnemyBehavior::Territorial);
    e.provoked = true;
    g.enemies.push(e);
    let dist_before = (g.enemies[0].x - g.player_x).abs() + (g.enemies[0].y - g.player_y).abs();
    g.enemy_turn();
    let dist_after = (g.enemies[0].x - g.player_x).abs() + (g.enemies[0].y - g.player_y).abs();
    assert!(dist_after < dist_before,
        "provoked territorial within leash should engage despite being beyond alert range");
}

// ── Aggressive ───────────────────────────────────────────────────────

#[test]
fn aggressive_chases_as_before() {
    let map = Map::new_filled(20, 20, Tile::Floor);
    let mut g = Game::new(map);
    g.player_x = 10;
    g.player_y = 10;
    g.enemies.push(test_enemy(14, 10, 10, 3, "Goblin"));
    let dist_before = (g.enemies[0].x - g.player_x).abs() + (g.enemies[0].y - g.player_y).abs();
    g.enemy_turn();
    let dist_after = (g.enemies[0].x - g.player_x).abs() + (g.enemies[0].y - g.player_y).abs();
    assert!(dist_after < dist_before, "aggressive should chase player");
}

// ── Stalker ──────────────────────────────────────────────────────────

#[test]
fn stalker_does_not_move_when_far() {
    let map = Map::new_filled(30, 30, Tile::Floor);
    let mut g = Game::new(map);
    g.player_x = 5;
    g.player_y = 5;
    // Distance 7 (> stalker_activation_range 5)
    g.enemies.push(enemy_with_behavior(12, 5, 10, 5, "Viper", EnemyBehavior::Stalker));
    let (ex, ey) = (g.enemies[0].x, g.enemies[0].y);
    g.enemy_turn();
    assert_eq!(g.enemies[0].x, ex, "stalker should not move when far from player");
    assert_eq!(g.enemies[0].y, ey);
    assert!(!g.enemies[0].provoked, "stalker should not activate when far");
}

#[test]
fn stalker_activates_when_close() {
    let map = Map::new_filled(20, 20, Tile::Floor);
    let mut g = Game::new(map);
    g.player_x = 10;
    g.player_y = 10;
    // Distance 4 (< stalker_activation_range 5)
    g.enemies.push(enemy_with_behavior(14, 10, 10, 5, "Viper", EnemyBehavior::Stalker));
    g.enemy_turn();
    assert!(g.enemies[0].provoked, "stalker should activate when within range");
    let dist = (g.enemies[0].x - g.player_x).abs() + (g.enemies[0].y - g.player_y).abs();
    assert!(dist < 4, "activated stalker should chase toward player");
}

#[test]
fn stalker_pursues_with_extended_range() {
    let map = Map::new_filled(30, 30, Tile::Floor);
    let mut g = Game::new(map);
    g.player_x = 5;
    g.player_y = 5;
    // Distance 10 (> normal chase_range 8, < stalker_chase_range 12)
    let mut e = enemy_with_behavior(15, 5, 10, 5, "Viper", EnemyBehavior::Stalker);
    e.provoked = true; // already activated
    g.enemies.push(e);
    let dist_before = (g.enemies[0].x - g.player_x).abs() + (g.enemies[0].y - g.player_y).abs();
    g.enemy_turn();
    let dist_after = (g.enemies[0].x - g.player_x).abs() + (g.enemies[0].y - g.player_y).abs();
    assert!(dist_after < dist_before,
        "activated stalker should chase with extended range: was {dist_before}, now {dist_after}");
}

// ── Smart pathfinding ────────────────────────────────────────────────

#[test]
fn smart_enemy_navigates_around_wall() {
    // Wall blocks the direct signum-based path
    let mut map = Map::new_filled(20, 20, Tile::Floor);
    for y in 3..=7 { map.set(7, y, Tile::Wall); }
    let mut g = Game::new(map);
    g.player_x = 5;
    g.player_y = 5;
    // Goblin (smart, aggressive) on the other side of the wall, right next to it
    g.enemies.push(test_enemy(8, 5, 10, 3, "Goblin"));
    let (ex, ey) = (g.enemies[0].x, g.enemies[0].y);
    g.enemy_turn();
    // Smart enemy should have moved (not stuck) — found a path around
    assert!(g.enemies[0].x != ex || g.enemies[0].y != ey,
        "smart enemy (Goblin) should navigate around wall via A*");
}

#[test]
fn dumb_enemy_stuck_behind_wall() {
    // Same wall setup, but with a non-smart aggressive enemy
    let mut map = Map::new_filled(20, 20, Tile::Floor);
    for y in 3..=7 { map.set(7, y, Tile::Wall); }
    let mut g = Game::new(map);
    g.player_x = 5;
    g.player_y = 5;
    // Skeleton is aggressive but NOT in smart_enemy_names
    g.enemies.push(enemy_with_behavior(8, 5, 10, 3, "Skeleton", EnemyBehavior::Aggressive));
    let (ex, ey) = (g.enemies[0].x, g.enemies[0].y);
    g.enemy_turn();
    assert_eq!(g.enemies[0].x, ex, "dumb enemy should get stuck behind wall");
    assert_eq!(g.enemies[0].y, ey);
}

// ── Integration ──────────────────────────────────────────────────────

#[test]
fn behavior_assigned_from_config() {
    use crate::config::enemy_behavior;
    assert_eq!(enemy_behavior("Fox"), EnemyBehavior::Timid);
    assert_eq!(enemy_behavior("Buzzard"), EnemyBehavior::Passive);
    assert_eq!(enemy_behavior("Wolf"), EnemyBehavior::Territorial);
    assert_eq!(enemy_behavior("Goblin"), EnemyBehavior::Aggressive);
    assert_eq!(enemy_behavior("Viper"), EnemyBehavior::Stalker);
    assert_eq!(enemy_behavior("Bear"), EnemyBehavior::Territorial);
    assert_eq!(enemy_behavior("Lynx"), EnemyBehavior::Stalker);
    assert_eq!(enemy_behavior("Giant Rat"), EnemyBehavior::Passive);
}

#[test]
fn spawned_enemies_have_behavior_field() {
    let g = overworld_game();
    // All spawned enemies should have a behavior that matches their name
    for e in &g.enemies {
        use crate::config::enemy_behavior;
        assert_eq!(e.behavior, enemy_behavior(e.name),
            "{} should have behavior {:?}", e.name, enemy_behavior(e.name));
    }
}
