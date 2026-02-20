use crate::game::Enemy;
use crate::map::OverworldBiome;
use super::enemies::*;

/// Roll an overworld enemy, choosing the table based on map position.
pub(super) fn roll_overworld_enemy(x: i32, y: i32, rng: u64, map_height: i32) -> Enemy {
    let biome = OverworldBiome::at_y(y, map_height);
    let roll = rng % 100;
    let (hp, attack, def, glyph, name, ranged) = match biome {
        OverworldBiome::TemperateForest => roll_temperate(roll),
        OverworldBiome::Jungle => roll_jungle(roll),
    };
    Enemy { x, y, hp, attack, defense: def, glyph, name, facing_left: false, is_ranged: ranged }
}

/// Temperate forest enemies: woodland wildlife + fey creatures.
fn roll_temperate(roll: u64) -> EnemyStats {
    if roll < 4        { GIANT_RAT }
    else if roll < 8   { GIANT_BAT }
    else if roll < 13  { FOX }
    else if roll < 17  { BUZZARD }
    else if roll < 22  { VIPER }
    else if roll < 25  { COYOTE }
    else if roll < 30  { WOLF }
    else if roll < 35  { GIANT_SPIDER }
    else if roll < 40  { BADGER }
    else if roll < 44  { HONEY_BADGER }
    else if roll < 48  { LYNX }
    else if roll < 52  { DRYAD }
    else if roll < 56  { FOREST_SPIRIT }
    else if roll < 62  { BOAR }
    else if roll < 67  { COUGAR }
    else if roll < 71  { CENTAUR }
    else if roll < 75  { BLACK_BEAR }
    else if roll < 81  { BEAR }
    else if roll < 86  { LYCANTHROPE }
    else if roll < 93  { WENDIGO }
    else               { DIRE_WOLF }
}

/// Jungle enemies: tropical predators, reptilians, and mythical creatures.
fn roll_jungle(roll: u64) -> EnemyStats {
    if roll < 5        { VIPER }
    else if roll < 10  { BLACK_MAMBA }
    else if roll < 15  { GIANT_CENTIPEDE }
    else if roll < 20  { GIANT_SPIDER }
    else if roll < 24  { GIANT_ANT }
    else if roll < 28  { JACKAL }
    else if roll < 32  { OCELOT }
    else if roll < 37  { HYENA }
    else if roll < 42  { COUGAR }
    else if roll < 46  { MONITOR_LIZARD }
    else if roll < 51  { ALLIGATOR }
    else if roll < 56  { MALE_LION }
    else if roll < 60  { WATER_BUFFALO }
    else if roll < 64  { DRYAD }
    else if roll < 68  { SATYR }
    else if roll < 73  { HARPY }
    else if roll < 77  { CENTAUR }
    else if roll < 82  { MANTICORE }
    else if roll < 87  { NAGA }
    else if roll < 92  { MEDUSA }
    else if roll < 97  { COCKATRICE }
    else               { LYCANTHROPE }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::game::xorshift64;

    #[test]
    fn temperate_enemies_from_north() {
        let mut seen_glyphs = std::collections::HashSet::new();
        let mut rng = 42u64;
        for _ in 0..2000 {
            rng = xorshift64(rng);
            let e = roll_overworld_enemy(50, 10, rng, 200); // y=10, northern
            seen_glyphs.insert(e.glyph);
        }
        // Temperate should have wolves, bears, foxes
        assert!(seen_glyphs.contains(&'w'), "temperate should have wolves");
        assert!(seen_glyphs.contains(&'B'), "temperate should have bears");
        assert!(seen_glyphs.contains(&'f'), "temperate should have foxes");
        // Should NOT have jungle-only creatures
        assert!(!seen_glyphs.contains(&'$'), "temperate should not have harpies");
        assert!(!seen_glyphs.contains(&'~'), "temperate should not have cockatrices");
    }

    #[test]
    fn jungle_enemies_from_south() {
        let mut seen_glyphs = std::collections::HashSet::new();
        let mut rng = 42u64;
        for _ in 0..2000 {
            rng = xorshift64(rng);
            let e = roll_overworld_enemy(50, 180, rng, 200); // y=180, southern jungle
            seen_glyphs.insert(e.glyph);
        }
        // Jungle should have snakes, alligators, harpies
        assert!(seen_glyphs.contains(&'n'), "jungle should have vipers");
        assert!(seen_glyphs.contains(&'Z'), "jungle should have alligators");
        assert!(seen_glyphs.contains(&'$'), "jungle should have harpies");
        // Should NOT have temperate-only creatures
        assert!(!seen_glyphs.contains(&'0'), "jungle should not have wendigos");
        assert!(!seen_glyphs.contains(&'U'), "jungle should not have dire wolves");
    }

    #[test]
    fn overworld_enemies_valid_stats() {
        let mut rng = 42u64;
        for y in [10, 180] {
            for _ in 0..500 {
                rng = xorshift64(rng);
                let e = roll_overworld_enemy(50, y, rng, 200);
                assert!(e.hp > 0, "{} has 0 hp", e.name);
                assert!(e.attack > 0, "{} has 0 attack", e.name);
                assert!(e.defense >= 0, "{} has negative defense", e.name);
                assert!(!e.name.is_empty(), "enemy has empty name");
            }
        }
    }
}
