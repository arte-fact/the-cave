/// Sprites from `animals.png` (9 cols x 16 rows, 32x32 cells).
/// Positions from `32rogues/animals.txt`.
///
/// NOTE: The txt file labels orangutan as "3.c." but it's actually at row 2
/// (same row as chimp/gorilla) — the image has 16 rows, not 17. All rows
/// from txt row 4 onward are offset by -1 compared to the txt numbering.
use super::{Sheet, SpriteRef};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnimalSprite {
    // Row 0: Bears
    GrizzlyBear,
    BlackBear,
    PolarBear,
    Panda,

    // Row 3: Cats (txt row 5)
    Cat,
    Bobcat,
    Cougar,
    Cheetah,
    Lynx,
    Ocelot,
    MaleLion,
    FemaleLion,

    // Row 4: Canines (txt row 6)
    Dog,
    Hyena,
    Fox,
    Jackal,
    Coyote,
    Wolf,

    // Row 5: Rodents (txt row 7)
    Capybara,
    Beaver,

    // Row 6: Misc mammals (txt row 8)
    Badger,
    Honeybadger,
    Rabbit,
    Hare,
    Rat,

    // Row 7: Snakes (txt row 9)
    Snake,
    Cobra,
    Kingsnake,
    BlackMamba,

    // Row 8: Reptiles (txt row 10)
    Alligator,
    MonitorLizard,
    Iguana,
    Tortoise,
    SnappingTurtle,

    // Row 9-10: Farm/large animals (txt rows 11-12)
    Cow,
    Horse,
    Donkey,
    Pig,
    Boar,
    Camel,
    WaterBuffalo,
    Yak,

    // Row 11: Birds (txt row 13)
    Seagull,
    BarnOwl,
    Buzzard,

    // Row 14: Poultry (txt row 16)
    Chicken,
    Rooster,

    // Row 15: Goats/sheep (txt row 17)
    Goat,
    MountainGoat,
    Ibex,
    Sheep,
}

impl AnimalSprite {
    pub const fn sprite_ref(self) -> SpriteRef {
        let (row, col) = match self {
            // Row 0: Bears
            Self::GrizzlyBear => (0, 0),
            Self::BlackBear => (0, 1),
            Self::PolarBear => (0, 2),
            Self::Panda => (0, 3),

            // Row 3: Cats (txt row 5, image row 3)
            Self::Cat => (3, 0),
            Self::Bobcat => (3, 1),
            Self::Cougar => (3, 2),
            Self::Cheetah => (3, 3),
            Self::Lynx => (3, 4),
            Self::Ocelot => (3, 5),
            Self::MaleLion => (3, 6),
            Self::FemaleLion => (3, 7),

            // Row 4: Canines (txt row 6, image row 4)
            Self::Dog => (4, 0),
            Self::Hyena => (4, 2),
            Self::Fox => (4, 3),
            Self::Jackal => (4, 4),
            Self::Coyote => (4, 5),
            Self::Wolf => (4, 6),

            // Row 5: Rodents (txt row 7, image row 5)
            Self::Capybara => (5, 0),
            Self::Beaver => (5, 1),

            // Row 6: Misc mammals (txt row 8, image row 6)
            Self::Badger => (6, 2),
            Self::Honeybadger => (6, 3),
            Self::Rabbit => (6, 6),
            Self::Hare => (6, 7),
            Self::Rat => (6, 8),

            // Row 7: Snakes (txt row 9, image row 7)
            Self::Snake => (7, 0),
            Self::Cobra => (7, 1),
            Self::Kingsnake => (7, 2),
            Self::BlackMamba => (7, 3),

            // Row 8: Reptiles (txt row 10, image row 8)
            Self::Alligator => (8, 0),
            Self::MonitorLizard => (8, 1),
            Self::Iguana => (8, 2),
            Self::Tortoise => (8, 3),
            Self::SnappingTurtle => (8, 4),

            // Row 9-10: Farm/large (txt rows 11-12, image rows 9-10)
            Self::Cow => (9, 0),
            Self::Horse => (9, 1),
            Self::Donkey => (9, 2),
            Self::Pig => (9, 6),
            Self::Boar => (9, 7),
            Self::Camel => (10, 0),
            Self::WaterBuffalo => (10, 2),
            Self::Yak => (10, 3),

            // Row 11: Birds (txt row 13, image row 11)
            Self::Seagull => (11, 0),
            Self::BarnOwl => (11, 1),
            Self::Buzzard => (11, 2),

            // Row 14: Poultry (txt row 16, image row 14)
            Self::Chicken => (14, 0),
            Self::Rooster => (14, 1),

            // Row 15: Goats/sheep (txt row 17, image row 15)
            Self::Goat => (15, 0),
            Self::MountainGoat => (15, 1),
            Self::Ibex => (15, 2),
            Self::Sheep => (15, 3),
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
            AnimalSprite::Cat, AnimalSprite::Bobcat, AnimalSprite::Cougar, AnimalSprite::Cheetah,
            AnimalSprite::Lynx, AnimalSprite::Ocelot, AnimalSprite::MaleLion, AnimalSprite::FemaleLion,
            AnimalSprite::Dog, AnimalSprite::Hyena, AnimalSprite::Fox, AnimalSprite::Jackal,
            AnimalSprite::Coyote, AnimalSprite::Wolf,
            AnimalSprite::Capybara, AnimalSprite::Beaver,
            AnimalSprite::Badger, AnimalSprite::Honeybadger, AnimalSprite::Rabbit, AnimalSprite::Hare, AnimalSprite::Rat,
            AnimalSprite::Snake, AnimalSprite::Cobra, AnimalSprite::Kingsnake, AnimalSprite::BlackMamba,
            AnimalSprite::Alligator, AnimalSprite::MonitorLizard, AnimalSprite::Iguana, AnimalSprite::Tortoise, AnimalSprite::SnappingTurtle,
            AnimalSprite::Cow, AnimalSprite::Horse, AnimalSprite::Donkey, AnimalSprite::Pig, AnimalSprite::Boar,
            AnimalSprite::Camel, AnimalSprite::WaterBuffalo, AnimalSprite::Yak,
            AnimalSprite::Seagull, AnimalSprite::BarnOwl, AnimalSprite::Buzzard,
            AnimalSprite::Chicken, AnimalSprite::Rooster,
            AnimalSprite::Goat, AnimalSprite::MountainGoat, AnimalSprite::Ibex, AnimalSprite::Sheep,
        ];
        for sprite in all {
            let r = sprite.sprite_ref();
            assert_eq!(r.sheet, Sheet::Animals);
            assert!(r.row < 16, "{:?} row {} >= 16", sprite, r.row);
            assert!(r.col < 9, "{:?} col {} >= 9", sprite, r.col);
        }
    }
}
