/// Centralized game configuration. All gameplay constants live here
/// so they can be tuned per-difficulty or exposed in settings.

mod combat;
mod enemies;
mod items;
mod mapgen;
mod progression;
mod spawn_tables;

pub use combat::CombatConfig;
pub use enemies::{EnemyBehavior, EnemyDef, ENEMY_DEFS, enemy_def, enemy_behavior, xp_for_enemy, enemy_description};
pub use items::ItemTableConfig;
pub use mapgen::MapGenConfig;
pub use spawn_tables::SpawnTableConfig;
pub use progression::ProgressionConfig;

/// Top-level config holding all sub-configs.
#[derive(Clone, Debug)]
pub struct GameConfig {
    pub player: PlayerConfig,
    pub survival: SurvivalConfig,
    pub progression: ProgressionConfig,
    pub combat: CombatConfig,
    pub fov: FovConfig,
    pub spawn: SpawnConfig,
    pub mapgen: MapGenConfig,
    pub enemies: &'static [EnemyDef],
    pub item_tables: ItemTableConfig,
    pub spawn_tables: SpawnTableConfig,
}

#[derive(Clone, Debug)]
pub struct PlayerConfig {
    pub starting_hp: i32,
    pub starting_attack: i32,
    pub starting_dexterity: i32,
    pub starting_stamina: i32,
    pub starting_hunger: i32,
    pub max_inventory: usize,
}

#[derive(Clone, Debug)]
pub struct SurvivalConfig {
    pub stamina_regen: i32,
    pub hunger_drain: i32,
    /// Hunger drains once every this many turns on the overworld.
    pub hunger_interval_overworld: u32,
    /// Hunger drains once every this many turns in dungeons.
    pub hunger_interval_dungeon: u32,
    /// Hunger drains once every this many turns in the cave (boss floor).
    pub hunger_interval_cave: u32,
    pub starvation_damage: i32,
    /// Hunger must be above this to trigger passive HP regen.
    pub regen_hunger_threshold: i32,
    /// Hunger cost per HP regenerated.
    pub regen_hunger_cost: i32,
}

#[derive(Clone, Debug)]
pub struct FovConfig {
    pub overworld_radius: i32,
    pub dungeon_radius: i32,
}

#[derive(Clone, Debug)]
pub struct SpawnConfig {
    /// Enemy spawn chance per walkable tile on overworld (percentage 0-100).
    pub overworld_enemy_pct: u64,
    /// Item spawn chance per road tile on overworld (per-mille 0-1000).
    pub overworld_item_road_pct: u64,
    /// Item spawn chance per grass tile on overworld (per-mille 0-1000).
    pub overworld_item_grass_pct: u64,
    /// Food spawn chance per grass tile (per-mille 0-1000).
    pub overworld_food_pct: u64,
    /// Enemy spawn chance per floor tile in dungeons (percentage 0-100).
    pub dungeon_enemy_pct: u64,
    /// Enemy spawn chance per floor tile in cave boss level (percentage 0-100).
    pub cave_enemy_pct: u64,
    /// Item spawn chance per floor tile in dungeons (percentage 0-100).
    pub dungeon_item_pct: u64,
    /// Item spawn chance per floor tile in cave (percentage 0-100).
    pub cave_item_pct: u64,
    /// Chebyshev distance around player spawn where no enemies may appear.
    pub spawn_safe_radius: i32,
}

/// Difficulty presets.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    Normal,
    Hard,
}

impl Difficulty {
    pub fn label(self) -> &'static str {
        match self {
            Difficulty::Easy => "Easy",
            Difficulty::Normal => "Normal",
            Difficulty::Hard => "Hard",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Difficulty::Easy => "Relaxed. More HP, slower hunger, weaker foes.",
            Difficulty::Normal => "The intended experience. Balanced challenge.",
            Difficulty::Hard => "Brutal. Less HP, faster hunger, stronger foes.",
        }
    }

    /// Detect difficulty from a config by matching starting HP.
    pub fn from_config(config: &GameConfig) -> Self {
        match config.player.starting_hp {
            30 => Difficulty::Easy,
            15 => Difficulty::Hard,
            _ => Difficulty::Normal,
        }
    }
}

