#[derive(Debug, Clone, PartialEq)]
pub struct Tile {
    pub char: char,
    pub size: f32,
    pub mirror: bool,
}

impl Tile {
    pub fn to_html(&self) -> String {
        let style = if self.mirror {
            "transform: scaleX(-1);".to_string()
        } else {
            "".to_string()
        };
        format!(
            "<span class='tile' style='{} font-size: {}; width: {}px; height: {}px;'>{}</span>",
            style, 24.0 * self.size, 24, 24, self.char
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
                size: 0.8,
                mirror: false,
            },
            MapTiles::Floor => Tile {
                char: '·',
                size: 0.8,
                mirror: false,
            },
            MapTiles::Fire => Tile {
                char: '🔥',
                size: 0.8,
                mirror: false,
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
            },
            EnemyTiles::Bat => Tile {
                char: '🦇',
                size: 0.6,
                mirror: false,
            },
            EnemyTiles::Snake => Tile {
                char: '🐍',
                size: 0.6,
                mirror: false,
            },
            EnemyTiles::Spider => Tile {
                char: '🕷',
                size: 0.6,
                mirror: false,
            },
            EnemyTiles::Scorpion => Tile {
                char: '🦂',
                size: 0.6,
                mirror: false,
            },
            EnemyTiles::TRex => Tile {
                char: '🦖',
                size: 0.9,
                mirror: false,
            },
            EnemyTiles::Aligator => Tile {
                char: '🐊',
                size: 0.8,
                mirror: false,
            },
            EnemyTiles::Dragon => Tile {
                char: '🐉',
                size: 1.1,
                mirror: false,
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
            },
            ItemTiles::Shield => Tile {
                char: '🛡',
                size: 0.5,
                mirror: false,
            },
            ItemTiles::Potion => Tile {
                char: '🧪',
                size: 0.5,
                mirror: false,
            },
            ItemTiles::Diamond => Tile {
                char: '💎',
                size: 0.5,
                mirror: false,
            },
            ItemTiles::Skull => Tile {
                char: '💀',
                size: 1.0,
                mirror: false,
            },
            ItemTiles::Crown => Tile {
                char: '👑',
                size: 1.0,
                mirror: false,
            },
            ItemTiles::Meat => Tile {
                char: '🍖',
                size: 0.5,
                mirror: false,
            },
            ItemTiles::Steak => Tile {
                char: '🥩',
                size: 0.5,
                mirror: false,
            },
        }
    }
}
