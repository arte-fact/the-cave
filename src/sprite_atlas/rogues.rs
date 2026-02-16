/// Every named sprite in `rogues.png` (7 cols x 8 rows, 32x32 cells).
/// Positions from `32rogues/rogues.txt`.
use super::{Sheet, SpriteRef};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RogueSprite {
    // Row 0 (1) — adventurers
    Dwarf,
    Elf,
    Ranger,
    Rogue,
    Bandit,

    // Row 1 (2) — knights and fighters
    Knight,
    MaleFighter,
    FemaleKnight,
    FemaleKnightHelmetless,
    ShieldKnight,

    // Row 2 (3) — clerics and monks
    Monk,
    Priest,
    FemaleWarCleric,
    MaleWarCleric,
    Templar,
    SchemaMonk,
    ElderSchemaMonk,

    // Row 3 (4) — barbarians and swordsmen
    MaleBarbarian,
    MaleWinterBarbarian,
    FemaleWinterBarbarian,
    Swordsman,
    Fencer,
    FemaleBarbarian,

    // Row 4 (5) — wizards and mages
    FemaleWizard,
    MaleWizard,
    Druid,
    DesertSage,
    DwarfMage,

    // Row 5 (6) — warlock
    /// Listed as 6.f in rogues.txt (row 5, col 5).
    Warlock,

    // Row 6 (7) — farmers and craftsmen
    FarmerWheatThresher,
    FarmerScythe,
    FarmerPitchfork,
    Baker,
    Blacksmith,
    Scholar,

    // Row 7 (8) — peasants and townfolk
    PeasantCoalburner,
    Peasant,
    Shopkeep,
    ElderlyWoman,
    ElderlyMan,
}

impl RogueSprite {
    pub const fn sprite_ref(self) -> SpriteRef {
        let (row, col) = match self {
            Self::Dwarf => (0, 0),
            Self::Elf => (0, 1),
            Self::Ranger => (0, 2),
            Self::Rogue => (0, 3),
            Self::Bandit => (0, 4),

            Self::Knight => (1, 0),
            Self::MaleFighter => (1, 1),
            Self::FemaleKnight => (1, 2),
            Self::FemaleKnightHelmetless => (1, 3),
            Self::ShieldKnight => (1, 4),

            Self::Monk => (2, 0),
            Self::Priest => (2, 1),
            Self::FemaleWarCleric => (2, 2),
            Self::MaleWarCleric => (2, 3),
            Self::Templar => (2, 4),
            Self::SchemaMonk => (2, 5),
            Self::ElderSchemaMonk => (2, 6),

            Self::MaleBarbarian => (3, 0),
            Self::MaleWinterBarbarian => (3, 1),
            Self::FemaleWinterBarbarian => (3, 2),
            Self::Swordsman => (3, 3),
            Self::Fencer => (3, 4),
            Self::FemaleBarbarian => (3, 5),

            Self::FemaleWizard => (4, 0),
            Self::MaleWizard => (4, 1),
            Self::Druid => (4, 2),
            Self::DesertSage => (4, 3),
            Self::DwarfMage => (4, 4),

            Self::Warlock => (5, 5),

            Self::FarmerWheatThresher => (6, 0),
            Self::FarmerScythe => (6, 1),
            Self::FarmerPitchfork => (6, 2),
            Self::Baker => (6, 3),
            Self::Blacksmith => (6, 4),
            Self::Scholar => (6, 5),

            Self::PeasantCoalburner => (7, 0),
            Self::Peasant => (7, 1),
            Self::Shopkeep => (7, 2),
            Self::ElderlyWoman => (7, 3),
            Self::ElderlyMan => (7, 4),
        };
        SpriteRef::new(Sheet::Rogues, row, col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_rogue_sprites_within_sheet_bounds() {
        // rogues.png: 7 cols x 8 rows
        let all = [
            RogueSprite::Dwarf, RogueSprite::Elf, RogueSprite::Ranger, RogueSprite::Rogue, RogueSprite::Bandit,
            RogueSprite::Knight, RogueSprite::MaleFighter, RogueSprite::FemaleKnight,
            RogueSprite::FemaleKnightHelmetless, RogueSprite::ShieldKnight,
            RogueSprite::Monk, RogueSprite::Priest, RogueSprite::FemaleWarCleric, RogueSprite::MaleWarCleric,
            RogueSprite::Templar, RogueSprite::SchemaMonk, RogueSprite::ElderSchemaMonk,
            RogueSprite::MaleBarbarian, RogueSprite::MaleWinterBarbarian, RogueSprite::FemaleWinterBarbarian,
            RogueSprite::Swordsman, RogueSprite::Fencer, RogueSprite::FemaleBarbarian,
            RogueSprite::FemaleWizard, RogueSprite::MaleWizard, RogueSprite::Druid,
            RogueSprite::DesertSage, RogueSprite::DwarfMage,
            RogueSprite::Warlock,
            RogueSprite::FarmerWheatThresher, RogueSprite::FarmerScythe, RogueSprite::FarmerPitchfork,
            RogueSprite::Baker, RogueSprite::Blacksmith, RogueSprite::Scholar,
            RogueSprite::PeasantCoalburner, RogueSprite::Peasant, RogueSprite::Shopkeep,
            RogueSprite::ElderlyWoman, RogueSprite::ElderlyMan,
        ];
        for sprite in all {
            let r = sprite.sprite_ref();
            assert_eq!(r.sheet, Sheet::Rogues);
            assert!(r.row < 8, "{:?} row {} >= 8", sprite, r.row);
            assert!(r.col < 7, "{:?} col {} >= 7", sprite, r.col);
        }
    }

    #[test]
    fn rogue_is_player_sprite() {
        // Matches existing player_sprite() → row 0, col 3
        let r = RogueSprite::Rogue.sprite_ref();
        assert_eq!(r.row, 0);
        assert_eq!(r.col, 3);
    }
}
