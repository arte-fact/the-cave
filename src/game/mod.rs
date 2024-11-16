pub mod action;
pub mod biome;
pub mod enemy;
pub mod features;
pub mod item;
pub mod player;
use rand::Rng;

use crate::assets::colors::Color;
use crate::assets::tiles::ItemTiles;
use crate::map::Map;
use crate::tile::Tile;

use self::action::Action;
use self::enemy::{Enemy, EnemyState};
use self::{
    features::{direction::Direction, position::Position},
    item::Item,
    player::Player,
};

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
        println!("Generating new game...");
        let (map, player_pos, enemies, items) = Map::generate(256, 1024);
        println!("Enemies generated!");
        println!("Items generated!");
        println!("Game generated!");

        Game {
            map: map.clone(),
            player: Player {
                character: Box::new('🧍'),
                health: 30,
                attack: 5,
                defense: 0,
                position: player_pos,
                direction: Direction::Down,
            },
            enemies,
            items,
            view_width: 30,
            view_height: 20,
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

        if !self.map.walkable.contains(&next_position) {
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
                        self.map.change_tile(
                            enemy.position.x,
                            enemy.position.y,
                            ItemTiles::Crown.to_tile(),
                            true,
                        );
                    }
                    if ["Rat", "Bat", "Snake"].contains(&enemy.name.as_str()) {
                        self.items.push(Item {
                            name: "Piece of Meat".to_string(),
                            tile: ItemTiles::Meat.to_tile(),
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
                            tile: ItemTiles::Steak.to_tile(),
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
                            tile: ItemTiles::Steak.to_tile(),
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

            let movement = 1;
            let mut next_position = enemy.position.clone();
            if distance < 5.0 {
                if enemy.state == EnemyState::Idle {
                    enemy.state = EnemyState::Attacking;
                    self.events
                        .push(format!("A {} has spotted you!", enemy.name));
                }
                if self.player.position.x > enemy.position.x {
                    enemy.tile.mirror = true;
                }
                if self.player.position.x < enemy.position.x {
                    enemy.tile.mirror = false;
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

            if !self.map.walkable.contains(&next_position) {
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
        output.push_str("</div>");
        output
    }

    pub fn preview_map(&self) -> String {
        let mut output = String::new();

        let this = &self;
        let view_y = 0;
        let view_height = self.map.width;
        let view_x = 0;
        let view_width = self.map.height;
        let output: &mut String = &mut output;
        for y in view_y..view_y + view_width {
            let mut row = "".to_string();
            for x in view_x..view_x + view_height {
                let mut enemy_present = false;
                for enemy in &this.enemies {
                    if enemy.position.x == x && enemy.position.y == y {
                        row.push_str(enemy.tile.char.to_string().as_str());
                        enemy_present = true;
                        break;
                    }
                }
                if enemy_present {
                    continue;
                }

                let mut item_present = false;
                for item in &this.items {
                    if item.position.x == x && item.position.y == y {
                        row.push_str(item.tile.char.to_string().as_str());
                        item_present = true;
                        break;
                    }
                }
                if item_present {
                    continue;
                }

                let tile = self.map.tiles[y as usize][x as usize].clone();
                row.push_str(tile.char.to_string().as_str());
            }

            output.push_str(
                format!(
                    "<div class='row'>{}</div>",
                    row.chars().into_iter().map(|c| 
                        format!("<span class='tile'>{}</span>", c)
                    ).collect::<String>()
                )
                .as_str(),
            );
        }

        output.clone()
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
                let map_tile = self.map.tiles[y as usize][x as usize].clone();
                if player_x == x && player_y == y {
                    if self.is_game_over {
                        row.push_str(ItemTiles::Skull.to_tile().to_html().as_str());
                        continue;
                    }
                    row.push_str(
                        Tile {
                            char: '🚶',
                            size: 0.9,
                            mirror: self.player.direction != Direction::Left,
                            opacity: 1.0,
                            hue: 0.0,
                            altitude: 0.25,
                            color: map_tile.color.clone(),
                        }
                        .to_html()
                        .as_str(),
                    );
                    continue;
                }

                let player_distance =
                    (((player_x - x) * (player_x - x) + (player_y - y) * (player_y - y)) as f64)
                        .sqrt();
                let opacity = ((15.0 / player_distance) as f32) - 1.0;

                let mut enemy_present = false;
                for enemy in &self.enemies {
                    if enemy.position.x == x && enemy.position.y == y {
                        let mut tile = enemy.tile.clone();
                        tile.opacity = opacity;
                        tile.color = map_tile.color.clone();
                        row.push_str(tile.to_html().as_str());
                        enemy_present = true;
                        continue;
                    }
                }
                if enemy_present {
                    continue;
                }

                let mut item_present = false;
                for item in &self.items {
                    if item.position.x == x && item.position.y == y {
                        let mut tile = item.tile.clone();
                        tile.opacity = opacity;
                        tile.color = map_tile.color.clone();
                        item_present = true;
                        row.push_str(tile.to_html().as_str());
                        continue;
                    }
                }
                if item_present {
                    continue;
                }

                let mut tile = self.map.tiles[y as usize][x as usize].clone();
                tile.opacity = opacity;
                tile.color = map_tile.color.clone();
                row.push_str(tile.to_html().as_str());
            }
            let screen_y = y - view_y;

            if screen_y < evts.len() as i32 {
                events_elem.push_str(
                    format!("<div class='event'>{}<div>", evts[screen_y as usize]).as_str(),
                );
            }
            events_elem.push_str("</div>");

            output.push_str(format!("<div class='row'>{}</div>", row).as_str());
        }
        output.push_str("</div>");
        output.push_str(events_elem.as_str());
    }
}
