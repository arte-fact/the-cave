use super::direction::Direction;

#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position { x, y }
    }

    pub fn distance_to(&self, other: &Position) -> f32 {
        let x = (other.x - self.x).abs().pow(2) as f32;
        let y = (other.y - self.y).abs().pow(2) as f32;
        (x + y).sqrt()
    }

    pub fn is_adjacent_to(&self, other: &Position) -> bool {
        self.x == other.x && (self.y - other.y).abs() == 1
            || self.y == other.y && (self.x - other.x).abs() == 1
    }

    pub fn add_position(&mut self, other: &Position) {
        self.x += other.x;
        self.y += other.y;
    }

    pub fn add_direction(&self, direction: &Direction) -> Position {
        match direction {
            Direction::Up => Position::new(self.x, self.y - 1),
            Direction::Down => Position::new(self.x, self.y + 1),
            Direction::Left => Position::new(self.x - 1, self.y),
            Direction::Right => Position::new(self.x + 1, self.y),
        }
    }

    pub fn adjacent_positions(&self) -> Vec<Position> {
        vec![
            Position::new(self.x + 1, self.y),
            Position::new(self.x - 1, self.y),
            Position::new(self.x, self.y + 1),
            Position::new(self.x, self.y - 1),
        ]
    }

    pub fn get_circle(&self, radius: i32) -> Vec<Position> {
        let mut positions = vec![];
        for x in -radius..radius {
            for y in -radius..radius {
                let new_pos = Position::new(self.x + x, self.y + y);
                if self.distance_to(&new_pos) <= radius as f32 {
                    positions.push(new_pos);
                }
            }
        }
        positions
    }

    pub fn get_square(&self, size: i32) -> Vec<Position> {
        let mut positions = vec![];
        for x in -size..size {
            for y in -size..size {
                positions.push(Position::new(self.x + x, self.y + y));
            }
        }
        positions
    }

    pub fn get_rectangle(&self, width: i32, height: i32) -> Vec<Position> {
        let mut positions = vec![];
        for x in 0..width {
            for y in 0..height {
                let x = self.x + x;
                let y = self.y + y;
                positions.push(Position::new(x, y));
            }
        }
        positions
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance_to() {
        let p1 = Position::new(0, 0);
        let p2 = Position::new(0, 4);
        assert_eq!(p1.distance_to(&p2), 4.0);

        let p1 = Position::new(4, 0);
        let p2 = Position::new(-3, 0);
        assert_eq!(p1.distance_to(&p2), 7.0);

        let p1 = Position::new(0, 0);
        let p2 = Position::new(3, -4);
        assert_eq!(p1.distance_to(&p2), 5.0);
    }

    #[test]
    fn test_is_adjacent_to() {
        let p1 = Position::new(0, 0);
        let p2 = Position::new(0, 1);
        assert_eq!(p1.is_adjacent_to(&p2), true);
    }

    #[test]
    fn test_add_position() {
        let mut p1 = Position::new(0, 0);
        let p2 = Position::new(3, 4);
        p1.add_position(&p2);
        assert_eq!(p1, Position::new(3, 4));
    }

}
