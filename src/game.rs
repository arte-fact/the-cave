pub struct Game {
    pub player_x: i32,
    pub player_y: i32,
    pub grid_cols: i32,
    pub grid_rows: i32,
}

impl Game {
    pub fn new(cols: i32, rows: i32) -> Self {
        Self {
            player_x: cols / 2,
            player_y: rows / 2,
            grid_cols: cols,
            grid_rows: rows,
        }
    }

    pub fn move_player(&mut self, dx: i32, dy: i32) {
        let nx = self.player_x + dx;
        let ny = self.player_y + dy;
        if nx >= 0 && nx < self.grid_cols && ny >= 0 && ny < self.grid_rows {
            self.player_x = nx;
            self.player_y = ny;
        }
    }
}
