use crate::map::{Map, Tile};
use crate::world::{Location, World};

#[derive(Clone)]
pub struct Enemy {
    pub x: i32,
    pub y: i32,
    pub hp: i32,
    pub attack: i32,
    pub glyph: char,
    pub name: &'static str,
    /// true when sprite should face left (mirrored).
    pub facing_left: bool,
}

pub enum TurnResult {
    Moved,
    Blocked,
    Attacked { target_name: &'static str, damage: i32 },
    Killed { target_name: &'static str },
    PlayerDied,
    Won,
    /// Player stepped on a transition tile and changed maps.
    MapChanged,
}

pub struct Game {
    pub player_x: i32,
    pub player_y: i32,
    pub player_hp: i32,
    pub player_max_hp: i32,
    pub player_attack: i32,
    /// true when player sprite should face left (mirrored).
    pub player_facing_left: bool,
    pub world: World,
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
            player_facing_left: false,
            world: World::from_single_map(map),
            enemies: Vec::new(),
            messages: vec!["You enter the cave.".into()],
            alive: true,
            won: false,
        }
    }

    pub fn new_overworld(world: World) -> Self {
        let (px, py) = world.overworld.find_road_spawn();
        Self {
            player_x: px,
            player_y: py,
            player_hp: 20,
            player_max_hp: 20,
            player_attack: 5,
            player_facing_left: false,
            world,
            enemies: Vec::new(),
            messages: vec!["You emerge into the forest.".into()],
            alive: true,
            won: false,
        }
    }

    /// Convenience accessor for the current map.
    pub fn current_map(&self) -> &Map {
        self.world.current_map()
    }

    /// FOV radius: 8 on overworld, 6 in dungeons.
    fn fov_radius(&self) -> i32 {
        match self.world.location {
            Location::Overworld => 8,
            Location::Dungeon { .. } => 6,
        }
    }

    /// Age Visible→Seen, then recompute FOV from player position.
    pub fn update_fov(&mut self) {
        let r = self.fov_radius();
        let map = self.world.current_map_mut();
        map.age_visibility();
        map.compute_fov(self.player_x, self.player_y, r);
    }

    /// Spawn forest animals on the overworld: wolves, boars, bears.
    pub fn spawn_enemies(&mut self, seed: u64) {
        let map = self.world.current_map();
        let mut rng = seed;
        for y in 2..map.height - 2 {
            for x in 2..map.width - 2 {
                if !map.is_walkable(x, y) {
                    continue;
                }
                if x == self.player_x && y == self.player_y {
                    continue;
                }
                rng = xorshift64(rng);
                // ~3% chance per walkable tile (forest is sparse)
                if rng % 100 < 3 {
                    rng = xorshift64(rng);
                    let roll = rng % 100;
                    let enemy = if roll < 60 {
                        Enemy { x, y, hp: 4, attack: 2, glyph: 'w', name: "Wolf", facing_left: false }
                    } else if roll < 85 {
                        Enemy { x, y, hp: 6, attack: 2, glyph: 'b', name: "Boar", facing_left: false }
                    } else {
                        Enemy { x, y, hp: 10, attack: 3, glyph: 'B', name: "Bear", facing_left: false }
                    };
                    self.enemies.push(enemy);
                }
            }
        }
    }

