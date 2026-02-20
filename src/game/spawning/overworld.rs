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
    if roll < 6        { GIANT_RAT }
    else if roll < 11  { GIANT_BAT }
    else if roll < 16  { FOX }
    else if roll < 20  { BUZZARD }
    else if roll < 25  { VIPER }
    else if roll < 28  { COYOTE }
    else if roll < 35  { WOLF }
    else if roll < 42  { GIANT_SPIDER }
    else if roll < 48  { BADGER }
    else if roll < 52  { HONEY_BADGER }
    else if roll < 57  { DRYAD }
    else if roll < 62  { FOREST_SPIRIT }
    else if roll < 68  { BOAR }
    else if roll < 73  { COUGAR }
    else if roll < 77  { CENTAUR }
    else if roll < 83  { BEAR }
    else if roll < 88  { LYCANTHROPE }
    else if roll < 95  { WENDIGO }
    else               { DIRE_WOLF }
}

/// Jungle enemies: tropical predators, reptilians, and mythical creatures.
fn roll_jungle(roll: u64) -> EnemyStats {
    if roll < 8        { VIPER }
    else if roll < 16  { BLACK_MAMBA }
    else if roll < 22  { GIANT_CENTIPEDE }
    else if roll < 28  { GIANT_SPIDER }
    else if roll < 34  { GIANT_ANT }
    else if roll < 40  { HYENA }
    else if roll < 46  { COUGAR }
    else if roll < 52  { ALLIGATOR }
    else if roll < 57  { MALE_LION }
    else if roll < 62  { DRYAD }
    else if roll < 67  { SATYR }
    else if roll < 72  { HARPY }
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
