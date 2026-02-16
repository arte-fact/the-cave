/// Every named sprite in `animals.png` (32x32 cells).
/// Positions from `32rogues/animals.txt`.
use super::{Sheet, SpriteRef};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AnimalSprite {
    // Row 0 (1) — bears
    GrizzlyBear,
    BlackBear,
    PolarBear,
    Panda,

    // Row 1 (2) — great apes
    Chimpanzee,
    Gorilla,

    // Row 2 (3)
    /// Listed as 3.c in animals.txt (row 2, col 2).
    Orangutan,

    // Row 3 (4) — small primates
    AyeAye,
    Gibbon,
    Mandrill,
    Capuchin,
    Langur,

    // Row 4 (5) — cats
    Cat,
    Bobcat,
    Cougar,
    Cheetah,
    Lynx,
    Ocelot,
    MaleLion,
    FemaleLion,

    // Row 5 (6) — canines
    Dog,
    Puppy,
    Hyena,
    Fox,
    Jackal,
    Coyote,
    Wolf,

    // Row 6 (7) — rodents and small mammals
    Capybara,
    Beaver,
    Mink,
    Mongoose,
    Marmot,
    Groundhog,
    Chinchilla,
    Echidna,

    // Row 7 (8) — burrowers and small mammals
    Aardvark,
    Armadillo,
    Badger,
    HoneyBadger,
    Coati,
    Opossum,
    Rabbit,
    Hare,
    Rat,

    // Row 8 (9) — snakes
    Snake,
    Cobra,
    Kingsnake,
    BlackMamba,

    // Row 9 (10) — reptiles
    Alligator,
    MonitorLizard,
    Iguana,
    Tortoise,
    SnappingTurtle,
    AlligatorSnappingTurtle,

    // Row 10 (11) — livestock
    Cow,
    Horse,
    Donkey,
    Mule,
    Alpaca,
    Llama,
    Pig,
    Boar,

    // Row 11 (12) — large herbivores
    Camel,
    ReindeerCaribou,
    WaterBuffalo,
    Yak,

    // Row 12 (13) — birds of prey
    Seagull,
    BarnOwl,
    CommonBuzzard,

    // Row 13 (14) — marsupials
    Kangaroo,
    Koala,

    // Row 14 (15) — flightless birds
    Penguin,
    LittlePenguin,
    Cassowary,
    Emu,

    // Row 15 (16) — poultry and waterfowl
    Chicken,
    Rooster,
    MallardDuck,
    Swan,
    Turkey,
    Guineafowl,
    Peacock,

    // Row 16 (17) — goats and sheep
    Goat,
    MountainGoat,
    Ibex,
    SheepRam,
    /// Listed as 16.e in animals.txt (likely a typo for 17.e).
    SheepEwe,
}

