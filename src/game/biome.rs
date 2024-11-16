use rand::Rng;

use crate::tile::Tile;

use super::enemy::Enemy;
use super::Item;

#[derive(Clone, Debug, PartialEq)]
pub enum Biome {
    Item(Item, u32),
    Enemy(Enemy, u32),
    Block(Tile, u32),
}

impl Biome {
    pub fn get_tile(&self) -> Tile {
        match self {
            Biome::Item(item, _) => item.tile.clone(),
            Biome::Enemy(ennemy, _) => ennemy.tile.clone(),
            Biome::Block(tile, _) => tile.clone(),
        }
    }

    fn get_occurences(&self) -> u32 {
        match self {
            Biome::Item(_, occurences) => *occurences,
            Biome::Enemy(_, occurences) => *occurences,
            Biome::Block(_, occurences) => *occurences,
        }
    }

    pub fn get_random_element(elements: Vec<Biome>) -> Biome {
        let mut rng = rand::thread_rng();
        let mut total = 0;
        for element in &elements {
            total += element.get_occurences();
        }
        let mut roll = rng.gen_range(0..total);
        for element in &elements {
            if roll < element.get_occurences() {
                return element.clone();
            }
            roll -= element.get_occurences();
        }
        elements[0].clone()
    }
}
