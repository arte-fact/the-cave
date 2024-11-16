#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Sand,
    Stone,
    Water,
    Grass,
    Black,
    None,
}

impl Color {
    pub fn to_html(&self) -> &str {
        match self {
            Color::Sand => "darkkhaki",
            Color::Stone => "darkgray",
            Color::Water => "steelblue",
            Color::Grass => "darkolivegreen",
            Color::Black => "black",
            Color::None => "",
        }
    }
}

