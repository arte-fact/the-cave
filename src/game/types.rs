use crate::config::EnemyBehavior;
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
    /// Restores stamina points.
    RestoreStamina(i32),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub kind: ItemKind,
    pub name: &'static str,
    pub glyph: char,
    pub effect: ItemEffect,
    /// Weight of the item (0–5). Heavier weapons cost more stamina to swing.
    /// 0 = non-weapon/weightless, 1 = very light, 5 = very heavy.
    pub weight: i32,
    /// Current durability. Weapons lose 1 per attack dealt, armor pieces lose 1 per
    /// hit absorbed. When durability reaches 0 the item breaks and is destroyed.
    /// 0 = not applicable (consumables).
    pub durability: i32,
    /// Whether this item is a legendary set piece (boss drop).
    pub legendary: bool,
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
    /// AI behavior type.
    pub behavior: EnemyBehavior,
    /// Spawn position for territorial leash.
    pub spawn_x: i32,
    pub spawn_y: i32,
    /// Whether this enemy has been provoked (attacked or triggered).
    pub provoked: bool,
    /// Whether this enemy is a dungeon boss (guaranteed legendary drop on kill).
    pub is_boss: bool,
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

/// Stamina cost to attack with a weapon of the given weight.
/// Melee: melee_base + weight * melee_weight_mult.
/// Ranged: ranged_base + weight * ranged_weight_mult.
/// Unarmed (no weapon / weight 0): uses melee formula.
pub fn weapon_stamina_cost(
    kind: &ItemKind,
    weight: i32,
    melee_base: i32,
    melee_weight_mult: i32,
    ranged_base: i32,
    ranged_weight_mult: i32,
) -> i32 {
    match kind {
        ItemKind::RangedWeapon => ranged_base + weight * ranged_weight_mult,
        _ => melee_base + weight * melee_weight_mult,
    }
}

/// Format item description with default stamina cost parameters.
/// Used by tests and as a convenience wrapper.
pub(super) fn item_info_desc(item: &Item) -> String {
    item_info_desc_with_config(item, 6, 2, 4, 1)
}

/// Format item description with configurable stamina cost parameters.
pub(super) fn item_info_desc_with_config(
    item: &Item,
    melee_base: i32,
    melee_weight_mult: i32,
    ranged_base: i32,
    ranged_weight_mult: i32,
) -> String {
    let effect = match &item.effect {
        ItemEffect::Heal(n) => format!("Restores {} HP", n),
        ItemEffect::DamageAoe(n) => format!("Deals {} damage in area", n),
        ItemEffect::BuffAttack(n) => {
            let cost = weapon_stamina_cost(&item.kind, item.weight, melee_base, melee_weight_mult, ranged_base, ranged_weight_mult);
            format!("+{} Attack, {} stamina", n, cost)
        }
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
        ItemEffect::RestoreStamina(n) => format!("Restores {} stamina", n),
    };
    if item.durability > 0 {
        format!("{} — {} [Dur: {}]", item.name, effect, item.durability)
    } else {
        format!("{} — {}", item.name, effect)
    }
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

impl ItemKind {
    /// Returns true for consumable items (Potion, Scroll, Food) that can be
    /// used from inventory or quick-bar and are eligible for quick-bar slots.
    pub fn is_consumable(&self) -> bool {
        matches!(self, ItemKind::Potion | ItemKind::Scroll | ItemKind::Food)
    }
}

/// Number of quick-bar slots.
pub const QUICKBAR_SLOTS: usize = 6;

/// The quick-use bar: fixed-size array of slots referencing inventory item indices.
/// `None` = empty slot.
#[derive(Clone, Debug)]
pub struct QuickBar {
    pub slots: [Option<usize>; QUICKBAR_SLOTS],
}

impl QuickBar {
    pub fn new() -> Self {
        Self { slots: [None; QUICKBAR_SLOTS] }
    }

    /// Assign an inventory item to a slot. Only consumables (Potion, Scroll, Food) allowed.
    /// Returns false if the item isn't consumable or the slot is out of bounds.
    /// If this inventory index is already assigned to another slot, that slot is cleared first.
    pub fn assign(&mut self, slot: usize, inv_index: usize, item: &Item) -> bool {
        if slot >= QUICKBAR_SLOTS || !item.kind.is_consumable() {
            return false;
        }
        // Remove any existing assignment of this inventory index
        for s in &mut self.slots {
            if *s == Some(inv_index) { *s = None; }
        }
        self.slots[slot] = Some(inv_index);
        true
    }

    /// Clear a slot.
    pub fn clear(&mut self, slot: usize) {
        if slot < QUICKBAR_SLOTS {
            self.slots[slot] = None;
        }
    }

    /// Fix up slot indices after an item at `removed_idx` was removed from inventory.
    /// Slots pointing to `removed_idx` are cleared. Slots pointing to higher indices
    /// are decremented by 1 (because `Vec::remove` shifts elements left).
    pub fn on_item_removed(&mut self, removed_idx: usize) {
        for slot in &mut self.slots {
            if let Some(idx) = slot {
                if *idx == removed_idx {
                    *slot = None;
                } else if *idx > removed_idx {
                    *idx -= 1;
                }
            }
        }
    }

    /// Swap the contents of two slots.
    pub fn swap(&mut self, a: usize, b: usize) {
        if a < QUICKBAR_SLOTS && b < QUICKBAR_SLOTS {
            self.slots.swap(a, b);
        }
    }
}
