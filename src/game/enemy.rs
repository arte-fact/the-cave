use crate::tile::Tile;

use super::features::position::Position;
use super::item::Item;

#[derive(Clone, Debug, PartialEq)]
pub struct Enemy {
    pub name: String,
    pub tile: Tile,
    pub health: i32,
    pub attack: i32,
    pub defense: i32,
    pub occurences: i32,
    pub position: Position,
    pub behavior: Behavior,
    pub state: EnemyState,
    pub drop: Vec<(Item, i32)>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EnemyState {
    Idle,
    Attacking,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Behavior {
    Aggressive,
    Defensive,
    Passive,
}
