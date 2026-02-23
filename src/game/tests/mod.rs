mod movement;
mod combat;
mod dungeon;
mod durability;
mod inventory;
mod progression;
mod survival;
mod quickbar;

use super::*;
use crate::map::Tile;
use crate::config::GameConfig;

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
    Item { kind: ItemKind::Weapon, name: "Rusty Sword", glyph: '/', effect: ItemEffect::BuffAttack(3), weight: 2, durability: 20 }
}

pub(super) fn overworld_game() -> Game {
    let mut map = Map::generate_forest(200, 200, 42);
    let entrances = map.place_dungeons(42);
    map.build_roads(&entrances);
    let world = World::new(map, entrances, 99);
    let mut g = Game::new_overworld(world);
    g.spawn_enemies(777);
    g
}