impl GameConfig {
    /// The default (Normal) configuration — matches the original hardcoded values.
    pub fn normal() -> Self {
        Self {
            player: PlayerConfig {
                starting_hp: 20,
                starting_attack: 5,
                starting_dexterity: 3,
                starting_stamina: 100,
                starting_hunger: 100,
                max_inventory: 10,
            },
            survival: SurvivalConfig {
                stamina_regen: 5,
                hunger_drain: 1,
                hunger_interval_overworld: 5,
                hunger_interval_dungeon: 3,
                hunger_interval_cave: 2,
                starvation_damage: 1,
                regen_hunger_threshold: 50,
                regen_hunger_cost: 2,
            },
            progression: ProgressionConfig::normal(),
            combat: CombatConfig::normal(),
            fov: FovConfig {
                overworld_radius: 8,
                dungeon_radius: 6,
            },
            spawn: SpawnConfig {
                overworld_enemy_pct: 2,
                overworld_item_road_pct: 3,
                overworld_item_grass_pct: 1,
                overworld_food_pct: 8,
                dungeon_enemy_pct: 8,
                cave_enemy_pct: 5,
                dungeon_item_pct: 2,
                cave_item_pct: 1,
                spawn_safe_radius: 5,
            },
            mapgen: MapGenConfig::normal(),
            enemies: ENEMY_DEFS,
            item_tables: ItemTableConfig::normal(),
            spawn_tables: SpawnTableConfig::normal(),
        }
    }

    pub fn easy() -> Self {
        let mut cfg = Self::normal();
        // More forgiving player stats
        cfg.player.starting_hp = 30;
        cfg.player.starting_attack = 6;
        cfg.player.starting_stamina = 120;
        // Slower hunger drain
        cfg.survival.hunger_interval_overworld = 7;
        cfg.survival.hunger_interval_dungeon = 5;
        cfg.survival.hunger_interval_cave = 3;
        cfg.survival.regen_hunger_threshold = 30;
        // More generous progression
        cfg.progression.skill_points_per_level = 4;
        cfg.progression.hp_per_level = 3;
        // Fewer enemies, more items
        cfg.spawn.overworld_enemy_pct = 1;
        cfg.spawn.dungeon_enemy_pct = 5;
        cfg.spawn.cave_enemy_pct = 3;
        cfg.spawn.dungeon_item_pct = 3;
        cfg.spawn.overworld_food_pct = 12;
        cfg.spawn.spawn_safe_radius = 7;
        cfg
    }

    pub fn hard() -> Self {
        let mut cfg = Self::normal();
        // Tougher starting conditions
        cfg.player.starting_hp = 15;
        cfg.player.starting_attack = 4;
        cfg.player.starting_stamina = 80;
        cfg.player.starting_hunger = 80;
        // Faster hunger drain
        cfg.survival.hunger_interval_overworld = 4;
        cfg.survival.hunger_interval_dungeon = 2;
        cfg.survival.hunger_interval_cave = 1;
        cfg.survival.starvation_damage = 2;
        cfg.survival.regen_hunger_threshold = 60;
        cfg.survival.regen_hunger_cost = 3;
        // Slower progression
        cfg.progression.xp_base = 25.0;
        cfg.progression.skill_points_per_level = 2;
        cfg.progression.hp_per_level = 1;
        // More enemies, fewer items
        cfg.spawn.overworld_enemy_pct = 3;
        cfg.spawn.dungeon_enemy_pct = 11;
        cfg.spawn.cave_enemy_pct = 7;
        cfg.spawn.dungeon_item_pct = 1;
        cfg.spawn.overworld_food_pct = 5;
        cfg.spawn.spawn_safe_radius = 3;
        cfg
    }

    pub fn from_difficulty(difficulty: Difficulty) -> Self {
        match difficulty {
            Difficulty::Easy => Self::easy(),
            Difficulty::Normal => Self::normal(),
            Difficulty::Hard => Self::hard(),
        }
    }
}

#[cfg(test)]
mod tests;