    /// Spawn enemies appropriate for a dungeon level.
    /// Level 0: goblins + skeletons. Level 1: goblins + skeletons + orcs.
    /// Level 2: skeletons + orcs + trolls.
    /// Cave level (level 3, only in the dragon's dungeon): troll minions + unique dragon boss.
    fn spawn_dungeon_enemies(&mut self, dungeon_index: usize, level: usize) {
        let total_levels = self.world.dungeons[dungeon_index].levels.len();
        let is_cave = total_levels == 4 && level == 3;

        let map = self.world.current_map();
        let seed = (dungeon_index as u64)
            .wrapping_mul(31)
            .wrapping_add(level as u64)
            .wrapping_mul(6364136223846793005);
        let mut rng = seed;
        for y in 1..map.height - 1 {
            for x in 1..map.width - 1 {
                if !map.is_walkable(x, y) {
                    continue;
                }
                if x == self.player_x && y == self.player_y {
                    continue;
                }
                let tile = map.get(x, y);
                if tile == Tile::StairsUp || tile == Tile::StairsDown {
                    continue;
                }
                rng = xorshift64(rng);
                let spawn_chance = if is_cave { 6 } else { 10 };
                if rng % 100 < spawn_chance {
                    rng = xorshift64(rng);
                    let roll = rng % 100;
                    let enemy = if is_cave {
                        // Cave level: troll minions guarding the dragon
                        if roll < 50 {
                            Enemy { x, y, hp: 15, attack: 5, glyph: 'T', name: "Troll", facing_left: false }
                        } else {
                            Enemy { x, y, hp: 12, attack: 5, glyph: 'o', name: "Orc", facing_left: false }
                        }
                    } else {
                        match level {
                            0 => {
                                if roll < 70 {
                                    Enemy { x, y, hp: 5, attack: 2, glyph: 'g', name: "Goblin", facing_left: false }
                                } else {
                                    Enemy { x, y, hp: 6, attack: 3, glyph: 's', name: "Skeleton", facing_left: false }
                                }
                            }
                            1 => {
                                if roll < 40 {
                                    Enemy { x, y, hp: 7, attack: 3, glyph: 'g', name: "Goblin", facing_left: false }
                                } else if roll < 70 {
                                    Enemy { x, y, hp: 8, attack: 4, glyph: 's', name: "Skeleton", facing_left: false }
                                } else {
                                    Enemy { x, y, hp: 10, attack: 4, glyph: 'o', name: "Orc", facing_left: false }
                                }
                            }
                            _ => {
                                if roll < 30 {
                                    Enemy { x, y, hp: 9, attack: 4, glyph: 's', name: "Skeleton", facing_left: false }
                                } else if roll < 60 {
                                    Enemy { x, y, hp: 12, attack: 5, glyph: 'o', name: "Orc", facing_left: false }
                                } else {
                                    Enemy { x, y, hp: 15, attack: 5, glyph: 'T', name: "Troll", facing_left: false }
                                }
                            }
                        }
                    };
                    self.enemies.push(enemy);
                }
            }
        }

        // Place unique dragon boss only in the cave level
        if is_cave {
            let map = self.world.current_map();
            for y in (1..map.height - 1).rev() {
                for x in (1..map.width - 1).rev() {
                    if map.is_walkable(x, y)
                        && map.get(x, y) == Tile::Floor
                        && (x - self.player_x).abs() + (y - self.player_y).abs() > 5
                        && !self.enemies.iter().any(|e| e.x == x && e.y == y)
                    {
                        self.enemies.push(Enemy {
                            x, y, hp: 30, attack: 8, glyph: 'D', name: "Dragon", facing_left: false,
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

        // Update facing direction on horizontal movement
        if dx < 0 { self.player_facing_left = true; }
        if dx > 0 { self.player_facing_left = false; }

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

        if !self.world.current_map().is_walkable(nx, ny) {
            return TurnResult::Blocked;
        }

        self.player_x = nx;
        self.player_y = ny;

        // Check for map transitions
        let tile = self.world.current_map().get(nx, ny);
        if self.try_transition(tile, nx, ny) {
            return TurnResult::MapChanged;
        }

        // Enemies take a turn
        self.enemy_turn();

        // Update fog of war
        self.update_fov();

        TurnResult::Moved
    }

    /// Handle map transitions based on the tile the player stepped on.
    /// Returns true if a transition occurred.
    fn try_transition(&mut self, tile: Tile, x: i32, y: i32) -> bool {
        match tile {
            Tile::DungeonEntrance => {
                if let Location::Overworld = self.world.location {
                    if let Some(di) = self.world.dungeon_at(x, y) {
                        self.enter_dungeon(di);
                        return true;
                    }
                }
            }
            Tile::StairsDown => {
                if let Location::Dungeon { index, level } = self.world.location.clone() {
                    if level + 1 < self.world.dungeons[index].levels.len() {
                        self.descend(index, level);
                        return true;
                    }
                }
            }
            Tile::StairsUp => {
                match self.world.location.clone() {
                    Location::Dungeon { level: 0, .. } => {
                        self.exit_dungeon();
                        return true;
                    }
                    Location::Dungeon { index, level } => {
                        self.ascend(index, level);
                        return true;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        false
    }

    fn enter_dungeon(&mut self, dungeon_index: usize) {
        // Save overworld state
        self.world.saved_overworld_pos = (self.player_x, self.player_y);
        self.world.saved_overworld_enemies = self.enemies.clone();

        // Switch to dungeon level 0
        self.world.location = Location::Dungeon { index: dungeon_index, level: 0 };
        let map = self.world.current_map();

        // Place player at StairsUp
        if let Some((sx, sy)) = map.find_tile(Tile::StairsUp) {
            self.player_x = sx;
            self.player_y = sy;
        } else {
            let (sx, sy) = map.find_spawn();
            self.player_x = sx;
            self.player_y = sy;
        }

        self.enemies.clear();
        self.spawn_dungeon_enemies(dungeon_index, 0);
        self.messages.push("You descend into the dungeon.".into());
        self.update_fov();
    }

    fn exit_dungeon(&mut self) {
        // Restore overworld state
        let (ox, oy) = self.world.saved_overworld_pos;
        self.player_x = ox;
        self.player_y = oy;
        self.enemies = std::mem::take(&mut self.world.saved_overworld_enemies);
        self.world.location = Location::Overworld;
        self.messages.push("You return to the overworld.".into());
        self.update_fov();
    }

    fn descend(&mut self, dungeon_index: usize, current_level: usize) {
        self.world.location = Location::Dungeon { index: dungeon_index, level: current_level + 1 };
        let map = self.world.current_map();
        if let Some((sx, sy)) = map.find_tile(Tile::StairsUp) {
            self.player_x = sx;
            self.player_y = sy;
        } else {
            let (sx, sy) = map.find_spawn();
            self.player_x = sx;
            self.player_y = sy;
        }
        self.enemies.clear();
        self.spawn_dungeon_enemies(dungeon_index, current_level + 1);
        self.messages.push(format!("You descend to level {}.", current_level + 2));
        self.update_fov();
    }

    fn ascend(&mut self, dungeon_index: usize, current_level: usize) {
        self.world.location = Location::Dungeon { index: dungeon_index, level: current_level - 1 };
        let map = self.world.current_map();
        if let Some((sx, sy)) = map.find_tile(Tile::StairsDown) {
            self.player_x = sx;
            self.player_y = sy;
        } else {
            let (sx, sy) = map.find_spawn();
            self.player_x = sx;
            self.player_y = sy;
        }
        self.enemies.clear();
        self.spawn_dungeon_enemies(dungeon_index, current_level - 1);
        self.messages.push(format!("You ascend to level {}.", current_level));
        self.update_fov();
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
                    if self.world.current_map().is_walkable(cx, cy)
                        && !self.enemies.iter().any(|e| e.hp > 0 && e.x == cx && e.y == cy)
                    {
                        let move_dx = cx - self.enemies[i].x;
                        if move_dx < 0 { self.enemies[i].facing_left = true; }
                        if move_dx > 0 { self.enemies[i].facing_left = false; }
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

    // === Movement tests ===

    #[test]
    fn player_spawns_on_floor() {
        let g = test_game();
        assert!(g.current_map().is_walkable(g.player_x, g.player_y));
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
            if g.current_map().is_walkable(sx + dx, sy + dy)
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
        let w = g.current_map().width;
        for _ in 0..w {
            g.move_player(-1, 0);
        }
        assert!(g.current_map().is_walkable(g.player_x, g.player_y));
    }

    #[test]
    fn blocked_by_out_of_bounds() {
        let mut g = test_game();
        let h = g.current_map().height;
        for _ in 0..h + 10 {
            g.move_player(0, -1);
        }
        assert!(g.player_y >= 0);
        assert!(g.current_map().is_walkable(g.player_x, g.player_y));
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
            assert!(g.current_map().is_walkable(e.x, e.y), "{} at ({},{}) not on floor", e.name, e.x, e.y);
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
    fn overworld_has_forest_animals() {
        let g = test_game();
        assert!(
            g.enemies.iter().any(|e| e.glyph == 'w'),
            "overworld should have wolves"
        );
        for e in &g.enemies {
            assert!(
                e.glyph == 'w' || e.glyph == 'b' || e.glyph == 'B',
                "unexpected overworld enemy: {} ({})", e.name, e.glyph
            );
        }
    }

    #[test]
    fn cave_level_has_dragon_boss() {
        let mut g = overworld_game();
        // Find the dungeon with 4 levels (the cave dungeon)
        let cave_di = g.world.dungeons.iter()
            .position(|d| d.levels.len() == 4)
            .expect("one dungeon should have a cave level");
        g.enter_dungeon(cave_di);
        // Descend to the cave (level 3)
        for level in 0..3 {
            g.descend(cave_di, level);
        }
        assert!(
            g.enemies.iter().any(|e| e.glyph == 'D'),
            "cave level should have dragon boss"
        );
        let dragon = g.enemies.iter().find(|e| e.glyph == 'D').unwrap();
        assert!(dragon.hp >= 30, "dragon hp should be >= 30, got {}", dragon.hp);
        assert!(dragon.attack >= 8, "dragon attack should be >= 8, got {}", dragon.attack);
    }

    #[test]
    fn non_cave_dungeon_has_no_dragon() {
        let mut g = overworld_game();
        // Find a dungeon without the cave (3 levels)
        let normal_di = g.world.dungeons.iter()
            .position(|d| d.levels.len() == 3)
            .expect("should have normal dungeons");
        g.enter_dungeon(normal_di);
        // Descend to deepest level (level 2)
        g.descend(normal_di, 0);
        g.descend(normal_di, 1);
        assert!(
            !g.enemies.iter().any(|e| e.glyph == 'D'),
            "non-cave dungeon should not have a dragon"
        );
    }

    // === Combat ===

    #[test]
    fn attacking_enemy_deals_damage() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 10, attack: 2, glyph: 'g', name: "Goblin", facing_left: false });
        g.move_player(1, 0);
        assert_eq!(g.enemies[0].hp, 10 - g.player_attack);
        assert_eq!(g.player_x, gx - 1);
    }

    #[test]
    fn killing_enemy() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 3, attack: 1, glyph: 'g', name: "Goblin", facing_left: false });
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
        g.enemies.push(Enemy { x: gx, y: gy, hp: 20, attack: 3, glyph: 'g', name: "Goblin", facing_left: false });
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
        g.enemies.push(Enemy { x: gx, y: gy, hp: 99, attack: 5, glyph: 'g', name: "Goblin", facing_left: false });
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
        g.enemies.push(Enemy { x: dx, y: dy, hp: 1, attack: 0, glyph: 'D', name: "Dragon", facing_left: false });
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
        g.enemies.push(Enemy { x: gx, y: gy, hp: 20, attack: 2, glyph: 'g', name: "Goblin", facing_left: false });
        let msg_count_before = g.messages.len();
        g.move_player(1, 0);
        assert!(g.messages.len() > msg_count_before, "combat should generate messages");
    }

    // === Enemy AI ===

    #[test]
    fn enemy_chases_player() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let ex = g.player_x + 3;
        let ey = g.player_y;
        if g.current_map().is_walkable(ex, ey) {
            g.enemies.push(Enemy { x: ex, y: ey, hp: 10, attack: 1, glyph: 'g', name: "Goblin", facing_left: false });
            if g.current_map().is_walkable(g.player_x, g.player_y + 1) {
                g.move_player(0, 1);
                let new_dist = (g.enemies[0].x - g.player_x).abs() + (g.enemies[0].y - g.player_y).abs();
                assert!(new_dist < 4, "enemy should have chased toward player, dist={new_dist}");
            }
        }
    }

    // === Dungeon traversal (Phase 3) ===

    fn overworld_game() -> Game {
        let mut map = Map::generate_forest(200, 200, 42);
        let entrances = map.place_dungeons(42);
        map.build_roads(&entrances);
        let world = World::new(map, entrances, 99);
        let mut g = Game::new_overworld(world);
        g.spawn_enemies(777);
        g
    }

    #[test]
    fn enter_dungeon_changes_location() {
        let mut g = overworld_game();
        let entrance = g.world.dungeon_entrances[0];
        // Teleport player to dungeon entrance
        g.player_x = entrance.0;
        g.player_y = entrance.1;
        g.enter_dungeon(0);
        assert_eq!(g.world.location, Location::Dungeon { index: 0, level: 0 });
    }

    #[test]
    fn enter_dungeon_places_player_at_stairs_up() {
        let mut g = overworld_game();
        g.enter_dungeon(0);
        let map = g.current_map();
        assert_eq!(map.get(g.player_x, g.player_y), Tile::StairsUp);
    }

    #[test]
    fn enter_dungeon_saves_overworld_pos() {
        let mut g = overworld_game();
        let (ox, oy) = (g.player_x, g.player_y);
        g.enter_dungeon(0);
        assert_eq!(g.world.saved_overworld_pos, (ox, oy));
    }

    #[test]
    fn exit_dungeon_restores_overworld() {
        let mut g = overworld_game();
        let (ox, oy) = (g.player_x, g.player_y);
        let enemy_count_before = g.enemies.len();
        g.enter_dungeon(0);
        // Now we're in dungeon
        assert_ne!(g.enemies.len(), enemy_count_before);
        g.exit_dungeon();
        assert_eq!(g.world.location, Location::Overworld);
        assert_eq!((g.player_x, g.player_y), (ox, oy));
        assert_eq!(g.enemies.len(), enemy_count_before);
    }

    #[test]
    fn descend_changes_level() {
        let mut g = overworld_game();
        g.enter_dungeon(0);
        assert_eq!(g.world.location, Location::Dungeon { index: 0, level: 0 });
        g.descend(0, 0);
        assert_eq!(g.world.location, Location::Dungeon { index: 0, level: 1 });
    }

    #[test]
    fn ascend_changes_level() {
        let mut g = overworld_game();
        g.enter_dungeon(0);
        g.descend(0, 0);
        assert_eq!(g.world.location, Location::Dungeon { index: 0, level: 1 });
        g.ascend(0, 1);
        assert_eq!(g.world.location, Location::Dungeon { index: 0, level: 0 });
    }

    #[test]
    fn round_trip_dungeon_preserves_overworld_position() {
        let mut g = overworld_game();
        let (ox, oy) = (g.player_x, g.player_y);
        // Enter dungeon
        g.enter_dungeon(0);
        // Descend to level 2
        g.descend(0, 0);
        g.descend(0, 1);
        assert_eq!(g.world.location, Location::Dungeon { index: 0, level: 2 });
        // Ascend back
        g.ascend(0, 2);
        g.ascend(0, 1);
        assert_eq!(g.world.location, Location::Dungeon { index: 0, level: 0 });
        // Exit to overworld
        g.exit_dungeon();
        assert_eq!(g.world.location, Location::Overworld);
        assert_eq!((g.player_x, g.player_y), (ox, oy));
    }

    #[test]
    fn stairs_connect_correct_levels() {
        let mut g = overworld_game();
        g.enter_dungeon(0);
        // On level 0, player should be at StairsUp
        assert_eq!(g.current_map().get(g.player_x, g.player_y), Tile::StairsUp);
        // Descend
        g.descend(0, 0);
        // On level 1, player should be at StairsUp
        assert_eq!(g.current_map().get(g.player_x, g.player_y), Tile::StairsUp);
    }

    #[test]
    fn dungeon_enemies_spawn_on_walkable() {
        let mut g = overworld_game();
        g.enter_dungeon(0);
        for e in &g.enemies {
            assert!(g.current_map().is_walkable(e.x, e.y),
                "{} at ({},{}) not walkable", e.name, e.x, e.y);
        }
    }

    #[test]
    fn dungeon_has_classic_enemies() {
        let mut g = overworld_game();
        g.enter_dungeon(0);
        // Level 0 should have goblins and/or skeletons, not forest animals
        for e in &g.enemies {
            assert!(
                e.glyph == 'g' || e.glyph == 's' || e.glyph == 'o' || e.glyph == 'T' || e.glyph == 'D',
                "unexpected dungeon enemy: {} ({})", e.name, e.glyph
            );
        }
    }

    #[test]
    fn no_dragon_on_shallow_levels() {
        let mut g = overworld_game();
        // Check level 0 of the first dungeon — no dragon
        g.enter_dungeon(0);
        assert!(
            !g.enemies.iter().any(|e| e.glyph == 'D'),
            "level 0 should not have a dragon"
        );
        // Check level 1 — no dragon
        g.descend(0, 0);
        assert!(
            !g.enemies.iter().any(|e| e.glyph == 'D'),
            "level 1 should not have a dragon"
        );
    }

    #[test]
    fn deeper_dungeon_enemies_are_stronger() {
        let mut g = overworld_game();
        g.enter_dungeon(0);
        let l0_max_hp = g.enemies.iter().filter(|e| e.glyph != 'D').map(|e| e.hp).max().unwrap_or(0);

        g.descend(0, 0);
        g.descend(0, 1);
        let l2_max_hp = g.enemies.iter().filter(|e| e.glyph != 'D').map(|e| e.hp).max().unwrap_or(0);

        assert!(l2_max_hp > l0_max_hp,
            "deeper enemies should be stronger: l0_max={l0_max_hp}, l2_max={l2_max_hp}");
    }

    #[test]
    fn transition_message_on_enter_dungeon() {
        let mut g = overworld_game();
        g.enter_dungeon(0);
        assert!(g.messages.iter().any(|m| m.contains("descend")));
    }

    #[test]
    fn transition_message_on_exit_dungeon() {
        let mut g = overworld_game();
        g.enter_dungeon(0);
        g.exit_dungeon();
        assert!(g.messages.iter().any(|m| m.contains("overworld")));
    }
}
