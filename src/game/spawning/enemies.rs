/// Shared enemy stat constants used by overworld and dungeon spawn tables.
/// Format: (hp, attack, defense, glyph, name, is_ranged)
pub(super) type EnemyStats = (i32, i32, i32, char, &'static str, bool);

// === Common fodder (appear across many biomes) ===
pub(super) const GIANT_RAT: EnemyStats       = (3, 1, 0, 'r', "Giant Rat", false);
pub(super) const GIANT_BAT: EnemyStats       = (4, 2, 0, 'a', "Giant Bat", false);
pub(super) const SMALL_SLIME: EnemyStats     = (4, 1, 0, 'S', "Small Slime", false);
pub(super) const GIANT_CENTIPEDE: EnemyStats = (4, 2, 0, 'e', "Giant Centipede", false);
pub(super) const VIPER: EnemyStats           = (5, 3, 0, 'n', "Viper", false);
pub(super) const GIANT_SPIDER: EnemyStats    = (6, 3, 0, 'i', "Giant Spider", false);

// === Overworld — temperate-only ===
pub(super) const FOX: EnemyStats          = (4, 2, 1, 'f', "Fox", false);
pub(super) const BUZZARD: EnemyStats      = (4, 2, 0, 'q', "Buzzard", false);
pub(super) const COYOTE: EnemyStats       = (5, 2, 1, 'y', "Coyote", false);
pub(super) const WOLF: EnemyStats         = (5, 2, 1, 'w', "Wolf", false);
pub(super) const BADGER: EnemyStats       = (5, 3, 1, 'j', "Badger", false);
pub(super) const HONEY_BADGER: EnemyStats = (6, 3, 2, 'J', "Honey Badger", false);
pub(super) const LYNX: EnemyStats         = (5, 3, 1, '#', "Lynx", false);
pub(super) const BLACK_BEAR: EnemyStats   = (9, 3, 2, '&', "Black Bear", false);
pub(super) const FOREST_SPIRIT: EnemyStats = (4, 1, 1, '2', "Forest Spirit", false);
pub(super) const BOAR: EnemyStats         = (8, 2, 2, 'b', "Boar", false);

// === Overworld — shared temperate/jungle ===
pub(super) const DRYAD: EnemyStats        = (5, 2, 0, '1', "Dryad", false);
pub(super) const COUGAR: EnemyStats       = (9, 4, 2, 'h', "Cougar", false);
pub(super) const CENTAUR: EnemyStats      = (10, 4, 2, '9', "Centaur", false);
pub(super) const BEAR: EnemyStats         = (12, 4, 2, 'B', "Bear", false);
pub(super) const LYCANTHROPE: EnemyStats  = (14, 5, 3, 'L', "Lycanthrope", false);

// === Overworld — jungle-only ===
pub(super) const BLACK_MAMBA: EnemyStats  = (5, 3, 0, 'v', "Black Mamba", false);
pub(super) const HYENA: EnemyStats        = (5, 3, 1, 'x', "Hyena", false);
pub(super) const OCELOT: EnemyStats       = (5, 2, 1, '+', "Ocelot", false);
pub(super) const JACKAL: EnemyStats       = (4, 2, 0, '-', "Jackal", false);
pub(super) const MONITOR_LIZARD: EnemyStats = (7, 3, 2, '|', "Monitor Lizard", false);
pub(super) const SATYR: EnemyStats        = (7, 3, 0, '4', "Satyr", false);
pub(super) const GIANT_ANT: EnemyStats    = (8, 3, 2, 'A', "Giant Ant", false);
pub(super) const HARPY: EnemyStats        = (8, 4, 1, '$', "Harpy", false);
pub(super) const WATER_BUFFALO: EnemyStats = (14, 4, 4, '%', "Water Buffalo", false);
pub(super) const ALLIGATOR: EnemyStats    = (10, 3, 3, 'Z', "Alligator", false);
pub(super) const COCKATRICE: EnemyStats   = (10, 5, 2, '~', "Cockatrice", false);
pub(super) const NAGA: EnemyStats         = (12, 6, 3, 'N', "Naga", false);
pub(super) const MEDUSA: EnemyStats       = (12, 7, 2, 'P', "Medusa", false);
pub(super) const MALE_LION: EnemyStats    = (16, 6, 2, 'F', "Male Lion", false);
pub(super) const MANTICORE: EnemyStats    = (18, 7, 4, 'X', "Manticore", false);
pub(super) const DIRE_WOLF: EnemyStats    = (10, 4, 2, 'U', "Dire Wolf", false);
pub(super) const WENDIGO: EnemyStats      = (12, 5, 1, '0', "Wendigo", false);
pub(super) const YAK: EnemyStats          = (12, 3, 4, '*', "Yak", false);

// === Dungeon — shallow (L0) ===
pub(super) const KOBOLD: EnemyStats        = (4, 2, 1, 'c', "Kobold", false);
pub(super) const GOBLIN: EnemyStats        = (5, 2, 1, 'g', "Goblin", false);
pub(super) const SKELETON: EnemyStats      = (6, 3, 2, 's', "Skeleton", false);
pub(super) const MYCONID: EnemyStats       = (3, 1, 1, 'p', "Myconid", false);
pub(super) const LARGE_MYCONID: EnemyStats = (4, 2, 1, 't', "Large Myconid", false);
pub(super) const GIANT_EARTHWORM: EnemyStats = (5, 2, 0, ']', "Giant Earthworm", false);
pub(super) const LESSER_GIANT_SPIDER: EnemyStats = (4, 2, 0, '<', "Lesser Giant Spider", false);
pub(super) const KOBOLD_CANINE: EnemyStats = (6, 3, 1, '{', "Kobold", false);
pub(super) const LIZARDFOLK: EnemyStats    = (6, 3, 1, '>', "Lizardfolk", false);
pub(super) const CULTIST: EnemyStats       = (6, 3, 1, '!', "Cultist", false);

// === Dungeon — mid (L1) ===
pub(super) const GOBLIN_BRUTE: EnemyStats   = (6, 3, 1, '3', "Goblin Brute", false);
pub(super) const GOBLIN_ARCHER: EnemyStats  = (6, 3, 1, 'G', "Goblin Archer", true);
pub(super) const GOBLIN_MAGE: EnemyStats    = (7, 5, 1, 'M', "Goblin Mage", true);
pub(super) const SKELETON_ARCHER: EnemyStats = (7, 4, 2, 'k', "Skeleton Archer", true);
pub(super) const WRAITH: EnemyStats         = (8, 6, 0, 'W', "Wraith", false);
pub(super) const HAG: EnemyStats            = (9, 4, 1, 'H', "Hag", false);
pub(super) const ZOMBIE: EnemyStats         = (10, 2, 1, 'z', "Zombie", false);
pub(super) const BIG_SLIME: EnemyStats      = (10, 2, 0, 'm', "Big Slime", false);
pub(super) const ORC: EnemyStats            = (10, 4, 3, 'o', "Orc", false);
pub(super) const LAMPREYMANDER: EnemyStats  = (8, 4, 1, '[', "Lampreymander", false);
pub(super) const FACELESS_MONK: EnemyStats  = (11, 5, 2, '6', "Faceless Monk", false);
pub(super) const ORC_WIZARD: EnemyStats     = (10, 6, 2, '}', "Orc Wizard", true);

// === Dungeon — deep (L2) ===
pub(super) const GHOUL: EnemyStats          = (10, 5, 2, 'u', "Ghoul", false);
pub(super) const BANSHEE: EnemyStats        = (10, 6, 1, 'Q', "Banshee", false);
pub(super) const SM_WRITHING_MASS: EnemyStats = (10, 5, 2, '(', "Small Writhing Mass", false);
pub(super) const WRITHING_HUMANOID: EnemyStats = (12, 6, 1, ')', "Writhing Humanoid", false);
pub(super) const ORC_WARCHIEF: EnemyStats   = (12, 5, 4, '5', "Orc Warchief", false);
pub(super) const ORC_BLADEMASTER: EnemyStats = (14, 5, 4, 'O', "Orc Blademaster", false);
pub(super) const UNHOLY_CARDINAL: EnemyStats = (14, 7, 3, '7', "Unholy Cardinal", false);
pub(super) const LG_WRITHING_MASS: EnemyStats = (15, 6, 2, '8', "Writhing Mass", false);
pub(super) const TROLL: EnemyStats          = (16, 5, 3, 'T', "Troll", false);
pub(super) const ETTIN: EnemyStats          = (18, 6, 4, 'E', "Ettin", false);
pub(super) const TWO_HEADED_ETTIN: EnemyStats = (22, 7, 5, '^', "Two-Headed Ettin", false);

// === Cave / boss-tier ===
pub(super) const IMP: EnemyStats          = (10, 6, 1, 'I', "Imp", false);
pub(super) const DRAKE: EnemyStats        = (14, 6, 3, 'd', "Drake", false);
pub(super) const LICH: EnemyStats         = (15, 8, 2, 'l', "Lich", false);
pub(super) const BASILISK: EnemyStats     = (16, 7, 4, 'C', "Basilisk", false);
pub(super) const DEATH_KNIGHT: EnemyStats = (20, 7, 5, 'K', "Death Knight", false);
pub(super) const REAPER: EnemyStats       = (20, 9, 3, 'V', "Reaper", false);
