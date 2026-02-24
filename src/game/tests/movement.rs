use super::*;
use super::test_game;

#[test]
fn player_spawns_on_floor() {
    let g = test_game();
    assert!(g.current_map().is_walkable(g.player_x, g.player_y));
}

#[test]
fn can_move_to_floor() {
    let mut g = test_game();
    let (sx, sy) = (g.player_x, g.player_y);
    let dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)];
    let mut moved = false;
    for (dx, dy) in dirs {
        g.player_x = sx;
        g.player_y = sy;
        if g.current_map().is_walkable(sx + dx, sy + dy)
            && !g.enemies.iter().any(|e| e.x == sx + dx && e.y == sy + dy)
        {
            g.move_player(dx, dy);
            assert_eq!(g.player_x, sx + dx);
            assert_eq!(g.player_y, sy + dy);
            moved = true;
            break;
        }
    }
    assert!(moved, "spawn should have at least one adjacent open floor");
}

#[test]
fn blocked_by_wall() {
    let mut g = test_game();
    let w = g.current_map().width;
    for _ in 0..w {
        g.move_player(-1, 0);
    }
    assert!(g.current_map().is_walkable(g.player_x, g.player_y));
}

#[test]
fn blocked_by_out_of_bounds() {
    let mut g = test_game();
    let h = g.current_map().height;
    for _ in 0..h + 10 {
        g.move_player(0, -1);
    }
    assert!(g.player_y >= 0);
    assert!(g.current_map().is_walkable(g.player_x, g.player_y));
}

#[test]
fn diagonal_move_works_on_open_floor() {
    // Create a small open map
    let map = Map::new_filled(10, 10, Tile::Floor);
    let mut g = Game::new(map);
    g.player_x = 5;
    g.player_y = 5;
    let result = g.move_player(1, 1); // DownRight
    assert_ne!(result, TurnResult::Blocked);
    assert_eq!(g.player_x, 6);
    assert_eq!(g.player_y, 6);
}

#[test]
fn diagonal_move_blocked_by_corner_cutting() {
    // Create a map where diagonal would cut a corner:
    // . W
    // W .
    // Player at (1,1), wall at (2,1) and (1,2): diagonal to (2,2) should be blocked
    let mut map = Map::new_filled(5, 5, Tile::Floor);
    map.set(2, 1, Tile::Wall);
    map.set(1, 2, Tile::Wall);
    let mut g = Game::new(map);
    g.player_x = 1;
    g.player_y = 1;
    let result = g.move_player(1, 1); // DownRight
    assert_eq!(result, TurnResult::Blocked);
    assert_eq!(g.player_x, 1);
    assert_eq!(g.player_y, 1);
}

#[test]
fn diagonal_move_allowed_when_one_adjacent_open() {
    // Wall only on one side — should still be blocked (both adjacents needed)
    let mut map = Map::new_filled(5, 5, Tile::Floor);
    map.set(2, 1, Tile::Wall); // wall east
    // (1,2) is floor — south is open
    let mut g = Game::new(map);
    g.player_x = 1;
    g.player_y = 1;
    let result = g.move_player(1, 1); // DownRight
    assert_eq!(result, TurnResult::Blocked);
}

#[test]
fn diagonal_move_all_four_diagonals() {
    let diags = [(-1, -1), (1, -1), (-1, 1), (1, 1)];
    for (dx, dy) in diags {
        let mut g = Game::new(Map::new_filled(10, 10, Tile::Floor));
        g.player_x = 5;
        g.player_y = 5;
        let result = g.move_player(dx, dy);
        assert_ne!(result, TurnResult::Blocked, "diagonal ({},{}) should work on open floor", dx, dy);
        assert_eq!(g.player_x, 5 + dx);
        assert_eq!(g.player_y, 5 + dy);
    }
}

#[test]
fn attack_adjacent_diagonal() {
    // Player can attack an enemy diagonally
    let map = Map::new_filled(10, 10, Tile::Floor);
    let mut g = Game::new(map);
    g.player_x = 5;
    g.player_y = 5;
    g.enemies.push(Enemy {
        x: 6, y: 6, hp: 10,
        name: "Goblin", glyph: 'g',
        attack: 1, defense: 0,
        facing_left: false, is_ranged: false,
        behavior: EnemyBehavior::Aggressive, spawn_x: 6, spawn_y: 6, provoked: false,
    });
    let result = g.attack_adjacent(6, 6);
    assert_ne!(result, TurnResult::Blocked, "diagonal attack should work");
}

#[test]
fn attack_adjacent_too_far() {
    // Chebyshev distance 2 should be blocked
    let map = Map::new_filled(10, 10, Tile::Floor);
    let mut g = Game::new(map);
    g.player_x = 5;
    g.player_y = 5;
    g.enemies.push(Enemy {
        x: 7, y: 7, hp: 10,
        name: "Goblin", glyph: 'g',
        attack: 1, defense: 0,
        facing_left: false, is_ranged: false,
        behavior: EnemyBehavior::Aggressive, spawn_x: 7, spawn_y: 7, provoked: false,
    });
    let result = g.attack_adjacent(7, 7);
    assert_eq!(result, TurnResult::Blocked, "distance 2 should be blocked");
}
