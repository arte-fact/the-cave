use crate::tile::Tile;

use super::features::position::Position;

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub name: String,
    pub tile: Tile,
    pub occurences: i32,
    pub health: i32,
    pub attack: i32,
    pub defense: i32,
    pub position: Position,
}

