/// Every named sprite in `monsters.png` (12 cols x 13 rows, 32x32 cells).
/// Positions from `32rogues/monsters.txt`.
use super::{Sheet, SpriteRef};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MonsterSprite {
    // Row 0 (1) — orcs and goblins
    Orc,
    OrcWizard,
    Goblin,
    OrcBlademaster,
    OrcWarchief,
    GoblinArcher,
    GoblinMage,
    GoblinBrute,

    // Row 1 (2) — ettins and troll
    Ettin,
    TwoHeadedEttin,
    Troll,

    // Row 2 (3) — slimes
    SmallSlime,
    BigSlime,
    Slimebody,
    MergedSlimebodies,

    // Row 3 (4) — monks
    FacelessMonk,
    UnholyCardinal,

    // Row 4 (5) — undead
    Skeleton,
    SkeletonArcher,
    Lich,
    DeathKnight,
    Zombie,
    Ghoul,

    // Row 5 (6) — spirits and witches
    Banshee,
    Reaper,
    Wraith,
    Cultist,
    HagWitch,

    // Row 6 (7) — beasts
    GiantCentipede,
    Lampreymander,
    GiantEarthworm,
    Manticore,
    GiantAnt,
    Lycanthrope,
    GiantBat,
    LesserGiantAnt,
    GiantSpider,
    LesserGiantSpider,
    WargDireWolf,
    GiantRat,

    // Row 7 (8) — mythical creatures
    Dryad,
    Wendigo,
    RockGolem,
    Centaur,
    Naga,
    ForestSpirit,
    Satyr,
    Minotaur,
    Harpy,
    GorgonMedusa,

    // Row 8 (9) — reptiles and dragons
    LizardfolkKobold,
    DrakeLesserDragon,
    Dragon,
    Cockatrice,
    Basilisk,

    // Row 9 (10) — canine kobolds
    SmallKoboldCanine,
    KoboldCanine,

    // Row 10 (11) — myconids
    SmallMyconid,
    LargeMyconid,

    // Row 11 (12) — celestials and fiends
    AngelArchangel,
    ImpDevil,

    // Row 12 (13) — writhing masses
    SmallWrithingMass,
    LargeWrithingMass,
    WrithingHumanoid,
}

