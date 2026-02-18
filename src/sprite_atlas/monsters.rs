/// Sprites from `monsters.png` (12 cols x 13 rows, 32x32 cells).
/// Positions from `32rogues/monsters.txt`.
use super::{Sheet, SpriteRef};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MonsterSprite {
    Orc,
    Goblin,
    OrcBlademaster,
    GoblinArcher,
    Troll,
    SmallSlime,
    BigSlime,
    Skeleton,
    SkeletonArcher,
    Lich,
    DeathKnight,
    Zombie,
    Ghoul,
    Wraith,
    Lycanthrope,
    GiantBat,
    GiantSpider,
    WargDireWolf,
    GiantRat,
    Naga,
    SmallKoboldCanine,
    Dragon,
}

impl MonsterSprite {
    pub const fn sprite_ref(self) -> SpriteRef {
        let (row, col) = match self {
            Self::Orc => (0, 0),
            Self::Goblin => (0, 2),
            Self::OrcBlademaster => (0, 3),
            Self::GoblinArcher => (0, 5),
            Self::Troll => (1, 2),
            Self::SmallSlime => (2, 0),
            Self::BigSlime => (2, 1),
            Self::Skeleton => (4, 0),
            Self::SkeletonArcher => (4, 1),
            Self::Lich => (4, 2),
            Self::DeathKnight => (4, 3),
            Self::Zombie => (4, 4),
            Self::Ghoul => (4, 5),
            Self::Wraith => (5, 2),
            Self::Lycanthrope => (6, 5),
            Self::GiantBat => (6, 6),
            Self::GiantSpider => (6, 8),
            Self::WargDireWolf => (6, 10),
            Self::GiantRat => (6, 11),
            Self::Naga => (7, 4),
            Self::SmallKoboldCanine => (9, 0),
            Self::Dragon => (8, 2),
        };
        SpriteRef::new(Sheet::Monsters, row, col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_monster_sprites_within_sheet_bounds() {
        let all = [
            MonsterSprite::Orc, MonsterSprite::Goblin,
            MonsterSprite::OrcBlademaster, MonsterSprite::GoblinArcher,
            MonsterSprite::Troll,
            MonsterSprite::SmallSlime, MonsterSprite::BigSlime,
            MonsterSprite::Skeleton, MonsterSprite::SkeletonArcher, MonsterSprite::Lich,
            MonsterSprite::DeathKnight, MonsterSprite::Zombie, MonsterSprite::Ghoul,
            MonsterSprite::Wraith,
            MonsterSprite::Lycanthrope, MonsterSprite::GiantBat,
            MonsterSprite::GiantSpider, MonsterSprite::WargDireWolf, MonsterSprite::GiantRat,
            MonsterSprite::Naga,
            MonsterSprite::SmallKoboldCanine,
            MonsterSprite::Dragon,
        ];
        for sprite in all {
            let r = sprite.sprite_ref();
            assert_eq!(r.sheet, Sheet::Monsters);
            assert!(r.row < 13, "{:?} row {} >= 13", sprite, r.row);
            assert!(r.col < 12, "{:?} col {} >= 12", sprite, r.col);
        }
    }

    #[test]
    fn dragon_position() {
        let r = MonsterSprite::Dragon.sprite_ref();
        assert_eq!(r.row, 8);
        assert_eq!(r.col, 2);
    }
}
