use super::features::direction::Direction;
use super::features::position::Position;

#[derive(Clone, Debug, PartialEq)]
pub struct Player {
    pub character: Box<char>,
    pub health: i32,
    pub attack: i32,
    pub defense: i32,
    pub position: Position,
    pub direction: Direction,
}

