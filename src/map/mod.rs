use rand::seq::{IteratorRandom, SliceRandom};
use rand::Rng;

use crate::game::Position;

#[derive(Debug, Clone)]
pub struct Tile {
    pub tile_type: TileType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TileType {
    RockWall,
    Floor,
    Skull,
    Crown,
}

impl TileType {
    pub fn character(&self) -> &str {
        match self {
            TileType::RockWall => "🪨",
            TileType::Floor => ".",
            TileType::Crown => "👑",
            TileType::Skull => "💀",
        }
    }

    pub fn is_walkable(&self) -> bool {
        match self {
            TileType::Floor => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
    pub width: i32,
    pub height: i32,
}

pub struct TileSeedZone {
    _name: String,
    tile_type: Vec<(TileType, u32)>,
    _occurences: i32,
    size: i32,
    size_variance: i32,
    min_distance: i32,
    max_distance: i32,
}

impl TileSeedZone {
    pub fn get_tile(&self) -> TileType {
        let mut rng = rand::thread_rng();
        let mut total = 0;
        for (_, chance) in &self.tile_type {
            total += chance;
        }
        let mut roll = rng.gen_range(0..total);
        for (tile, chance) in &self.tile_type {
            if roll < *chance {
                return tile.clone();
            }
            roll -= chance;
        }
        self.tile_type[0].0.clone()
    }
}

impl Default for TileSeedZone {
    fn default() -> Self {
        TileSeedZone {
            _name: "Cave floors".to_string(),
            tile_type: vec![(TileType::Floor, 100), (TileType::RockWall, 1)],
            _occurences: 4,
            size: 10,
            size_variance: 20,
            min_distance: 5,
            max_distance: 30,
        }
    }
}

pub fn distance(x1: &i32, y1: &i32, x2: &i32, y2: &i32) -> f32 {
    let x = (x1 - x2).abs() as f32;
    let y = (y1 - y2).abs() as f32;
    (x * x + y * y).sqrt()
}

impl Map {
    pub fn random_valid_position(&self, ban: &Vec<Position>) -> Position {
        let mut rng = rand::thread_rng();
        let walkable = self
            .tiles
            .iter()
            .enumerate()
            .map(|(y, row)| {
                let ban = ban.clone();
                row.iter().enumerate().filter_map(move |(x, tile)| {
                    if tile.tile_type.is_walkable()
                        && !ban.contains(&&Position {
                            x: x as i32,
                            y: y as i32,
                        })
                    {
                        Some(Position {
                            x: x as i32,
                            y: y as i32,
                        })
                    } else {
                        None
                    }
                })
            })
            .flatten()
            .collect::<Vec<Position>>();
        walkable.iter().choose(&mut rng).unwrap().clone()
    }

    pub fn generate(width: i32, height: i32) -> Map {
        let seeds = vec![TileSeedZone::default()];
        let mut tiles = vec![];
        for _ in 0..height {
            let mut row = vec![];
            for _ in 0..width {
                row.push(Tile {
                    tile_type: TileType::RockWall,
                });
            }
            tiles.push(row);
        }
        let mut map = Map {
            tiles,
            width,
            height,
        };
        map.generate_tiles(seeds);

        map
    }

    fn generate_tiles(&mut self, seeds: Vec<TileSeedZone>) {
        for seed in seeds {
            self.generate_seed(&seed);
        }
    }

    fn generate_seed(&mut self, seed: &TileSeedZone) {
        let mut rng = rand::thread_rng();

        let mut size =
            rng.gen_range(seed.size - seed.size_variance..seed.size + seed.size_variance);
        let mut x = rng.gen_range(size..self.width - size);
        let mut y = rng.gen_range(size..self.height - size);
        let mut seeds_origins = vec![(x, y)];

        let mut _has_next = true;
        let mut max_tries = 50;
        while _has_next && max_tries > 0 {
            size = rng.gen_range(seed.size - seed.size_variance..seed.size + seed.size_variance);

            let valid_next_x = [
                (x + seed.min_distance..x + seed.max_distance)
                    .filter(|x| *x >= size && *x < self.width - size)
                    .collect::<Vec<i32>>(),
                (x - seed.max_distance..x - seed.min_distance)
                    .filter(|x| *x >= size && *x < self.width - size)
                    .collect::<Vec<i32>>(),
            ]
            .concat();

            let mut valid_positions = vec![];
            for valid_x in valid_next_x {
                let valid_next_y = [
                    (valid_x + seed.min_distance..valid_x + seed.max_distance)
                        .filter(|y| *y >= size && *y < self.height - size)
                        .collect::<Vec<i32>>(),
                    (y - seed.max_distance..y - seed.min_distance)
                        .filter(|y| *y >= size && *y < self.height - size)
                        .collect::<Vec<i32>>(),
                ]
                .concat();
                for valid_y in valid_next_y {
                    let mut valid = true;
                    for (seed_x, seed_y) in &seeds_origins {
                        if distance(&seed_x, &seed_y, &valid_x, &valid_y) < seed.min_distance as f32
                        {
                            valid = false;
                            break;
                        }
                    }
                    if valid {
                        valid_positions.push((valid_x, valid_y));
                    }
                }
            }

            let (next_x, next_y) = match valid_positions.choose(&mut rng) {
                Some((x, y)) => (x.clone(), y.clone()),
                None => {
                    max_tries -= 1;
                    continue;
                }
            };

            max_tries -= 1;

            self.connect_zones(&x, &y, &next_x, &next_y);
            x = next_x.clone().clone();
            y = next_y.clone().clone();
            self.generate_zone(x, y, size, seed);
            seeds_origins.push((x, y));
        }
    }

    fn connect_zones(&mut self, x1: &i32, y1: &i32, x2: &i32, y2: &i32) {
        let mut x = x1.clone();
        let mut y = y1.clone();
        while x != x2.clone() {
            let _x = (x2 - x1).signum();

            for i in -1..1 {
                let y = (y + i as i32).clamp(0, self.height - 1);
                let x = (x + _x).clamp(0, self.width - 1);
                self.tiles[y as usize][x as usize] = Tile {
                    tile_type: TileType::Floor,
                };
            }

            x += _x;
        }
        while y != y2.clone() {
            let _y = (y2 - y1).signum();

            for i in -1..1 {
                let x = (x + i as i32).clamp(0, self.width - 1);
                let y = (y + _y).clamp(0, self.height - 1);
                self.tiles[y as usize][x as usize] = Tile {
                    tile_type: TileType::Floor,
                };
            }

            y += _y;
        }
    }

    fn generate_zone(&mut self, x: i32, y: i32, size: i32, seed: &TileSeedZone) {
        let mut x = x - size / 2;
        let mut y = y - size / 2;
        for _ in 0..size {
            for _ in 0..size {
                if distance(&x, &y, &(x + size / 2), &(y + size / 2)) > size as f32 {
                    x += 1;
                    continue;
                }
                let y1 = y.clamp(0, self.height - 1);
                let x1 = x.clamp(0, self.width - 1);
                self.tiles[y1 as usize][x1 as usize] = Tile {
                    tile_type: seed.get_tile(),
                };
                x += 1;
            }
            x -= size;
            y += 1;
        }
    }
}
