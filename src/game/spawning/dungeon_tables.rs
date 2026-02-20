use crate::map::DungeonBiome;
use super::super::types::Enemy;

type EnemyStats = (i32, i32, i32, char, &'static str, bool);

/// Roll an enemy based on the dungeon biome and level.
pub(super) fn roll_biome_enemy(x: i32, y: i32, biome: DungeonBiome, level: usize, rng: u64) -> Enemy {
    let roll = rng % 100;
    let stats = match biome {
        DungeonBiome::GoblinWarren => roll_goblin_warren(level, roll),
        DungeonBiome::UndeadCrypt => roll_undead_crypt(level, roll),
        DungeonBiome::FungalGrotto => roll_fungal_grotto(level, roll),
        DungeonBiome::OrcStronghold => roll_orc_stronghold(level, roll),
        DungeonBiome::AbyssalTemple => roll_abyssal_temple(level, roll),
        DungeonBiome::BeastDen => roll_beast_den(level, roll),
        DungeonBiome::SerpentPit => roll_serpent_pit(level, roll),
        DungeonBiome::DragonLair => roll_cave_stats(roll),
    };
    let (hp, attack, def, glyph, name, ranged) = stats;
    Enemy { x, y, hp, attack, defense: def, glyph, name, facing_left: false, is_ranged: ranged }
}

/// Roll a cave-level (dragon's lair) enemy.
pub(super) fn roll_cave_enemy(x: i32, y: i32, rng: u64) -> Enemy {
    let roll = rng % 100;
    let (hp, attack, def, glyph, name, ranged) = roll_cave_stats(roll);
    Enemy { x, y, hp, attack, defense: def, glyph, name, facing_left: false, is_ranged: ranged }
}

fn roll_cave_stats(roll: u64) -> EnemyStats {
    if roll < 20 {
        (20, 7, 5, 'K', "Death Knight", false)
    } else if roll < 35 {
        (16, 5, 3, 'T', "Troll", false)
    } else if roll < 50 {
        (15, 8, 2, 'l', "Lich", false)
    } else if roll < 60 {
        (14, 6, 3, 'd', "Drake", false)
    } else if roll < 70 {
        (16, 7, 4, 'C', "Basilisk", false)
    } else if roll < 80 {
        (10, 6, 1, 'I', "Imp", false)
    } else if roll < 90 {
        (18, 7, 4, 'X', "Manticore", false)
    } else {
        (20, 9, 3, 'V', "Reaper", false)
    }
}

// === Goblin Warren ===
// L0: Kobolds, Giant Rats, Small Slimes, Goblins
// L1: Goblin Archers, Goblin Mages, Big Slimes, Goblin Brutes
// L2: Orcs, Orc Blademasters, Trolls
// Boss: Orc Warchief

fn roll_goblin_warren(level: usize, roll: u64) -> EnemyStats {
    match level {
        0 => roll_goblin_warren_l0(roll),
        1 => roll_goblin_warren_l1(roll),
        _ => roll_goblin_warren_deep(roll),
    }
}

fn roll_goblin_warren_l0(roll: u64) -> EnemyStats {
    if roll < 15 {
        (3, 1, 0, 'r', "Giant Rat", false)
    } else if roll < 30 {
        (4, 2, 1, 'c', "Kobold", false)
    } else if roll < 42 {
        (4, 1, 0, 'S', "Small Slime", false)
    } else if roll < 70 {
        (5, 2, 1, 'g', "Goblin", false)
    } else if roll < 85 {
        (4, 2, 0, 'e', "Giant Centipede", false)
    } else {
        (6, 3, 2, 's', "Skeleton", false)
    }
}

fn roll_goblin_warren_l1(roll: u64) -> EnemyStats {
    if roll < 15 {
        (6, 3, 1, 'G', "Goblin Archer", true)
    } else if roll < 30 {
        (7, 5, 1, 'M', "Goblin Mage", true)
    } else if roll < 45 {
        (10, 2, 0, 'm', "Big Slime", false)
    } else if roll < 65 {
        (6, 3, 1, '3', "Goblin Brute", false)
    } else if roll < 80 {
        (5, 2, 1, 'g', "Goblin", false)
    } else {
        (10, 4, 3, 'o', "Orc", false)
    }
}

fn roll_goblin_warren_deep(roll: u64) -> EnemyStats {
    if roll < 20 {
        (10, 4, 3, 'o', "Orc", false)
    } else if roll < 40 {
        (14, 5, 4, 'O', "Orc Blademaster", false)
    } else if roll < 55 {
        (16, 5, 3, 'T', "Troll", false)
    } else if roll < 70 {
        (6, 3, 1, 'G', "Goblin Archer", true)
    } else if roll < 85 {
        (7, 5, 1, 'M', "Goblin Mage", true)
    } else {
        (12, 5, 4, '5', "Orc Warchief", false)
    }
}

// === Undead Crypt ===
// L0: Skeletons, Zombies, Giant Rats, Giant Bats
// L1: Skeleton Archers, Ghouls, Wraiths, Hag Witch
// L2: Banshees, Death Knights, Unholy Cardinals
// Boss: Lich

fn roll_undead_crypt(level: usize, roll: u64) -> EnemyStats {
    match level {
        0 => roll_undead_crypt_l0(roll),
        1 => roll_undead_crypt_l1(roll),
        _ => roll_undead_crypt_deep(roll),
    }
}

fn roll_undead_crypt_l0(roll: u64) -> EnemyStats {
    if roll < 25 {
        (6, 3, 2, 's', "Skeleton", false)
    } else if roll < 45 {
        (10, 2, 1, 'z', "Zombie", false)
    } else if roll < 60 {
        (3, 1, 0, 'r', "Giant Rat", false)
    } else if roll < 75 {
        (4, 2, 0, 'a', "Giant Bat", false)
    } else if roll < 88 {
        (4, 1, 0, 'S', "Small Slime", false)
    } else {
        (4, 2, 0, 'e', "Giant Centipede", false)
    }
}

fn roll_undead_crypt_l1(roll: u64) -> EnemyStats {
    if roll < 20 {
        (7, 4, 2, 'k', "Skeleton Archer", true)
    } else if roll < 40 {
        (10, 5, 2, 'u', "Ghoul", false)
    } else if roll < 55 {
        (8, 6, 0, 'W', "Wraith", false)
    } else if roll < 70 {
        (9, 4, 1, 'H', "Hag", false)
    } else if roll < 85 {
        (10, 2, 1, 'z', "Zombie", false)
    } else {
        (6, 3, 2, 's', "Skeleton", false)
    }
}

fn roll_undead_crypt_deep(roll: u64) -> EnemyStats {
    if roll < 20 {
        (10, 6, 1, 'Q', "Banshee", false)
    } else if roll < 40 {
        (20, 7, 5, 'K', "Death Knight", false)
    } else if roll < 55 {
        (14, 7, 3, '7', "Unholy Cardinal", false)
    } else if roll < 70 {
        (8, 6, 0, 'W', "Wraith", false)
    } else if roll < 85 {
        (10, 5, 2, 'u', "Ghoul", false)
    } else {
        (15, 8, 2, 'l', "Lich", false)
    }
}

// === Fungal Grotto ===
// L0: Small Myconids, Small Slimes, Giant Centipedes, Giant Earthworms
// L1: Large Myconids, Big Slimes, Giant Ants, Lampreymanders
// L2: Writhing Masses (small + large), Writhing Humanoid
// Boss: Large Writhing Mass

fn roll_fungal_grotto(level: usize, roll: u64) -> EnemyStats {
    match level {
        0 => roll_fungal_grotto_l0(roll),
        1 => roll_fungal_grotto_l1(roll),
        _ => roll_fungal_grotto_deep(roll),
    }
}

fn roll_fungal_grotto_l0(roll: u64) -> EnemyStats {
    if roll < 25 {
        (3, 1, 1, 'p', "Myconid", false)
    } else if roll < 45 {
        (4, 1, 0, 'S', "Small Slime", false)
    } else if roll < 60 {
        (4, 2, 0, 'e', "Giant Centipede", false)
    } else if roll < 75 {
        // Giant Earthworm — activates GiantEarthworm sprite
        (5, 2, 0, ']', "Giant Earthworm", false)
    } else if roll < 88 {
        (3, 1, 0, 'r', "Giant Rat", false)
    } else {
        (4, 2, 1, 't', "Large Myconid", false)
    }
}

fn roll_fungal_grotto_l1(roll: u64) -> EnemyStats {
    if roll < 20 {
        (4, 2, 1, 't', "Large Myconid", false)
    } else if roll < 40 {
        (10, 2, 0, 'm', "Big Slime", false)
    } else if roll < 55 {
        (8, 3, 2, 'A', "Giant Ant", false)
    } else if roll < 70 {
        // Lampreymander — activates Lampreymander sprite
        (8, 4, 1, '[', "Lampreymander", false)
    } else if roll < 85 {
        (4, 2, 0, 'e', "Giant Centipede", false)
    } else {
        (5, 2, 0, ']', "Giant Earthworm", false)
    }
}

fn roll_fungal_grotto_deep(roll: u64) -> EnemyStats {
    if roll < 20 {
        // Small Writhing Mass — activates SmallWrithingMass sprite
        (10, 5, 2, '(', "Small Writhing Mass", false)
    } else if roll < 40 {
        (15, 6, 2, '8', "Writhing Mass", false)
    } else if roll < 55 {
        // Writhing Humanoid — activates WrithingHumanoid sprite
        (12, 6, 1, ')', "Writhing Humanoid", false)
    } else if roll < 70 {
        (10, 2, 0, 'm', "Big Slime", false)
    } else if roll < 85 {
        (8, 4, 1, '[', "Lampreymander", false)
    } else {
        (4, 2, 1, 't', "Large Myconid", false)
    }
}

// === Orc Stronghold ===
// L0: Orcs, Kobolds (canine), Goblin Brutes
// L1: Orc Blademasters, Orc Wizards, Trolls, Giant Ants
// L2: Ettins, Two-Headed Ettins, Orc Warchiefs
// Boss: Two-Headed Ettin

fn roll_orc_stronghold(level: usize, roll: u64) -> EnemyStats {
    match level {
        0 => roll_orc_stronghold_l0(roll),
        1 => roll_orc_stronghold_l1(roll),
        _ => roll_orc_stronghold_deep(roll),
    }
}

fn roll_orc_stronghold_l0(roll: u64) -> EnemyStats {
    if roll < 30 {
        (10, 4, 3, 'o', "Orc", false)
    } else if roll < 50 {
        // Kobold Canine — activates KoboldCanine sprite
        (6, 3, 1, '{', "Kobold", false)
    } else if roll < 70 {
        (6, 3, 1, '3', "Goblin Brute", false)
    } else if roll < 85 {
        (5, 2, 1, 'g', "Goblin", false)
    } else {
        (8, 3, 2, 'A', "Giant Ant", false)
    }
}

fn roll_orc_stronghold_l1(roll: u64) -> EnemyStats {
    if roll < 20 {
        (14, 5, 4, 'O', "Orc Blademaster", false)
    } else if roll < 35 {
        // Orc Wizard — activates OrcWizard sprite
        (10, 6, 2, '}', "Orc Wizard", true)
    } else if roll < 50 {
        (16, 5, 3, 'T', "Troll", false)
    } else if roll < 65 {
        (8, 3, 2, 'A', "Giant Ant", false)
    } else if roll < 80 {
        (10, 4, 3, 'o', "Orc", false)
    } else {
        (6, 3, 1, '3', "Goblin Brute", false)
    }
}

fn roll_orc_stronghold_deep(roll: u64) -> EnemyStats {
    if roll < 25 {
        (18, 6, 4, 'E', "Ettin", false)
    } else if roll < 40 {
        // Two-Headed Ettin — activates TwoHeadedEttin sprite
        (22, 7, 5, '^', "Two-Headed Ettin", false)
    } else if roll < 55 {
        (12, 5, 4, '5', "Orc Warchief", false)
    } else if roll < 70 {
        (14, 5, 4, 'O', "Orc Blademaster", false)
    } else if roll < 85 {
        (16, 5, 3, 'T', "Troll", false)
    } else {
        (10, 6, 2, '}', "Orc Wizard", true)
    }
}

// === Abyssal Temple ===
// L0: Cultists, Faceless Monks, Small Slimes
// L1: Unholy Cardinals, Hag Witches, Wraiths
// L2: Writhing Humanoids, Imps, Nagas
// Boss: Reaper

fn roll_abyssal_temple(level: usize, roll: u64) -> EnemyStats {
    match level {
        0 => roll_abyssal_temple_l0(roll),
        1 => roll_abyssal_temple_l1(roll),
        _ => roll_abyssal_temple_deep(roll),
    }
}

fn roll_abyssal_temple_l0(roll: u64) -> EnemyStats {
    if roll < 30 {
        // Cultist — activates Cultist sprite
        (6, 3, 1, '!', "Cultist", false)
    } else if roll < 55 {
        (11, 5, 2, '6', "Faceless Monk", false)
    } else if roll < 75 {
        (4, 1, 0, 'S', "Small Slime", false)
    } else if roll < 88 {
        (8, 6, 0, 'W', "Wraith", false)
    } else {
        (4, 2, 0, 'e', "Giant Centipede", false)
    }
}

fn roll_abyssal_temple_l1(roll: u64) -> EnemyStats {
    if roll < 25 {
        (14, 7, 3, '7', "Unholy Cardinal", false)
    } else if roll < 45 {
        (9, 4, 1, 'H', "Hag", false)
    } else if roll < 60 {
        (8, 6, 0, 'W', "Wraith", false)
    } else if roll < 75 {
        (11, 5, 2, '6', "Faceless Monk", false)
    } else if roll < 88 {
        (6, 3, 1, '!', "Cultist", false)
    } else {
        (10, 2, 0, 'm', "Big Slime", false)
    }
}

fn roll_abyssal_temple_deep(roll: u64) -> EnemyStats {
    if roll < 20 {
        (12, 6, 1, ')', "Writhing Humanoid", false)
    } else if roll < 35 {
        (10, 6, 1, 'I', "Imp", false)
    } else if roll < 50 {
        (12, 6, 3, 'N', "Naga", false)
    } else if roll < 65 {
        (14, 7, 3, '7', "Unholy Cardinal", false)
    } else if roll < 80 {
        (8, 6, 0, 'W', "Wraith", false)
    } else {
        (20, 9, 3, 'V', "Reaper", false)
    }
}

// === Beast Den ===
// L0: Giant Rats, Giant Bats, Lesser Giant Spiders, Cobras
// L1: Giant Spiders, Dire Wolves, Lycanthropes
// L2: Wendigos, Manticores
// Boss: Wendigo

fn roll_beast_den(level: usize, roll: u64) -> EnemyStats {
    match level {
        0 => roll_beast_den_l0(roll),
        1 => roll_beast_den_l1(roll),
        _ => roll_beast_den_deep(roll),
    }
}

fn roll_beast_den_l0(roll: u64) -> EnemyStats {
    if roll < 20 {
        (3, 1, 0, 'r', "Giant Rat", false)
    } else if roll < 40 {
        (4, 2, 0, 'a', "Giant Bat", false)
    } else if roll < 55 {
        // Lesser Giant Spider — activates LesserGiantSpider sprite
        (4, 2, 0, '<', "Lesser Giant Spider", false)
    } else if roll < 70 {
        (5, 3, 0, 'n', "Viper", false)
    } else if roll < 85 {
        (5, 2, 1, 'w', "Wolf", false)
    } else {
        (4, 2, 1, 'c', "Kobold", false)
    }
}

fn roll_beast_den_l1(roll: u64) -> EnemyStats {
    if roll < 25 {
        (6, 3, 0, 'i', "Giant Spider", false)
    } else if roll < 45 {
        // Dire Wolf — activates WargDireWolf sprite
        (10, 4, 2, 'U', "Dire Wolf", false)
    } else if roll < 60 {
        (14, 5, 3, 'L', "Lycanthrope", false)
    } else if roll < 75 {
        (12, 4, 2, 'B', "Bear", false)
    } else if roll < 88 {
        (4, 2, 0, 'a', "Giant Bat", false)
    } else {
        (9, 4, 2, 'h', "Cougar", false)
    }
}

fn roll_beast_den_deep(roll: u64) -> EnemyStats {
    if roll < 30 {
        (12, 5, 1, '0', "Wendigo", false)
    } else if roll < 50 {
        (18, 7, 4, 'X', "Manticore", false)
    } else if roll < 65 {
        (14, 5, 3, 'L', "Lycanthrope", false)
    } else if roll < 80 {
        (10, 4, 2, 'U', "Dire Wolf", false)
    } else {
        (6, 3, 0, 'i', "Giant Spider", false)
    }
}

// === Serpent Pit ===
// L0: Cobras, Lizardfolk/Kobold, Giant Centipedes
// L1: Black Mambas, Cockatrices, Nagas
// L2: Basilisks, Gorgon/Medusa
// Boss: Basilisk

fn roll_serpent_pit(level: usize, roll: u64) -> EnemyStats {
    match level {
        0 => roll_serpent_pit_l0(roll),
        1 => roll_serpent_pit_l1(roll),
        _ => roll_serpent_pit_deep(roll),
    }
}

fn roll_serpent_pit_l0(roll: u64) -> EnemyStats {
    if roll < 25 {
        (5, 3, 0, 'n', "Viper", false)
    } else if roll < 45 {
        // Lizardfolk — activates LizardfolkKobold sprite
        (6, 3, 1, '>', "Lizardfolk", false)
    } else if roll < 60 {
        (4, 2, 0, 'e', "Giant Centipede", false)
    } else if roll < 75 {
        (4, 2, 0, 'a', "Giant Bat", false)
    } else if roll < 88 {
        (3, 1, 0, 'r', "Giant Rat", false)
    } else {
        (4, 1, 0, 'S', "Small Slime", false)
    }
}

fn roll_serpent_pit_l1(roll: u64) -> EnemyStats {
    if roll < 25 {
        (5, 3, 0, 'v', "Black Mamba", false)
    } else if roll < 45 {
        // Cockatrice — activates Cockatrice sprite
        (10, 5, 2, '~', "Cockatrice", false)
    } else if roll < 60 {
        (12, 6, 3, 'N', "Naga", false)
    } else if roll < 75 {
        (6, 3, 1, '>', "Lizardfolk", false)
    } else if roll < 88 {
        (5, 3, 0, 'n', "Viper", false)
    } else {
        (4, 2, 0, 'e', "Giant Centipede", false)
    }
}

fn roll_serpent_pit_deep(roll: u64) -> EnemyStats {
    if roll < 25 {
        (16, 7, 4, 'C', "Basilisk", false)
    } else if roll < 45 {
        (12, 7, 2, 'P', "Medusa", false)
    } else if roll < 60 {
        (12, 6, 3, 'N', "Naga", false)
    } else if roll < 75 {
        (10, 5, 2, '~', "Cockatrice", false)
    } else if roll < 88 {
        (5, 3, 0, 'v', "Black Mamba", false)
    } else {
        (6, 3, 1, '>', "Lizardfolk", false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::xorshift64;

    fn collect_glyphs(biome: DungeonBiome, level: usize, count: usize) -> std::collections::HashSet<char> {
        let mut seen = std::collections::HashSet::new();
        let mut rng = 42u64;
        for _ in 0..count {
            rng = xorshift64(rng);
            let e = roll_biome_enemy(5, 5, biome, level, rng);
            seen.insert(e.glyph);
        }
        seen
    }

    #[test]
    fn goblin_warren_has_goblins() {
        let glyphs = collect_glyphs(DungeonBiome::GoblinWarren, 0, 1000);
        assert!(glyphs.contains(&'g'), "L0 should have goblins");
        assert!(glyphs.contains(&'r'), "L0 should have rats");
        assert!(glyphs.contains(&'c'), "L0 should have kobolds");
    }

    #[test]
    fn goblin_warren_deep_has_orcs() {
        let glyphs = collect_glyphs(DungeonBiome::GoblinWarren, 2, 1000);
        assert!(glyphs.contains(&'o') || glyphs.contains(&'O'),
            "deep Goblin Warren should have orcs");
        assert!(glyphs.contains(&'5'), "deep should have Orc Warchief");
    }

    #[test]
    fn undead_crypt_has_undead() {
        let glyphs = collect_glyphs(DungeonBiome::UndeadCrypt, 0, 1000);
        assert!(glyphs.contains(&'s'), "L0 should have skeletons");
        assert!(glyphs.contains(&'z'), "L0 should have zombies");
    }

    #[test]
    fn undead_crypt_deep_has_boss_enemies() {
        let glyphs = collect_glyphs(DungeonBiome::UndeadCrypt, 2, 1000);
        assert!(glyphs.contains(&'Q'), "deep should have banshees");
        assert!(glyphs.contains(&'K'), "deep should have death knights");
        assert!(glyphs.contains(&'l'), "deep should have lich");
    }

    #[test]
    fn fungal_grotto_has_myconids() {
        let glyphs = collect_glyphs(DungeonBiome::FungalGrotto, 0, 1000);
        assert!(glyphs.contains(&'p'), "L0 should have myconids");
        assert!(glyphs.contains(&']'), "L0 should have giant earthworms");
    }

    #[test]
    fn fungal_grotto_deep_has_aberrations() {
        let glyphs = collect_glyphs(DungeonBiome::FungalGrotto, 2, 1000);
        assert!(glyphs.contains(&'8'), "deep should have writhing mass");
        assert!(glyphs.contains(&'('), "deep should have small writhing mass");
        assert!(glyphs.contains(&')'), "deep should have writhing humanoid");
    }

    #[test]
    fn orc_stronghold_has_orcs() {
        let glyphs = collect_glyphs(DungeonBiome::OrcStronghold, 0, 1000);
        assert!(glyphs.contains(&'o'), "L0 should have orcs");
    }

    #[test]
    fn abyssal_temple_has_cultists() {
        let glyphs = collect_glyphs(DungeonBiome::AbyssalTemple, 0, 1000);
        assert!(glyphs.contains(&'!'), "L0 should have cultists");
        assert!(glyphs.contains(&'6'), "L0 should have faceless monks");
    }

    #[test]
    fn beast_den_has_beasts() {
        let glyphs = collect_glyphs(DungeonBiome::BeastDen, 0, 1000);
        assert!(glyphs.contains(&'r'), "L0 should have rats");
        assert!(glyphs.contains(&'a'), "L0 should have bats");
        assert!(glyphs.contains(&'<'), "L0 should have lesser giant spiders");
    }

    #[test]
    fn beast_den_deep_has_apex_predators() {
        let glyphs = collect_glyphs(DungeonBiome::BeastDen, 2, 1000);
        assert!(glyphs.contains(&'0'), "deep should have wendigo");
        assert!(glyphs.contains(&'X'), "deep should have manticore");
    }

    #[test]
    fn serpent_pit_has_reptiles() {
        let glyphs = collect_glyphs(DungeonBiome::SerpentPit, 0, 1000);
        assert!(glyphs.contains(&'n'), "L0 should have vipers");
        assert!(glyphs.contains(&'>'), "L0 should have lizardfolk");
    }

    #[test]
    fn serpent_pit_deep_has_petrifiers() {
        let glyphs = collect_glyphs(DungeonBiome::SerpentPit, 2, 1000);
        assert!(glyphs.contains(&'C'), "deep should have basilisks");
        assert!(glyphs.contains(&'P'), "deep should have medusa");
    }

    #[test]
    fn all_biome_enemies_have_valid_stats() {
        let biomes = [
            DungeonBiome::GoblinWarren, DungeonBiome::UndeadCrypt,
            DungeonBiome::FungalGrotto, DungeonBiome::OrcStronghold,
            DungeonBiome::AbyssalTemple, DungeonBiome::BeastDen,
            DungeonBiome::SerpentPit,
        ];
        let mut rng = 42u64;
        for biome in biomes {
            for level in 0..3 {
                for _ in 0..500 {
                    rng = xorshift64(rng);
                    let e = roll_biome_enemy(5, 5, biome, level, rng);
                    assert!(e.hp > 0, "{:?} L{level} {}: hp={}", biome, e.name, e.hp);
                    assert!(e.attack > 0, "{:?} L{level} {}: atk={}", biome, e.name, e.attack);
                }
            }
        }
    }

    #[test]
    fn deeper_biome_levels_are_stronger() {
        let biomes = [
            DungeonBiome::GoblinWarren, DungeonBiome::UndeadCrypt,
            DungeonBiome::FungalGrotto, DungeonBiome::OrcStronghold,
            DungeonBiome::AbyssalTemple, DungeonBiome::BeastDen,
            DungeonBiome::SerpentPit,
        ];
        for biome in biomes {
            let mut rng = 42u64;
            let mut l0_total_hp = 0i64;
            let mut l2_total_hp = 0i64;
            for _ in 0..500 {
                rng = xorshift64(rng);
                l0_total_hp += roll_biome_enemy(5, 5, biome, 0, rng).hp as i64;
                rng = xorshift64(rng);
                l2_total_hp += roll_biome_enemy(5, 5, biome, 2, rng).hp as i64;
            }
            assert!(l2_total_hp > l0_total_hp,
                "{:?}: L2 avg hp ({}) should exceed L0 avg hp ({})", biome, l2_total_hp / 500, l0_total_hp / 500);
        }
    }

    #[test]
    fn cave_enemies_are_strong() {
        let mut rng = 42u64;
        for _ in 0..200 {
            rng = xorshift64(rng);
            let e = roll_cave_enemy(5, 5, rng);
            assert!(e.hp >= 10, "cave enemy {} has too low hp: {}", e.name, e.hp);
        }
    }
}
