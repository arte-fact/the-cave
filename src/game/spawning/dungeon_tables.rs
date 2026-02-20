use crate::game::Enemy;
use crate::map::DungeonBiome;
use super::enemies::*;

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
        DungeonBiome::DragonLair => roll_cave(roll),
    };
    let (hp, attack, def, glyph, name, ranged) = stats;
    Enemy { x, y, hp, attack, defense: def, glyph, name, facing_left: false, is_ranged: ranged }
}

// === Dragon's Lair cave level ===

fn roll_cave(roll: u64) -> EnemyStats {
    if roll < 20      { DEATH_KNIGHT }
    else if roll < 35 { TROLL }
    else if roll < 50 { LICH }
    else if roll < 60 { DRAKE }
    else if roll < 70 { BASILISK }
    else if roll < 80 { IMP }
    else if roll < 90 { MANTICORE }
    else              { REAPER }
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
    if roll < 15      { GIANT_RAT }
    else if roll < 30 { KOBOLD }
    else if roll < 42 { SMALL_SLIME }
    else if roll < 70 { GOBLIN }
    else if roll < 85 { GIANT_CENTIPEDE }
    else              { SKELETON }
}

fn roll_goblin_warren_l1(roll: u64) -> EnemyStats {
    if roll < 15      { GOBLIN_ARCHER }
    else if roll < 30 { GOBLIN_MAGE }
    else if roll < 45 { BIG_SLIME }
    else if roll < 65 { GOBLIN_BRUTE }
    else if roll < 80 { GOBLIN }
    else              { ORC }
}

fn roll_goblin_warren_deep(roll: u64) -> EnemyStats {
    if roll < 20      { ORC }
    else if roll < 40 { ORC_BLADEMASTER }
    else if roll < 55 { TROLL }
    else if roll < 70 { GOBLIN_ARCHER }
    else if roll < 85 { GOBLIN_MAGE }
    else              { ORC_WARCHIEF }
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
    if roll < 25      { SKELETON }
    else if roll < 45 { ZOMBIE }
    else if roll < 60 { GIANT_RAT }
    else if roll < 75 { GIANT_BAT }
    else if roll < 88 { SMALL_SLIME }
    else              { GIANT_CENTIPEDE }
}

fn roll_undead_crypt_l1(roll: u64) -> EnemyStats {
    if roll < 20      { SKELETON_ARCHER }
    else if roll < 40 { GHOUL }
    else if roll < 55 { WRAITH }
    else if roll < 70 { HAG }
    else if roll < 85 { ZOMBIE }
    else              { SKELETON }
}

fn roll_undead_crypt_deep(roll: u64) -> EnemyStats {
    if roll < 20      { BANSHEE }
    else if roll < 40 { DEATH_KNIGHT }
    else if roll < 55 { UNHOLY_CARDINAL }
    else if roll < 70 { WRAITH }
    else if roll < 85 { GHOUL }
    else              { LICH }
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
    if roll < 25      { MYCONID }
    else if roll < 45 { SMALL_SLIME }
    else if roll < 60 { GIANT_CENTIPEDE }
    else if roll < 75 { GIANT_EARTHWORM }
    else if roll < 88 { GIANT_RAT }
    else              { LARGE_MYCONID }
}

fn roll_fungal_grotto_l1(roll: u64) -> EnemyStats {
    if roll < 20      { LARGE_MYCONID }
    else if roll < 40 { BIG_SLIME }
    else if roll < 55 { GIANT_ANT }
    else if roll < 70 { LAMPREYMANDER }
    else if roll < 85 { GIANT_CENTIPEDE }
    else              { GIANT_EARTHWORM }
}

fn roll_fungal_grotto_deep(roll: u64) -> EnemyStats {
    if roll < 20      { SM_WRITHING_MASS }
    else if roll < 40 { LG_WRITHING_MASS }
    else if roll < 55 { WRITHING_HUMANOID }
    else if roll < 70 { BIG_SLIME }
    else if roll < 85 { LAMPREYMANDER }
    else              { LARGE_MYCONID }
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
    if roll < 30      { ORC }
    else if roll < 50 { KOBOLD_CANINE }
    else if roll < 70 { GOBLIN_BRUTE }
    else if roll < 85 { GOBLIN }
    else              { GIANT_ANT }
}