impl AnimalSprite {
    pub const fn sprite_ref(self) -> SpriteRef {
        let (row, col) = match self {
            Self::GrizzlyBear => (0, 0),
            Self::BlackBear => (0, 1),
            Self::PolarBear => (0, 2),
            Self::Panda => (0, 3),

            Self::Chimpanzee => (1, 0),
            Self::Gorilla => (1, 1),

            // 3.c in txt → row 2, col 2
            Self::Orangutan => (2, 2),

            Self::AyeAye => (3, 0),
            Self::Gibbon => (3, 1),
            Self::Mandrill => (3, 2),
            Self::Capuchin => (3, 3),
            Self::Langur => (3, 4),

            Self::Cat => (4, 0),
            Self::Bobcat => (4, 1),
            Self::Cougar => (4, 2),
            Self::Cheetah => (4, 3),
            Self::Lynx => (4, 4),
            Self::Ocelot => (4, 5),
            Self::MaleLion => (4, 6),
            Self::FemaleLion => (4, 7),

            Self::Dog => (5, 0),
            Self::Puppy => (5, 1),
            Self::Hyena => (5, 2),
            Self::Fox => (5, 3),
            Self::Jackal => (5, 4),
            Self::Coyote => (5, 5),
            Self::Wolf => (5, 6),

            Self::Capybara => (6, 0),
            Self::Beaver => (6, 1),
            Self::Mink => (6, 2),
            Self::Mongoose => (6, 3),
            Self::Marmot => (6, 4),
            Self::Groundhog => (6, 5),
            Self::Chinchilla => (6, 6),
            Self::Echidna => (6, 7),

            Self::Aardvark => (7, 0),
            Self::Armadillo => (7, 1),
            Self::Badger => (7, 2),
            Self::HoneyBadger => (7, 3),
            Self::Coati => (7, 4),
            Self::Opossum => (7, 5),
            Self::Rabbit => (7, 6),
            Self::Hare => (7, 7),
            Self::Rat => (7, 8),

            Self::Snake => (8, 0),
            Self::Cobra => (8, 1),
            Self::Kingsnake => (8, 2),
            Self::BlackMamba => (8, 3),

            Self::Alligator => (9, 0),
            Self::MonitorLizard => (9, 1),
            Self::Iguana => (9, 2),
            Self::Tortoise => (9, 3),
            Self::SnappingTurtle => (9, 4),
            Self::AlligatorSnappingTurtle => (9, 5),

            Self::Cow => (10, 0),
            Self::Horse => (10, 1),
            Self::Donkey => (10, 2),
            Self::Mule => (10, 3),
            Self::Alpaca => (10, 4),
            Self::Llama => (10, 5),
            Self::Pig => (10, 6),
            Self::Boar => (10, 7),

            Self::Camel => (11, 0),
            Self::ReindeerCaribou => (11, 1),
            Self::WaterBuffalo => (11, 2),
            Self::Yak => (11, 3),

            Self::Seagull => (12, 0),
            Self::BarnOwl => (12, 1),
            Self::CommonBuzzard => (12, 2),

            Self::Kangaroo => (13, 0),
            Self::Koala => (13, 1),

            Self::Penguin => (14, 0),
            Self::LittlePenguin => (14, 1),
            Self::Cassowary => (14, 2),
            Self::Emu => (14, 3),

            Self::Chicken => (15, 0),
            Self::Rooster => (15, 1),
            Self::MallardDuck => (15, 2),
            Self::Swan => (15, 3),
            Self::Turkey => (15, 4),
            Self::Guineafowl => (15, 5),
            Self::Peacock => (15, 6),

            Self::Goat => (16, 0),
            Self::MountainGoat => (16, 1),
            Self::Ibex => (16, 2),
            Self::SheepRam => (16, 3),
            // txt says 16.e (row 15, col 4) — likely typo for 17.e (row 16, col 4)
            Self::SheepEwe => (16, 4),
        };
        SpriteRef::new(Sheet::Animals, row, col)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_animal_sprites_within_sheet_bounds() {
        // animals.png: at least 9 cols x 17 rows
        let all = [
            AnimalSprite::GrizzlyBear, AnimalSprite::BlackBear, AnimalSprite::PolarBear, AnimalSprite::Panda,
            AnimalSprite::Chimpanzee, AnimalSprite::Gorilla,
            AnimalSprite::Orangutan,
            AnimalSprite::AyeAye, AnimalSprite::Gibbon, AnimalSprite::Mandrill, AnimalSprite::Capuchin, AnimalSprite::Langur,
            AnimalSprite::Cat, AnimalSprite::Bobcat, AnimalSprite::Cougar, AnimalSprite::Cheetah,
            AnimalSprite::Lynx, AnimalSprite::Ocelot, AnimalSprite::MaleLion, AnimalSprite::FemaleLion,
            AnimalSprite::Dog, AnimalSprite::Puppy, AnimalSprite::Hyena, AnimalSprite::Fox,
            AnimalSprite::Jackal, AnimalSprite::Coyote, AnimalSprite::Wolf,
            AnimalSprite::Capybara, AnimalSprite::Beaver, AnimalSprite::Mink, AnimalSprite::Mongoose,
            AnimalSprite::Marmot, AnimalSprite::Groundhog, AnimalSprite::Chinchilla, AnimalSprite::Echidna,
            AnimalSprite::Aardvark, AnimalSprite::Armadillo, AnimalSprite::Badger, AnimalSprite::HoneyBadger,
            AnimalSprite::Coati, AnimalSprite::Opossum, AnimalSprite::Rabbit, AnimalSprite::Hare, AnimalSprite::Rat,
            AnimalSprite::Snake, AnimalSprite::Cobra, AnimalSprite::Kingsnake, AnimalSprite::BlackMamba,
            AnimalSprite::Alligator, AnimalSprite::MonitorLizard, AnimalSprite::Iguana,
            AnimalSprite::Tortoise, AnimalSprite::SnappingTurtle, AnimalSprite::AlligatorSnappingTurtle,
            AnimalSprite::Cow, AnimalSprite::Horse, AnimalSprite::Donkey, AnimalSprite::Mule,
            AnimalSprite::Alpaca, AnimalSprite::Llama, AnimalSprite::Pig, AnimalSprite::Boar,
            AnimalSprite::Camel, AnimalSprite::ReindeerCaribou, AnimalSprite::WaterBuffalo, AnimalSprite::Yak,
            AnimalSprite::Seagull, AnimalSprite::BarnOwl, AnimalSprite::CommonBuzzard,
            AnimalSprite::Kangaroo, AnimalSprite::Koala,
            AnimalSprite::Penguin, AnimalSprite::LittlePenguin, AnimalSprite::Cassowary, AnimalSprite::Emu,
            AnimalSprite::Chicken, AnimalSprite::Rooster, AnimalSprite::MallardDuck, AnimalSprite::Swan,
            AnimalSprite::Turkey, AnimalSprite::Guineafowl, AnimalSprite::Peacock,
            AnimalSprite::Goat, AnimalSprite::MountainGoat, AnimalSprite::Ibex,
            AnimalSprite::SheepRam, AnimalSprite::SheepEwe,
        ];
        for sprite in all {
            let r = sprite.sprite_ref();
            assert_eq!(r.sheet, Sheet::Animals);
            assert!(r.row < 17, "{:?} row {} >= 17", sprite, r.row);
            assert!(r.col < 9, "{:?} col {} >= 9", sprite, r.col);
        }
    }

    #[test]
    fn wolf_position() {
        let r = AnimalSprite::Wolf.sprite_ref();
        assert_eq!(r.row, 5);
        assert_eq!(r.col, 6);
    }
}
