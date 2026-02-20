mod generation;
mod fov;

pub use generation::{Dungeon, DungeonBiome, DungeonStyle, OverworldBiome};
pub use fov::bresenham_line;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Tile {
    Wall,
    Floor,
    Tree,
    Grass,
    Road,
    DungeonEntrance,
    StairsDown,
    StairsUp,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Visibility {
    Hidden,
    Seen,
    Visible,
}

impl Tile {
    pub fn is_walkable(self) -> bool {
        matches!(self, Tile::Floor | Tile::Grass | Tile::Road | Tile::DungeonEntrance | Tile::StairsDown | Tile::StairsUp)
    }

    /// Whether this tile blocks line of sight.
    pub fn is_opaque(self) -> bool {
        matches!(self, Tile::Wall | Tile::Tree)
    }

    pub fn glyph(self) -> char {
        match self {
            Tile::Wall => '#',
            Tile::Floor => '.',
            Tile::Tree => 'T',
            Tile::Grass => '.',
            Tile::Road => '=',
            Tile::DungeonEntrance => '>',
            Tile::StairsDown => '>',
            Tile::StairsUp => '<',
        }
    }

    pub fn color(self) -> &'static str {
        match self {
            Tile::Wall => "#333",
            Tile::Floor => "#111",
            Tile::Tree => "#050",
            Tile::Grass => "#141",
            Tile::Road => "#543",
            Tile::DungeonEntrance => "#a70",
            Tile::StairsDown => "#88f",
            Tile::StairsUp => "#88f",
        }
    }
}

pub struct Map {
    pub width: i32,
    pub height: i32,
    tiles: Vec<Tile>,
    visibility: Vec<Visibility>,
}

impl Map {
    /// Create an empty map filled with a single tile type.
    pub fn new_filled(width: i32, height: i32, tile: Tile) -> Self {
        let len = (width * height) as usize;
        Self {
            width,
            height,
            tiles: vec![tile; len],
            visibility: vec![Visibility::Hidden; len],
        }
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= 0 && y >= 0 && x < self.width && y < self.height
    }

    pub fn get(&self, x: i32, y: i32) -> Tile {
        if !self.in_bounds(x, y) {
            return Tile::Wall;
        }
        self.tiles[(y * self.width + x) as usize]
    }

    pub fn set(&mut self, x: i32, y: i32, tile: Tile) {
        if !self.in_bounds(x, y) {
            return;
        }
        self.tiles[(y * self.width + x) as usize] = tile;
    }

    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        self.in_bounds(x, y) && self.get(x, y).is_walkable()
    }

    /// Find the first tile of the given type, scanning top-to-bottom, left-to-right.
    pub fn find_tile(&self, tile: Tile) -> Option<(i32, i32)> {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get(x, y) == tile {
                    return Some((x, y));
                }
            }
        }
        None
    }

    /// Find a floor tile to spawn the player on, searching from the center outward.
    pub fn find_spawn(&self) -> (i32, i32) {
        let cx = self.width / 2;
        let cy = self.height / 2;
        let max_r = self.width.max(self.height);
        for r in 0..max_r {
            for dy in -r..=r {
                for dx in -r..=r {
                    let x = cx + dx;
                    let y = cy + dy;
                    if self.is_walkable(x, y) {
                        return (x, y);
                    }
                }
            }
        }
        (cx, cy)
    }
}

#[cfg(test)]
mod tests;
