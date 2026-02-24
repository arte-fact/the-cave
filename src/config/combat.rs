#[derive(Clone, Debug)]
pub struct CombatConfig {
    /// Overworld kill thresholds for XP diminishing returns.
    pub xp_diminish_half: u32,
    pub xp_diminish_quarter: u32,

    // --- Dodge ---
    /// Dodge chance per point of dexterity (percentage).
    pub dodge_pct_per_dex: i32,
    /// Maximum dodge chance (percentage).
    pub dodge_cap_pct: i32,

    // --- Enemy AI ---
    /// Manhattan distance within which enemies will chase the player.
    pub enemy_chase_range: i32,
    /// Ranged enemies shoot when Manhattan distance is in [min, max].
    pub enemy_ranged_min: i32,
    pub enemy_ranged_max: i32,
    /// Enemy ranged attack miss chance: if roll >= threshold, arrow misses.
    pub enemy_ranged_miss_threshold: u64,

    // --- Scroll AoE ---
    /// Manhattan distance for scroll area-of-effect damage.
    pub scroll_aoe_range: i32,

    // --- Melee stamina ---
    /// Base stamina cost for melee attacks (unarmed).
    pub melee_stamina_base: i32,
    /// Additional stamina cost per point of weapon weight for melee.
    pub melee_stamina_weight_mult: i32,

    // --- Ranged stamina ---
    /// Base stamina cost for ranged attacks.
    pub ranged_stamina_base: i32,
    /// Additional stamina cost per point of weapon weight for ranged.
    pub ranged_stamina_weight_mult: i32,

    // --- Ranged hit formula ---
    /// Minimum base hit chance (floor) for ranged attacks.
    pub ranged_hit_floor: i32,
    /// Maximum base hit chance (ceiling, before distance falloff).
    pub ranged_hit_ceiling: i32,
    /// Distance falloff numerator: ceiling - distance * falloff / max_range.
    pub ranged_hit_falloff: i32,
    /// Accuracy bonus per point of dexterity.
    pub ranged_accuracy_per_dex: i32,
    /// Hard cap on ranged hit chance.
    pub ranged_hit_cap: i32,

    // --- Ranged damage formula ---
    /// Distance bonus divisor: damage += distance / divisor.
    pub ranged_dist_bonus_divisor: i32,
    /// Dexterity bonus divisor: damage += dex / divisor.
    pub ranged_dex_bonus_divisor: i32,
    /// Dexterity bonus divisor for range extension: range += dex / divisor.
    pub ranged_range_dex_divisor: i32,

    // --- Dragon boss ---
    pub dragon_hp: i32,
    pub dragon_attack: i32,
    pub dragon_defense: i32,
    /// Minimum Manhattan distance from player when placing dragon boss.
    pub dragon_min_distance: i32,
}

impl CombatConfig {
    pub(super) fn normal() -> Self {
        Self {
            xp_diminish_half: 50,
            xp_diminish_quarter: 100,
            dodge_pct_per_dex: 2,
            dodge_cap_pct: 20,
            enemy_chase_range: 8,
            enemy_ranged_min: 2,
            enemy_ranged_max: 4,
            enemy_ranged_miss_threshold: 70,
            scroll_aoe_range: 3,
            melee_stamina_base: 6,
            melee_stamina_weight_mult: 2,
            ranged_stamina_base: 4,
            ranged_stamina_weight_mult: 1,
            ranged_hit_floor: 20,
            ranged_hit_ceiling: 90,
            ranged_hit_falloff: 70,
            ranged_accuracy_per_dex: 3,
            ranged_hit_cap: 95,
            ranged_dist_bonus_divisor: 2,
            ranged_dex_bonus_divisor: 2,
            ranged_range_dex_divisor: 3,
            dragon_hp: 40,
            dragon_attack: 10,
            dragon_defense: 6,
            dragon_min_distance: 5,
        }
    }
}
