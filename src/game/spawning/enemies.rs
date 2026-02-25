use crate::config::EnemyBehavior;
use EnemyBehavior::{Passive as P, Timid as Ti, Territorial as Te, Aggressive as A, Stalker as S};

/// Shared enemy stat constants used by overworld and dungeon spawn tables.
/// Format: (hp, attack, defense, glyph, name, is_ranged, behavior)
pub(super) type EnemyStats = (i32, i32, i32, char, &'static str, bool, EnemyBehavior);

// === Common fodder (appear across many biomes) ===
pub(super) const GIANT_RAT: EnemyStats       = (4, 2, 0, 'r', "Giant Rat", false, Ti);
pub(super) const GIANT_BAT: EnemyStats       = (5, 3, 0, 'a', "Giant Bat", false, Ti);
pub(super) const SMALL_SLIME: EnemyStats     = (5, 2, 0, 'S', "Small Slime", false, P);
pub(super) const GIANT_CENTIPEDE: EnemyStats = (5, 3, 0, 'e', "Giant Centipede", false, Ti);
pub(super) const VIPER: EnemyStats           = (6, 4, 0, 'n', "Viper", false, S);
pub(super) const GIANT_SPIDER: EnemyStats    = (7, 4, 1, 'i', "Giant Spider", false, A);

// === Overworld — temperate-only ===
pub(super) const FOX: EnemyStats          = (5, 3, 2, 'f', "Fox", false, Ti);
pub(super) const BUZZARD: EnemyStats      = (5, 3, 0, 'q', "Buzzard", false, P);
pub(super) const COYOTE: EnemyStats       = (6, 3, 2, 'y', "Coyote", false, Ti);
pub(super) const WOLF: EnemyStats         = (6, 3, 2, 'w', "Wolf", false, Te);
pub(super) const BADGER: EnemyStats       = (6, 4, 2, 'j', "Badger", false, Ti);
pub(super) const HONEY_BADGER: EnemyStats = (7, 4, 3, 'J', "Honey Badger", false, Te);
pub(super) const LYNX: EnemyStats         = (6, 4, 2, '#', "Lynx", false, S);
pub(super) const BLACK_BEAR: EnemyStats   = (11, 4, 3, '&', "Black Bear", false, Te);
pub(super) const BOAR: EnemyStats         = (10, 3, 3, 'b', "Boar", false, Te);

// === Overworld — shared temperate/jungle ===
pub(super) const COUGAR: EnemyStats       = (11, 5, 3, 'h', "Cougar", false, S);
pub(super) const BEAR: EnemyStats         = (14, 5, 3, 'B', "Bear", false, Te);

// === Overworld — rare monsters (mini-boss encounters) ===
pub(super) const DRYAD: EnemyStats        = (22, 6, 4, '1', "Dryad", false, Te);
pub(super) const FOREST_SPIRIT: EnemyStats = (19, 7, 3, '2', "Forest Spirit", false, Te);
pub(super) const CENTAUR: EnemyStats      = (26, 8, 5, '9', "Centaur", false, A);
pub(super) const LYCANTHROPE: EnemyStats  = (34, 10, 5, 'L', "Lycanthrope", false, A);
pub(super) const DIRE_WOLF: EnemyStats    = (24, 7, 4, 'U', "Dire Wolf", false, A);
pub(super) const WENDIGO: EnemyStats      = (36, 11, 4, '0', "Wendigo", false, A);

// === Dungeon-only (tropical/exotic — used in dungeon biome tables) ===
pub(super) const BLACK_MAMBA: EnemyStats  = (6, 4, 0, 'v', "Black Mamba", false, S);
pub(super) const MONITOR_LIZARD: EnemyStats = (8, 4, 3, '|', "Monitor Lizard", false, Te);
pub(super) const GIANT_ANT: EnemyStats    = (10, 4, 3, 'A', "Giant Ant", false, A);
pub(super) const WATER_BUFFALO: EnemyStats = (17, 5, 5, '%', "Water Buffalo", false, Te);
pub(super) const COCKATRICE: EnemyStats   = (12, 6, 3, '~', "Cockatrice", false, A);
pub(super) const NAGA: EnemyStats         = (14, 7, 4, 'N', "Naga", false, S);
pub(super) const MEDUSA: EnemyStats       = (14, 8, 3, 'P', "Medusa", false, S);
pub(super) const MANTICORE: EnemyStats    = (22, 8, 5, 'X', "Manticore", false, A);
pub(super) const YAK: EnemyStats          = (14, 4, 5, '*', "Yak", false, Te);

// === Dungeon — shallow (L0) ===
pub(super) const KOBOLD: EnemyStats        = (5, 3, 2, 'c', "Kobold", false, Ti);
pub(super) const GOBLIN: EnemyStats        = (6, 3, 2, 'g', "Goblin", false, A);
pub(super) const SKELETON: EnemyStats      = (7, 4, 3, 's', "Skeleton", false, A);
pub(super) const MYCONID: EnemyStats       = (4, 2, 2, 'p', "Myconid", false, P);
pub(super) const LARGE_MYCONID: EnemyStats = (5, 3, 2, 't', "Large Myconid", false, P);
pub(super) const GIANT_EARTHWORM: EnemyStats = (6, 3, 0, ']', "Giant Earthworm", false, Ti);
pub(super) const LESSER_GIANT_SPIDER: EnemyStats = (5, 3, 0, '<', "Lesser Giant Spider", false, Ti);
pub(super) const KOBOLD_CANINE: EnemyStats = (7, 4, 2, '{', "Kobold", false, Ti);
pub(super) const LIZARDFOLK: EnemyStats    = (7, 4, 2, '>', "Lizardfolk", false, A);
pub(super) const CULTIST: EnemyStats       = (7, 4, 2, '!', "Cultist", false, A);

// === Dungeon — mid (L1) ===
pub(super) const GOBLIN_BRUTE: EnemyStats   = (14, 6, 3, '3', "Goblin Brute", false, A);
pub(super) const GOBLIN_ARCHER: EnemyStats  = (12, 6, 3, 'G', "Goblin Archer", true, A);
pub(super) const GOBLIN_MAGE: EnemyStats    = (14, 8, 3, 'M', "Goblin Mage", true, A);
pub(super) const SKELETON_ARCHER: EnemyStats = (14, 7, 4, 'k', "Skeleton Archer", true, A);
pub(super) const WRAITH: EnemyStats         = (17, 11, 2, 'W', "Wraith", false, S);
pub(super) const HAG: EnemyStats            = (17, 7, 3, 'H', "Hag", false, A);
pub(super) const ZOMBIE: EnemyStats         = (19, 5, 3, 'z', "Zombie", false, A);
pub(super) const BIG_SLIME: EnemyStats      = (19, 5, 2, 'm', "Big Slime", false, P);
pub(super) const ORC: EnemyStats            = (19, 8, 5, 'o', "Orc", false, A);
pub(super) const LAMPREYMANDER: EnemyStats  = (16, 7, 3, '[', "Lampreymander", false, A);
pub(super) const FACELESS_MONK: EnemyStats  = (20, 8, 4, '6', "Faceless Monk", false, S);
pub(super) const ORC_WIZARD: EnemyStats     = (18, 10, 4, '}', "Orc Wizard", true, A);

// === Dungeon — deep (L2) ===
pub(super) const GHOUL: EnemyStats          = (24, 11, 5, 'u', "Ghoul", false, A);
pub(super) const BANSHEE: EnemyStats        = (24, 12, 4, 'Q', "Banshee", false, A);
pub(super) const SM_WRITHING_MASS: EnemyStats = (24, 10, 5, '(', "Small Writhing Mass", false, A);
pub(super) const WRITHING_HUMANOID: EnemyStats = (26, 12, 4, ')', "Writhing Humanoid", false, A);
pub(super) const ORC_WARCHIEF: EnemyStats   = (29, 11, 7, '5', "Orc Warchief", false, A);
pub(super) const ORC_BLADEMASTER: EnemyStats = (31, 11, 7, 'O', "Orc Blademaster", false, A);
pub(super) const UNHOLY_CARDINAL: EnemyStats = (30, 13, 6, '7', "Unholy Cardinal", false, A);
pub(super) const LG_WRITHING_MASS: EnemyStats = (34, 12, 5, '8', "Writhing Mass", false, A);
pub(super) const TROLL: EnemyStats          = (36, 11, 6, 'T', "Troll", false, A);
pub(super) const ETTIN: EnemyStats          = (38, 12, 8, 'E', "Ettin", false, A);
pub(super) const TWO_HEADED_ETTIN: EnemyStats = (46, 14, 8, '^', "Two-Headed Ettin", false, A);

// === Cave / boss-tier ===
pub(super) const IMP: EnemyStats          = (26, 12, 4, 'I', "Imp", false, A);
pub(super) const DRAKE: EnemyStats        = (34, 13, 6, 'd', "Drake", false, A);
pub(super) const LICH: EnemyStats         = (36, 16, 6, 'l', "Lich", false, A);
pub(super) const BASILISK: EnemyStats     = (36, 14, 7, 'C', "Basilisk", false, A);
pub(super) const DEATH_KNIGHT: EnemyStats = (43, 14, 8, 'K', "Death Knight", false, A);
pub(super) const REAPER: EnemyStats       = (43, 17, 7, 'V', "Reaper", false, A);
