use crate::game::Enemy;
use super::enemies::*;

/// Roll an overworld enemy. All overworld encounters are temperate forest
/// wildlife with a small chance (~12%) of a rare, powerful monster.
/// `roll` must be in 0..100.
pub(super) fn roll_overworld_enemy(x: i32, y: i32, roll: u64, _map_height: i32) -> Enemy {
    let (hp, attack, def, glyph, name, ranged, behavior) = roll_forest(roll);
    Enemy { x, y, hp, attack, defense: def, glyph, name, facing_left: false, is_ranged: ranged, behavior, spawn_x: x, spawn_y: y, provoked: false, is_boss: false }
}

/// Temperate forest table: ~88% wildlife, ~12% rare monsters.
fn roll_forest(roll: u64) -> EnemyStats {
    // --- Wildlife (88%) ---
    if roll < 5        { FOX }
    else if roll < 9   { BUZZARD }
    else if roll < 14  { VIPER }
    else if roll < 19  { COYOTE }
    else if roll < 27  { WOLF }
    else if roll < 33  { BADGER }
    else if roll < 38  { HONEY_BADGER }
    else if roll < 44  { LYNX }
    else if roll < 54  { BOAR }
    else if roll < 62  { COUGAR }
    else if roll < 72  { BLACK_BEAR }
    else if roll < 88  { BEAR }
    // --- Rare monsters (12%) — mini-boss encounters ---
    else if roll < 90  { DRYAD }
    else if roll < 92  { FOREST_SPIRIT }
    else if roll < 94  { CENTAUR }
    else if roll < 96  { DIRE_WOLF }
    else if roll < 98  { LYCANTHROPE }
    else               { WENDIGO }
}

/// Returns true if the enemy is a rare overworld monster (not common wildlife).
pub fn is_rare_monster(name: &str, rare_names: &[&str]) -> bool {
    rare_names.contains(&name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use crate::config::GameConfig;

    fn rare_names() -> &'static [&'static str] {
        GameConfig::normal().spawn_tables.rare_monster_names
    }

    #[test]
    fn overworld_spawns_forest_animals() {
        let mut seen_glyphs = std::collections::HashSet::new();
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        for _ in 0..2000 {
            let roll = rng.gen_range(0u64..100);
            let e = roll_overworld_enemy(50, 10, roll, 200);
            seen_glyphs.insert(e.glyph);
        }
        // Should have common temperate forest animals
        assert!(seen_glyphs.contains(&'w'), "should have wolves");
        assert!(seen_glyphs.contains(&'B'), "should have bears");
        assert!(seen_glyphs.contains(&'f'), "should have foxes");
        assert!(seen_glyphs.contains(&'b'), "should have boars");
        assert!(seen_glyphs.contains(&'h'), "should have cougars");
        assert!(seen_glyphs.contains(&'&'), "should have black bears");
        assert!(seen_glyphs.contains(&'#'), "should have lynxes");
    }

    #[test]
    fn overworld_has_no_jungle_animals() {
        let mut seen_glyphs = std::collections::HashSet::new();
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        // Test both north and south positions — should be same table
        for y in [10, 180] {
            for _ in 0..2000 {
                let roll = rng.gen_range(0u64..100);
                let e = roll_overworld_enemy(50, y, roll, 200);
                seen_glyphs.insert(e.glyph);
            }
        }
        // No jungle-only animals
        assert!(!seen_glyphs.contains(&'x'), "should not have hyenas");
        assert!(!seen_glyphs.contains(&'v'), "should not have black mambas");
        assert!(!seen_glyphs.contains(&'Z'), "should not have alligators");
        assert!(!seen_glyphs.contains(&'F'), "should not have male lions");
        assert!(!seen_glyphs.contains(&'$'), "should not have harpies");
        assert!(!seen_glyphs.contains(&'~'), "should not have cockatrices");
    }

    #[test]
    fn rare_monsters_appear_but_are_uncommon() {
        let mut monster_count = 0;
        let total = 5000;
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        for _ in 0..total {
            let roll = rng.gen_range(0u64..100);
            let e = roll_overworld_enemy(50, 50, roll, 200);
            if is_rare_monster(e.name, rare_names()) {
                monster_count += 1;
            }
        }
        let rate = monster_count as f64 / total as f64;
        assert!(rate > 0.05 && rate < 0.20,
            "monster rate should be ~12%, got {:.1}% ({monster_count}/{total})", rate * 100.0);
    }

    #[test]
    fn rare_monsters_are_strong() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let mut found_any = false;
        for _ in 0..5000 {
            let roll = rng.gen_range(0u64..100);
            let e = roll_overworld_enemy(50, 50, roll, 200);
            if is_rare_monster(e.name, rare_names()) {
                found_any = true;
                assert!(e.hp >= 16, "{} should have >= 16 hp, got {}", e.name, e.hp);
                assert!(e.attack >= 5, "{} should have >= 5 attack, got {}", e.name, e.attack);
            }
        }
        assert!(found_any, "should have found at least one rare monster in 5000 rolls");
    }

    #[test]
    fn overworld_enemies_valid_stats() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        for _ in 0..1000 {
            let roll = rng.gen_range(0u64..100);
            let e = roll_overworld_enemy(50, 50, roll, 200);
            assert!(e.hp > 0, "{} has 0 hp", e.name);
            assert!(e.attack > 0, "{} has 0 attack", e.name);
            assert!(e.defense >= 0, "{} has negative defense", e.name);
            assert!(!e.name.is_empty(), "enemy has empty name");
        }
    }

    #[test]
    fn north_and_south_spawn_same_enemies() {
        let mut rng = ChaCha8Rng::seed_from_u64(42);
        let mut north_glyphs = std::collections::HashSet::new();
        let mut south_glyphs = std::collections::HashSet::new();
        for _ in 0..2000 {
            let roll = rng.gen_range(0u64..100);
            north_glyphs.insert(roll_overworld_enemy(50, 10, roll, 200).glyph);
            south_glyphs.insert(roll_overworld_enemy(50, 180, roll, 200).glyph);
        }
        assert_eq!(north_glyphs, south_glyphs,
            "north and south should spawn same enemies");
    }
}
