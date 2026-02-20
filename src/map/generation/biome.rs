use super::dungeon::DungeonStyle;
use super::xorshift64;

/// A dungeon biome determines the thematic identity of a dungeon:
/// its visual style per level, enemy roster, and boss encounter.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DungeonBiome {
    /// Chaotic tunnels, goblins and kobolds, Orc Warchief boss.
    GoblinWarren,
    /// Cold catacombs, undead enemies, Lich boss.
    UndeadCrypt,
    /// Damp organic caves, myconids and slimes, Writhing Mass boss.
    FungalGrotto,
    /// Fortified orcish stronghold, orcs and trolls, Two-Headed Ettin boss.
    OrcStronghold,
    /// Dark eldritch temple, cultists and wraiths, Reaper boss.
    AbyssalTemple,
    /// Volcanic dragon's lair. Only appears as deepest level of 4-level dungeons.
    DragonLair,
    /// Natural beast caves, wolves and spiders, Wendigo boss.
    BeastDen,
    /// Winding serpent tunnels, reptilians and nagas, Basilisk boss.
    SerpentPit,
}

/// Overworld biome regions — determined by position on the map.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OverworldBiome {
    /// Northern/central region: temperate woodland.
    TemperateForest,
    /// Southern region: dense tropical jungle.
    Jungle,
}

impl DungeonBiome {
    /// All biome variants (excluding DragonLair, which is special-cased).
    pub const PLACEABLE: [DungeonBiome; 7] = [
        DungeonBiome::GoblinWarren,
        DungeonBiome::UndeadCrypt,
        DungeonBiome::FungalGrotto,
        DungeonBiome::OrcStronghold,
        DungeonBiome::AbyssalTemple,
        DungeonBiome::BeastDen,
        DungeonBiome::SerpentPit,
    ];

    /// Select a biome for a dungeon based on seed and overworld position.
    pub fn for_dungeon(seed: u64, entrance_y: i32, map_height: i32) -> Self {
        let overworld = OverworldBiome::at_y(entrance_y, map_height);

        // Filter biomes by overworld region affinity
        let candidates: &[DungeonBiome] = match overworld {
            OverworldBiome::TemperateForest => &[
                DungeonBiome::GoblinWarren,
                DungeonBiome::UndeadCrypt,
                DungeonBiome::OrcStronghold,
                DungeonBiome::BeastDen,
                DungeonBiome::FungalGrotto,
            ],
            OverworldBiome::Jungle => &[
                DungeonBiome::SerpentPit,
                DungeonBiome::UndeadCrypt,
                DungeonBiome::FungalGrotto,
                DungeonBiome::AbyssalTemple,
                DungeonBiome::GoblinWarren,
            ],
        };

        let rng = xorshift64(seed);
        candidates[(rng % candidates.len() as u64) as usize]
    }

    /// Get the visual style for a specific level of this biome.
    pub fn style_for_level(&self, level: usize, is_cave: bool) -> DungeonStyle {
        if is_cave {
            return DungeonStyle::RedCavern;
        }
        match self {
            DungeonBiome::GoblinWarren => match level {
                0 | 1 => DungeonStyle::DirtCaves,
                _ => DungeonStyle::StoneBrick,
            },
            DungeonBiome::UndeadCrypt => match level {
                0 | 1 => DungeonStyle::Catacombs,
                _ => DungeonStyle::BoneCrypt,
            },
            DungeonBiome::FungalGrotto => DungeonStyle::MossyCavern,
            DungeonBiome::OrcStronghold => match level {
                0 => DungeonStyle::StoneBrick,
                _ => DungeonStyle::LargeStone,
            },
            DungeonBiome::AbyssalTemple => match level {
                0 => DungeonStyle::Igneous,
                _ => DungeonStyle::BlueTemple,
            },
            DungeonBiome::DragonLair => DungeonStyle::RedCavern,
            DungeonBiome::BeastDen => match level {
                0 | 1 => DungeonStyle::BoneCave,
                _ => DungeonStyle::BoneCrypt,
            },
            DungeonBiome::SerpentPit => match level {
                0 | 1 => DungeonStyle::MossyTunnel,
                _ => DungeonStyle::MossyCavern,
            },
        }
    }

    /// Human-readable name for this biome.
    pub fn name(&self) -> &'static str {
        match self {
            DungeonBiome::GoblinWarren => "Goblin Warren",
            DungeonBiome::UndeadCrypt => "Undead Crypt",
            DungeonBiome::FungalGrotto => "Fungal Grotto",
            DungeonBiome::OrcStronghold => "Orc Stronghold",
            DungeonBiome::AbyssalTemple => "Abyssal Temple",
            DungeonBiome::DragonLair => "Dragon's Lair",
            DungeonBiome::BeastDen => "Beast Den",
            DungeonBiome::SerpentPit => "Serpent Pit",
        }
    }
}

