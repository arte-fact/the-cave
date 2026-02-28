use super::*;

#[test]
fn normal_defaults_match_original_hardcoded_values() {
    let cfg = GameConfig::normal();
    assert_eq!(cfg.player.starting_hp, 20);
    assert_eq!(cfg.player.starting_attack, 5);
    assert_eq!(cfg.player.starting_dexterity, 3);
    assert_eq!(cfg.player.starting_stamina, 100);
    assert_eq!(cfg.player.starting_hunger, 100);
    assert_eq!(cfg.player.max_inventory, 10);
    assert_eq!(cfg.survival.stamina_regen, 5);
    assert_eq!(cfg.survival.hunger_drain, 1);
    assert_eq!(cfg.survival.hunger_interval_overworld, 5);
    assert_eq!(cfg.survival.hunger_interval_dungeon, 3);
    assert_eq!(cfg.survival.hunger_interval_cave, 2);
    assert_eq!(cfg.survival.starvation_damage, 1);
    assert_eq!(cfg.survival.regen_hunger_threshold, 50);
    assert_eq!(cfg.survival.regen_hunger_cost, 2);
    assert_eq!(cfg.fov.overworld_radius, 8);
    assert_eq!(cfg.fov.dungeon_radius, 6);
    assert_eq!(cfg.spawn.overworld_enemy_pct, 2);
    assert_eq!(cfg.spawn.spawn_safe_radius, 5);
}

#[test]
fn easy_is_more_forgiving_than_normal() {
    let easy = GameConfig::easy();
    let normal = GameConfig::normal();
    assert!(easy.player.starting_hp > normal.player.starting_hp);
    assert!(easy.survival.hunger_interval_overworld > normal.survival.hunger_interval_overworld);
    assert!(easy.spawn.overworld_enemy_pct < normal.spawn.overworld_enemy_pct);
    assert!(easy.spawn.spawn_safe_radius > normal.spawn.spawn_safe_radius);
    assert!(easy.progression.skill_points_per_level > normal.progression.skill_points_per_level);
}

#[test]
fn hard_is_tougher_than_normal() {
    let hard = GameConfig::hard();
    let normal = GameConfig::normal();
    assert!(hard.player.starting_hp < normal.player.starting_hp);
    assert!(hard.survival.hunger_interval_overworld < normal.survival.hunger_interval_overworld);
    assert!(hard.spawn.overworld_enemy_pct > normal.spawn.overworld_enemy_pct);
    assert!(hard.spawn.spawn_safe_radius < normal.spawn.spawn_safe_radius);
    assert!(hard.survival.starvation_damage > normal.survival.starvation_damage);
}

#[test]
fn from_difficulty_returns_correct_preset() {
    let easy = GameConfig::from_difficulty(Difficulty::Easy);
    let normal = GameConfig::from_difficulty(Difficulty::Normal);
    let hard = GameConfig::from_difficulty(Difficulty::Hard);
    assert_eq!(easy.player.starting_hp, 30);
    assert_eq!(normal.player.starting_hp, 20);
    assert_eq!(hard.player.starting_hp, 15);
}

#[test]
fn difficulty_labels() {
    assert_eq!(Difficulty::Easy.label(), "Easy");
    assert_eq!(Difficulty::Normal.label(), "Normal");
    assert_eq!(Difficulty::Hard.label(), "Hard");
}

#[test]
fn enemy_registry_has_entries() {
    let cfg = GameConfig::normal();
    assert!(!cfg.enemies.is_empty(), "enemy registry should not be empty");
    assert!(cfg.enemies.len() >= 70, "expected at least 70 enemies, got {}", cfg.enemies.len());
}

#[test]
fn enemy_def_lookup_works() {
    let def = enemy_def("Dragon").expect("Dragon should be in registry");
    assert_eq!(def.hp, 108);
    assert_eq!(def.attack, 19);
    assert_eq!(def.defense, 11);
    assert_eq!(def.glyph, 'D');
    assert!(!def.is_ranged);
    assert_eq!(def.xp, 200);
}

#[test]
fn enemy_def_returns_none_for_unknown() {
    assert!(enemy_def("NonExistent").is_none());
}

#[test]
fn all_enemies_have_valid_stats() {
    for def in ENEMY_DEFS {
        assert!(def.hp > 0, "{} has 0 hp", def.name);
        assert!(def.attack > 0, "{} has 0 attack", def.name);
        assert!(def.defense >= 0, "{} has negative defense", def.name);
        assert!(def.xp > 0, "{} has 0 xp", def.name);
        assert!(!def.name.is_empty(), "enemy has empty name");
        assert!(!def.description.is_empty(), "enemy {} has empty description", def.name);
    }
}

#[test]
fn no_duplicate_enemy_names() {
    let mut names = std::collections::HashSet::new();
    for def in ENEMY_DEFS {
        assert!(names.insert(def.name), "duplicate enemy name: {}", def.name);
    }
}

