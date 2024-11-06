#[derive(Debug, Clone, PartialEq)]
pub struct Tile {
    pub char: char,
    pub size: f32,
    pub mirror: bool,
    pub opacity: f32,
}

impl Tile {
    pub fn to_html(&self) -> String {
        let style = if self.mirror {
            "transform: scaleX(-1);".to_string()
        } else {
            "".to_string()
        };
        format!(
            "<span class='tile' style='{} font-size: {}; width: {}px; height: {}px; opacity: {}'>{}</span>",
            style, 24.0 * self.size, 24, 24, self.opacity, self.char
        )
    }
}

pub enum MapTiles {
    Rock,
    Floor,
    Fire,
}

impl MapTiles {
    pub fn to_tile(&self) -> Tile {
        match self {
            MapTiles::Rock => Tile {
                char: '🪨',
                size: 1.0,
                mirror: false,
                opacity: 1.0,
            },
            MapTiles::Floor => Tile {
                char: '.',
                size: 0.8,
                mirror: false,
                opacity: 1.0,
            },
            MapTiles::Fire => Tile {
                char: '🔥',
                size: 0.8,
                mirror: false,
                opacity: 1.0,
            },
        }
    }
}

pub enum EnemyTiles {
    Rat,
    Bat,
    Snake,
    Spider,
    Scorpion,
    Aligator,
    TRex,
    Dragon,
}

impl EnemyTiles {
    pub fn to_tile(&self) -> Tile {
        match self {
            EnemyTiles::Rat => Tile {
                char: '🐀',
                size: 0.6,
                mirror: false,
                opacity: 1.0,
            },
            EnemyTiles::Bat => Tile {
                char: '🦇',
                size: 0.6,
                mirror: false,
                opacity: 1.0,
            },
            EnemyTiles::Snake => Tile {
                char: '🐍',
                size: 0.6,
                mirror: false,
                opacity: 1.0,
            },
            EnemyTiles::Spider => Tile {
                char: '🕷',
                size: 0.6,
                mirror: false,
                opacity: 1.0,
            },
            EnemyTiles::Scorpion => Tile {
                char: '🦂',
                size: 0.6,
                mirror: false,
                opacity: 1.0,
            },
            EnemyTiles::TRex => Tile {
                char: '🦖',
                size: 0.9,
                mirror: false,
                opacity: 1.0,
            },
            EnemyTiles::Aligator => Tile {
                char: '🐊',
                size: 0.8,
                mirror: false,
                opacity: 1.0,
            },
            EnemyTiles::Dragon => Tile {
                char: '🐉',
                size: 1.1,
                mirror: false,
                opacity: 1.0,
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
            },
            ItemTiles::Shield => Tile {
                char: '🛡',
                size: 0.5,
                mirror: false,
                opacity: 1.0,
            },
            ItemTiles::Potion => Tile {
                char: '🧪',
                size: 0.5,
                mirror: false,
                opacity: 1.0,
            },
            ItemTiles::Diamond => Tile {
                char: '💎',
                size: 0.5,
                mirror: false,
                opacity: 1.0,
            },
            ItemTiles::Skull => Tile {
                char: '💀',
                size: 0.5,
                mirror: false,
                opacity: 1.0,
            },
            ItemTiles::Crown => Tile {
                char: '👑',
                size: 1.0,
                mirror: false,
                opacity: 1.0,
            },
            ItemTiles::Meat => Tile {
                char: '🍖',
                size: 0.5,
                mirror: false,
                opacity: 1.0,
            },
            ItemTiles::Steak => Tile {
                char: '🥩',
                size: 0.5,
                mirror: false,
                opacity: 1.0,
            },
        }
    }
}
