use super::*;
use super::{test_game, overworld_game, rusty_sword};

    // --- Tile info ---

    #[test]
    fn inspect_player_tile() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.update_fov();
        let info = g.inspect_tile(g.player_x, g.player_y).unwrap();
        assert_eq!(info.tile_name, "Stone Floor");
        assert!(info.walkable);
        assert!(info.is_player);
        assert!(info.enemy.is_none());
    }

    #[test]
    fn inspect_hidden_tile_returns_none() {
        let map = Map::generate(30, 20, 42);
        let g = Game::new(map);
        // Without update_fov, all tiles are Hidden
        assert!(g.inspect_tile(0, 0).is_none());
    }

    #[test]
    fn inspect_enemy_tile() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.update_fov();
        let (ex, ey) = (g.player_x + 1, g.player_y);
        g.enemies.push(Enemy {
            x: ex, y: ey, hp: 10, attack: 3, defense: 0, glyph: 'g', name: "Goblin", facing_left: false, is_ranged: false,
            behavior: EnemyBehavior::Aggressive, spawn_x: ex, spawn_y: ey, provoked: false, is_boss: false,
        });
        let info = g.inspect_tile(ex, ey).unwrap();
        let enemy = info.enemy.unwrap();
        assert_eq!(enemy.name, "Goblin");
        assert_eq!(enemy.hp, 10);
        assert_eq!(enemy.attack, 3);
        assert_eq!(enemy.desc, "A sneaky green creature. Dangerous in numbers.");
    }

    #[test]
    fn inspect_item_tile() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.update_fov();
        let (ix, iy) = (g.player_x + 1, g.player_y);
        g.ground_items.push(GroundItem { x: ix, y: iy, item: rusty_sword() });
        let info = g.inspect_tile(ix, iy).unwrap();
        let item = info.item.unwrap();
        assert_eq!(item.name, "Rusty Sword");
        assert!(item.desc.contains("+3 Attack")); // Rusty Sword: +3 ATK, 10 stamina
    }

    #[test]
    fn inspect_out_of_bounds_returns_none() {
        let map = Map::generate(30, 20, 42);
        let g = Game::new(map);
        assert!(g.inspect_tile(-1, -1).is_none());
        assert!(g.inspect_tile(999, 999).is_none());
    }

    #[test]
    fn every_tile_has_name_and_desc() {
        let tiles = [
            Tile::Wall, Tile::Floor, Tile::Tree, Tile::Grass,
            Tile::Road, Tile::DungeonEntrance, Tile::StairsDown, Tile::StairsUp,
        ];
        for tile in tiles {
            assert!(!tile_name(tile).is_empty(), "{:?} has no name", tile);
            assert!(!tile_desc(tile).is_empty(), "{:?} has no desc", tile);
        }
    }

    #[test]
    fn every_enemy_has_desc() {
        use crate::config::{ENEMY_DEFS, enemy_description};
        // Every enemy in the registry should have a non-default description
        for def in ENEMY_DEFS {
            let desc = enemy_description(def.name);
            assert!(!desc.is_empty(), "{} has no desc", def.name);
            assert_ne!(desc, "A mysterious creature.", "{} should have a unique desc", def.name);
        }
    }

    // --- Location name ---

    #[test]
    fn location_name_overworld() {
        let g = overworld_game();
        assert_eq!(g.location_name(), "Overworld");
    }

    #[test]
    fn location_name_dungeon() {
        let mut g = overworld_game();
        g.enter_dungeon(0);
        let name = g.location_name();
        assert!(name.contains("(B1)"),
            "unexpected location name: {name}");
        assert!(name.contains("B1"));
    }

    // --- Drawers ---

    #[test]
    fn toggle_drawer_opens_and_closes() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        assert_eq!(g.ui.drawer, Drawer::None);
        g.toggle_drawer(Drawer::Inventory);
        assert_eq!(g.ui.drawer, Drawer::Inventory);
        g.toggle_drawer(Drawer::Inventory);
        assert_eq!(g.ui.drawer, Drawer::None);
    }

    #[test]
    fn toggle_drawer_switches_between_drawers() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.toggle_drawer(Drawer::Inventory);
        g.toggle_drawer(Drawer::Stats);
        assert_eq!(g.ui.drawer, Drawer::Stats);
    }

    // --- XP and leveling ---

    #[test]
    fn xp_granted_on_kill() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 1, attack: 0, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false, behavior: EnemyBehavior::Aggressive, spawn_x: gx, spawn_y: gy, provoked: false, is_boss: false });
        g.attack_adjacent(gx, gy);
        assert_eq!(g.player_xp, 4); // goblin = 4 XP
    }

    #[test]
    fn level_up_awards_skill_points() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let old_max = g.player_max_hp;
        // Force enough XP for level 2 (need 20 XP at level 1)
        g.player_xp = 20;
        g.check_level_up();
        assert_eq!(g.player_level, 2);
        assert_eq!(g.skill_points, 3); // 3 skill points per level
        assert_eq!(g.player_max_hp, old_max + 2); // base +2 HP per level
        // Partial heal (50% of missing + 1), at full HP → stays at max
        assert_eq!(g.player_hp, g.player_max_hp);
    }

    #[test]
    fn xp_to_next_level_scales() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let l1 = g.xp_to_next_level();
        g.player_level = 3;
        let l3 = g.xp_to_next_level();
        assert!(l3 > l1, "higher levels should need more XP");
    }

    #[test]
    fn xp_for_each_enemy_type() {
        // Forest
        assert_eq!(xp_for_enemy("Giant Rat"), 3);
        assert_eq!(xp_for_enemy("Giant Bat"), 4);
        assert_eq!(xp_for_enemy("Wolf"), 5);
        assert_eq!(xp_for_enemy("Giant Spider"), 6);
        assert_eq!(xp_for_enemy("Boar"), 7);
        assert_eq!(xp_for_enemy("Bear"), 12);
        // Rare overworld monsters (buffed)
        assert_eq!(xp_for_enemy("Dryad"), 20);
        assert_eq!(xp_for_enemy("Forest Spirit"), 18);
        assert_eq!(xp_for_enemy("Centaur"), 25);
        assert_eq!(xp_for_enemy("Dire Wolf"), 22);
        assert_eq!(xp_for_enemy("Lycanthrope"), 35);
        assert_eq!(xp_for_enemy("Wendigo"), 40);
        // Dungeon shallow
        assert_eq!(xp_for_enemy("Kobold"), 3);
        assert_eq!(xp_for_enemy("Small Slime"), 3);
        assert_eq!(xp_for_enemy("Goblin"), 4);
        assert_eq!(xp_for_enemy("Skeleton"), 6);
        // Dungeon mid
        assert_eq!(xp_for_enemy("Goblin Archer"), 7);
        assert_eq!(xp_for_enemy("Zombie"), 8);
        assert_eq!(xp_for_enemy("Skeleton Archer"), 9);
        assert_eq!(xp_for_enemy("Big Slime"), 9);
        assert_eq!(xp_for_enemy("Orc"), 14);
        // Dungeon deep
        assert_eq!(xp_for_enemy("Ghoul"), 16);
        assert_eq!(xp_for_enemy("Orc Blademaster"), 20);
        assert_eq!(xp_for_enemy("Wraith"), 16);
        assert_eq!(xp_for_enemy("Naga"), 22);
        assert_eq!(xp_for_enemy("Troll"), 22);
        // Cave boss
        assert_eq!(xp_for_enemy("Death Knight"), 32);
        assert_eq!(xp_for_enemy("Lich"), 35);
        assert_eq!(xp_for_enemy("Dragon"), 200);
    }

    #[test]
    fn kill_message_includes_xp() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 1, attack: 0, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false, behavior: EnemyBehavior::Aggressive, spawn_x: gx, spawn_y: gy, provoked: false, is_boss: false });
        g.attack_adjacent(gx, gy);
        assert!(g.messages.iter().any(|m| m.contains("+4 XP")));
    }

    #[test]
    fn player_starts_with_zero_skill_points() {
        let g = test_game();
        assert_eq!(g.skill_points, 0);
        assert_eq!(g.strength, 0);
        assert_eq!(g.vitality, 0);
    }

    #[test]
    fn allocate_strength_increases_attack() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.skill_points = 1;
        let atk_before = g.effective_attack();
        assert!(g.allocate_skill_point(SkillKind::Strength));
        assert_eq!(g.strength, 1);
        assert_eq!(g.effective_attack(), atk_before + 1);
        assert_eq!(g.skill_points, 0);
    }

    #[test]
    fn allocate_vitality_increases_max_hp() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.skill_points = 1;
        let hp_before = g.player_max_hp;
        assert!(g.allocate_skill_point(SkillKind::Vitality));
        assert_eq!(g.vitality, 1);
        assert_eq!(g.player_max_hp, hp_before + 3);
        assert_eq!(g.skill_points, 0);
    }

    #[test]
    fn allocate_dexterity_increases_dex() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.skill_points = 1;
        let dex_before = g.player_dexterity;
        assert!(g.allocate_skill_point(SkillKind::Dexterity));
        assert_eq!(g.player_dexterity, dex_before + 1);
        assert_eq!(g.skill_points, 0);
    }

    #[test]
    fn allocate_stamina_increases_max_stamina() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.skill_points = 1;
        let stam_before = g.max_stamina;
        assert!(g.allocate_skill_point(SkillKind::Stamina));
        assert_eq!(g.max_stamina, stam_before + 5); // max stamina +5
        assert_eq!(g.skill_points, 0);
    }

    #[test]
    fn allocate_fails_with_no_points() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        assert_eq!(g.skill_points, 0);
        assert!(!g.allocate_skill_point(SkillKind::Strength));
        assert_eq!(g.strength, 0);
    }

    #[test]
    fn allocate_generates_message() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.skill_points = 1;
        let msg_before = g.messages.len();
        g.allocate_skill_point(SkillKind::Strength);
        assert!(g.messages.len() > msg_before);
        assert!(g.messages.last().unwrap().contains("Strength"));
    }

    #[test]
    fn strength_affects_combat_damage() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.strength = 3;
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 20, attack: 1, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false, behavior: EnemyBehavior::Aggressive, spawn_x: gx, spawn_y: gy, provoked: false, is_boss: false });
        g.attack_adjacent(gx, gy);
        // Base attack 5 + strength 3 = 8 damage
        assert_eq!(g.enemies[0].hp, 20 - 8);
    }

    #[test]
    fn vitality_hp_gained_on_allocate() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.player_hp = 15; // damaged
        g.skill_points = 1;
        g.allocate_skill_point(SkillKind::Vitality);
        // HP should increase by 3 but not exceed new max
        assert_eq!(g.player_hp, 18);
        assert_eq!(g.player_max_hp, 23); // 20 + 3
    }

    #[test]
    fn level_up_message_mentions_skill_points() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.player_xp = 20;
        g.check_level_up();
        assert!(g.messages.iter().any(|m| m.contains("skill points")));
    }

    #[test]
    fn new_with_config_applies_player_stats() {
        let mut config = GameConfig::normal();
        config.player.starting_hp = 50;
        config.player.starting_attack = 10;
        config.player.starting_dexterity = 7;
        config.player.starting_stamina = 200;
        config.player.starting_hunger = 80;
        let map = Map::generate(30, 20, 42);
        let g = Game::new_with_config(map, config);
        assert_eq!(g.player_hp, 50);
        assert_eq!(g.player_max_hp, 50);
        assert_eq!(g.player_attack, 10);
        assert_eq!(g.player_dexterity, 7);
        assert_eq!(g.stamina, 200);
        assert_eq!(g.max_stamina, 200);
        assert_eq!(g.hunger, 80);
        assert_eq!(g.max_hunger, 80);
    }

    #[test]
    fn easy_config_gives_more_hp() {
        let easy = Game::new_with_config(Map::generate(30, 20, 42), GameConfig::easy());
        let normal = Game::new_with_config(Map::generate(30, 20, 42), GameConfig::normal());
        assert!(easy.player_hp > normal.player_hp);
    }

    #[test]
    fn hard_config_gives_less_hp() {
        let hard = Game::new_with_config(Map::generate(30, 20, 42), GameConfig::hard());
        let normal = Game::new_with_config(Map::generate(30, 20, 42), GameConfig::normal());
        assert!(hard.player_hp < normal.player_hp);
    }

    #[test]
    fn config_fov_radius_used() {
        let mut config = GameConfig::normal();
        config.fov.overworld_radius = 12;
        config.fov.dungeon_radius = 4;
        let map = Map::generate(30, 20, 42);
        let g = Game::new_with_config(map, config);
        // from_single_map uses Location::Overworld
        assert_eq!(g.fov_radius(), 12);
    }

    #[test]
    fn config_xp_formula_used() {
        let mut config = GameConfig::normal();
        config.progression.xp_base = 10.0;
        config.progression.xp_exponent = 1.0;
        let map = Map::generate(30, 20, 42);
        let g = Game::new_with_config(map, config);
        // xp_to_next = 10.0 * 1^1.0 = 10
        assert_eq!(g.xp_to_next_level(), 10);
    }