#[test]
fn xp_for_known_enemies() {
    assert_eq!(xp_for_enemy("Giant Rat"), 3);
    assert_eq!(xp_for_enemy("Dragon"), 200);
    assert_eq!(xp_for_enemy("Goblin"), 4);
}

#[test]
fn xp_for_unknown_defaults_to_3() {
    assert_eq!(xp_for_enemy("Unknown Monster"), 3);
}

#[test]
fn enemy_description_for_known() {
    let desc = enemy_description("Dragon");
    assert_ne!(desc, "A mysterious creature.");
    assert!(desc.contains("guardian"));
}

#[test]
fn enemy_description_for_unknown() {
    assert_eq!(enemy_description("Unknown Monster"), "A mysterious creature.");
}

#[test]
fn item_table_config_defaults() {
    let cfg = GameConfig::normal();
    assert_eq!(cfg.item_tables.tier_bleed_up_pct, 20);
    assert_eq!(cfg.item_tables.tier_bleed_down_pct, 10);
    assert_eq!(cfg.item_tables.ranged_default_range, 4);
    assert!(!cfg.item_tables.ranged_base_ranges.is_empty());
}

#[test]
fn spawn_tables_rare_monsters() {
    let cfg = GameConfig::normal();
    assert!(!cfg.spawn_tables.rare_monster_names.is_empty());
    assert!(cfg.spawn_tables.rare_monster_names.contains(&"Wendigo"));
    assert!(cfg.spawn_tables.rare_monster_names.contains(&"Dryad"));
    assert!(!cfg.spawn_tables.rare_monster_names.contains(&"Wolf"));
}

#[test]
fn spawn_tables_loot_tiers() {
    let cfg = GameConfig::normal();
    assert!(!cfg.spawn_tables.monster_loot_tiers.is_empty());
    // Weaker rares drop tier 1, stronger drop tier 2
    let dryad_tier = cfg.spawn_tables.monster_loot_tiers.iter().find(|&&(n, _)| n == "Dryad").map(|&(_, t)| t);
    assert_eq!(dryad_tier, Some(1));
    let wendigo_tier = cfg.spawn_tables.monster_loot_tiers.iter().find(|&&(n, _)| n == "Wendigo").map(|&(_, t)| t);
    assert_eq!(wendigo_tier, Some(2));
}

#[test]
fn ranged_base_ranges_cover_all_bows() {
    let cfg = GameConfig::normal();
    let names: Vec<&str> = cfg.item_tables.ranged_base_ranges.iter().map(|&(n, _)| n).collect();
    assert!(names.contains(&"Short Bow"));
    assert!(names.contains(&"Long Bow"));
    assert!(names.contains(&"Elven Bow"));
    assert!(names.contains(&"Crossbow"));
    assert!(names.contains(&"Heavy Crossbow"));
}

#[test]
fn ai_behavior_config_defaults() {
    let cfg = GameConfig::normal();
    assert_eq!(cfg.combat.territorial_alert_range, 4);
    assert_eq!(cfg.combat.territorial_leash_range, 8);
    assert_eq!(cfg.combat.stalker_activation_range, 5);
    assert_eq!(cfg.combat.stalker_chase_range, 12);
    assert_eq!(cfg.combat.timid_flee_range, 5);
    assert_eq!(cfg.combat.passive_flee_range, 4);
    assert_eq!(cfg.combat.smart_pathfind_range, 10);
}

#[test]
fn stalker_chase_range_exceeds_normal() {
    let cfg = GameConfig::normal();
    assert!(cfg.combat.stalker_chase_range > cfg.combat.enemy_chase_range,
        "stalker chase range should exceed normal chase range");
}

#[test]
fn smart_enemy_names_has_humanoids() {
    let cfg = GameConfig::normal();
    assert!(cfg.spawn_tables.smart_enemy_names.contains(&"Goblin"));
    assert!(cfg.spawn_tables.smart_enemy_names.contains(&"Dragon"));
    assert!(cfg.spawn_tables.smart_enemy_names.contains(&"Orc"));
    assert!(!cfg.spawn_tables.smart_enemy_names.contains(&"Wolf"));
    assert!(!cfg.spawn_tables.smart_enemy_names.contains(&"Big Slime"));
}

#[test]
fn every_enemy_has_behavior() {
    for def in ENEMY_DEFS {
        // Just verify the field exists and is one of the valid variants
        let _ = def.behavior;
        assert_eq!(enemy_behavior(def.name), def.behavior,
            "{} behavior mismatch", def.name);
    }
}

#[test]
fn behavior_lookup_unknown_defaults_to_aggressive() {
    assert_eq!(enemy_behavior("NonExistent"), EnemyBehavior::Aggressive);
}
