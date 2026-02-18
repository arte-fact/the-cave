/// Sprites from `animals.png` (32x32 cells).
/// Positions from `32rogues/animals.txt`.
use super::{Sheet, SpriteRef};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnimalSprite {
    GrizzlyBear,
    Boar,
}

impl AnimalSprite {
    pub const fn sprite_ref(self) -> SpriteRef {
        let (row, col) = match self {
            Self::GrizzlyBear => (0, 0),
            Self::Boar => (10, 7),
        };
        SpriteRef::new(Sheet::Animals, row, col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn animal_sprites_within_sheet_bounds() {
        let all = [AnimalSprite::GrizzlyBear, AnimalSprite::Boar];
        for sprite in all {
            let r = sprite.sprite_ref();
            assert_eq!(r.sheet, Sheet::Animals);
            assert!(r.row < 17, "{:?} row {} >= 17", sprite, r.row);
            assert!(r.col < 9, "{:?} col {} >= 9", sprite, r.col);
        }
    }
}
