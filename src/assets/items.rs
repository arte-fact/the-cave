use crate::game::features::position::Position;
use crate::game::item::Item;

use super::tiles::ItemTiles;

pub enum Items {
    Potion,
    Sword,
    Shield,
}

impl Items {
    pub fn to_item(&self) -> Item {
        match self {
            Items::Potion => Item {
                name: "Health potion".to_string(),
                tile: ItemTiles::Potion.to_tile(),
                health: 20,
                occurences: 10,
                attack: 0,
                defense: 0,
                position: Position { x: 0, y: 0 },
            },
            Items::Sword => Item {
                name: "Sword Upgrade".to_string(),
                tile: ItemTiles::Sword.to_tile(),
                health: 0,
                occurences: 5,
                attack: 5,
                defense: 0,
                position: Position { x: 0, y: 0 },
            },
            Items::Shield => Item {
                name: "Shield Upgrade".to_string(),
                tile: ItemTiles::Shield.to_tile(),
                health: 0,
                occurences: 5,
                attack: 0,
                defense: 5,
                position: Position { x: 0, y: 0 },
            },
        }
    }
}
