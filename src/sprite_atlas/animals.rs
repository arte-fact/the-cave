/// Sprites from `animals.png` (9 cols x 16 rows, 32x32 cells).
/// Positions from `32rogues/animals.txt`.
///
/// Variants are forward declarations for future sprite usage.
///
/// NOTE: The txt file labels orangutan as "3.c." but it's actually at row 2
/// (same row as chimp/gorilla) — the image has 16 rows, not 17. All rows
/// from txt row 4 onward are offset by -1 compared to the txt numbering.
///
/// Only wild animals relevant to the game's biomes are mapped here.
/// Domestic animals (cat, dog, cow, horse, donkey, pig, chicken, rooster,
/// goat, sheep, camel) are omitted.
use super::{Sheet, SpriteRef};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnimalSprite {
    // Row 0: Bears (temperate/cold)
    GrizzlyBear,
    BlackBear,
    PolarBear,
    Panda,

    // Row 3: Wild cats (jungle/temperate)
    Bobcat,
    Cougar,
    Cheetah,
    Lynx,
    Ocelot,
    MaleLion,
    FemaleLion,

    // Row 4: Wild canines (temperate/jungle)
    Hyena,
    Fox,
    Jackal,
    Coyote,
    Wolf,

    // Row 5: Rodents (temperate/jungle)
    Capybara,
    Beaver,

    // Row 6: Wild mammals (temperate/jungle)
    Badger,
    Honeybadger,
    Rabbit,
    Hare,
    Rat,

    // Row 7: Snakes (jungle/temperate)
    Snake,
    Cobra,
    Kingsnake,
    BlackMamba,

    // Row 8: Reptiles (jungle/swamp)
    Alligator,
    MonitorLizard,
    Iguana,
    Tortoise,
    SnappingTurtle,

    // Row 9: Wild large animals
    Boar,
    WaterBuffalo,
    Yak,

    // Row 11: Wild birds (temperate/coastal)
    Seagull,
    BarnOwl,
    Buzzard,

    // Row 15: Wild mountain goats
    MountainGoat,
    Ibex,
}

impl AnimalSprite {
    pub const fn sprite_ref(self) -> SpriteRef {
        let (row, col) = match self {
            // Row 0: Bears
            Self::GrizzlyBear => (0, 0),
            Self::BlackBear => (0, 1),
            Self::PolarBear => (0, 2),
            Self::Panda => (0, 3),

            // Row 3: Wild cats
            Self::Bobcat => (3, 1),
            Self::Cougar => (3, 2),
            Self::Cheetah => (3, 3),
            Self::Lynx => (3, 4),
            Self::Ocelot => (3, 5),
            Self::MaleLion => (3, 6),
            Self::FemaleLion => (3, 7),

            // Row 4: Wild canines
            Self::Hyena => (4, 2),
            Self::Fox => (4, 3),
            Self::Jackal => (4, 4),
            Self::Coyote => (4, 5),
            Self::Wolf => (4, 6),

            // Row 5: Rodents
            Self::Capybara => (5, 0),
            Self::Beaver => (5, 1),

            // Row 6: Wild mammals
            Self::Badger => (6, 2),
            Self::Honeybadger => (6, 3),
            Self::Rabbit => (6, 6),
            Self::Hare => (6, 7),
            Self::Rat => (6, 8),

            // Row 7: Snakes
            Self::Snake => (7, 0),
            Self::Cobra => (7, 1),
            Self::Kingsnake => (7, 2),
            Self::BlackMamba => (7, 3),

            // Row 8: Reptiles
            Self::Alligator => (8, 0),
            Self::MonitorLizard => (8, 1),
            Self::Iguana => (8, 2),
            Self::Tortoise => (8, 3),
            Self::SnappingTurtle => (8, 4),

            // Row 9-10: Wild large animals
            Self::Boar => (9, 7),
            Self::WaterBuffalo => (10, 2),
            Self::Yak => (10, 3),

            // Row 11: Wild birds
            Self::Seagull => (11, 0),
            Self::BarnOwl => (11, 1),
            Self::Buzzard => (11, 2),

            // Row 15: Wild mountain goats
            Self::MountainGoat => (15, 1),
            Self::Ibex => (15, 2),
        };
        SpriteRef::new(Sheet::Animals, row, col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn animal_sprites_within_sheet_bounds() {
        let all = [
            AnimalSprite::GrizzlyBear, AnimalSprite::BlackBear, AnimalSprite::PolarBear, AnimalSprite::Panda,
            AnimalSprite::Bobcat, AnimalSprite::Cougar, AnimalSprite::Cheetah,
            AnimalSprite::Lynx, AnimalSprite::Ocelot, AnimalSprite::MaleLion, AnimalSprite::FemaleLion,
            AnimalSprite::Hyena, AnimalSprite::Fox, AnimalSprite::Jackal,
            AnimalSprite::Coyote, AnimalSprite::Wolf,
            AnimalSprite::Capybara, AnimalSprite::Beaver,
            AnimalSprite::Badger, AnimalSprite::Honeybadger, AnimalSprite::Rabbit, AnimalSprite::Hare, AnimalSprite::Rat,
            AnimalSprite::Snake, AnimalSprite::Cobra, AnimalSprite::Kingsnake, AnimalSprite::BlackMamba,
            AnimalSprite::Alligator, AnimalSprite::MonitorLizard, AnimalSprite::Iguana, AnimalSprite::Tortoise, AnimalSprite::SnappingTurtle,
            AnimalSprite::Boar,
            AnimalSprite::WaterBuffalo, AnimalSprite::Yak,
            AnimalSprite::Seagull, AnimalSprite::BarnOwl, AnimalSprite::Buzzard,
            AnimalSprite::MountainGoat, AnimalSprite::Ibex,
        ];
        for sprite in all {
            let r = sprite.sprite_ref();
            assert_eq!(r.sheet, Sheet::Animals);
            assert!(r.row < 16, "{:?} row {} >= 16", sprite, r.row);
            assert!(r.col < 9, "{:?} col {} >= 9", sprite, r.col);
        }
    }
}
