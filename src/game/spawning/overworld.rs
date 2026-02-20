use crate::map::OverworldBiome;
use super::super::types::Enemy;

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
fn roll_temperate(roll: u64) -> (i32, i32, i32, char, &'static str, bool) {
    if roll < 6 {
        (3, 1, 0, 'r', "Giant Rat", false)
    } else if roll < 11 {
        (4, 2, 0, 'a', "Giant Bat", false)
    } else if roll < 16 {
        (4, 2, 1, 'f', "Fox", false)
    } else if roll < 20 {
        (4, 2, 0, 'q', "Buzzard", false)
    } else if roll < 25 {
        (5, 3, 0, 'n', "Viper", false)
    } else if roll < 28 {
        (5, 2, 1, 'y', "Coyote", false)
    } else if roll < 35 {
        (5, 2, 1, 'w', "Wolf", false)
    } else if roll < 42 {
        (6, 3, 0, 'i', "Giant Spider", false)
    } else if roll < 48 {
        (5, 3, 1, 'j', "Badger", false)
    } else if roll < 52 {
        (6, 3, 2, 'J', "Honey Badger", false)
    } else if roll < 57 {
        (5, 2, 0, '1', "Dryad", false)
    } else if roll < 62 {
        (4, 1, 1, '2', "Forest Spirit", false)
    } else if roll < 68 {
        (8, 2, 2, 'b', "Boar", false)
    } else if roll < 73 {
        (9, 4, 2, 'h', "Cougar", false)
    } else if roll < 77 {
        (10, 4, 2, '9', "Centaur", false)
    } else if roll < 83 {
        (12, 4, 2, 'B', "Bear", false)
    } else if roll < 88 {
        (14, 5, 3, 'L', "Lycanthrope", false)
    } else if roll < 95 {
        (12, 5, 1, '0', "Wendigo", false)
    } else {
        // Dire Wolf — activates WargDireWolf sprite
        (10, 4, 2, 'U', "Dire Wolf", false)
    }
}

/// Jungle enemies: tropical predators, reptilians, and mythical creatures.
fn roll_jungle(roll: u64) -> (i32, i32, i32, char, &'static str, bool) {
    if roll < 8 {
        (5, 3, 0, 'n', "Viper", false)
    } else if roll < 16 {
        (5, 3, 0, 'v', "Black Mamba", false)
    } else if roll < 22 {
        (4, 2, 0, 'e', "Giant Centipede", false)
    } else if roll < 28 {
        (6, 3, 0, 'i', "Giant Spider", false)
    } else if roll < 34 {
        (8, 3, 2, 'A', "Giant Ant", false)
    } else if roll < 40 {
        (5, 3, 1, 'x', "Hyena", false)
    } else if roll < 46 {
        (9, 4, 2, 'h', "Cougar", false)
    } else if roll < 52 {
        (10, 3, 3, 'Z', "Alligator", false)
    } else if roll < 57 {
        (16, 6, 2, 'F', "Male Lion", false)
    } else if roll < 62 {
        (5, 2, 0, '1', "Dryad", false)
    } else if roll < 67 {
        (7, 3, 0, '4', "Satyr", false)
    } else if roll < 72 {
        // Harpy — activates Harpy sprite
        (8, 4, 1, '$', "Harpy", false)
    } else if roll < 77 {
        (10, 4, 2, '9', "Centaur", false)
    } else if roll < 82 {
        (18, 7, 4, 'X', "Manticore", false)
    } else if roll < 87 {
        (12, 6, 3, 'N', "Naga", false)
    } else if roll < 92 {
        (12, 7, 2, 'P', "Medusa", false)
    } else if roll < 97 {
        // Cockatrice — activates Cockatrice sprite
        (10, 5, 2, '~', "Cockatrice", false)
    } else {
        (14, 5, 3, 'L', "Lycanthrope", false)
    }
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
