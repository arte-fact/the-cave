use rand::Rng;

use crate::map::{Map, Tile, TileType};

#[derive(Clone, Debug, PartialEq)]
pub struct Player {
    pub character: Box<char>,
    pub health: i32,
    pub attack: i32,
    pub defense: i32,
    pub position: Position,
    pub direction: Direction,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Confirm,
    Unhandled,
}

impl Action {
    pub fn from_key(key: &str) -> Option<Action> {
        let up = Some(Action::Up);
        let down = Some(Action::Down);
        let left = Some(Action::Left);
        let right = Some(Action::Right);
        match key.trim() {
            "k" => up,
            "ArrowUp" => up,
            "z" => up,
            "ArrowDown" => down,
            "s" => down,
            "j" => down,
            "ArrowLeft" => left,
            "q" => left,
            "h" => left,
            "ArrowRight" => right,
            "d" => right,
            "l" => right,
            "Enter" => Some(Action::Confirm),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Enemy {
    pub name: String,
    pub char: Box<char>,
    pub health: i32,
    pub attack: i32,
    pub defense: i32,
    pub occurences: i32,
    pub position: Position,
    pub behavior: Behavior,
    pub state: EnemyState,
}

#[derive(Clone, Debug, PartialEq)]
pub enum EnemyState {
    Idle,
    Attacking,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Behavior {
    Aggressive,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    name: String,
    char: Box<char>,
    occurences: i32,
    health: i32,
    attack: i32,
    defense: i32,
    position: Position,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    pub player: Player,
    pub enemies: Vec<Enemy>,
    pub items: Vec<Item>,
    pub view_width: i32,
    pub view_height: i32,
    pub events: Vec<String>,
    pub map: Map,
    pub is_game_over: bool,
    pub is_game_won: bool,
}

impl Game {
    pub fn new() -> Game {
        let map = Map::generate(400, 400);
        let player_position = map.random_valid_position(&vec![]);

        let ennemies = [
            Enemy {
                name: "Rat".to_string(),
                char: Box::new('🐀'),
                health: 20,
                attack: 2,
                occurences: 10,
                defense: 0,
                position: Position { x: 0, y: 0 },
                behavior: Behavior::Aggressive,
                state: EnemyState::Idle,
            },
            Enemy {
                name: "Bat".to_string(),
                char: Box::new('🦇'),
                health: 30,
                attack: 5,
                occurences: 10,
                defense: 0,
                position: Position { x: 0, y: 0 },
                behavior: Behavior::Aggressive,
                state: EnemyState::Idle,
            },
            Enemy {
                name: "Snake".to_string(),
                char: Box::new('🐍'),
                health: 30,
                attack: 5,
                occurences: 20,
                defense: 0,
                position: Position { x: 0, y: 0 },
                behavior: Behavior::Aggressive,
                state: EnemyState::Idle,
            },
            Enemy {
                name: "Aligator".to_string(),
                char: Box::new('🐊'),
                health: 75,
                attack: 15,
                occurences: 5,
                defense: 5,
                position: Position { x: 0, y: 0 },
                behavior: Behavior::Aggressive,
                state: EnemyState::Idle,
            },
            Enemy {
                name: "T-rex".to_string(),
                char: Box::new('🦖'),
                health: 100,
                attack: 20,
                occurences: 5,
                defense: 0,
                position: Position { x: 0, y: 0 },
                behavior: Behavior::Aggressive,
                state: EnemyState::Idle,
            },
            Enemy {
                name: "Dragon".to_string(),
                char: Box::new('🐉'),
                health: 200,
                occurences: 1,
                attack: 40,
                defense: 10,
                position: Position { x: 0, y: 0 },
                behavior: Behavior::Aggressive,
                state: EnemyState::Idle,
            },
        ];

        let mut banned_positions: Vec<Position> = vec![player_position];

        let mut enemies: Vec<Enemy> = vec![];
        for enemy in &ennemies {
            (0..enemy.occurences).for_each(|_| {
                let position = map.random_valid_position(&banned_positions);
                banned_positions.push(position.clone());
                let mut e = enemy.clone();
                e.position = position.clone();
                enemies.push(e);
            });
        }

        let _items = [
            Item {
                name: "Health potion".to_string(),
                char: Box::new('🧪'),
                health: 20,
                occurences: 10,
                attack: 0,
                defense: 0,
                position: Position { x: 0, y: 0 },
            },
            Item {
                name: "Sword Upgrade".to_string(),
                char: Box::new('🗡'),
                health: 0,
                occurences: 5,
                attack: 5,
                defense: 0,
                position: Position { x: 0, y: 0 },
            },
            Item {
                name: "Shield Upgrade".to_string(),
                char: Box::new('🛡'),
                health: 0,
                occurences: 5,
                attack: 0,
                defense: 5,
                position: Position { x: 0, y: 0 },
            },
        ];

        let mut items: Vec<Item> = vec![];
        for item in &_items {
            (0..item.occurences).for_each(|_| {
                let position = map.random_valid_position(&banned_positions);
                banned_positions.push(position.clone());
                let mut i = item.clone();
                i.position = position.clone();
                items.push(i);
            });
        }

        Game {
            map: map.clone(),
            player: Player {
                character: Box::new('🧍'),
                health: 30,
                attack: 5,
                defense: 0,
                position: map.random_valid_position(&banned_positions),
                direction: Direction::Down,
            },
            enemies,
            items,
            view_width: 40,
            view_height: 30,
            events: vec!["Find and kill the dragon!".to_string()],
            is_game_over: false,
            is_game_won: false,
        }
    }

    pub fn reset(&mut self) {
        let new_game = Game::new();
        self.map = new_game.map;
        self.player = new_game.player;
        self.enemies = new_game.enemies;
        self.events = vec!["A new game has started!".to_string()];
        self.is_game_over = new_game.is_game_over;
        self.is_game_won = new_game.is_game_won;
        self.items = new_game.items;
    }

    pub fn handle_key(&mut self, key: Action) {
        if self.is_game_over || self.is_game_won {
            if key == Action::Confirm {
                self.reset();
            }
            return;
        }
        self.tick();
        if self.is_game_over || self.is_game_won {
            return;
        }

        let mut next_position = self.player.position.clone();
        match key {
            Action::Up => next_position.y = (next_position.y - 1).max(0),
            Action::Down => next_position.y = (next_position.y + 1).min(self.map.height - 1),
            Action::Left => next_position.x = (next_position.x - 1).max(0),
            Action::Right => next_position.x = (next_position.x + 1).min(self.map.width - 1),
            _ => (),
        }

        if !self.map.tiles[next_position.y as usize][next_position.x as usize]
            .tile_type
            .is_walkable()
        {
            return;
        }

        for item in &mut self.items {
            if item.position.x == next_position.x && item.position.y == next_position.y {
                self.player.health = (self.player.health + item.health).min(100);
                self.player.attack = (self.player.attack + item.attack).min(60);
                self.player.defense = (self.player.defense + item.defense).min(20);
                self.events.push(format!("You found a {}!", item.name));
                self.items.retain(|i| i.position != next_position);
                return;
            }
        }

        for enemy in &mut self.enemies {
            if enemy.position.x == next_position.x && enemy.position.y == next_position.y {
                let damage = (self.player.attack - enemy.defense).max(1);
                enemy.health -= damage;
                self.events
                    .push(format!("You hit {} for {} damage!", enemy.name, damage));
                if enemy.health <= 0 {
                    self.events
                        .push(format!("You killed {}!", enemy.name).to_string());
                    if enemy.name == "Dragon" {
                        self.is_game_won = true;
                        self.events
                            .push("You won! Press Enter to start a new game.".to_string());
                        self.map.tiles[enemy.position.y as usize][enemy.position.x as usize] =
                            Tile {
                                tile_type: TileType::Crown,
                                size: 1.0,
                            };
                    }
                    if ["Rat", "Bat", "Snake"].contains(&enemy.name.as_str()) {
                        self.items.push(Item {
                            name: "Piece of Meat".to_string(),
                            char: Box::new('🍖'),
                            health: 5,
                            occurences: 0,
                            attack: 0,
                            defense: 0,
                            position: enemy.position.clone(),
                        });
                    }
                    if enemy.name == "Aligator" {
                        self.items.push(Item {
                            name: "Piece Of Aligator Meat".to_string(),
                            char: Box::new('🍖'),
                            health: 10,
                            occurences: 0,
                            attack: 5,
                            defense: 0,
                            position: enemy.position.clone(),
                        });
                    }
                    if enemy.name == "T-rex" {
                        self.items.push(Item {
                            name: "Piece Of T-rex Meat".to_string(),
                            char: Box::new('🍖'),
                            health: 10,
                            occurences: 0,
                            attack: 5,
                            defense: 5,
                            position: enemy.position.clone(),
                        });
                    }
                    self.enemies.retain(|e| e.health > 0);
                }
                return;
            }
        }

        self.events.push(format!(
            "You moved {}",
            match key {
                Action::Up => "up",
                Action::Down => "down",
                Action::Left => "left",
                Action::Right => "right",
                _ => "unknown",
            }
        ));

        self.player.position = next_position;
        self.player.direction = match key {
            Action::Up => Direction::Up,
            Action::Down => Direction::Down,
            Action::Left => Direction::Left,
            Action::Right => Direction::Right,
            _ => self.player.direction.clone(),
        };
    }

    pub fn tick(&mut self) -> &mut Self {
        let enemies = self.enemies.clone();
        for enemy in &mut self.enemies {
            let dx = self.player.position.x - enemy.position.x;
            let dy = self.player.position.y - enemy.position.y;
            let distance = ((dx * dx + dy * dy) as f64).sqrt();

            let movement = match enemy.behavior {
                Behavior::Aggressive => 1,
            };

            let mut next_position = enemy.position.clone();
            if distance < 5.0 {
                if enemy.state == EnemyState::Idle {
                    enemy.state = EnemyState::Attacking;
                    self.events
                        .push(format!("A {} has spotted you!", enemy.name));
                }
                if dx.abs() > dy.abs() {
                    if dx > 0 {
                        next_position.x += movement;
                    } else {
                        next_position.x -= movement;
                    }
                } else {
                    if dy > 0 {
                        next_position.y += movement;
                    } else {
                        next_position.y -= movement;
                    }
                }
            } else if enemy.state == EnemyState::Attacking {
                enemy.state = EnemyState::Idle;
                self.events
                    .push(format!("{} has lost track of you!", enemy.name));
            }

            for other_enemy in &enemies {
                if other_enemy.position.x == next_position.x
                    && other_enemy.position.y == next_position.y
                {
                    continue;
                }
            }

            if !self.map.tiles[next_position.y as usize][next_position.x as usize]
                .tile_type
                .is_walkable()
            {
                continue;
            }

            if next_position.x == self.player.position.x
                && next_position.y == self.player.position.y
            {
                let damage = (enemy.attack - self.player.defense).max(1);
                self.player.health -= damage;
                if self.player.health <= 0 {
                    self.player.health = 0;
                    self.is_game_over = true;
                    self.events
                        .push("You died! Press Enter to start a new game.".to_string());

                    self.map.tiles[self.player.position.y as usize]
                        [self.player.position.x as usize] = Tile {
                        tile_type: TileType::Skull,
                        size: 1.2,
                    };
                    return self;
                }
                if enemy.name == "T-rex" {
                    let mut rng = rand::thread_rng();
                    let armor_damage = rng.gen_range(0..2);
                    self.player.defense = (self.player.defense - armor_damage).max(0);
                    self.events.push(
                        format!("T-rex damaged your armor you lost {armor_damage} defense!")
                            .to_string(),
                    );
                }

                self.events
                    .push(format!("{} hit you for {} damage!", enemy.name, damage));

                return self;
            }

            enemy.position = next_position;
        }
        self
    }

    pub fn event_list(&self) -> Vec<String> {
        let mut events = vec![];
        let mut n = 0;
        let mut last_event: Option<String> = None;

        for event in self.events.iter().rev() {
            if events.len() >= self.view_height as usize {
                break;
            }
            if let Some(last) = last_event.clone() {
                if last == event.clone() {
                    n += 1;
                    continue;
                } else {
                    if n > 0 {
                        events.push(format!("{} (x{})", last, n + 1));
                    } else {
                        events.push(last);
                    }
                }
            }
            n = 0;
            last_event = Some(event.clone());
        }
        events.reverse();
        events
    }

    pub fn draw(&self) -> String {
        let mut output = String::new();
        output.push_str(&format!(
            "<div class='stats'> ❤️  {} 🗡️ {} 🛡️ {} </div>",
            self.player.health, self.player.attack, self.player.defense
        ));
        let evts = self.event_list();

        let view_height = self.view_height;
        let view_width = self.view_width;
        let player_x = self.player.position.x;
        let player_y = self.player.position.y;
        let map_height = self.map.height;
        let map_width = self.map.width;

        let view_x = (player_x - view_width / 2).clamp(0, map_width - view_width);
        let view_y = (player_y - view_height / 2).clamp(0, map_height - view_height);
        self.draw_map(
            view_y,
            view_height,
            view_x,
            view_width,
            player_x,
            player_y,
            evts,
            &mut output,
        );

        output
    }

    pub fn preview_map(&self) -> String {
        let mut output = String::new();

        self.draw_map(
            0,
            self.map.width,
            0,
            self.map.height,
            0,
            0,
            vec![],
            &mut output,
        );

        output
    }

    pub fn draw_map(
        &self,
        view_y: i32,
        view_height: i32,
        view_x: i32,
        view_width: i32,
        player_x: i32,
        player_y: i32,
        evts: Vec<String>,
        output: &mut String,
    ) {
        output.push_str("<div class='game'><div class='map'>");
        let mut events_elem = "<div class='events'>".to_string();
        for y in view_y..view_y + view_height {
            let mut row = "".to_string();
            for x in view_x..view_x + view_width {
                if player_x == x && player_y == y {
                    row.push_str(match self.player.direction {
                        Direction::Up => "🧍",
                        Direction::Down => "🧍",
                        Direction::Left => "🚶",
                        Direction::Right => "🚶",
                    });
                    continue;
                }

                let mut enemy_present = false;
                for enemy in &self.enemies {
                    if enemy.position.x == x && enemy.position.y == y {
                        row.push_str(enemy.char.to_string().as_str());
                        enemy_present = true;
                        break;
                    }
                }
                if enemy_present {
                    continue;
                }

                let mut item_present = false;
                for item in &self.items {
                    if item.position.x == x && item.position.y == y {
                        row.push_str(item.char.to_string().as_str());
                        item_present = true;
                        break;
                    }
                }
                if item_present {
                    continue;
                }

                let tile = &self.map.tiles[y as usize][x as usize];
                row.push_str(tile.tile_type.character().to_string().as_str());
            }
            let screen_y = y - view_y;


            let row = row.chars().map(|c| format!("<span class='tile'>{}</span>", c)).collect::<Vec<String>>().join("");

            if screen_y < evts.len() as i32 {
                events_elem.push_str(format!("<div class='event'>{}<div>", evts[screen_y as usize]).as_str());
            }
            events_elem.push_str("</div>");

            output.push_str(format!("<div>{}</div>", row).as_str());
        }
        output.push_str("</div>");
        output.push_str(events_elem.as_str());
    }
}
