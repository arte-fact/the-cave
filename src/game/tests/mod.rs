mod movement;
mod combat;
mod dungeon;
mod durability;
mod inventory;
mod progression;
mod survival;
mod quickbar;
mod ai;

use super::*;
use crate::map::Tile;
use crate::config::{GameConfig, MapGenConfig, EnemyBehavior};

/// Helper to create a test enemy with default aggressive behavior.
pub(super) fn test_enemy(x: i32, y: i32, hp: i32, attack: i32, name: &'static str) -> Enemy {
    Enemy {
        x, y, hp, attack, defense: 0, glyph: name.chars().next().unwrap_or('?'),
        name, facing_left: false, is_ranged: false,
        behavior: EnemyBehavior::Aggressive, spawn_x: x, spawn_y: y, provoked: false,
    }
}

pub(super) fn test_game() -> Game {
    let map = Map::generate(30, 20, 42);
    let mut g = Game::new(map);
    g.spawn_enemies(123);
    g
}

pub(super) fn health_potion() -> Item {
    Item { kind: ItemKind::Potion, name: "Health Potion", glyph: '!', effect: ItemEffect::Heal(5), weight: 0, durability: 0 }
}

pub(super) fn rusty_sword() -> Item {
    Item { kind: ItemKind::Weapon, name: "Rusty Sword", glyph: '/', effect: ItemEffect::BuffAttack(3), weight: 2, durability: 200 }
}

pub(super) fn overworld_game() -> Game {
    let cfg = MapGenConfig::normal();
    let mut map = Map::generate_forest(200, 200, 42, &cfg);
    let entrances = map.place_dungeons(42, &cfg);
    map.build_roads(&entrances, &cfg);
    let world = World::new(map, entrances, 99, &cfg);
    let mut g = Game::new_overworld(world);
    g.spawn_enemies(777);
    g
}
