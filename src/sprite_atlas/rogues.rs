/// Sprites from `rogues.png` (7 cols x 8 rows, 32x32 cells).
/// Positions from `32rogues/rogues.txt`.
use super::{Sheet, SpriteRef};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RogueSprite {
    Rogue,
}

impl RogueSprite {
    pub const fn sprite_ref(self) -> SpriteRef {
        let (row, col) = match self {
            Self::Rogue => (0, 3),
        };
        SpriteRef::new(Sheet::Rogues, row, col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rogue_is_player_sprite() {
        let r = RogueSprite::Rogue.sprite_ref();
        assert_eq!(r.row, 0);
        assert_eq!(r.col, 3);
    }
}
