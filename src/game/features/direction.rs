use rand::Rng;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn all() -> Vec<Direction> {
        vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right]
    }
    pub fn random() -> Direction {
        let directions = Direction::all();
        let random = Direction::all()[rand::thread_rng().gen_range(0..directions.len())].clone();
        random
    }
}