fn roll_orc_stronghold_l1(roll: u64) -> EnemyStats {
    if roll < 20      { ORC_BLADEMASTER }
    else if roll < 35 { ORC_WIZARD }
    else if roll < 50 { TROLL }
    else if roll < 65 { GIANT_ANT }
    else if roll < 80 { ORC }
    else              { GOBLIN_BRUTE }
}

fn roll_orc_stronghold_deep(roll: u64) -> EnemyStats {
    if roll < 25      { ETTIN }
    else if roll < 40 { TWO_HEADED_ETTIN }
    else if roll < 55 { ORC_WARCHIEF }
    else if roll < 70 { ORC_BLADEMASTER }
    else if roll < 85 { TROLL }
    else              { ORC_WIZARD }
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
    if roll < 30      { CULTIST }
    else if roll < 55 { FACELESS_MONK }
    else if roll < 75 { SMALL_SLIME }
    else if roll < 88 { WRAITH }
    else              { GIANT_CENTIPEDE }
}

fn roll_abyssal_temple_l1(roll: u64) -> EnemyStats {
    if roll < 25      { UNHOLY_CARDINAL }
    else if roll < 45 { HAG }
    else if roll < 60 { WRAITH }
    else if roll < 75 { FACELESS_MONK }
    else if roll < 88 { CULTIST }
    else              { BIG_SLIME }
}

fn roll_abyssal_temple_deep(roll: u64) -> EnemyStats {
    if roll < 20      { WRITHING_HUMANOID }
    else if roll < 35 { IMP }
    else if roll < 50 { NAGA }
    else if roll < 65 { UNHOLY_CARDINAL }
    else if roll < 80 { WRAITH }
    else              { REAPER }
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
    if roll < 20      { GIANT_RAT }
    else if roll < 40 { GIANT_BAT }
    else if roll < 55 { LESSER_GIANT_SPIDER }
    else if roll < 70 { VIPER }
    else if roll < 85 { WOLF }
    else              { KOBOLD }
}

fn roll_beast_den_l1(roll: u64) -> EnemyStats {
    if roll < 18      { GIANT_SPIDER }
    else if roll < 33 { DIRE_WOLF }
    else if roll < 45 { LYCANTHROPE }
    else if roll < 57 { BEAR }
    else if roll < 67 { LYNX }
    else if roll < 77 { BLACK_BEAR }
    else if roll < 88 { GIANT_BAT }
    else              { COUGAR }
}

fn roll_beast_den_deep(roll: u64) -> EnemyStats {
    if roll < 22      { WENDIGO }
    else if roll < 38 { MANTICORE }
    else if roll < 50 { LYCANTHROPE }
    else if roll < 62 { DIRE_WOLF }
    else if roll < 74 { WATER_BUFFALO }
    else if roll < 86 { YAK }
    else              { GIANT_SPIDER }
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
    if roll < 22      { VIPER }
    else if roll < 40 { LIZARDFOLK }
    else if roll < 52 { GIANT_CENTIPEDE }
    else if roll < 65 { MONITOR_LIZARD }
    else if roll < 77 { GIANT_BAT }
    else if roll < 88 { GIANT_RAT }
    else              { SMALL_SLIME }
}

fn roll_serpent_pit_l1(roll: u64) -> EnemyStats {
    if roll < 25      { BLACK_MAMBA }
    else if roll < 45 { COCKATRICE }
    else if roll < 60 { NAGA }
    else if roll < 75 { LIZARDFOLK }
    else if roll < 88 { VIPER }
    else              { GIANT_CENTIPEDE }
}

fn roll_serpent_pit_deep(roll: u64) -> EnemyStats {
    if roll < 25      { BASILISK }
    else if roll < 45 { MEDUSA }
    else if roll < 60 { NAGA }
    else if roll < 75 { COCKATRICE }
    else if roll < 88 { BLACK_MAMBA }
    else              { LIZARDFOLK }
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
        let mut rng = 42u64;
        for biome in DungeonBiome::PLACEABLE {
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
        for biome in DungeonBiome::PLACEABLE {
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
            let e = roll_biome_enemy(5, 5, DungeonBiome::DragonLair, 0, rng);
            assert!(e.hp >= 10, "cave enemy {} has too low hp: {}", e.name, e.hp);
        }
    }
}
