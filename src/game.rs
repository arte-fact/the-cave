use crate::map::Map;

pub struct Enemy {
    pub x: i32,
    pub y: i32,
    pub hp: i32,
    pub attack: i32,
    pub glyph: char,
    pub name: &'static str,
}

pub enum TurnResult {
    Moved,
    Blocked,
    Attacked { target_name: &'static str, damage: i32 },
    Killed { target_name: &'static str },
    PlayerDied,
    Won,
}

pub struct Game {
    pub player_x: i32,
    pub player_y: i32,
    pub player_hp: i32,
    pub player_max_hp: i32,
    pub player_attack: i32,
    pub map: Map,
    pub enemies: Vec<Enemy>,
    pub messages: Vec<String>,
    pub alive: bool,
    pub won: bool,
}

impl Game {
    pub fn new(map: Map) -> Self {
        let (px, py) = map.find_spawn();
        Self {
            player_x: px,
            player_y: py,
            player_hp: 20,
            player_max_hp: 20,
            player_attack: 5,
            map,
            enemies: Vec::new(),
            messages: vec!["You enter the cave.".into()],
            alive: true,
            won: false,
        }
    }

    pub fn new_overworld(map: Map) -> Self {
        let (px, py) = map.find_road_spawn();
        Self {
            player_x: px,
            player_y: py,
            player_hp: 20,
            player_max_hp: 20,
            player_attack: 5,
            map,
            enemies: Vec::new(),
            messages: vec!["You emerge into the forest.".into()],
            alive: true,
            won: false,
        }
    }

    pub fn spawn_enemies(&mut self, seed: u64) {
        let mut rng = seed;
        let mut placed = 0;
        for y in 2..self.map.height - 2 {
            for x in 2..self.map.width - 2 {
                if !self.map.is_walkable(x, y) {
                    continue;
                }
                if x == self.player_x && y == self.player_y {
                    continue;
                }
                rng = xorshift64(rng);
                // ~8% chance per floor tile
                if rng % 100 < 8 {
                    let is_dragon = placed > 0 && rng % 7 == 0;
                    self.enemies.push(if is_dragon {
                        Enemy { x, y, hp: 15, attack: 4, glyph: 'D', name: "Dragon" }
                    } else {
                        Enemy { x, y, hp: 5, attack: 2, glyph: 'g', name: "Goblin" }
                    });
                    placed += 1;
                }
            }
        }
        // Guarantee at least one dragon if none spawned
        if !self.enemies.iter().any(|e| e.glyph == 'D') {
            // Find a floor tile away from the player
            for y in (2..self.map.height - 2).rev() {
                for x in (2..self.map.width - 2).rev() {
                    if self.map.is_walkable(x, y)
                        && (x - self.player_x).abs() + (y - self.player_y).abs() > 5
                        && !self.enemies.iter().any(|e| e.x == x && e.y == y)
                    {
                        self.enemies.push(Enemy {
                            x, y, hp: 15, attack: 4, glyph: 'D', name: "Dragon",
                        });
                        return;
                    }
                }
            }
        }
    }

    pub fn move_player(&mut self, dx: i32, dy: i32) -> TurnResult {
        if !self.alive || self.won {
            return TurnResult::Blocked;
        }

        let nx = self.player_x + dx;
        let ny = self.player_y + dy;

        // Check for enemy at target
        if let Some(idx) = self.enemies.iter().position(|e| e.x == nx && e.y == ny && e.hp > 0) {
            let dmg = self.player_attack;
            self.enemies[idx].hp -= dmg;
            let name = self.enemies[idx].name;

            if self.enemies[idx].hp <= 0 {
                self.messages.push(format!("You slay the {name}!"));
                // Check win: dragon killed
                if self.enemies[idx].glyph == 'D' {
                    self.won = true;
                    self.messages.push("You conquered the cave!".into());
                    return TurnResult::Won;
                }
                return TurnResult::Killed { target_name: name };
            }
            self.messages.push(format!("You hit {name} for {dmg} damage."));

            // Enemy retaliates
            let retaliation = self.enemies[idx].attack;
            self.player_hp -= retaliation;
            self.messages.push(format!("{name} hits you for {retaliation} damage."));
            if self.player_hp <= 0 {
                self.alive = false;
                self.messages.push("You died.".into());
                return TurnResult::PlayerDied;
            }

            return TurnResult::Attacked { target_name: name, damage: dmg };
        }

        if !self.map.is_walkable(nx, ny) {
            return TurnResult::Blocked;
        }

        self.player_x = nx;
        self.player_y = ny;

        // Enemies take a turn
        self.enemy_turn();

        TurnResult::Moved
    }

    fn enemy_turn(&mut self) {
        let px = self.player_x;
        let py = self.player_y;

        for i in 0..self.enemies.len() {
            if self.enemies[i].hp <= 0 {
                continue;
            }
            let ex = self.enemies[i].x;
            let ey = self.enemies[i].y;
            let dist = (ex - px).abs() + (ey - py).abs();

            // Chase if within 5 tiles
            if dist <= 5 && dist > 1 {
                let dx = (px - ex).signum();
                let dy = (py - ey).signum();
                let candidates = [(ex + dx, ey), (ex, ey + dy)];
                for (cx, cy) in candidates {
                    if cx == px && cy == py {
                        // Attack player
                        let atk = self.enemies[i].attack;
                        let name = self.enemies[i].name;
                        self.player_hp -= atk;
                        self.messages.push(format!("{name} hits you for {atk} damage."));
                        if self.player_hp <= 0 {
                            self.alive = false;
                            self.messages.push("You died.".into());
                        }
                        break;
                    }
                    if self.map.is_walkable(cx, cy)
                        && !self.enemies.iter().any(|e| e.hp > 0 && e.x == cx && e.y == cy)
                    {
                        self.enemies[i].x = cx;
                        self.enemies[i].y = cy;
                        break;
                    }
                }
            }
        }
    }
}

fn xorshift64(mut state: u64) -> u64 {
    state ^= state << 13;
    state ^= state >> 7;
    state ^= state << 17;
    state
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_game() -> Game {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.spawn_enemies(123);
        g
    }

    // === Existing movement tests ===

    #[test]
    fn player_spawns_on_floor() {
        let g = test_game();
        assert!(g.map.is_walkable(g.player_x, g.player_y));
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
            if g.map.is_walkable(sx + dx, sy + dy)
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
        for _ in 0..g.map.width {
            g.move_player(-1, 0);
        }
        assert!(g.map.is_walkable(g.player_x, g.player_y));
    }

    #[test]
    fn blocked_by_out_of_bounds() {
        let mut g = test_game();
        for _ in 0..g.map.height + 10 {
            g.move_player(0, -1);
        }
        assert!(g.player_y >= 0);
        assert!(g.map.is_walkable(g.player_x, g.player_y));
    }

    // === Player stats ===

    #[test]
    fn player_starts_with_full_hp() {
        let g = test_game();
        assert_eq!(g.player_hp, 20);
        assert_eq!(g.player_max_hp, 20);
        assert_eq!(g.player_attack, 5);
        assert!(g.alive);
        assert!(!g.won);
    }

    // === Enemy spawning ===

    #[test]
    fn enemies_spawn_on_floor() {
        let g = test_game();
        for e in &g.enemies {
            assert!(g.map.is_walkable(e.x, e.y), "{} at ({},{}) not on floor", e.name, e.x, e.y);
        }
    }

    #[test]
    fn enemies_not_on_player() {
        let g = test_game();
        for e in &g.enemies {
            assert!(
                e.x != g.player_x || e.y != g.player_y,
                "enemy spawned on player"
            );
        }
    }

    #[test]
    fn at_least_one_dragon() {
        let g = test_game();
        assert!(
            g.enemies.iter().any(|e| e.glyph == 'D'),
            "must have a dragon to win"
        );
    }

    #[test]
    fn at_least_one_goblin() {
        let g = test_game();
        assert!(
            g.enemies.iter().any(|e| e.glyph == 'g'),
            "should have goblins"
        );
    }

    // === Combat ===

    #[test]
    fn attacking_enemy_deals_damage() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        // Place a goblin adjacent to the player
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 10, attack: 2, glyph: 'g', name: "Goblin" });
        g.move_player(1, 0);
        assert_eq!(g.enemies[0].hp, 10 - g.player_attack);
        // Player didn't move onto the enemy's tile
        assert_eq!(g.player_x, gx - 1);
    }

    #[test]
    fn killing_enemy() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 3, attack: 1, glyph: 'g', name: "Goblin" });
        let result = g.move_player(1, 0);
        assert!(matches!(result, TurnResult::Killed { .. }));
        assert!(g.enemies[0].hp <= 0);
    }

    #[test]
    fn enemy_retaliates() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 20, attack: 3, glyph: 'g', name: "Goblin" });
        let hp_before = g.player_hp;
        g.move_player(1, 0);
        assert_eq!(g.player_hp, hp_before - 3);
    }

    #[test]
    fn player_can_die() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.player_hp = 1;
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 99, attack: 5, glyph: 'g', name: "Goblin" });
        let result = g.move_player(1, 0);
        assert!(matches!(result, TurnResult::PlayerDied));
        assert!(!g.alive);
    }

    #[test]
    fn dead_player_cant_move() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.alive = false;
        let (x, y) = (g.player_x, g.player_y);
        g.move_player(1, 0);
        assert_eq!((g.player_x, g.player_y), (x, y));
    }

    #[test]
    fn killing_dragon_wins() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let dx = g.player_x + 1;
        let dy = g.player_y;
        g.enemies.push(Enemy { x: dx, y: dy, hp: 1, attack: 0, glyph: 'D', name: "Dragon" });
        let result = g.move_player(1, 0);
        assert!(matches!(result, TurnResult::Won));
        assert!(g.won);
    }

    #[test]
    fn won_player_cant_move() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.won = true;
        let (x, y) = (g.player_x, g.player_y);
        g.move_player(1, 0);
        assert_eq!((g.player_x, g.player_y), (x, y));
    }

    // === Messages ===

    #[test]
    fn initial_message() {
        let g = test_game();
        assert_eq!(g.messages[0], "You enter the cave.");
    }

    #[test]
    fn combat_generates_messages() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 20, attack: 2, glyph: 'g', name: "Goblin" });
        let msg_count_before = g.messages.len();
        g.move_player(1, 0);
        assert!(g.messages.len() > msg_count_before, "combat should generate messages");
    }

    // === Enemy AI ===

    #[test]
    fn enemy_chases_player() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        // Find a floor tile 3 steps to the right of player
        let ex = g.player_x + 3;
        let ey = g.player_y;
        if g.map.is_walkable(ex, ey) {
            g.enemies.push(Enemy { x: ex, y: ey, hp: 10, attack: 1, glyph: 'g', name: "Goblin" });
            // Move player somewhere (down if possible, to trigger enemy turn)
            if g.map.is_walkable(g.player_x, g.player_y + 1) {
                g.move_player(0, 1);
                // Enemy should have moved closer
                let new_dist = (g.enemies[0].x - g.player_x).abs() + (g.enemies[0].y - g.player_y).abs();
                assert!(new_dist < 4, "enemy should have chased toward player, dist={new_dist}");
            }
        }
    }
}
