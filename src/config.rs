/// Centralized game configuration. All gameplay constants live here
/// so they can be tuned per-difficulty or exposed in settings.
/// Top-level config holding all sub-configs.
#[derive(Clone, Debug)]
pub struct GameConfig {
    pub player: PlayerConfig,
    pub survival: SurvivalConfig,
    pub progression: ProgressionConfig,
    pub combat: CombatConfig,
    pub fov: FovConfig,
    pub spawn: SpawnConfig,
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
    pub sprint_cost: i32,
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
pub struct ProgressionConfig {
    /// XP formula multiplier: xp_needed = xp_base * level^xp_exponent.
    pub xp_base: f64,
    pub xp_exponent: f64,
    /// Skill points awarded per level up.
    pub skill_points_per_level: u32,
    /// Max HP increase per level up.
    pub hp_per_level: i32,
}

#[derive(Clone, Debug)]
pub struct CombatConfig {
    /// Overworld kill thresholds for XP diminishing returns.
    pub xp_diminish_half: u32,
    pub xp_diminish_quarter: u32,
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
                sprint_cost: 15,
                stamina_regen: 5,
                hunger_drain: 1,
                hunger_interval_overworld: 5,
                hunger_interval_dungeon: 3,
                hunger_interval_cave: 2,
                starvation_damage: 1,
                regen_hunger_threshold: 50,
                regen_hunger_cost: 2,
            },
            progression: ProgressionConfig {
                xp_base: 20.0,
                xp_exponent: 1.5,
                skill_points_per_level: 3,
                hp_per_level: 2,
            },
            combat: CombatConfig {
                xp_diminish_half: 50,
                xp_diminish_quarter: 100,
            },
            fov: FovConfig {
                overworld_radius: 8,
                dungeon_radius: 6,
            },
            spawn: SpawnConfig {
                overworld_enemy_pct: 3,
                overworld_item_road_pct: 3,
                overworld_item_grass_pct: 1,
                overworld_food_pct: 8,
                dungeon_enemy_pct: 10,
                cave_enemy_pct: 6,
                dungeon_item_pct: 2,
                cave_item_pct: 1,
            },
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
        cfg.spawn.overworld_enemy_pct = 2;
        cfg.spawn.dungeon_enemy_pct = 7;
        cfg.spawn.cave_enemy_pct = 4;
        cfg.spawn.dungeon_item_pct = 3;
        cfg.spawn.overworld_food_pct = 12;
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
        cfg.spawn.overworld_enemy_pct = 4;
        cfg.spawn.dungeon_enemy_pct = 14;
        cfg.spawn.cave_enemy_pct = 8;
        cfg.spawn.dungeon_item_pct = 1;
        cfg.spawn.overworld_food_pct = 5;
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
mod tests {
    use super::*;

    #[test]
    fn normal_defaults_match_original_hardcoded_values() {
        let cfg = GameConfig::normal();
        assert_eq!(cfg.player.starting_hp, 20);
        assert_eq!(cfg.player.starting_attack, 5);
        assert_eq!(cfg.player.starting_dexterity, 3);
        assert_eq!(cfg.player.starting_stamina, 100);
        assert_eq!(cfg.player.starting_hunger, 100);
        assert_eq!(cfg.player.max_inventory, 10);
        assert_eq!(cfg.survival.sprint_cost, 15);
        assert_eq!(cfg.survival.stamina_regen, 5);
        assert_eq!(cfg.survival.hunger_drain, 1);
        assert_eq!(cfg.survival.hunger_interval_overworld, 5);
        assert_eq!(cfg.survival.hunger_interval_dungeon, 3);
        assert_eq!(cfg.survival.hunger_interval_cave, 2);
        assert_eq!(cfg.survival.starvation_damage, 1);
        assert_eq!(cfg.survival.regen_hunger_threshold, 50);
        assert_eq!(cfg.survival.regen_hunger_cost, 2);
        assert_eq!(cfg.fov.overworld_radius, 8);
        assert_eq!(cfg.fov.dungeon_radius, 6);
        assert_eq!(cfg.spawn.overworld_enemy_pct, 3);
    }

    #[test]
    fn easy_is_more_forgiving_than_normal() {
        let easy = GameConfig::easy();
        let normal = GameConfig::normal();
        assert!(easy.player.starting_hp > normal.player.starting_hp);
        assert!(easy.survival.hunger_interval_overworld > normal.survival.hunger_interval_overworld);
        assert!(easy.spawn.overworld_enemy_pct < normal.spawn.overworld_enemy_pct);
        assert!(easy.progression.skill_points_per_level > normal.progression.skill_points_per_level);
    }

    #[test]
    fn hard_is_tougher_than_normal() {
        let hard = GameConfig::hard();
        let normal = GameConfig::normal();
        assert!(hard.player.starting_hp < normal.player.starting_hp);
        assert!(hard.survival.hunger_interval_overworld < normal.survival.hunger_interval_overworld);
        assert!(hard.spawn.overworld_enemy_pct > normal.spawn.overworld_enemy_pct);
        assert!(hard.survival.starvation_damage > normal.survival.starvation_damage);
    }

    #[test]
    fn from_difficulty_returns_correct_preset() {
        let easy = GameConfig::from_difficulty(Difficulty::Easy);
        let normal = GameConfig::from_difficulty(Difficulty::Normal);
        let hard = GameConfig::from_difficulty(Difficulty::Hard);
        assert_eq!(easy.player.starting_hp, 30);
        assert_eq!(normal.player.starting_hp, 20);
        assert_eq!(hard.player.starting_hp, 15);
    }

    #[test]
    fn difficulty_labels() {
        assert_eq!(Difficulty::Easy.label(), "Easy");
        assert_eq!(Difficulty::Normal.label(), "Normal");
        assert_eq!(Difficulty::Hard.label(), "Hard");
    }
}
