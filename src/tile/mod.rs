use rand::Rng;

use crate::assets::colors::Color;
use crate::html::div;

#[derive(Debug, Clone, PartialEq)]
pub struct Tile {
    pub char: char,
    pub size: f32,
    pub mirror: bool,
    pub opacity: f32,
    pub hue: f32,
    pub altitude: f32,
    pub color: Color,
}

impl Tile {
    pub fn to_html(&self) -> String {
        let _scale_x = if self.mirror { "scaleX(-1)" } else { "" };

        let transform = _scale_x;

        let size = 48.0 * self.size;

        div()
            .style("width", "32px")
            .style("height", "32px")
            .style("display", "flex")
            .style("justify-content", "center")
            .style("align-items", "end")
            .style("background-color", &self.color.to_html())
            .style("overflow", "visible")
            .child(
                div()
                    .style("filter", &format!("hue-rotate({}deg)", self.hue))
                    .style("transform", &transform)
                    .style("font-size", &(size.to_string() + "px"))
                    .text(&self.char.to_string()),
            )
            .render()
    }
}

pub enum MapTiles {
    Rock,
    Floor,
    Fire,
    Cactus,
    Pine,
    Palm,
    Tree,
    Wave,
}

impl MapTiles {
    pub fn to_tile(&self) -> Tile {
        let rng = &mut rand::thread_rng();
        match self {
            MapTiles::Rock => Tile {
                char: '🪨',
                size: rng.gen_range(0.8..1.2),
                mirror: rng.gen_bool(0.5),
                opacity: 1.0,
                hue: rng.gen_range(0..6) as f32,
                altitude: 0.0,
                color: Color::None,
            },
            MapTiles::Floor => Tile {
                char: '.',
                size: 0.8,
                mirror: false,
                opacity: 1.0,
                hue: 0.0,
                altitude: 0.0,
                color: Color::None,
            },
            MapTiles::Fire => Tile {
                char: '🔥',
                size: 0.8,
                mirror: false,
                opacity: 1.0,
                hue: 0.0,
                altitude: 0.0,
                color: Color::Stone,
            },
            MapTiles::Cactus => Tile {
                char: '🌵',
                size: 0.8,
                mirror: false,
                opacity: 1.0,
                hue: 0.0,
                altitude: 0.0,
                color: Color::Stone,
            },
            MapTiles::Pine => Tile {
                char: '🌲',
                size: 1.2,
                mirror: false,
                opacity: 1.0,
                hue: 0.0,
                altitude: 0.0,
                color: Color::Stone,
            },
            MapTiles::Palm => Tile {
                char: '🌴',
                altitude: rng.gen_range(-0.5..0.5),
                color: Color::Stone,
                size: rng.gen_range(0.6..1.2),
                mirror: false,
                opacity: 1.0,
                hue: rng.gen_range(-30..30) as f32,
            },
            MapTiles::Tree => Tile {
                char: '🌳',
                size: rng.gen_range(0.8..1.2),
                mirror: rng.gen_bool(0.5),
                opacity: 1.0,
                hue: rng.gen_range(0..6) as f32,
                altitude: 0.0,
                color: Color::Stone,
            },
            MapTiles::Wave => Tile {
                char: '🌊',
                size: rng.gen_range(0.8..1.0),
                mirror: false,
                opacity: 1.0,
                hue: 0.0,
                altitude: -1.0,
                color: Color::Stone,
            },
        }
    }
}
