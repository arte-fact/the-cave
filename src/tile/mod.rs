pub struct Tile {
    pub char: char,
    pub size: f32,
    pub mirror: bool,
}

impl Tile {
    pub fn to_html(&self) -> String {
        let style = if self.mirror {
            format!(
                "font-size: {}px; transform: scaleX(-1);",
                self.size
            )
        } else {
            format!("font-size: {}px;", self.size)
        };
        format!(
            "<span style=\"{}\">{}</span>",
            style, self.char
        )
    }
}

pub enum MapTiles {
    Rock,
    Floor,
    Lava,
    Fire,
}

impl MapTiles {
    pub fn to_tile(&self) -> Tile {
        match self {
            MapTiles::Rock => Tile {
                char: '🪨',
                size: 20.0,
                mirror: false,
            },
            MapTiles::Floor => Tile {
                char: '·',
                size: 20.0,
                mirror: false,
            },
            MapTiles::Lava => Tile {
                char: '🔥',
                size: 20.0,
                mirror: false,
            },
            MapTiles::Fire => Tile {
                char: '🔥',
                size: 20.0,
                mirror: true,
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
    Crocodile,
    TRex,
    Dragon,
}

impl EnemyTiles {
    pub fn to_tile(&self) -> Tile {
        match self {
            EnemyTiles::Rat => Tile {
                char: '🐀',
                size: 20.0,
                mirror: false,
            },
            EnemyTiles::Bat => Tile {
                char: '🦇',
                size: 20.0,
                mirror: false,
            },
            EnemyTiles::Snake => Tile {
                char: '🐍',
                size: 20.0,
                mirror: false,
            },
            EnemyTiles::Spider => Tile {
                char: '🕷',
                size: 20.0,
                mirror: false,
            },
            EnemyTiles::Scorpion => Tile {
                char: '🦂',
                size: 20.0,
                mirror: false,
            },
            EnemyTiles::TRex => Tile {
                char: '🦖',
                size: 20.0,
                mirror: false,
            },
            EnemyTiles::Crocodile => Tile {
                char: '🐊',
                size: 20.0,
                mirror: false,
            },
            EnemyTiles::Dragon => Tile {
                char: '🐉',
                size: 20.0,
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
}

impl ItemTiles {
    pub fn to_tile(&self) -> Tile {
        match self {
            ItemTiles::Sword => Tile {
                char: '🗡',
                size: 20.0,
                mirror: false,
            },
            ItemTiles::Shield => Tile {
                char: '🛡',
                size: 20.0,
                mirror: false,
            },
            ItemTiles::Potion => Tile {
                char: '🧪',
                size: 20.0,
                mirror: false,
            },
            ItemTiles::Diamond => Tile {
                char: '💎',
                size: 20.0,
                mirror: false,
            },
            ItemTiles::Skull => Tile {
                char: '💀',
                size: 20.0,
                mirror: false,
            },
            ItemTiles::Crown => Tile {
                char: '👑',
                size: 20.0,
                mirror: false,
            },
        }
    }
}
