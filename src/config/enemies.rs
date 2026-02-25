/// Centralized enemy definitions. Every spawnable enemy's base stats,
/// XP reward, and inspect-panel description live here.

/// AI behavior type for enemies.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EnemyBehavior {
    /// Won't attack. Flees when provoked.
    Passive,
    /// Flees from player. Fights when cornered (adjacent + provoked).
    Timid,
    /// Ignores until close. Leashed to spawn. Returns home when far.
    Territorial,
    /// Standard chase-and-attack behavior.
    Aggressive,
    /// Motionless until player enters range. Then relentless extended chase.
    Stalker,
}

/// Complete definition of an enemy type.
#[derive(Clone, Debug)]
pub struct EnemyDef {
    pub name: &'static str,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub glyph: char,
    pub is_ranged: bool,
    pub xp: u32,
    pub description: &'static str,
    pub behavior: EnemyBehavior,
}

/// Look up an enemy definition by name.
pub fn enemy_def(name: &str) -> Option<&'static EnemyDef> {
    ENEMY_DEFS.iter().find(|d| d.name == name)
}

/// Look up the behavior for an enemy by name. Falls back to Aggressive.
pub fn enemy_behavior(name: &str) -> EnemyBehavior {
    enemy_def(name).map_or(EnemyBehavior::Aggressive, |d| d.behavior)
}

/// XP reward for killing an enemy. Falls back to 3 for unknown names.
pub fn xp_for_enemy(name: &str) -> u32 {
    enemy_def(name).map_or(3, |d| d.xp)
}

/// Flavour description shown in the inspect panel.
pub fn enemy_description(name: &str) -> &'static str {
    enemy_def(name).map_or("A mysterious creature.", |d| d.description)
}

// Shorthand aliases for behavior variants used in ENEMY_DEFS below.
use EnemyBehavior::{Passive as P, Timid as Ti, Territorial as Te, Aggressive as A, Stalker as S};

