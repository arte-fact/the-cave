/// Item-related configuration: tier bleed percentages, ranged weapon base
/// ranges, and starting equipment definitions.

/// Tunable item generation and equipment parameters.
#[derive(Clone, Debug)]
pub struct ItemTableConfig {
    /// Chance (0-100) that a dungeon item rolls one tier higher than the level.
    pub tier_bleed_up_pct: u64,
    /// Chance (0-100) that a dungeon item rolls one tier lower than the level.
    /// Applied only when the bleed-up roll fails: threshold = bleed_up + bleed_down.
    pub tier_bleed_down_pct: u64,

    /// Base range for each ranged weapon (before dexterity bonus).
    /// Looked up by weapon name; falls back to `ranged_default_range`.
    pub ranged_base_ranges: &'static [(&'static str, i32)],
    /// Default base range if weapon not found in ranged_base_ranges.
    pub ranged_default_range: i32,
}

/// Base ranges for ranged weapons. Order doesn't matter.
pub static RANGED_BASE_RANGES: &[(&str, i32)] = &[
    ("Short Bow", 4),
    ("Crossbow", 3),
    ("Long Bow", 6),
    ("Heavy Crossbow", 4),
    ("Elven Bow", 8),
];

impl ItemTableConfig {
    pub(super) fn normal() -> Self {
        Self {
            tier_bleed_up_pct: 20,
            tier_bleed_down_pct: 10,
            ranged_base_ranges: RANGED_BASE_RANGES,
            ranged_default_range: 4,
        }
    }
}