impl MonsterSprite {
    pub const fn sprite_ref(self) -> SpriteRef {
        let (row, col) = match self {
            Self::Orc => (0, 0),
            Self::OrcWizard => (0, 1),
            Self::Goblin => (0, 2),
            Self::OrcBlademaster => (0, 3),
            Self::OrcWarchief => (0, 4),
            Self::GoblinArcher => (0, 5),
            Self::GoblinMage => (0, 6),
            Self::GoblinBrute => (0, 7),

            Self::Ettin => (1, 0),
            Self::TwoHeadedEttin => (1, 1),
            Self::Troll => (1, 2),

            Self::SmallSlime => (2, 0),
            Self::BigSlime => (2, 1),
            Self::Slimebody => (2, 2),
            Self::MergedSlimebodies => (2, 3),

            Self::FacelessMonk => (3, 0),
            Self::UnholyCardinal => (3, 1),

            Self::Skeleton => (4, 0),
            Self::SkeletonArcher => (4, 1),
            Self::Lich => (4, 2),
            Self::DeathKnight => (4, 3),
            Self::Zombie => (4, 4),
            Self::Ghoul => (4, 5),

            Self::Banshee => (5, 0),
            Self::Reaper => (5, 1),
            Self::Wraith => (5, 2),
            Self::Cultist => (5, 3),
            Self::HagWitch => (5, 4),

            Self::GiantCentipede => (6, 0),
            Self::Lampreymander => (6, 1),
            Self::GiantEarthworm => (6, 2),
            Self::Manticore => (6, 3),
            Self::GiantAnt => (6, 4),
            Self::Lycanthrope => (6, 5),
            Self::GiantBat => (6, 6),
            Self::LesserGiantAnt => (6, 7),
            Self::GiantSpider => (6, 8),
            Self::LesserGiantSpider => (6, 9),
            Self::WargDireWolf => (6, 10),
            Self::GiantRat => (6, 11),

            Self::Dryad => (7, 0),
            Self::Wendigo => (7, 1),
            Self::RockGolem => (7, 2),
            Self::Centaur => (7, 3),
            Self::Naga => (7, 4),
            Self::ForestSpirit => (7, 5),
            Self::Satyr => (7, 6),
            Self::Minotaur => (7, 7),
            Self::Harpy => (7, 8),
            Self::GorgonMedusa => (7, 9),

            Self::LizardfolkKobold => (8, 0),
            Self::DrakeLesserDragon => (8, 1),
            Self::Dragon => (8, 2),
            Self::Cockatrice => (8, 3),
            Self::Basilisk => (8, 4),

            Self::SmallKoboldCanine => (9, 0),
            Self::KoboldCanine => (9, 1),

            Self::SmallMyconid => (10, 0),
            Self::LargeMyconid => (10, 1),

            Self::AngelArchangel => (11, 0),
            Self::ImpDevil => (11, 1),

            Self::SmallWrithingMass => (12, 0),
            Self::LargeWrithingMass => (12, 1),
            Self::WrithingHumanoid => (12, 2),
        };
        SpriteRef::new(Sheet::Monsters, row, col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_monster_sprites_within_sheet_bounds() {
        // monsters.png: 12 cols x 13 rows
        let all = [
            MonsterSprite::Orc, MonsterSprite::OrcWizard, MonsterSprite::Goblin,
            MonsterSprite::OrcBlademaster, MonsterSprite::OrcWarchief,
            MonsterSprite::GoblinArcher, MonsterSprite::GoblinMage, MonsterSprite::GoblinBrute,
            MonsterSprite::Ettin, MonsterSprite::TwoHeadedEttin, MonsterSprite::Troll,
            MonsterSprite::SmallSlime, MonsterSprite::BigSlime, MonsterSprite::Slimebody, MonsterSprite::MergedSlimebodies,
            MonsterSprite::FacelessMonk, MonsterSprite::UnholyCardinal,
            MonsterSprite::Skeleton, MonsterSprite::SkeletonArcher, MonsterSprite::Lich,
            MonsterSprite::DeathKnight, MonsterSprite::Zombie, MonsterSprite::Ghoul,
            MonsterSprite::Banshee, MonsterSprite::Reaper, MonsterSprite::Wraith,
            MonsterSprite::Cultist, MonsterSprite::HagWitch,
            MonsterSprite::GiantCentipede, MonsterSprite::Lampreymander, MonsterSprite::GiantEarthworm,
            MonsterSprite::Manticore, MonsterSprite::GiantAnt, MonsterSprite::Lycanthrope,
            MonsterSprite::GiantBat, MonsterSprite::LesserGiantAnt,
            MonsterSprite::GiantSpider, MonsterSprite::LesserGiantSpider,
            MonsterSprite::WargDireWolf, MonsterSprite::GiantRat,
            MonsterSprite::Dryad, MonsterSprite::Wendigo, MonsterSprite::RockGolem,
            MonsterSprite::Centaur, MonsterSprite::Naga, MonsterSprite::ForestSpirit,
            MonsterSprite::Satyr, MonsterSprite::Minotaur, MonsterSprite::Harpy, MonsterSprite::GorgonMedusa,
            MonsterSprite::LizardfolkKobold, MonsterSprite::DrakeLesserDragon, MonsterSprite::Dragon,
            MonsterSprite::Cockatrice, MonsterSprite::Basilisk,
            MonsterSprite::SmallKoboldCanine, MonsterSprite::KoboldCanine,
            MonsterSprite::SmallMyconid, MonsterSprite::LargeMyconid,
            MonsterSprite::AngelArchangel, MonsterSprite::ImpDevil,
            MonsterSprite::SmallWrithingMass, MonsterSprite::LargeWrithingMass, MonsterSprite::WrithingHumanoid,
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
