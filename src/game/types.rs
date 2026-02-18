use crate::map::Tile;

#[derive(Clone, Debug, PartialEq)]
pub enum ItemKind {
    Potion,
    Scroll,
    Weapon,
    /// Bows and crossbows — equips in weapon slot but uses ranged attack via swipe aiming.
    RangedWeapon,
    Armor,
    Helmet,
    Shield,
    Boots,
    Food,
    Ring,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FoodSideEffect {
    None,
    /// Restores HP.
    Heal(i32),
    /// Deals damage (toxic food).
    Poison(i32),
    /// Restores stamina.
    Energize(i32),
    /// Drains stamina (nausea).
    Sicken(i32),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ItemEffect {
    Heal(i32),
    DamageAoe(i32),
    BuffAttack(i32),
    BuffDefense(i32),
    /// Restores hunger points with an optional side effect.
    Feed(i32, FoodSideEffect),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub kind: ItemKind,
    pub name: &'static str,
    pub glyph: char,
    pub effect: ItemEffect,
}

#[derive(Clone, Debug)]
pub struct GroundItem {
    pub x: i32,
    pub y: i32,
    pub item: Item,
}

#[derive(Clone)]
pub struct Enemy {
    pub x: i32,
    pub y: i32,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub glyph: char,
    pub name: &'static str,
    /// true when sprite should face left (mirrored).
    pub facing_left: bool,
    /// true if this enemy has a ranged attack (archers).
    pub is_ranged: bool,
}

#[derive(Debug, PartialEq)]
pub enum TurnResult {
    Moved,
    Blocked,
    Attacked { target_name: &'static str, damage: i32 },
    Killed { target_name: &'static str },
    PlayerDied,
    Won,
    /// Player stepped on a transition tile and changed maps.
    MapChanged,
}

/// Floating text indicator (damage numbers, XP, healing).
#[derive(Clone)]
pub struct FloatingText {
    pub world_x: i32,
    pub world_y: i32,
    pub text: String,
    pub color: &'static str,
    /// 0.0 = just created, 1.0 = expired.
    pub age: f64,
}

/// Brief position offset animation (attack lunge, damage recoil).
#[derive(Clone)]
pub struct BumpAnim {
    pub is_player: bool,
    pub enemy_idx: usize,
    pub dx: f64,
    pub dy: f64,
    /// 0.0 = start, 1.0 = done.
    pub progress: f64,
}

/// Visual effect overlay (AOE blast, healing glow, etc.).
#[derive(Clone)]
pub struct VisualEffect {
    pub kind: EffectKind,
    pub x: i32,
    pub y: i32,
    /// 0.0 = start, 1.0 = done.
    pub age: f64,
}

#[derive(Clone)]
pub enum EffectKind {
    /// Expanding ring of damage.
    AoeBlast,
    /// Healing glow on target.
    HealGlow,
    /// Poison cloud on target.
    PoisonCloud,
    /// Energize sparkle on target.
    EnergizeEffect,
}

/// Structured info about a tile at a specific world position.
#[derive(Debug, Clone, PartialEq)]
pub struct TileInfo {
    pub tile_name: &'static str,
    pub tile_desc: &'static str,
    pub walkable: bool,
    pub enemy: Option<EnemyInfo>,
    pub item: Option<ItemInfo>,
    pub is_player: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnemyInfo {
    pub name: &'static str,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub desc: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemInfo {
    pub name: &'static str,
    pub desc: String,
}

pub(super) fn tile_name(tile: Tile) -> &'static str {
    match tile {
        Tile::Wall => "Stone Wall",
        Tile::Floor => "Stone Floor",
        Tile::Tree => "Dense Tree",
        Tile::Grass => "Grass",
        Tile::Road => "Road",
        Tile::DungeonEntrance => "Dungeon Entrance",
        Tile::StairsDown => "Stairs Down",
        Tile::StairsUp => "Stairs Up",
    }
}

pub(super) fn tile_desc(tile: Tile) -> &'static str {
    match tile {
        Tile::Wall => "A solid stone wall. Impassable.",
        Tile::Floor => "Rough stone floor. Watch your step.",
        Tile::Tree => "Thick forest. Cannot pass through.",
        Tile::Grass => "An open clearing in the forest.",
        Tile::Road => "A well-worn path between dungeons.",
        Tile::DungeonEntrance => "A dark passage leading underground. Step on to enter.",
        Tile::StairsDown => "Crumbling stairs descending deeper.",
        Tile::StairsUp => "Stairs leading back up.",
    }
}

pub(super) fn enemy_desc(name: &str) -> &'static str {
    match name {
        // Forest beasts
        "Giant Rat"       => "A disease-carrying rodent the size of a dog.",
        "Giant Bat"       => "A bat with a wingspan wider than a man.",
        "Wolf"            => "A cunning pack hunter. Fast and relentless.",
        "Giant Spider"    => "A venomous arachnid that lurks in the shadows.",
        "Boar"            => "A ferocious wild pig with razor-sharp tusks.",
        "Bear"            => "A massive predator. Top of the forest chain.",
        "Lycanthrope"     => "A cursed shapeshifter. Savage in beast form.",
        // Dungeon — shallow
        "Kobold"          => "A small reptilian scavenger. Cowardly but cunning.",
        "Small Slime"     => "A translucent ooze. Dissolves what it touches.",
        "Goblin"          => "A sneaky green creature. Dangerous in numbers.",
        "Skeleton"        => "Animated bones bound by dark magic.",
        // Dungeon — mid
        "Goblin Archer"   => "A goblin with a crude bow. Deadly at range.",
        "Zombie"          => "A shambling corpse. Slow but relentless.",
        "Skeleton Archer" => "Dead bones with unerring aim.",
        "Big Slime"       => "A massive ooze. Absorbs blows like nothing.",
        "Orc"             => "A fierce tribal warrior. Bred for battle.",
        // Dungeon — deep
        "Ghoul"           => "A ravenous undead. Paralyzes with its claws.",
        "Orc Blademaster" => "An elite orc warrior. Master of the blade.",
        "Wraith"          => "A hateful spirit. Drains the life from victims.",
        "Naga"            => "A serpentine spellcaster. Ancient and cunning.",
        "Troll"           => "A towering brute. Regenerates from any wound.",
        // Cave — boss floor
        "Death Knight"    => "A fallen paladin. Commands undead legions.",
        "Lich"            => "An undead sorcerer of immense power.",
        "Dragon"          => "The cave's ancient guardian. Legendary power.",
        _ => "A mysterious creature.",
    }
}

pub(super) fn xp_for_enemy(name: &str) -> u32 {
    match name {
        // Forest
        "Giant Rat" => 3,
        "Giant Bat" => 4,
        "Wolf" => 5,
        "Giant Spider" => 6,
        "Boar" => 7,
        "Bear" => 12,
        "Lycanthrope" => 18,
        // Dungeon — shallow
        "Kobold" => 3,
        "Small Slime" => 3,
        "Goblin" => 4,
        "Skeleton" => 6,
        // Dungeon — mid
        "Goblin Archer" => 5,
        "Zombie" => 6,
        "Skeleton Archer" => 7,
        "Big Slime" => 7,
        "Orc" => 10,
        // Dungeon — deep
        "Ghoul" => 11,
        "Orc Blademaster" => 14,
        "Wraith" => 13,
        "Naga" => 16,
        "Troll" => 15,
        // Cave
        "Death Knight" => 22,
        "Lich" => 25,
        "Dragon" => 100,
        _ => 3,
    }
}

pub(super) fn item_info_desc(item: &Item) -> String {
    let effect = match &item.effect {
        ItemEffect::Heal(n) => format!("Restores {} HP", n),
        ItemEffect::DamageAoe(n) => format!("Deals {} damage in area", n),
        ItemEffect::BuffAttack(n) => format!("+{} Attack", n),
        ItemEffect::BuffDefense(n) => format!("+{} Defense", n),
        ItemEffect::Feed(n, side) => {
            let base = format!("Restores {} hunger", n);
            let suffix = match side {
                FoodSideEffect::None => String::new(),
                FoodSideEffect::Heal(h) => format!(", +{} HP", h),
                FoodSideEffect::Poison(d) => format!(", toxic (-{} HP)", d),
                FoodSideEffect::Energize(s) => format!(", +{} stamina", s),
                FoodSideEffect::Sicken(s) => format!(", nauseating (-{} stamina)", s),
            };
            format!("{}{}", base, suffix)
        }
    };
    format!("{} — {}", item.name, effect)
}

/// Which bottom drawer is currently open.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Drawer {
    None,
    Inventory,
    Stats,
    Settings,
}

/// Allocatable skill attributes.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SkillKind {
    Strength,
    Vitality,
    Dexterity,
    Stamina,
}