impl OverworldBiome {
    /// Determine overworld biome from y-coordinate.
    /// Top 60% = Temperate, bottom 40% = Jungle.
    pub fn at_y(y: i32, map_height: i32) -> Self {
        let jungle_start = map_height * 60 / 100;
        if y >= jungle_start {
            OverworldBiome::Jungle
        } else {
            OverworldBiome::TemperateForest
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // --- OverworldBiome ---

    #[test]
    fn overworld_biome_north_is_temperate() {
        assert_eq!(OverworldBiome::at_y(0, 200), OverworldBiome::TemperateForest);
        assert_eq!(OverworldBiome::at_y(50, 200), OverworldBiome::TemperateForest);
        assert_eq!(OverworldBiome::at_y(119, 200), OverworldBiome::TemperateForest);
    }

    #[test]
    fn overworld_biome_south_is_jungle() {
        assert_eq!(OverworldBiome::at_y(120, 200), OverworldBiome::Jungle);
        assert_eq!(OverworldBiome::at_y(150, 200), OverworldBiome::Jungle);
        assert_eq!(OverworldBiome::at_y(199, 200), OverworldBiome::Jungle);
    }

    // --- DungeonBiome selection ---

    #[test]
    fn biome_selection_is_deterministic() {
        let a = DungeonBiome::for_dungeon(42, 50, 200);
        let b = DungeonBiome::for_dungeon(42, 50, 200);
        assert_eq!(a, b);
    }

    #[test]
    fn different_seeds_can_produce_different_biomes() {
        let mut seen = std::collections::HashSet::new();
        for seed in 0..100 {
            seen.insert(DungeonBiome::for_dungeon(seed, 50, 200));
        }
        assert!(seen.len() >= 3, "should produce at least 3 distinct biomes, got {}", seen.len());
    }

    #[test]
    fn temperate_dungeons_dont_include_serpent_pit() {
        // SerpentPit is jungle-only
        for seed in 0..200 {
            let biome = DungeonBiome::for_dungeon(seed, 10, 200);
            assert_ne!(biome, DungeonBiome::SerpentPit,
                "SerpentPit should not appear in temperate zone (seed={seed})");
        }
    }

    #[test]
    fn jungle_dungeons_dont_include_orc_stronghold() {
        // OrcStronghold is temperate-only
        for seed in 0..200 {
            let biome = DungeonBiome::for_dungeon(seed, 180, 200);
            assert_ne!(biome, DungeonBiome::OrcStronghold,
                "OrcStronghold should not appear in jungle zone (seed={seed})");
        }
    }

    #[test]
    fn biome_never_dragon_lair() {
        // DragonLair is never selected by for_dungeon (it's added separately)
        for seed in 0..500 {
            let biome = DungeonBiome::for_dungeon(seed, 100, 200);
            assert_ne!(biome, DungeonBiome::DragonLair);
        }
    }

    // --- Style per level ---

    #[test]
    fn goblin_warren_styles() {
        let b = DungeonBiome::GoblinWarren;
        assert_eq!(b.style_for_level(0, false), DungeonStyle::DirtCaves);
        assert_eq!(b.style_for_level(1, false), DungeonStyle::DirtCaves);
        assert_eq!(b.style_for_level(2, false), DungeonStyle::StoneBrick);
    }

    #[test]
    fn undead_crypt_styles() {
        let b = DungeonBiome::UndeadCrypt;
        assert_eq!(b.style_for_level(0, false), DungeonStyle::Catacombs);
        assert_eq!(b.style_for_level(2, false), DungeonStyle::BoneCrypt);
    }

    #[test]
    fn fungal_grotto_uses_mossy_style() {
        let b = DungeonBiome::FungalGrotto;
        assert_eq!(b.style_for_level(0, false), DungeonStyle::MossyCavern);
        assert_eq!(b.style_for_level(2, false), DungeonStyle::MossyCavern);
    }

    #[test]
    fn abyssal_temple_uses_blue_stone() {
        let b = DungeonBiome::AbyssalTemple;
        assert_eq!(b.style_for_level(1, false), DungeonStyle::BlueTemple);
        assert_eq!(b.style_for_level(2, false), DungeonStyle::BlueTemple);
    }

    #[test]
    fn beast_den_uses_bone_cave() {
        let b = DungeonBiome::BeastDen;
        assert_eq!(b.style_for_level(0, false), DungeonStyle::BoneCave);
        assert_eq!(b.style_for_level(2, false), DungeonStyle::BoneCrypt);
    }

    #[test]
    fn serpent_pit_uses_mossy_tunnel() {
        let b = DungeonBiome::SerpentPit;
        assert_eq!(b.style_for_level(0, false), DungeonStyle::MossyTunnel);
    }

    #[test]
    fn cave_level_always_red_cavern_style() {
        for biome in DungeonBiome::PLACEABLE {
            assert_eq!(biome.style_for_level(3, true), DungeonStyle::RedCavern,
                "{:?} cave level should be RedCavern", biome);
        }
    }

    // --- Names ---

    #[test]
    fn every_biome_has_nonempty_name() {
        for biome in DungeonBiome::PLACEABLE {
            assert!(!biome.name().is_empty(), "{:?} has empty name", biome);
        }
        assert_eq!(DungeonBiome::DragonLair.name(), "Dragon's Lair");
    }
}
