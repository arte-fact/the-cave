use crate::config::EnemyBehavior;
use EnemyBehavior::{Passive as P, Timid as Ti, Territorial as Te, Aggressive as A, Stalker as S};

/// Shared enemy stat constants used by overworld and dungeon spawn tables.
/// Format: (hp, attack, defense, glyph, name, is_ranged, behavior)
pub(super) type EnemyStats = (i32, i32, i32, char, &'static str, bool, EnemyBehavior);

// === Common fodder (appear across many biomes) ===
pub(super) const GIANT_RAT: EnemyStats       = (3, 1, 0, 'r', "Giant Rat", false, Ti);
pub(super) const GIANT_BAT: EnemyStats       = (4, 2, 0, 'a', "Giant Bat", false, Ti);
pub(super) const SMALL_SLIME: EnemyStats     = (4, 1, 0, 'S', "Small Slime", false, P);
pub(super) const GIANT_CENTIPEDE: EnemyStats = (4, 2, 0, 'e', "Giant Centipede", false, Ti);
pub(super) const VIPER: EnemyStats           = (5, 3, 0, 'n', "Viper", false, S);
pub(super) const GIANT_SPIDER: EnemyStats    = (6, 3, 0, 'i', "Giant Spider", false, A);

// === Overworld — temperate-only ===
pub(super) const FOX: EnemyStats          = (4, 2, 1, 'f', "Fox", false, Ti);
pub(super) const BUZZARD: EnemyStats      = (4, 2, 0, 'q', "Buzzard", false, P);
pub(super) const COYOTE: EnemyStats       = (5, 2, 1, 'y', "Coyote", false, Ti);
pub(super) const WOLF: EnemyStats         = (5, 2, 1, 'w', "Wolf", false, Te);
pub(super) const BADGER: EnemyStats       = (5, 3, 1, 'j', "Badger", false, Ti);
pub(super) const HONEY_BADGER: EnemyStats = (6, 3, 2, 'J', "Honey Badger", false, Te);
pub(super) const LYNX: EnemyStats         = (5, 3, 1, '#', "Lynx", false, S);
pub(super) const BLACK_BEAR: EnemyStats   = (9, 3, 2, '&', "Black Bear", false, Te);
pub(super) const BOAR: EnemyStats         = (8, 2, 2, 'b', "Boar", false, Te);

// === Overworld — shared temperate/jungle ===
pub(super) const COUGAR: EnemyStats       = (9, 4, 2, 'h', "Cougar", false, S);
pub(super) const BEAR: EnemyStats         = (12, 4, 2, 'B', "Bear", false, Te);

// === Overworld — rare monsters (mini-boss encounters) ===
pub(super) const DRYAD: EnemyStats        = (18, 5, 3, '1', "Dryad", false, Te);
pub(super) const FOREST_SPIRIT: EnemyStats = (16, 6, 2, '2', "Forest Spirit", false, Te);
pub(super) const CENTAUR: EnemyStats      = (22, 7, 4, '9', "Centaur", false, A);
pub(super) const LYCANTHROPE: EnemyStats  = (28, 8, 4, 'L', "Lycanthrope", false, A);
pub(super) const DIRE_WOLF: EnemyStats    = (20, 6, 3, 'U', "Dire Wolf", false, A);
pub(super) const WENDIGO: EnemyStats      = (30, 9, 3, '0', "Wendigo", false, A);

// === Dungeon-only (tropical/exotic — used in dungeon biome tables) ===
pub(super) const BLACK_MAMBA: EnemyStats  = (5, 3, 0, 'v', "Black Mamba", false, S);
pub(super) const MONITOR_LIZARD: EnemyStats = (7, 3, 2, '|', "Monitor Lizard", false, Te);
pub(super) const GIANT_ANT: EnemyStats    = (8, 3, 2, 'A', "Giant Ant", false, A);
pub(super) const WATER_BUFFALO: EnemyStats = (14, 4, 4, '%', "Water Buffalo", false, Te);
pub(super) const COCKATRICE: EnemyStats   = (10, 5, 2, '~', "Cockatrice", false, A);
pub(super) const NAGA: EnemyStats         = (12, 6, 3, 'N', "Naga", false, S);
pub(super) const MEDUSA: EnemyStats       = (12, 7, 2, 'P', "Medusa", false, S);
pub(super) const MANTICORE: EnemyStats    = (18, 7, 4, 'X', "Manticore", false, A);
pub(super) const YAK: EnemyStats          = (12, 3, 4, '*', "Yak", false, Te);

// === Dungeon — shallow (L0) ===
pub(super) const KOBOLD: EnemyStats        = (4, 2, 1, 'c', "Kobold", false, Ti);
pub(super) const GOBLIN: EnemyStats        = (5, 2, 1, 'g', "Goblin", false, A);
pub(super) const SKELETON: EnemyStats      = (6, 3, 2, 's', "Skeleton", false, A);
pub(super) const MYCONID: EnemyStats       = (3, 1, 1, 'p', "Myconid", false, P);
pub(super) const LARGE_MYCONID: EnemyStats = (4, 2, 1, 't', "Large Myconid", false, P);
pub(super) const GIANT_EARTHWORM: EnemyStats = (5, 2, 0, ']', "Giant Earthworm", false, Ti);
pub(super) const LESSER_GIANT_SPIDER: EnemyStats = (4, 2, 0, '<', "Lesser Giant Spider", false, Ti);
pub(super) const KOBOLD_CANINE: EnemyStats = (6, 3, 1, '{', "Kobold", false, Ti);
pub(super) const LIZARDFOLK: EnemyStats    = (6, 3, 1, '>', "Lizardfolk", false, A);
pub(super) const CULTIST: EnemyStats       = (6, 3, 1, '!', "Cultist", false, A);

// === Dungeon — mid (L1) ===
pub(super) const GOBLIN_BRUTE: EnemyStats   = (6, 3, 1, '3', "Goblin Brute", false, A);
pub(super) const GOBLIN_ARCHER: EnemyStats  = (6, 3, 1, 'G', "Goblin Archer", true, A);
pub(super) const GOBLIN_MAGE: EnemyStats    = (7, 5, 1, 'M', "Goblin Mage", true, A);
pub(super) const SKELETON_ARCHER: EnemyStats = (7, 4, 2, 'k', "Skeleton Archer", true, A);
pub(super) const WRAITH: EnemyStats         = (8, 6, 0, 'W', "Wraith", false, S);
pub(super) const HAG: EnemyStats            = (9, 4, 1, 'H', "Hag", false, A);
pub(super) const ZOMBIE: EnemyStats         = (10, 2, 1, 'z', "Zombie", false, A);
pub(super) const BIG_SLIME: EnemyStats      = (10, 2, 0, 'm', "Big Slime", false, P);
pub(super) const ORC: EnemyStats            = (10, 4, 3, 'o', "Orc", false, A);
pub(super) const LAMPREYMANDER: EnemyStats  = (8, 4, 1, '[', "Lampreymander", false, A);
pub(super) const FACELESS_MONK: EnemyStats  = (11, 5, 2, '6', "Faceless Monk", false, S);
pub(super) const ORC_WIZARD: EnemyStats     = (10, 6, 2, '}', "Orc Wizard", true, A);

// === Dungeon — deep (L2) ===
pub(super) const GHOUL: EnemyStats          = (10, 5, 2, 'u', "Ghoul", false, A);
pub(super) const BANSHEE: EnemyStats        = (10, 6, 1, 'Q', "Banshee", false, A);
pub(super) const SM_WRITHING_MASS: EnemyStats = (10, 5, 2, '(', "Small Writhing Mass", false, A);
pub(super) const WRITHING_HUMANOID: EnemyStats = (12, 6, 1, ')', "Writhing Humanoid", false, A);
pub(super) const ORC_WARCHIEF: EnemyStats   = (12, 5, 4, '5', "Orc Warchief", false, A);
pub(super) const ORC_BLADEMASTER: EnemyStats = (14, 5, 4, 'O', "Orc Blademaster", false, A);
pub(super) const UNHOLY_CARDINAL: EnemyStats = (14, 7, 3, '7', "Unholy Cardinal", false, A);
pub(super) const LG_WRITHING_MASS: EnemyStats = (15, 6, 2, '8', "Writhing Mass", false, A);
pub(super) const TROLL: EnemyStats          = (16, 5, 3, 'T', "Troll", false, A);
pub(super) const ETTIN: EnemyStats          = (18, 6, 4, 'E', "Ettin", false, A);
pub(super) const TWO_HEADED_ETTIN: EnemyStats = (22, 7, 5, '^', "Two-Headed Ettin", false, A);

// === Cave / boss-tier ===
pub(super) const IMP: EnemyStats          = (10, 6, 1, 'I', "Imp", false, A);
pub(super) const DRAKE: EnemyStats        = (14, 6, 3, 'd', "Drake", false, A);
pub(super) const LICH: EnemyStats         = (15, 8, 2, 'l', "Lich", false, A);
pub(super) const BASILISK: EnemyStats     = (16, 7, 4, 'C', "Basilisk", false, A);
pub(super) const DEATH_KNIGHT: EnemyStats = (20, 7, 5, 'K', "Death Knight", false, A);
pub(super) const REAPER: EnemyStats       = (20, 9, 3, 'V', "Reaper", false, A);
