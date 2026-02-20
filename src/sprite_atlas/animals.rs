/// Sprites from `animals.png` (9 cols x 17 rows, 32x32 cells).
/// Positions from `32rogues/animals.txt`.
use super::{Sheet, SpriteRef};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnimalSprite {
    // Row 1: Bears
    GrizzlyBear,
    BlackBear,
    PolarBear,
    Panda,

    // Row 5: Cats
    Cat,
    Bobcat,
    Cougar,
    Cheetah,
    Lynx,
    Ocelot,
    MaleLion,
    FemaleLion,

    // Row 6: Canines
    Dog,
    Hyena,
    Fox,
    Jackal,
    Coyote,
    Wolf,

    // Row 7: Rodents
    Capybara,
    Beaver,

    // Row 8: Misc mammals
    Badger,
    Honeybadger,
    Rabbit,
    Hare,
    Rat,

    // Row 9: Snakes
    Snake,
    Cobra,
    Kingsnake,
    BlackMamba,

    // Row 10: Reptiles
    Alligator,
    MonitorLizard,
    Iguana,
    Tortoise,
    SnappingTurtle,

    // Row 11-12: Farm/large animals
    Cow,
    Horse,
    Donkey,
    Pig,
    Boar,
    Camel,
    WaterBuffalo,
    Yak,

    // Row 13: Birds
    Seagull,
    BarnOwl,
    Buzzard,

    // Row 16: Poultry
    Chicken,
    Rooster,

    // Row 17: Goats/sheep
    Goat,
    MountainGoat,
    Ibex,
    Sheep,
}

impl AnimalSprite {
    pub const fn sprite_ref(self) -> SpriteRef {
        let (row, col) = match self {
            // Row 1: Bears
            Self::GrizzlyBear => (0, 0),
            Self::BlackBear => (0, 1),
            Self::PolarBear => (0, 2),
            Self::Panda => (0, 3),

            // Row 5: Cats
            Self::Cat => (4, 0),
            Self::Bobcat => (4, 1),
            Self::Cougar => (4, 2),
            Self::Cheetah => (4, 3),
            Self::Lynx => (4, 4),
            Self::Ocelot => (4, 5),
            Self::MaleLion => (4, 6),
            Self::FemaleLion => (4, 7),

            // Row 6: Canines
            Self::Dog => (5, 0),
            Self::Hyena => (5, 2),
            Self::Fox => (5, 3),
            Self::Jackal => (5, 4),
            Self::Coyote => (5, 5),
            Self::Wolf => (5, 6),

            // Row 7: Rodents
            Self::Capybara => (6, 0),
            Self::Beaver => (6, 1),

            // Row 8: Misc mammals
            Self::Badger => (7, 2),
            Self::Honeybadger => (7, 3),
            Self::Rabbit => (7, 6),
            Self::Hare => (7, 7),
            Self::Rat => (7, 8),

            // Row 9: Snakes
            Self::Snake => (8, 0),
            Self::Cobra => (8, 1),
            Self::Kingsnake => (8, 2),
            Self::BlackMamba => (8, 3),

            // Row 10: Reptiles
            Self::Alligator => (9, 0),
            Self::MonitorLizard => (9, 1),
            Self::Iguana => (9, 2),
            Self::Tortoise => (9, 3),
            Self::SnappingTurtle => (9, 4),

            // Row 11-12: Farm/large
            Self::Cow => (10, 0),
            Self::Horse => (10, 1),
            Self::Donkey => (10, 2),
            Self::Pig => (10, 6),
            Self::Boar => (10, 7),
            Self::Camel => (11, 0),
            Self::WaterBuffalo => (11, 2),
            Self::Yak => (11, 3),

            // Row 13: Birds
            Self::Seagull => (12, 0),
            Self::BarnOwl => (12, 1),
            Self::Buzzard => (12, 2),

            // Row 16: Poultry
            Self::Chicken => (15, 0),
            Self::Rooster => (15, 1),

            // Row 17: Goats/sheep
            Self::Goat => (16, 0),
            Self::MountainGoat => (16, 1),
            Self::Ibex => (16, 2),
            Self::Sheep => (16, 3),
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
            assert!(r.row < 17, "{:?} row {} >= 17", sprite, r.row);
            assert!(r.col < 9, "{:?} col {} >= 9", sprite, r.col);
        }
    }
}
