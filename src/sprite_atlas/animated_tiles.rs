/// Every named sprite in `animated-tiles.png` (32x32 cells).
/// Positions from `32rogues/animated-tiles.txt`.
/// Each entry is a row; animation frames are in successive columns.
use super::{Sheet, SpriteRef};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnimatedTileSprite {
    BrazierUnlit,
    BrazierLit,
    FirePitUnlit,
    FirePitLit,
    TorchUnlit,
    TorchLit,
    LampUnlit,
    LampLit,
    Fire,
    SmallFire,
    WaterWaves,
    PoisonBubbles,
}

impl AnimatedTileSprite {
    /// Base sprite (first frame). Animation frames follow in subsequent columns.
    pub const fn sprite_ref(self) -> SpriteRef {
        let row = match self {
            Self::BrazierUnlit => 0,
            Self::BrazierLit => 1,
            Self::FirePitUnlit => 2,
            Self::FirePitLit => 3,
            Self::TorchUnlit => 4,
            Self::TorchLit => 5,
            Self::LampUnlit => 6,
            Self::LampLit => 7,
            Self::Fire => 8,
            Self::SmallFire => 9,
            Self::WaterWaves => 10,
            Self::PoisonBubbles => 11,
        };
        SpriteRef::new(Sheet::AnimatedTiles, row, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_animated_tile_sprites_valid() {
        let all = [
            AnimatedTileSprite::BrazierUnlit, AnimatedTileSprite::BrazierLit,
            AnimatedTileSprite::FirePitUnlit, AnimatedTileSprite::FirePitLit,
            AnimatedTileSprite::TorchUnlit, AnimatedTileSprite::TorchLit,
            AnimatedTileSprite::LampUnlit, AnimatedTileSprite::LampLit,
            AnimatedTileSprite::Fire, AnimatedTileSprite::SmallFire,
            AnimatedTileSprite::WaterWaves, AnimatedTileSprite::PoisonBubbles,
        ];
        for (i, sprite) in all.iter().enumerate() {
            let r = sprite.sprite_ref();
            assert_eq!(r.sheet, Sheet::AnimatedTiles);
            assert_eq!(r.row, i as u16);
            assert_eq!(r.col, 0);
        }
    }
}