/// Complete enemy registry. Sorted roughly by encounter order
/// (overworld → dungeon shallow → mid → deep → cave/boss).
pub static ENEMY_DEFS: &[EnemyDef] = &[
    // ── Overworld — common wildlife ──────────────────────────────────
    EnemyDef { name: "Fox",             hp: 5,  attack: 3, defense: 2, glyph: 'f', is_ranged: false, xp: 4,  behavior: Ti, description: "A quick and sly predator. Hard to pin down." },
    EnemyDef { name: "Buzzard",         hp: 5,  attack: 3, defense: 0, glyph: 'q', is_ranged: false, xp: 4,  behavior: P,  description: "A circling scavenger. Swoops down on the weak." },
    EnemyDef { name: "Giant Rat",       hp: 4,  attack: 2, defense: 0, glyph: 'r', is_ranged: false, xp: 3,  behavior: Ti, description: "A disease-carrying rodent the size of a dog." },
    EnemyDef { name: "Giant Bat",       hp: 5,  attack: 3, defense: 0, glyph: 'a', is_ranged: false, xp: 4,  behavior: Ti, description: "A bat with a wingspan wider than a man." },
    EnemyDef { name: "Small Slime",     hp: 5,  attack: 2, defense: 0, glyph: 'S', is_ranged: false, xp: 3,  behavior: P,  description: "A translucent ooze. Dissolves what it touches." },
    EnemyDef { name: "Giant Centipede", hp: 5,  attack: 3, defense: 0, glyph: 'e', is_ranged: false, xp: 4,  behavior: Ti, description: "A writhing mass of legs and mandibles." },
    EnemyDef { name: "Viper",           hp: 6,  attack: 4, defense: 0, glyph: 'n', is_ranged: false, xp: 5,  behavior: S,  description: "A venomous snake. Strikes without warning." },
    EnemyDef { name: "Coyote",          hp: 6,  attack: 3, defense: 2, glyph: 'y', is_ranged: false, xp: 5,  behavior: Ti, description: "A crafty opportunist. Hunts in pairs." },
    EnemyDef { name: "Wolf",            hp: 6,  attack: 3, defense: 2, glyph: 'w', is_ranged: false, xp: 5,  behavior: Te, description: "A cunning pack hunter. Fast and relentless." },
    EnemyDef { name: "Badger",          hp: 6,  attack: 4, defense: 2, glyph: 'j', is_ranged: false, xp: 5,  behavior: Ti, description: "Small but ferocious. Fights to the death." },
    EnemyDef { name: "Giant Spider",    hp: 7,  attack: 4, defense: 1, glyph: 'i', is_ranged: false, xp: 6,  behavior: A,  description: "A venomous arachnid that lurks in the shadows." },
    EnemyDef { name: "Honey Badger",    hp: 7,  attack: 4, defense: 3, glyph: 'J', is_ranged: false, xp: 7,  behavior: Te, description: "Fearless and relentless. Never backs down." },
    EnemyDef { name: "Lynx",            hp: 6,  attack: 4, defense: 2, glyph: '#', is_ranged: false, xp: 5,  behavior: S,  description: "A ghost of the forest. Strikes from ambush." },
    EnemyDef { name: "Boar",            hp: 10, attack: 3, defense: 3, glyph: 'b', is_ranged: false, xp: 7,  behavior: Te, description: "A ferocious wild pig with razor-sharp tusks." },
    EnemyDef { name: "Black Bear",      hp: 11, attack: 4, defense: 3, glyph: '&', is_ranged: false, xp: 8,  behavior: Te, description: "A powerful woodland bear. Protective of its territory." },
    EnemyDef { name: "Cougar",          hp: 11, attack: 5, defense: 3, glyph: 'h', is_ranged: false, xp: 8,  behavior: S,  description: "A stealthy big cat. Silent and deadly." },
    EnemyDef { name: "Bear",            hp: 14, attack: 5, defense: 3, glyph: 'B', is_ranged: false, xp: 12, behavior: Te, description: "A massive predator. Top of the forest chain." },

    // ── Overworld — rare monsters (mini-boss) ───────────────────────
    EnemyDef { name: "Dryad",           hp: 22, attack: 6,  defense: 4, glyph: '1', is_ranged: false, xp: 20, behavior: Te, description: "A woodland spirit. Protects the ancient trees." },
    EnemyDef { name: "Forest Spirit",   hp: 19, attack: 7,  defense: 3, glyph: '2', is_ranged: false, xp: 18, behavior: Te, description: "An ethereal guardian of the deep woods." },
    EnemyDef { name: "Centaur",         hp: 26, attack: 8,  defense: 5, glyph: '9', is_ranged: false, xp: 25, behavior: A,  description: "Half-man, half-horse. Patrols the forest trails." },
    EnemyDef { name: "Dire Wolf",       hp: 24, attack: 7,  defense: 4, glyph: 'U', is_ranged: false, xp: 22, behavior: A,  description: "An enormous wolf. Pack leader and apex predator." },
    EnemyDef { name: "Lycanthrope",     hp: 34, attack: 10, defense: 5, glyph: 'L', is_ranged: false, xp: 35, behavior: A,  description: "A cursed shapeshifter. Savage in beast form." },
    EnemyDef { name: "Wendigo",         hp: 36, attack: 11, defense: 4, glyph: '0', is_ranged: false, xp: 40, behavior: A,  description: "A gaunt horror of the frozen woods. Insatiable hunger." },

    // ── Dungeon — exotic fauna (used across biomes) ─────────────────
    EnemyDef { name: "Black Mamba",     hp: 6,  attack: 4, defense: 0, glyph: 'v', is_ranged: false, xp: 6,  behavior: S,  description: "The deadliest snake. Lightning-fast venom." },
    EnemyDef { name: "Monitor Lizard",  hp: 8,  attack: 4, defense: 3, glyph: '|', is_ranged: false, xp: 7,  behavior: Te, description: "A large tropical reptile with a venomous bite." },
    EnemyDef { name: "Giant Ant",       hp: 10, attack: 4, defense: 3, glyph: 'A', is_ranged: false, xp: 6,  behavior: A,  description: "An oversized insect with crushing mandibles." },
    EnemyDef { name: "Water Buffalo",   hp: 17, attack: 5, defense: 5, glyph: '%', is_ranged: false, xp: 17, behavior: Te, description: "A massive beast. Charges with unstoppable force." },
    EnemyDef { name: "Cockatrice",      hp: 12, attack: 6, defense: 3, glyph: '~', is_ranged: false, xp: 13, behavior: A,  description: "A reptilian bird with a petrifying gaze." },
    EnemyDef { name: "Naga",            hp: 14, attack: 7, defense: 4, glyph: 'N', is_ranged: false, xp: 22, behavior: S,  description: "A serpentine spellcaster. Ancient and cunning." },
    EnemyDef { name: "Medusa",          hp: 14, attack: 8, defense: 3, glyph: 'P', is_ranged: false, xp: 22, behavior: S,  description: "Her gaze turns flesh to stone." },
    EnemyDef { name: "Manticore",       hp: 22, attack: 8, defense: 5, glyph: 'X', is_ranged: false, xp: 30, behavior: A,  description: "Lion, scorpion, and bat in one terrible form." },
    EnemyDef { name: "Yak",             hp: 14, attack: 4, defense: 5, glyph: '*', is_ranged: false, xp: 15, behavior: Te, description: "A shaggy mountain beast. Tough and thick-skinned." },

    // ── Dungeon — shallow (L0) ──────────────────────────────────────
    EnemyDef { name: "Kobold",              hp: 5,  attack: 3, defense: 2, glyph: 'c', is_ranged: false, xp: 3,  behavior: Ti, description: "A small reptilian scavenger. Cowardly but cunning." },
    EnemyDef { name: "Goblin",              hp: 6,  attack: 3, defense: 2, glyph: 'g', is_ranged: false, xp: 4,  behavior: A,  description: "A sneaky green creature. Dangerous in numbers." },
    EnemyDef { name: "Skeleton",            hp: 7,  attack: 4, defense: 3, glyph: 's', is_ranged: false, xp: 6,  behavior: A,  description: "Animated bones bound by dark magic." },
    EnemyDef { name: "Myconid",             hp: 4,  attack: 2, defense: 2, glyph: 'p', is_ranged: false, xp: 3,  behavior: P,  description: "A walking mushroom that releases toxic spores." },
    EnemyDef { name: "Large Myconid",       hp: 5,  attack: 3, defense: 2, glyph: 't', is_ranged: false, xp: 4,  behavior: P,  description: "A towering fungal creature. Its spores choke the air." },
    EnemyDef { name: "Giant Earthworm",     hp: 6,  attack: 3, defense: 0, glyph: ']', is_ranged: false, xp: 4,  behavior: Ti, description: "A massive burrowing worm. Ambushes from below." },
    EnemyDef { name: "Lesser Giant Spider", hp: 5,  attack: 3, defense: 0, glyph: '<', is_ranged: false, xp: 3,  behavior: Ti, description: "A small but venomous forest spider." },
    EnemyDef { name: "Lizardfolk",          hp: 7,  attack: 4, defense: 2, glyph: '>', is_ranged: false, xp: 5,  behavior: A,  description: "A scaled warrior. Territorial and aggressive." },
    EnemyDef { name: "Cultist",             hp: 7,  attack: 4, defense: 2, glyph: '!', is_ranged: false, xp: 5,  behavior: A,  description: "A hooded zealot. Serves dark powers." },

    // ── Dungeon — mid (L1) ──────────────────────────────────────────
    EnemyDef { name: "Goblin Brute",    hp: 14, attack: 6,  defense: 3, glyph: '3', is_ranged: false, xp: 8,  behavior: A,  description: "A hulking goblin. All muscle, no brains." },
    EnemyDef { name: "Goblin Archer",   hp: 12, attack: 6,  defense: 3, glyph: 'G', is_ranged: true,  xp: 7,  behavior: A,  description: "A goblin with a crude bow. Deadly at range." },
    EnemyDef { name: "Goblin Mage",     hp: 14, attack: 8,  defense: 3, glyph: 'M', is_ranged: true,  xp: 10, behavior: A,  description: "A goblin versed in crude fire magic." },
    EnemyDef { name: "Skeleton Archer", hp: 14, attack: 7,  defense: 4, glyph: 'k', is_ranged: true,  xp: 9,  behavior: A,  description: "Dead bones with unerring aim." },
    EnemyDef { name: "Wraith",          hp: 17, attack: 11, defense: 2, glyph: 'W', is_ranged: false, xp: 16, behavior: S,  description: "A hateful spirit. Drains the life from victims." },
    EnemyDef { name: "Hag",             hp: 17, attack: 7,  defense: 3, glyph: 'H', is_ranged: false, xp: 11, behavior: A,  description: "A wretched crone. Curses those who draw near." },
    EnemyDef { name: "Zombie",          hp: 19, attack: 5,  defense: 3, glyph: 'z', is_ranged: false, xp: 8,  behavior: A,  description: "A shambling corpse. Slow but relentless." },
    EnemyDef { name: "Big Slime",       hp: 19, attack: 5,  defense: 2, glyph: 'm', is_ranged: false, xp: 9,  behavior: P,  description: "A massive ooze. Absorbs blows like nothing." },
    EnemyDef { name: "Orc",             hp: 19, attack: 8,  defense: 5, glyph: 'o', is_ranged: false, xp: 14, behavior: A,  description: "A fierce tribal warrior. Bred for battle." },
    EnemyDef { name: "Lampreymander",   hp: 16, attack: 7,  defense: 3, glyph: '[', is_ranged: false, xp: 10, behavior: A,  description: "An amphibian predator with a lamprey-like mouth." },
    EnemyDef { name: "Faceless Monk",   hp: 20, attack: 8,  defense: 4, glyph: '6', is_ranged: false, xp: 15, behavior: S,  description: "A silent cultist. Its face is smooth, featureless skin." },
    EnemyDef { name: "Orc Wizard",      hp: 18, attack: 10, defense: 4, glyph: '}', is_ranged: true,  xp: 14, behavior: A,  description: "An orc versed in destructive magic. Attacks from range." },

    // ── Dungeon — deep (L2) ─────────────────────────────────────────
    EnemyDef { name: "Ghoul",              hp: 24, attack: 11, defense: 5, glyph: 'u', is_ranged: false, xp: 16, behavior: A,  description: "A ravenous undead. Paralyzes with its claws." },
    EnemyDef { name: "Banshee",            hp: 24, attack: 12, defense: 4, glyph: 'Q', is_ranged: false, xp: 20, behavior: A,  description: "A wailing spirit. Her screams can kill." },
    EnemyDef { name: "Small Writhing Mass",hp: 24, attack: 10, defense: 5, glyph: '(', is_ranged: false, xp: 15, behavior: A,  description: "A small pulsing mound of flesh. Deeply unsettling." },
    EnemyDef { name: "Writhing Humanoid",  hp: 26, attack: 12, defense: 4, glyph: ')', is_ranged: false, xp: 19, behavior: A,  description: "A human-shaped mass of writhing tissue." },
    EnemyDef { name: "Orc Warchief",       hp: 29, attack: 11, defense: 7, glyph: '5', is_ranged: false, xp: 20, behavior: A,  description: "A battle-scarred commander. Inspires fury in others." },
    EnemyDef { name: "Orc Blademaster",    hp: 31, attack: 11, defense: 7, glyph: 'O', is_ranged: false, xp: 20, behavior: A,  description: "An elite orc warrior. Master of the blade." },
    EnemyDef { name: "Unholy Cardinal",    hp: 30, attack: 13, defense: 6, glyph: '7', is_ranged: false, xp: 24, behavior: A,  description: "A heretic priest wreathed in dark flame." },
    EnemyDef { name: "Writhing Mass",      hp: 34, attack: 12, defense: 5, glyph: '8', is_ranged: false, xp: 22, behavior: A,  description: "A pulsing mound of flesh. Origin unknown." },
    EnemyDef { name: "Troll",              hp: 36, attack: 11, defense: 6, glyph: 'T', is_ranged: false, xp: 22, behavior: A,  description: "A towering brute. Regenerates from any wound." },
    EnemyDef { name: "Ettin",              hp: 38, attack: 12, defense: 8, glyph: 'E', is_ranged: false, xp: 24, behavior: A,  description: "A two-headed giant. Twice the fury." },
    EnemyDef { name: "Two-Headed Ettin",   hp: 46, attack: 14, defense: 8, glyph: '^', is_ranged: false, xp: 28, behavior: A,  description: "A four-armed twin-headed giant. Devastatingly strong." },

    // ── Cave / boss-tier ─────────────────────────────────────────────
    EnemyDef { name: "Imp",          hp: 26, attack: 12, defense: 4, glyph: 'I', is_ranged: false, xp: 22, behavior: A, description: "A fiendish creature from the abyss." },
    EnemyDef { name: "Drake",        hp: 34, attack: 13, defense: 6, glyph: 'd', is_ranged: false, xp: 26, behavior: A, description: "A young dragon. Still deadly." },
    EnemyDef { name: "Lich",         hp: 36, attack: 16, defense: 6, glyph: 'l', is_ranged: false, xp: 35, behavior: A, description: "An undead sorcerer of immense power." },
    EnemyDef { name: "Basilisk",     hp: 36, attack: 14, defense: 7, glyph: 'C', is_ranged: false, xp: 28, behavior: A, description: "Its gaze paralyzes. Its bite kills." },
    EnemyDef { name: "Death Knight", hp: 43, attack: 14, defense: 8, glyph: 'K', is_ranged: false, xp: 32, behavior: A, description: "A fallen paladin. Commands undead legions." },
    EnemyDef { name: "Reaper",       hp: 43, attack: 17, defense: 7, glyph: 'V', is_ranged: false, xp: 34, behavior: A, description: "Death incarnate. Few survive its scythe." },

    // ── Dragon (boss) — runtime stats come from CombatConfig ────────
    EnemyDef { name: "Dragon",       hp: 108, attack: 19, defense: 11, glyph: 'D', is_ranged: false, xp: 200, behavior: A, description: "The cave's ancient guardian. Legendary power." },
];
