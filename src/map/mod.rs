use rand::seq::IteratorRandom;
use rand::Rng;

use crate::assets::biomes::Biomes;
use crate::game::biome::Biome;
use crate::game::enemy::Enemy;
use crate::game::features::position::Position;
use crate::game::item::Item;
use crate::tile::{MapTiles, Tile};

#[derive(Debug, Clone, PartialEq)]
pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
    pub walkable: Vec<Position>,
    pub width: i32,
    pub height: i32,
}

#[derive(PartialEq)]
enum TileState {
    Used,
    Unused,
}

struct Blueprint {
    width: usize,
    height: usize,
    tiles: Vec<TileState>,
}

impl Blueprint {
    pub fn new(width: usize, height: usize) -> Blueprint {
        let mut tiles = vec![];
        for _ in 0..width * height {
            tiles.push(TileState::Unused);
        }
        Blueprint {
            width,
            height,
            tiles,
        }
    }

    pub fn get_tile(&self, x: usize, y: usize) -> &TileState {
        &self.tiles[y * self.width + x]
    }

    pub fn set_tile(&mut self, x: usize, y: usize, tile: TileState) {
        self.tiles[y * self.width + x] = tile;
    }

    pub fn random_position(&self) -> Position {
        let mut rng = rand::thread_rng();
        Position {
            x: rng.gen_range(0..self.width as i32),
            y: rng.gen_range(0..self.height as i32),
        }
    }

    pub fn generate(height: usize, width: usize) -> Vec<Position> {
        let mut nodes = vec![];
        let dist = 30;

        let start = Position { x: 128, y: 996 };
        let mut current = start.clone();
        nodes.push(start.clone());
        let mut vectors: Vec<(Position, Position)> = vec![];
        let mut positions = vec![start.clone()];
        let mut rng = rand::thread_rng();

        let mut total_tries = 0;
        let mut tries = 0;
        while vectors.len() < 512 {
            total_tries += 1;
            if total_tries > 1000000 {
                println!("max total tries");
                break;
            }
            let x_dir = if rng.gen_bool(0.5) { 1 } else { -1 };
            let y_dir = if rng.gen_bool(0.5) { 1 } else { -1 };
            let move_x = rng.gen_range(dist / 2..dist);
            let move_y = ((dist * dist - move_x * move_x) as f32).sqrt() as i32;

            let mut next = current.clone();
            next.add_position(&Position {
                x: x_dir * move_x,
                y: y_dir * move_y,
            });

            let is_far_enough = vectors
                .iter()
                .all(|(_, p)| p.distance_to(&next) > (dist / 2) as f32);

            let intersect_any_vector = vectors.iter().any(|(from, to)| {
                let pad = 10;
                let from_x = from.x.min(to.x);
                let from_y = from.y.min(to.y);
                let to_x = from.x.max(to.x);
                let to_y = from.y.max(to.y);
                let x = next.x;
                let y = next.y;
                x >= from_x - pad && x <= to_x + pad && y >= from_y - pad && y <= to_y + pad
            });

            if next.x < 0
                || next.y < 0
                || next.x >= width as i32
                || next.y >= height as i32
                || !is_far_enough
                || intersect_any_vector
            {
                tries += 1;
                if tries > 15 {
                    current = nodes.pop().unwrap_or(start.clone());
                    tries = 0;
                }
                continue;
            }
            nodes.push(next.clone());
            vectors.push((current.clone(), next.clone()));
            current = next.clone();
        }

        for (pos_from, pos_to) in vectors {
            let min_x = pos_from.x.min(pos_to.x);
            let max_x = pos_from.x.max(pos_to.x);
            let min_y = pos_from.y.min(pos_to.y);
            let max_y = pos_from.y.max(pos_to.y);
            for x in min_x..max_x {
                for y in min_y..max_y {
                    positions.push(Position { x, y });
                }
            }
            for pos in pos_from.get_circle(5) {
                if pos.x < 0
                    || pos.y < 0
                    || pos.x >= width as i32
                    || pos.y >= height as i32
                    || min_x <= pos.x && pos.x < max_x && min_y <= pos.y && pos.y < max_y
                {
                    continue;
                }
                positions.push(pos.clone());
            }
        }

        positions
    }
}

impl Map {
    pub fn random_valid_position(&self, ban: &Vec<Position>) -> Position {
        self.walkable
            .iter()
            .filter(|p| !ban.contains(p))
            .choose(&mut rand::thread_rng())
            .unwrap()
            .clone()
    }

    pub fn generate(width: i32, height: i32) -> (Map, Position, Vec<Enemy>, Vec<Item>) {
        let mut tiles = vec![];
        let mut walkable = vec![];
        let mut ennemies = vec![];
        let mut items = vec![];
        let player_position = Position { x: 128, y: 996 };

        for y in 0..height {
            let mut row = vec![];
            let biome = get_biome_at_height(y);
            for _ in 0..width {
                let mut tile = biome.get_filler_tile().to_tile();
                tile.color = biome.get_background();
                row.push(tile);
            }
            tiles.push(row);
        }

        for p in Blueprint::generate(height as usize, width as usize) {
            let biome = get_biome_at_height(p.y);
            let element = Biome::get_random_element(biome.get_tiles());
            match element.clone() {
                Biome::Item(item, _) => {
                    let mut item = item.clone();
                    item.position = p.clone();
                    items.push(item);
                },
                Biome::Enemy(enemy, _) => {
                    let mut enemy = enemy.clone();
                    enemy.position = p.clone();
                    ennemies.push(enemy);
                },
                Biome::Block(_, _) => {
                    let tile = element.get_tile();
                    tiles[p.y as usize][p.x as usize] = tile;
                },
            }
            let mut tile = MapTiles::Floor.to_tile();
            tile.color = biome.get_background();
            tiles[p.y as usize][p.x as usize] = tile;
            walkable.push(p.clone());
        }
        let map = Map {
            tiles,
            walkable,
            width,
            height,
        };

        (
            map,
            player_position,
            ennemies,
            items,
        )
    }

    pub fn change_tile(&mut self, x: i32, y: i32, tile: Tile, walkable: bool) {
        self.tiles[y as usize][x as usize] = tile;
        let was_walkable = self.walkable.contains(&Position { x, y });
        if walkable && !was_walkable {
            self.walkable.push(Position { x, y });
            return;
        }
        if !walkable && was_walkable {
            self.walkable.retain(|p| p != &Position { x, y });
        }
    }
}

fn get_biome_at_height(y: i32) -> Biomes {
    let biome = if y < 96 {
        Biomes::Cave
    } else if y < 400 {
        Biomes::Forest
    } else if y < 700 {
        Biomes::Desert
    } else if y < 980 {
        Biomes::Jungle
    } else {
        Biomes::Shore
    };
    biome
}
