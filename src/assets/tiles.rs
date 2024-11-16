use crate::tile::Tile;

use super::colors::Color;

pub enum EnemyTiles {
    Crab,
    Rat,
    Bat,
    Snake,
    Spider,
    Scorpion,
    Aligator,
    Tiger,
    TRex,
    Dragon,
}

impl EnemyTiles {
    pub fn to_tile(&self) -> Tile {
        match self {
            EnemyTiles::Crab => Tile {
                char: '🦀',
                size: 0.6,
                mirror: false,
                opacity: 1.0,
                hue: 0.0,
                altitude: 0.0,
                color: Color::None,
            },
            EnemyTiles::Rat => Tile {
                char: '🐀',
                size: 0.6,
                mirror: false,
                opacity: 1.0,
                hue: 0.0,
                altitude: 0.0,
                color: Color::None,
            },
            EnemyTiles::Bat => Tile {
                char: '🦇',
                size: 0.6,
                mirror: false,
                opacity: 1.0,
                hue: 0.0,
                altitude: 0.0,
                color: Color::None,
            },
            EnemyTiles::Snake => Tile {
                char: '🐍',
                size: 0.6,
                mirror: false,
                opacity: 1.0,
                hue: 0.0,
                altitude: 0.0,
                color: Color::None,
            },
            EnemyTiles::Spider => Tile {
                char: '🕷',
                size: 0.6,
                mirror: false,
                opacity: 1.0,
                hue: 0.0,
                altitude: 0.0,
                color: Color::None,
            },
            EnemyTiles::Scorpion => Tile {
                char: '🦂',
                size: 0.6,
                mirror: false,
                opacity: 1.0,
                hue: 0.0,
                altitude: 0.0,
                color: Color::None,
            },
            EnemyTiles::Tiger => Tile {
                char: '🐅',
                size: 0.9,
                mirror: false,
                opacity: 1.0,
                hue: 0.0,
                altitude: 0.0,
                color: Color::None,
            },
            EnemyTiles::TRex => Tile {
                char: '🦖',
                size: 0.9,
                mirror: false,
                opacity: 1.0,
                hue: 0.0,
                altitude: 0.0,
                color: Color::None,
            },
            EnemyTiles::Aligator => Tile {
                char: '🐊',
                size: 0.8,
                mirror: false,
                opacity: 1.0,
                hue: 0.0,
                altitude: 0.0,
                color: Color::None,
            },
            EnemyTiles::Dragon => Tile {
                char: '🐉',
                size: 1.1,
                mirror: false,
                opacity: 1.0,
                hue: 0.0,
                altitude: 0.0,
                color: Color::None,
            },
        }
    }
}

pub enum ItemTiles {
    Sword,
    Shield,
    Potion,
    Diamond,
    Skull,
    Crown,
    Meat,
    Steak,
}

impl ItemTiles {
    pub fn to_tile(&self) -> Tile {
        match self {
            ItemTiles::Sword => Tile {
                char: '🗡',
                size: 0.5,
                mirror: false,
                opacity: 1.0,
                hue: 0.0,
                altitude: 0.0,
                color: Color::None,
            },
            ItemTiles::Shield => Tile {
                char: '🛡',
                size: 0.5,
                mirror: false,
                opacity: 1.0,
                hue: 0.0,
                altitude: 0.0,
                color: Color::None,
            },
            ItemTiles::Potion => Tile {
                char: '🧪',
                size: 0.5,
                mirror: false,
                opacity: 1.0,
                hue: 0.0,
                altitude: 0.0,
                color: Color::None,
            },
            ItemTiles::Diamond => Tile {
                char: '💎',
                size: 0.5,
                mirror: false,
                opacity: 1.0,
                hue: 0.0,
                color: Color::None,
                altitude: 0.0,
            },
            ItemTiles::Skull => Tile {
                char: '💀',
                size: 0.5,
                mirror: false,
                opacity: 1.0,
                hue: 0.0,
                altitude: 0.0,
                color: Color::None,
            },
            ItemTiles::Crown => Tile {
                char: '👑',
                size: 1.0,
                mirror: false,
                opacity: 1.0,
                hue: 0.0,
                altitude: 0.0,
                color: Color::None,
            },
            ItemTiles::Meat => Tile {
                char: '🍖',
                size: 0.5,
                mirror: false,
                opacity: 1.0,
                hue: 0.0,
                altitude: 0.0,
                color: Color::None,
            },
            ItemTiles::Steak => Tile {
                char: '🥩',
                size: 0.5,
                mirror: false,
                opacity: 1.0,
                hue: 0.0,
                altitude: 0.0,
                color: Color::None,
            },
        }
    }
}
