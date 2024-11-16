use crate::game::biome::Biome;
use crate::tile::MapTiles;

use super::colors::Color;
use super::enemies::Enemies;
use super::items::Items;

#[derive(Clone, Debug, PartialEq)]
pub enum Biomes {
    Desert,
    Forest,
    Shore,
    Ocean,
    Cave,
    Jungle,
}

impl Biomes {
    pub fn get_background(&self) -> Color {
        match self {
            Biomes::Desert => Color::Sand,
            Biomes::Forest => Color::Grass,
            Biomes::Shore => Color::Sand,
            Biomes::Ocean => Color::Water,
            Biomes::Cave => Color::Black,
            Biomes::Jungle => Color::Grass,
        }
    }
    pub fn get_filler_tile(&self) -> MapTiles {
        match self {
            Biomes::Jungle => MapTiles::Palm,
            Biomes::Desert => MapTiles::Rock,
            Biomes::Forest => MapTiles::Tree,
            Biomes::Shore => MapTiles::Wave,
            Biomes::Ocean => MapTiles::Wave,
            Biomes::Cave => MapTiles::Rock,
        }
    }
    pub fn get_tiles(&self) -> Vec<Biome> {
        match self {
            Biomes::Desert => vec![
                Biome::Item(Items::Potion.to_item(), 10),
                Biome::Item(Items::Sword.to_item(), 5),
                Biome::Item(Items::Shield.to_item(), 5),
                Biome::Enemy(Enemies::Rat.to_enemy(), 10),
                Biome::Enemy(Enemies::Bat.to_enemy(), 5),
                Biome::Enemy(Enemies::Snake.to_enemy(), 5),
                Biome::Enemy(Enemies::Spider.to_enemy(), 5),
                Biome::Enemy(Enemies::Scorpion.to_enemy(), 5),
                Biome::Block(MapTiles::Floor.to_tile(), 10000),
            ],
            Biomes::Forest => vec![
                Biome::Item(Items::Potion.to_item(), 10),
                Biome::Item(Items::Sword.to_item(), 5),
                Biome::Item(Items::Shield.to_item(), 5),
                Biome::Enemy(Enemies::Rat.to_enemy(), 10),
                Biome::Enemy(Enemies::Bat.to_enemy(), 5),
                Biome::Enemy(Enemies::Snake.to_enemy(), 5),
                Biome::Enemy(Enemies::Spider.to_enemy(), 5),
                Biome::Enemy(Enemies::Scorpion.to_enemy(), 5),
                Biome::Block(MapTiles::Floor.to_tile(), 10000),
            ],
            Biomes::Ocean => vec![
                Biome::Block(MapTiles::Wave.to_tile(), 10),
            ],
            Biomes::Cave => vec![
                Biome::Item(Items::Potion.to_item(), 10),
                Biome::Item(Items::Sword.to_item(), 5),
                Biome::Item(Items::Shield.to_item(), 5),
                Biome::Enemy(Enemies::Rat.to_enemy(), 10),
                Biome::Enemy(Enemies::Bat.to_enemy(), 5),
                Biome::Enemy(Enemies::Snake.to_enemy(), 5),
                Biome::Enemy(Enemies::Spider.to_enemy(), 5),
                Biome::Enemy(Enemies::Scorpion.to_enemy(), 5),
                Biome::Block(MapTiles::Floor.to_tile(), 10000),
            ],
            Biomes::Shore => vec![
                Biome::Enemy(Enemies::Crab.to_enemy(), 10),
                Biome::Block(MapTiles::Floor.to_tile(), 10000),
                Biome::Block(MapTiles::Palm.to_tile(), 10),
            ],
            Biomes::Jungle => vec![
                Biome::Item(Items::Potion.to_item(), 10),
                Biome::Item(Items::Sword.to_item(), 5),
                Biome::Item(Items::Shield.to_item(), 5),
                Biome::Enemy(Enemies::Snake.to_enemy(), 5),
                Biome::Enemy(Enemies::Spider.to_enemy(), 5),
                Biome::Enemy(Enemies::Tiger.to_enemy(), 5),
                Biome::Block(MapTiles::Palm.to_tile(), 100),
                Biome::Block(MapTiles::Floor.to_tile(), 10000),
            ],
        }
    }
}


