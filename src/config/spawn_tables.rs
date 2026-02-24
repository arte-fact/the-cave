/// Spawn-related configuration: rare monster lists, loot tier assignments.

/// Configuration for spawn behavior beyond density (which is in SpawnConfig).
#[derive(Clone, Debug)]
pub struct SpawnTableConfig {
    /// Names of overworld enemies considered "rare monsters" (mini-boss encounters).
    /// Used for loot drops and detection.
    pub rare_monster_names: &'static [&'static str],

    /// Loot tier for rare overworld monsters. (enemy_name, tier).
    /// Monsters not listed here don't drop loot.
    pub monster_loot_tiers: &'static [(&'static str, usize)],
}

impl SpawnTableConfig {
    pub(super) fn normal() -> Self {
        Self {
            rare_monster_names: &[
                "Dryad", "Forest Spirit", "Centaur", "Dire Wolf", "Lycanthrope", "Wendigo",
            ],
            monster_loot_tiers: &[
                ("Dryad", 1),
                ("Forest Spirit", 1),
                ("Dire Wolf", 1),
                ("Centaur", 2),
                ("Lycanthrope", 2),
                ("Wendigo", 2),
            ],
        }
    }
}
