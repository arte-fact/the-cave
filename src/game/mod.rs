mod types;
mod items;
mod combat;
mod movement;
mod survival;
mod spawning;
mod progression;

pub use types::*;
#[cfg(test)]
use items::{random_item, meat_drop};
#[cfg(test)]
use types::{tile_name, tile_desc, enemy_desc, xp_for_enemy, item_info_desc};

use crate::config::GameConfig;
use crate::map::Map;
use crate::world::{Location, World};

pub struct Game {
    pub config: GameConfig,
    pub player_x: i32,
    pub player_y: i32,
    pub player_hp: i32,
    pub player_max_hp: i32,
    pub player_attack: i32,
    /// Dexterity: affects ranged weapon max range and hit chance.
    pub player_dexterity: i32,
    /// true when player sprite should face left (mirrored).
    pub player_facing_left: bool,
    pub world: World,
    pub enemies: Vec<Enemy>,
    pub messages: Vec<String>,
    pub alive: bool,
    pub won: bool,
    pub inventory: Vec<Item>,
    pub equipped_weapon: Option<Item>,
    pub equipped_armor: Option<Item>,
    pub equipped_helmet: Option<Item>,
    pub equipped_shield: Option<Item>,
    pub equipped_boots: Option<Item>,
    pub equipped_ring: Option<Item>,
    pub player_defense: i32,
    pub ground_items: Vec<GroundItem>,
    /// Scroll offset for the inventory item list (first visible item index).
    pub inventory_scroll: usize,
    /// Currently selected inventory item index (for detail view / drop).
    pub selected_inventory_item: Option<usize>,
    /// Currently open drawer (slides up from bottom).
    pub drawer: Drawer,
    /// Tile currently being inspected (shown in HUD detail strip).
    pub inspected: Option<TileInfo>,
    /// Player XP and level for progression.
    pub player_xp: u32,
    pub player_level: u32,
    /// Unspent skill points (awarded on level up).
    pub skill_points: u32,
    /// Strength: bonus to melee attack damage.
    pub strength: i32,
    /// Vitality: bonus to max HP.
    pub vitality: i32,
    /// Scroll offset for the stats drawer (in CSS-space pixels).
    pub stats_scroll: f64,
    /// Stamina for sprinting. Max 100, regen 5/turn while walking.
    pub stamina: i32,
    pub max_stamina: i32,
    /// Whether sprint mode is active (toggle).
    pub sprinting: bool,
    /// Hunger. Starts full (100). Decreases 1/turn. 0 = starvation.
    pub hunger: i32,
    pub max_hunger: i32,
    /// Turn counter for hunger tracking.
    pub turn: u32,
    /// Overworld kill counter for XP diminishing returns.
    pub overworld_kills: u32,
    /// Current sprint cost per turn (reduced by Stamina skill).
    pub sprint_cost: i32,
    /// Active floating text indicators.
    pub floating_texts: Vec<FloatingText>,
    /// Active bump/lunge animations.
    pub bump_anims: Vec<BumpAnim>,
    /// Active visual effects.
    pub visual_effects: Vec<VisualEffect>,
    /// Quick-use bar: consumable item slots for one-tap use.
    pub quick_bar: QuickBar,
}

impl Game {
    #[cfg(test)]
    pub fn new(map: Map) -> Self {
        Self::new_with_config(map, GameConfig::normal())
    }

    #[cfg(test)]
    pub fn new_with_config(map: Map, config: GameConfig) -> Self {
        let (px, py) = map.find_spawn();
        let p = &config.player;
        let sprint_cost = config.survival.sprint_cost;
        Self {
            player_x: px,
            player_y: py,
            player_hp: p.starting_hp,
            player_max_hp: p.starting_hp,
            player_attack: p.starting_attack,
            player_dexterity: p.starting_dexterity,
            player_facing_left: false,
            world: World::from_single_map(map),
            enemies: Vec::new(),
            messages: vec!["You enter the cave.".into()],
            alive: true,
            won: false,
            inventory: Vec::new(),
            equipped_weapon: None,
            equipped_armor: None,
            equipped_helmet: None,
            equipped_shield: None,
            equipped_boots: None,
            equipped_ring: None,
            player_defense: 0,
            ground_items: Vec::new(),
            inventory_scroll: 0,
            selected_inventory_item: None,

            drawer: Drawer::None,
            inspected: None,
            player_xp: 0,
            player_level: 1,
            skill_points: 0,
            strength: 0,
            vitality: 0,
            stats_scroll: 0.0,
            stamina: p.starting_stamina,
            max_stamina: p.starting_stamina,
            sprinting: false,
            hunger: p.starting_hunger,
            max_hunger: p.starting_hunger,
            turn: 0,
            overworld_kills: 0,
            sprint_cost,
            floating_texts: Vec::new(),
            bump_anims: Vec::new(),
            visual_effects: Vec::new(),
            quick_bar: QuickBar::new(),
            config,
        }
    }

