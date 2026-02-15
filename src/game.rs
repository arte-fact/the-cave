use crate::map::Map;

pub struct Game {
    pub player_x: i32,
    pub player_y: i32,
    pub map: Map,
}

impl Game {
    pub fn new(map: Map) -> Self {
        let (px, py) = map.find_spawn();
        Self {
            player_x: px,
            player_y: py,
            map,
        }
    }

    pub fn move_player(&mut self, dx: i32, dy: i32) {
        let nx = self.player_x + dx;
        let ny = self.player_y + dy;
        if self.map.is_walkable(nx, ny) {
            self.player_x = nx;
            self.player_y = ny;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_game() -> Game {
        let map = Map::generate(30, 20, 42);
        Game::new(map)
    }

    #[test]
    fn player_spawns_on_floor() {
        let g = test_game();
        assert!(g.map.is_walkable(g.player_x, g.player_y));
    }

    #[test]
    fn can_move_to_floor() {
        let mut g = test_game();
        let (sx, sy) = (g.player_x, g.player_y);
        // Try all four directions; at least one should be walkable
        let dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        let mut moved = false;
        for (dx, dy) in dirs {
            g.player_x = sx;
            g.player_y = sy;
            if g.map.is_walkable(sx + dx, sy + dy) {
                g.move_player(dx, dy);
                assert_eq!(g.player_x, sx + dx);
                assert_eq!(g.player_y, sy + dy);
                moved = true;
                break;
            }
        }
        assert!(moved, "spawn should have at least one adjacent floor");
    }

    #[test]
    fn blocked_by_wall() {
        let mut g = test_game();
        // Walk toward the border (always walls) — player should stop before it
        for _ in 0..g.map.width {
            g.move_player(-1, 0);
        }
        // Must still be on a floor tile (never entered a wall)
        assert!(g.map.is_walkable(g.player_x, g.player_y));
    }

    #[test]
    fn blocked_by_out_of_bounds() {
        let mut g = test_game();
        // Spam upward well past the map edge
        for _ in 0..g.map.height + 10 {
            g.move_player(0, -1);
        }
        assert!(g.player_y >= 0);
        assert!(g.map.is_walkable(g.player_x, g.player_y));
    }
}
