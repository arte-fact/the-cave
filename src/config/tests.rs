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
}

#[test]
fn easy_is_more_forgiving_than_normal() {
    let easy = GameConfig::easy();
    let normal = GameConfig::normal();
    assert!(easy.player.starting_hp > normal.player.starting_hp);
    assert!(easy.survival.hunger_interval_overworld > normal.survival.hunger_interval_overworld);
    assert!(easy.spawn.overworld_enemy_pct < normal.spawn.overworld_enemy_pct);
    assert!(easy.progression.skill_points_per_level > normal.progression.skill_points_per_level);
}

#[test]
fn hard_is_tougher_than_normal() {
    let hard = GameConfig::hard();
    let normal = GameConfig::normal();
    assert!(hard.player.starting_hp < normal.player.starting_hp);
    assert!(hard.survival.hunger_interval_overworld < normal.survival.hunger_interval_overworld);
    assert!(hard.spawn.overworld_enemy_pct > normal.spawn.overworld_enemy_pct);
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