    #[cfg(test)]
    pub fn new_overworld(world: World) -> Self {
        Self::new_overworld_with_config(world, GameConfig::normal())
    }

    pub fn new_overworld_with_config(world: World, config: GameConfig) -> Self {
        let (px, py) = world.overworld.find_road_spawn();
        let p = &config.player;
        let sprint_cost = config.survival.sprint_cost;
        Self {
            player_x: px,
            player_y: py,
            player_hp: p.starting_hp,
            player_max_hp: p.starting_hp,
            player_attack: p.starting_attack,
            player_dexterity: p.starting_dexterity,
            player_facing_left: false,
            world,
            enemies: Vec::new(),
            messages: vec!["You emerge into the forest.".into()],
            alive: true,
            won: false,
            inventory: Vec::new(),
            equipped_weapon: None,
            equipped_armor: None,
            equipped_helmet: None,
            equipped_shield: None,
            equipped_boots: None,
            equipped_ring: None,
            player_defense: 0,
            ground_items: Vec::new(),
            inventory_scroll: 0,
            selected_inventory_item: None,

            drawer: Drawer::None,
            inspected: None,
            player_xp: 0,
            player_level: 1,
            skill_points: 0,
            strength: 0,
            vitality: 0,
            stats_scroll: 0.0,
            stamina: p.starting_stamina,
            max_stamina: p.starting_stamina,
            sprinting: false,
            hunger: p.starting_hunger,
            max_hunger: p.starting_hunger,
            turn: 0,
            overworld_kills: 0,
            sprint_cost,
            floating_texts: Vec::new(),
            bump_anims: Vec::new(),
            visual_effects: Vec::new(),
            quick_bar: QuickBar::new(),
            config,
        }
    }

    /// Convenience accessor for the current map.
    pub fn current_map(&self) -> &Map {
        self.world.current_map()
    }

    /// FOV radius: configurable per overworld vs dungeon.
    fn fov_radius(&self) -> i32 {
        match self.world.location {
            Location::Overworld => self.config.fov.overworld_radius,
            Location::Dungeon { .. } => self.config.fov.dungeon_radius,
        }
    }

    /// Age Visible→Seen, then recompute FOV from player position.
    pub fn update_fov(&mut self) {
        let r = self.fov_radius();
        let map = self.world.current_map_mut();
        map.age_visibility();
        map.compute_fov(self.player_x, self.player_y, r);
    }

    pub fn location_name(&self) -> String {
        match &self.world.location {
            crate::world::Location::Overworld => "Overworld".into(),
            crate::world::Location::Dungeon { index, level } => {
                let depth = level + 1;
                let total = self.world.dungeons[*index].levels.len();
                if *level == total - 1 && total == 4 {
                    format!("Dragon's Lair (B{})", depth)
                } else {
                    let style_name = self.world.dungeons.get(*index)
                        .and_then(|d| d.styles.get(*level))
                        .map(|s| match s {
                            crate::map::DungeonStyle::DirtCaves => "Dirt Caves",
                            crate::map::DungeonStyle::StoneBrick => "Stone Dungeon",
                            crate::map::DungeonStyle::Igneous => "Volcanic Dungeon",
                            crate::map::DungeonStyle::LargeStone => "Ancient Ruins",
                            crate::map::DungeonStyle::Catacombs => "Catacombs",
                            crate::map::DungeonStyle::DragonLair => "Dragon's Lair",
                        })
                        .unwrap_or("Dungeon");
                    format!("{} {} (B{})", style_name, index + 1, depth)
                }
            }
        }
    }

    pub fn advance_turn(&mut self) {
        if !self.alive || self.won { return; }
        if self.sprinting {
            self.enemy_turn_inner(true);
        } else {
            self.enemy_turn();
        }
        self.tick_survival();
        self.update_fov();
    }

    /// Tick all animations forward by one frame. Returns true if any are still active.
    pub fn tick_animations(&mut self) -> bool {
        for ft in &mut self.floating_texts {
            ft.age += 0.025;
        }
        self.floating_texts.retain(|ft| ft.age < 1.0);

        for ba in &mut self.bump_anims {
            ba.progress += 0.12;
        }
        self.bump_anims.retain(|ba| ba.progress < 1.0);

        for ve in &mut self.visual_effects {
            ve.age += 0.035;
        }
        self.visual_effects.retain(|ve| ve.age < 1.0);

        !self.floating_texts.is_empty() || !self.bump_anims.is_empty() || !self.visual_effects.is_empty()
    }
}

pub(super) fn calc_damage(atk: i32, def: i32) -> i32 {
    if atk <= 0 { return 1; }
    let d = def.max(0);
    ((atk * atk) / (atk + d)).max(1)
}

pub(super) fn xorshift64(mut state: u64) -> u64 {
    state ^= state << 13;
    state ^= state >> 7;
    state ^= state << 17;
    state
}

#[cfg(test)]
mod tests;
