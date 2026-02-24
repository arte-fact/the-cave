#[derive(Clone, Debug)]
pub struct ProgressionConfig {
    /// XP formula multiplier: xp_needed = xp_base * level^xp_exponent.
    pub xp_base: f64,
    pub xp_exponent: f64,
    /// Skill points awarded per level up.
    pub skill_points_per_level: u32,
    /// Max HP increase per level up.
    pub hp_per_level: i32,

    // --- Skill bonuses ---
    /// Max HP gained per Vitality point.
    pub vitality_hp_per_point: i32,
    /// Max stamina gained per Stamina skill point.
    pub stamina_per_point: i32,

    // --- Level-up healing ---
    /// Percentage of missing HP healed on level up.
    pub levelup_heal_missing_pct: i32,
    /// Minimum HP healed on level up.
    pub levelup_heal_min: i32,
}

impl ProgressionConfig {
    pub(super) fn normal() -> Self {
        Self {
            xp_base: 20.0,
            xp_exponent: 1.5,
            skill_points_per_level: 3,
            hp_per_level: 2,
            vitality_hp_per_point: 3,
            stamina_per_point: 5,
            levelup_heal_missing_pct: 50,
            levelup_heal_min: 1,
        }
    }
}
