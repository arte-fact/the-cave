/// Centralized enemy definitions. Every spawnable enemy's base stats,
/// XP reward, and inspect-panel description live here.

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
}

/// Look up an enemy definition by name.
pub fn enemy_def(name: &str) -> Option<&'static EnemyDef> {
    ENEMY_DEFS.iter().find(|d| d.name == name)
}

/// XP reward for killing an enemy. Falls back to 3 for unknown names.
pub fn xp_for_enemy(name: &str) -> u32 {
    enemy_def(name).map_or(3, |d| d.xp)
}

/// Flavour description shown in the inspect panel.
pub fn enemy_description(name: &str) -> &'static str {
    enemy_def(name).map_or("A mysterious creature.", |d| d.description)
}

/// Complete enemy registry. Sorted roughly by encounter order
/// (overworld → dungeon shallow → mid → deep → cave/boss).
pub static ENEMY_DEFS: &[EnemyDef] = &[
    // ── Overworld — common wildlife ──────────────────────────────────
    EnemyDef { name: "Fox",             hp: 4,  attack: 2, defense: 1, glyph: 'f', is_ranged: false, xp: 4,  description: "A quick and sly predator. Hard to pin down." },
    EnemyDef { name: "Buzzard",         hp: 4,  attack: 2, defense: 0, glyph: 'q', is_ranged: false, xp: 4,  description: "A circling scavenger. Swoops down on the weak." },
    EnemyDef { name: "Giant Rat",       hp: 3,  attack: 1, defense: 0, glyph: 'r', is_ranged: false, xp: 3,  description: "A disease-carrying rodent the size of a dog." },
    EnemyDef { name: "Giant Bat",       hp: 4,  attack: 2, defense: 0, glyph: 'a', is_ranged: false, xp: 4,  description: "A bat with a wingspan wider than a man." },
    EnemyDef { name: "Small Slime",     hp: 4,  attack: 1, defense: 0, glyph: 'S', is_ranged: false, xp: 3,  description: "A translucent ooze. Dissolves what it touches." },
    EnemyDef { name: "Giant Centipede", hp: 4,  attack: 2, defense: 0, glyph: 'e', is_ranged: false, xp: 4,  description: "A writhing mass of legs and mandibles." },
    EnemyDef { name: "Viper",           hp: 5,  attack: 3, defense: 0, glyph: 'n', is_ranged: false, xp: 5,  description: "A venomous snake. Strikes without warning." },
    EnemyDef { name: "Coyote",          hp: 5,  attack: 2, defense: 1, glyph: 'y', is_ranged: false, xp: 5,  description: "A crafty opportunist. Hunts in pairs." },
    EnemyDef { name: "Wolf",            hp: 5,  attack: 2, defense: 1, glyph: 'w', is_ranged: false, xp: 5,  description: "A cunning pack hunter. Fast and relentless." },
    EnemyDef { name: "Badger",          hp: 5,  attack: 3, defense: 1, glyph: 'j', is_ranged: false, xp: 5,  description: "Small but ferocious. Fights to the death." },
    EnemyDef { name: "Giant Spider",    hp: 6,  attack: 3, defense: 0, glyph: 'i', is_ranged: false, xp: 6,  description: "A venomous arachnid that lurks in the shadows." },
    EnemyDef { name: "Honey Badger",    hp: 6,  attack: 3, defense: 2, glyph: 'J', is_ranged: false, xp: 7,  description: "Fearless and relentless. Never backs down." },
    EnemyDef { name: "Lynx",            hp: 5,  attack: 3, defense: 1, glyph: '#', is_ranged: false, xp: 5,  description: "A ghost of the forest. Strikes from ambush." },
    EnemyDef { name: "Boar",            hp: 8,  attack: 2, defense: 2, glyph: 'b', is_ranged: false, xp: 7,  description: "A ferocious wild pig with razor-sharp tusks." },
    EnemyDef { name: "Black Bear",      hp: 9,  attack: 3, defense: 2, glyph: '&', is_ranged: false, xp: 8,  description: "A powerful woodland bear. Protective of its territory." },
    EnemyDef { name: "Cougar",          hp: 9,  attack: 4, defense: 2, glyph: 'h', is_ranged: false, xp: 8,  description: "A stealthy big cat. Silent and deadly." },
    EnemyDef { name: "Bear",            hp: 12, attack: 4, defense: 2, glyph: 'B', is_ranged: false, xp: 12, description: "A massive predator. Top of the forest chain." },

    // ── Overworld — rare monsters (mini-boss) ───────────────────────
    EnemyDef { name: "Dryad",           hp: 18, attack: 5, defense: 3, glyph: '1', is_ranged: false, xp: 20, description: "A woodland spirit. Protects the ancient trees." },
    EnemyDef { name: "Forest Spirit",   hp: 16, attack: 6, defense: 2, glyph: '2', is_ranged: false, xp: 18, description: "An ethereal guardian of the deep woods." },
    EnemyDef { name: "Centaur",         hp: 22, attack: 7, defense: 4, glyph: '9', is_ranged: false, xp: 25, description: "Half-man, half-horse. Patrols the forest trails." },
    EnemyDef { name: "Dire Wolf",       hp: 20, attack: 6, defense: 3, glyph: 'U', is_ranged: false, xp: 22, description: "An enormous wolf. Pack leader and apex predator." },
    EnemyDef { name: "Lycanthrope",     hp: 28, attack: 8, defense: 4, glyph: 'L', is_ranged: false, xp: 35, description: "A cursed shapeshifter. Savage in beast form." },
    EnemyDef { name: "Wendigo",         hp: 30, attack: 9, defense: 3, glyph: '0', is_ranged: false, xp: 40, description: "A gaunt horror of the frozen woods. Insatiable hunger." },

    // ── Dungeon — exotic fauna (used across biomes) ─────────────────
    EnemyDef { name: "Black Mamba",     hp: 5,  attack: 3, defense: 0, glyph: 'v', is_ranged: false, xp: 6,  description: "The deadliest snake. Lightning-fast venom." },
    EnemyDef { name: "Monitor Lizard",  hp: 7,  attack: 3, defense: 2, glyph: '|', is_ranged: false, xp: 7,  description: "A large tropical reptile with a venomous bite." },
    EnemyDef { name: "Giant Ant",       hp: 8,  attack: 3, defense: 2, glyph: 'A', is_ranged: false, xp: 6,  description: "An oversized insect with crushing mandibles." },
    EnemyDef { name: "Water Buffalo",   hp: 14, attack: 4, defense: 4, glyph: '%', is_ranged: false, xp: 14, description: "A massive beast. Charges with unstoppable force." },
    EnemyDef { name: "Cockatrice",      hp: 10, attack: 5, defense: 2, glyph: '~', is_ranged: false, xp: 10, description: "A reptilian bird with a petrifying gaze." },
    EnemyDef { name: "Naga",            hp: 12, attack: 6, defense: 3, glyph: 'N', is_ranged: false, xp: 16, description: "A serpentine spellcaster. Ancient and cunning." },
    EnemyDef { name: "Medusa",          hp: 12, attack: 7, defense: 2, glyph: 'P', is_ranged: false, xp: 16, description: "Her gaze turns flesh to stone." },
    EnemyDef { name: "Manticore",       hp: 18, attack: 7, defense: 4, glyph: 'X', is_ranged: false, xp: 22, description: "Lion, scorpion, and bat in one terrible form." },
    EnemyDef { name: "Yak",             hp: 12, attack: 3, defense: 4, glyph: '*', is_ranged: false, xp: 12, description: "A shaggy mountain beast. Tough and thick-skinned." },

    // ── Dungeon — shallow (L0) ──────────────────────────────────────
    EnemyDef { name: "Kobold",              hp: 4,  attack: 2, defense: 1, glyph: 'c', is_ranged: false, xp: 3,  description: "A small reptilian scavenger. Cowardly but cunning." },
    EnemyDef { name: "Goblin",              hp: 5,  attack: 2, defense: 1, glyph: 'g', is_ranged: false, xp: 4,  description: "A sneaky green creature. Dangerous in numbers." },
    EnemyDef { name: "Skeleton",            hp: 6,  attack: 3, defense: 2, glyph: 's', is_ranged: false, xp: 6,  description: "Animated bones bound by dark magic." },
    EnemyDef { name: "Myconid",             hp: 3,  attack: 1, defense: 1, glyph: 'p', is_ranged: false, xp: 3,  description: "A walking mushroom that releases toxic spores." },
    EnemyDef { name: "Large Myconid",       hp: 4,  attack: 2, defense: 1, glyph: 't', is_ranged: false, xp: 4,  description: "A towering fungal creature. Its spores choke the air." },
    EnemyDef { name: "Giant Earthworm",     hp: 5,  attack: 2, defense: 0, glyph: ']', is_ranged: false, xp: 4,  description: "A massive burrowing worm. Ambushes from below." },
    EnemyDef { name: "Lesser Giant Spider", hp: 4,  attack: 2, defense: 0, glyph: '<', is_ranged: false, xp: 3,  description: "A small but venomous forest spider." },
    EnemyDef { name: "Lizardfolk",          hp: 6,  attack: 3, defense: 1, glyph: '>', is_ranged: false, xp: 5,  description: "A scaled warrior. Territorial and aggressive." },
    EnemyDef { name: "Cultist",             hp: 6,  attack: 3, defense: 1, glyph: '!', is_ranged: false, xp: 5,  description: "A hooded zealot. Serves dark powers." },

    // ── Dungeon — mid (L1) ──────────────────────────────────────────
    EnemyDef { name: "Goblin Brute",    hp: 6,  attack: 3, defense: 1, glyph: '3', is_ranged: false, xp: 6,  description: "A hulking goblin. All muscle, no brains." },
    EnemyDef { name: "Goblin Archer",   hp: 6,  attack: 3, defense: 1, glyph: 'G', is_ranged: true,  xp: 5,  description: "A goblin with a crude bow. Deadly at range." },
    EnemyDef { name: "Goblin Mage",     hp: 7,  attack: 5, defense: 1, glyph: 'M', is_ranged: true,  xp: 7,  description: "A goblin versed in crude fire magic." },
    EnemyDef { name: "Skeleton Archer", hp: 7,  attack: 4, defense: 2, glyph: 'k', is_ranged: true,  xp: 7,  description: "Dead bones with unerring aim." },
    EnemyDef { name: "Wraith",          hp: 8,  attack: 6, defense: 0, glyph: 'W', is_ranged: false, xp: 13, description: "A hateful spirit. Drains the life from victims." },
    EnemyDef { name: "Hag",             hp: 9,  attack: 4, defense: 1, glyph: 'H', is_ranged: false, xp: 8,  description: "A wretched crone. Curses those who draw near." },
    EnemyDef { name: "Zombie",          hp: 10, attack: 2, defense: 1, glyph: 'z', is_ranged: false, xp: 6,  description: "A shambling corpse. Slow but relentless." },
    EnemyDef { name: "Big Slime",       hp: 10, attack: 2, defense: 0, glyph: 'm', is_ranged: false, xp: 7,  description: "A massive ooze. Absorbs blows like nothing." },
    EnemyDef { name: "Orc",             hp: 10, attack: 4, defense: 3, glyph: 'o', is_ranged: false, xp: 10, description: "A fierce tribal warrior. Bred for battle." },
    EnemyDef { name: "Lampreymander",   hp: 8,  attack: 4, defense: 1, glyph: '[', is_ranged: false, xp: 8,  description: "An amphibian predator with a lamprey-like mouth." },
    EnemyDef { name: "Faceless Monk",   hp: 11, attack: 5, defense: 2, glyph: '6', is_ranged: false, xp: 12, description: "A silent cultist. Its face is smooth, featureless skin." },
    EnemyDef { name: "Orc Wizard",      hp: 10, attack: 6, defense: 2, glyph: '}', is_ranged: true,  xp: 11, description: "An orc versed in destructive magic. Attacks from range." },

    // ── Dungeon — deep (L2) ─────────────────────────────────────────
    EnemyDef { name: "Ghoul",              hp: 10, attack: 5, defense: 2, glyph: 'u', is_ranged: false, xp: 11, description: "A ravenous undead. Paralyzes with its claws." },
    EnemyDef { name: "Banshee",            hp: 10, attack: 6, defense: 1, glyph: 'Q', is_ranged: false, xp: 14, description: "A wailing spirit. Her screams can kill." },
    EnemyDef { name: "Small Writhing Mass",hp: 10, attack: 5, defense: 2, glyph: '(', is_ranged: false, xp: 10, description: "A small pulsing mound of flesh. Deeply unsettling." },
    EnemyDef { name: "Writhing Humanoid",  hp: 12, attack: 6, defense: 1, glyph: ')', is_ranged: false, xp: 13, description: "A human-shaped mass of writhing tissue." },
    EnemyDef { name: "Orc Warchief",       hp: 12, attack: 5, defense: 4, glyph: '5', is_ranged: false, xp: 14, description: "A battle-scarred commander. Inspires fury in others." },
    EnemyDef { name: "Orc Blademaster",    hp: 14, attack: 5, defense: 4, glyph: 'O', is_ranged: false, xp: 14, description: "An elite orc warrior. Master of the blade." },
    EnemyDef { name: "Unholy Cardinal",    hp: 14, attack: 7, defense: 3, glyph: '7', is_ranged: false, xp: 17, description: "A heretic priest wreathed in dark flame." },
    EnemyDef { name: "Writhing Mass",      hp: 15, attack: 6, defense: 2, glyph: '8', is_ranged: false, xp: 16, description: "A pulsing mound of flesh. Origin unknown." },
    EnemyDef { name: "Troll",              hp: 16, attack: 5, defense: 3, glyph: 'T', is_ranged: false, xp: 15, description: "A towering brute. Regenerates from any wound." },
    EnemyDef { name: "Ettin",              hp: 18, attack: 6, defense: 4, glyph: 'E', is_ranged: false, xp: 16, description: "A two-headed giant. Twice the fury." },
    EnemyDef { name: "Two-Headed Ettin",   hp: 22, attack: 7, defense: 5, glyph: '^', is_ranged: false, xp: 20, description: "A four-armed twin-headed giant. Devastatingly strong." },

    // ── Cave / boss-tier ─────────────────────────────────────────────
    EnemyDef { name: "Imp",          hp: 10, attack: 6, defense: 1, glyph: 'I', is_ranged: false, xp: 16, description: "A fiendish creature from the abyss." },
    EnemyDef { name: "Drake",        hp: 14, attack: 6, defense: 3, glyph: 'd', is_ranged: false, xp: 18, description: "A young dragon. Still deadly." },
    EnemyDef { name: "Lich",         hp: 15, attack: 8, defense: 2, glyph: 'l', is_ranged: false, xp: 25, description: "An undead sorcerer of immense power." },
    EnemyDef { name: "Basilisk",     hp: 16, attack: 7, defense: 4, glyph: 'C', is_ranged: false, xp: 20, description: "Its gaze paralyzes. Its bite kills." },
    EnemyDef { name: "Death Knight", hp: 20, attack: 7, defense: 5, glyph: 'K', is_ranged: false, xp: 22, description: "A fallen paladin. Commands undead legions." },
    EnemyDef { name: "Reaper",       hp: 20, attack: 9, defense: 3, glyph: 'V', is_ranged: false, xp: 24, description: "Death incarnate. Few survive its scythe." },

    // ── Dragon (boss) — runtime stats come from CombatConfig ────────
    EnemyDef { name: "Dragon",       hp: 40, attack: 10, defense: 6, glyph: 'D', is_ranged: false, xp: 100, description: "The cave's ancient guardian. Legendary power." },
];
