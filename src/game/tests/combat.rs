use super::*;
use super::{test_game, rusty_sword};

    #[test]
    fn player_starts_with_full_hp() {
        let g = test_game();
        assert_eq!(g.player_hp, 20);
        assert_eq!(g.player_max_hp, 20);
        assert_eq!(g.player_attack, 5);
        assert!(g.alive);
        assert!(!g.won);
    }

    #[test]
    fn enemies_spawn_on_floor() {
        let g = test_game();
        for e in &g.enemies {
            assert!(g.current_map().is_walkable(e.x, e.y), "{} at ({},{}) not on floor", e.name, e.x, e.y);
        }
    }

    #[test]
    fn enemies_not_on_player() {
        let g = test_game();
        for e in &g.enemies {
            assert!(
                e.x != g.player_x || e.y != g.player_y,
                "enemy spawned on player"
            );
        }
    }

    #[test]
    fn overworld_has_biome_enemies() {
        let g = test_game();
        // All valid overworld glyphs (temperate forest animals + rare monsters)
        let valid_glyphs = [
            // Temperate forest animals
            'f', 'q', 'n', 'y', 'w', 'j', 'J', '#', '&', 'b', 'h', 'B',
            // Rare monsters
            '1', '2', '9', 'U', 'L', '0',
        ];
        for e in &g.enemies {
            assert!(
                valid_glyphs.contains(&e.glyph),
                "unexpected overworld enemy: {} ('{}')", e.name, e.glyph
            );
        }
        // Should have at least some enemies
        assert!(!g.enemies.is_empty(), "overworld should have enemies");
    }

    #[test]
    fn cave_level_has_dragon_boss() {
        let mut g = overworld_game();
        // Find the dungeon with 4 levels (the cave dungeon)
        let cave_di = g.world.dungeons.iter()
            .position(|d| d.levels.len() == 4)
            .expect("one dungeon should have a cave level");
        g.enter_dungeon(cave_di);
        // Descend to the cave (level 3)
        for level in 0..3 {
            g.descend(cave_di, level);
        }
        assert!(
            g.enemies.iter().any(|e| e.glyph == 'D'),
            "cave level should have dragon boss"
        );
        let dragon = g.enemies.iter().find(|e| e.glyph == 'D').unwrap();
        assert!(dragon.hp >= 30, "dragon hp should be >= 30, got {}", dragon.hp);
        assert!(dragon.attack >= 8, "dragon attack should be >= 8, got {}", dragon.attack);
    }

    #[test]
    fn non_cave_dungeon_has_no_dragon() {
        let mut g = overworld_game();
        // Find a dungeon without the cave (3 levels)
        let normal_di = g.world.dungeons.iter()
            .position(|d| d.levels.len() == 3)
            .expect("should have normal dungeons");
        g.enter_dungeon(normal_di);
        // Descend to deepest level (level 2)
        g.descend(normal_di, 0);
        g.descend(normal_di, 1);
        assert!(
            !g.enemies.iter().any(|e| e.glyph == 'D'),
            "non-cave dungeon should not have a dragon"
        );
    }

    #[test]
    fn attacking_enemy_deals_damage() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 10, attack: 2, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        g.attack_adjacent(gx, gy);
        assert_eq!(g.enemies[0].hp, 10 - g.player_attack);
        assert_eq!(g.player_x, gx - 1); // player didn't move
    }

    #[test]
    fn killing_enemy() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 3, attack: 1, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        let result = g.attack_adjacent(gx, gy);
        assert!(matches!(result, TurnResult::Killed { .. }));
        assert!(g.enemies[0].hp <= 0);
    }

    #[test]
    fn enemy_attacks_on_its_turn() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.player_dexterity = 0; // disable dodge for deterministic test
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 20, attack: 3, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        let hp_before = g.player_hp;
        g.attack_adjacent(gx, gy);
        // Enemy attacks back during enemy_turn (adjacent, calc_damage(3, 0) = 3)
        assert_eq!(g.player_hp, hp_before - 3);
    }

    #[test]
    fn player_can_die() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.player_hp = 1;
        g.player_dexterity = 0; // disable dodge
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 99, attack: 5, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        let result = g.attack_adjacent(gx, gy);
        assert!(matches!(result, TurnResult::PlayerDied));
        assert!(!g.alive);
    }

    #[test]
    fn dead_player_cant_move() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.alive = false;
        let (x, y) = (g.player_x, g.player_y);
        g.move_player(1, 0);
        assert_eq!((g.player_x, g.player_y), (x, y));
    }

    #[test]
    fn killing_dragon_wins() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let dx = g.player_x + 1;
        let dy = g.player_y;
        g.enemies.push(Enemy { x: dx, y: dy, hp: 1, attack: 0, glyph: 'D', name: "Dragon", facing_left: false, defense: 0, is_ranged: false });
        let result = g.attack_adjacent(dx, dy);
        assert!(matches!(result, TurnResult::Won));
        assert!(g.won);
    }

    #[test]
    fn won_player_cant_move() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.won = true;
        let (x, y) = (g.player_x, g.player_y);
        g.move_player(1, 0);
        assert_eq!((g.player_x, g.player_y), (x, y));
    }

    #[test]
    fn initial_message() {
        let g = test_game();
        assert_eq!(g.messages[0], "You enter the cave.");
    }

    #[test]
    fn combat_generates_messages() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 20, attack: 2, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        let msg_count_before = g.messages.len();
        g.attack_adjacent(gx, gy);
        assert!(g.messages.len() > msg_count_before, "combat should generate messages");
    }

    #[test]
    fn enemy_chases_player() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let ex = g.player_x + 3;
        let ey = g.player_y;
        if g.current_map().is_walkable(ex, ey) {
            g.enemies.push(Enemy { x: ex, y: ey, hp: 10, attack: 1, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
            if g.current_map().is_walkable(g.player_x, g.player_y + 1) {
                g.move_player(0, 1);
                let new_dist = (g.enemies[0].x - g.player_x).abs() + (g.enemies[0].y - g.player_y).abs();
                assert!(new_dist < 4, "enemy should have chased toward player, dist={new_dist}");
            }
        }
    }

    fn short_bow() -> Item {
        Item { kind: ItemKind::RangedWeapon, name: "Short Bow", glyph: '}', effect: ItemEffect::BuffAttack(1), weight: 2, durability: 250 }
    }
    fn crossbow() -> Item {
        Item { kind: ItemKind::RangedWeapon, name: "Crossbow", glyph: '}', effect: ItemEffect::BuffAttack(2), weight: 3, durability: 250 }
    }
    fn long_bow() -> Item {
        Item { kind: ItemKind::RangedWeapon, name: "Long Bow", glyph: '}', effect: ItemEffect::BuffAttack(3), weight: 2, durability: 400 }
    }
    fn elven_bow() -> Item {
        Item { kind: ItemKind::RangedWeapon, name: "Elven Bow", glyph: '}', effect: ItemEffect::BuffAttack(6), weight: 1, durability: 600 }
    }

    #[test]
    fn player_starts_with_dexterity() {
        let g = test_game();
        assert_eq!(g.player_dexterity, 3);
    }

    #[test]
    fn equip_ranged_weapon() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(short_bow());
        assert!(g.equip_item(0));
        assert!(g.inventory.is_empty());
        assert_eq!(g.equipped_weapon.as_ref().unwrap().name, "Short Bow");
        assert!(g.has_ranged_weapon());
    }

    #[test]
    fn ranged_weapon_goes_in_weapon_slot() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        // Equip melee weapon first
        g.inventory.push(rusty_sword());
        g.equip_item(0);
        assert!(!g.has_ranged_weapon());
        // Equip ranged weapon — swaps melee to inventory
        g.inventory.push(short_bow());
        g.equip_item(0);
        assert!(g.has_ranged_weapon());
        assert_eq!(g.inventory.len(), 1);
        assert_eq!(g.inventory[0].name, "Rusty Sword");
    }

    #[test]
    fn ranged_weapon_attack_bonus() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_weapon = Some(short_bow());
        // Base attack 5 + bow 1 = 6
        assert_eq!(g.effective_attack(), 6);
    }

    #[test]
    fn ranged_max_range_with_dexterity() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_weapon = Some(short_bow());
        // Short Bow base range 4, dex 3 → 4 + 3/3 = 5
        assert_eq!(g.ranged_max_range(), 5);
        g.player_dexterity = 9;
        // dex 9 → 4 + 9/3 = 7
        assert_eq!(g.ranged_max_range(), 7);
    }

    #[test]
    fn ranged_max_range_crossbow() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_weapon = Some(crossbow());
        // Crossbow base range 3, dex 3 → 3 + 1 = 4
        assert_eq!(g.ranged_max_range(), 4);
    }

    #[test]
    fn ranged_max_range_elven_bow() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_weapon = Some(elven_bow());
        // Elven Bow base range 8, dex 3 → 8 + 1 = 9
        assert_eq!(g.ranged_max_range(), 9);
    }

    #[test]
    fn ranged_hit_chance_decreases_with_distance() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_weapon = Some(short_bow());
        let chance_1 = g.ranged_hit_chance(1);
        let chance_3 = g.ranged_hit_chance(3);
        let chance_5 = g.ranged_hit_chance(5);
        assert!(chance_1 > chance_3, "closer should be more accurate");
        assert!(chance_3 > chance_5, "closer should be more accurate");
    }

    #[test]
    fn ranged_hit_chance_zero_beyond_range() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_weapon = Some(short_bow());
        let max_range = g.ranged_max_range();
        assert_eq!(g.ranged_hit_chance(max_range + 1), 0);
    }

    #[test]
    fn ranged_hit_chance_zero_at_zero_distance() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_weapon = Some(short_bow());
        assert_eq!(g.ranged_hit_chance(0), 0);
    }

    #[test]
    fn ranged_hit_chance_capped_at_95() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_weapon = Some(short_bow());
        g.player_dexterity = 100; // absurdly high
        assert!(g.ranged_hit_chance(1) <= 95);
    }

    #[test]
    fn ranged_hit_chance_improves_with_dexterity() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_weapon = Some(long_bow());
        g.player_dexterity = 3;
        let low_dex = g.ranged_hit_chance(4);
        g.player_dexterity = 9;
        let high_dex = g.ranged_hit_chance(4);
        assert!(high_dex > low_dex, "higher dex should improve hit chance");
    }

    #[test]
    fn ranged_attack_hits_enemy() {
        let mut map = Map::new_filled(20, 20, Tile::Floor);
        // Ensure edges are walls for valid map
        for x in 0..20 { map.set(x, 0, Tile::Wall); map.set(x, 19, Tile::Wall); }
        for y in 0..20 { map.set(0, y, Tile::Wall); map.set(19, y, Tile::Wall); }
        let mut g = Game::new(map);
        g.player_x = 5;
        g.player_y = 5;
        g.player_dexterity = 100; // guarantee hit
        g.equipped_weapon = Some(short_bow());
        g.enemies.push(Enemy { x: 8, y: 5, hp: 100, attack: 2, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        let result = g.ranged_attack(8, 5);
        assert!(matches!(result, TurnResult::Moved));
        // With dex 100, should definitely hit. Ranged damage includes distance + DEX bonuses.
        assert!(g.enemies[0].hp < 100, "enemy should have taken damage");
    }

    #[test]
    fn ranged_attack_kills_enemy() {
        let mut map = Map::new_filled(20, 20, Tile::Floor);
        for x in 0..20 { map.set(x, 0, Tile::Wall); map.set(x, 19, Tile::Wall); }
        for y in 0..20 { map.set(0, y, Tile::Wall); map.set(19, y, Tile::Wall); }
        let mut g = Game::new(map);
        g.player_x = 5;
        g.player_y = 5;
        g.player_dexterity = 100;
        g.equipped_weapon = Some(short_bow());
        g.enemies.push(Enemy { x: 8, y: 5, hp: 3, attack: 1, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        g.ranged_attack(8, 5);
        assert!(g.enemies[0].hp <= 0);
        assert!(g.messages.iter().any(|m| m.contains("slay")));
    }

    #[test]
    fn ranged_attack_no_retaliation() {
        let mut map = Map::new_filled(20, 20, Tile::Floor);
        for x in 0..20 { map.set(x, 0, Tile::Wall); map.set(x, 19, Tile::Wall); }
        for y in 0..20 { map.set(0, y, Tile::Wall); map.set(19, y, Tile::Wall); }
        let mut g = Game::new(map);
        g.player_x = 5;
        g.player_y = 5;
        g.player_dexterity = 100;
        g.equipped_weapon = Some(short_bow());
        g.enemies.push(Enemy { x: 8, y: 5, hp: 99, attack: 10, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        let hp_before = g.player_hp;
        g.ranged_attack(8, 5);
        // Enemy is 3 tiles away — no retaliation from the ranged shot itself
        // (enemy_turn may still attack if it chases into melee)
        // At distance 3, the enemy won't reach the player in one turn
        assert_eq!(g.player_hp, hp_before, "ranged attack should not cause retaliation from distant enemy");
    }

    #[test]
    fn ranged_attack_out_of_range() {
        let mut map = Map::new_filled(30, 30, Tile::Floor);
        for x in 0..30 { map.set(x, 0, Tile::Wall); map.set(x, 29, Tile::Wall); }
        for y in 0..30 { map.set(0, y, Tile::Wall); map.set(29, y, Tile::Wall); }
        let mut g = Game::new(map);
        g.player_x = 5;
        g.player_y = 5;
        g.equipped_weapon = Some(short_bow());
        // Place enemy far beyond range
        g.enemies.push(Enemy { x: 25, y: 5, hp: 10, attack: 1, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        let result = g.ranged_attack(25, 5);
        assert!(matches!(result, TurnResult::Blocked));
        assert_eq!(g.enemies[0].hp, 10, "out-of-range shot should not damage");
    }

    #[test]
    fn ranged_attack_blocked_by_wall() {
        let mut map = Map::new_filled(20, 20, Tile::Floor);
        for x in 0..20 { map.set(x, 0, Tile::Wall); map.set(x, 19, Tile::Wall); }
        for y in 0..20 { map.set(0, y, Tile::Wall); map.set(19, y, Tile::Wall); }
        // Place a wall between player and enemy
        map.set(7, 5, Tile::Wall);
        let mut g = Game::new(map);
        g.player_x = 5;
        g.player_y = 5;
        g.equipped_weapon = Some(short_bow());
        g.enemies.push(Enemy { x: 9, y: 5, hp: 10, attack: 1, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        let result = g.ranged_attack(9, 5);
        assert!(matches!(result, TurnResult::Blocked));
        assert!(g.messages.iter().any(|m| m.contains("line of sight")));
    }

    #[test]
    fn ranged_attack_no_enemy() {
        let mut map = Map::new_filled(20, 20, Tile::Floor);
        for x in 0..20 { map.set(x, 0, Tile::Wall); map.set(x, 19, Tile::Wall); }
        for y in 0..20 { map.set(0, y, Tile::Wall); map.set(19, y, Tile::Wall); }
        let mut g = Game::new(map);
        g.player_x = 5;
        g.player_y = 5;
        g.equipped_weapon = Some(short_bow());
        let result = g.ranged_attack(8, 5);
        assert!(matches!(result, TurnResult::Blocked));
        assert!(g.messages.iter().any(|m| m.contains("Nothing to shoot")));
    }

    #[test]
    fn ranged_attack_needs_ranged_weapon() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_weapon = Some(rusty_sword()); // melee weapon
        let result = g.ranged_attack(g.player_x + 3, g.player_y);
        assert!(matches!(result, TurnResult::Blocked));
    }

    #[test]
    fn ranged_attack_updates_facing() {
        let mut map = Map::new_filled(20, 20, Tile::Floor);
        for x in 0..20 { map.set(x, 0, Tile::Wall); map.set(x, 19, Tile::Wall); }
        for y in 0..20 { map.set(0, y, Tile::Wall); map.set(19, y, Tile::Wall); }
        let mut g = Game::new(map);
        g.player_x = 10;
        g.player_y = 10;
        g.player_dexterity = 100;
        g.equipped_weapon = Some(short_bow());
        g.enemies.push(Enemy { x: 7, y: 10, hp: 20, attack: 1, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        g.ranged_attack(7, 10); // shooting left
        assert!(g.player_facing_left);
    }

    #[test]
    fn use_ranged_weapon_returns_false() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(short_bow());
        assert!(!g.use_item(0), "ranged weapon should not be usable as consumable");
        assert_eq!(g.inventory.len(), 1);
    }

    #[test]
    fn has_ranged_weapon_false_without_weapon() {
        let g = test_game();
        assert!(!g.has_ranged_weapon());
    }

    #[test]
    fn has_ranged_weapon_false_with_melee() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_weapon = Some(rusty_sword());
        assert!(!g.has_ranged_weapon());
    }

    #[test]
    fn multiple_level_ups_stack_skill_points() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        // Give enough XP for 3 level-ups
        g.player_xp = 200;
        g.check_level_up();
        assert!(g.player_level >= 3);
        assert!(g.skill_points >= 4); // at least 2 level ups * 2 points
    }

    #[test]
    fn ranged_weapons_in_loot_tables() {
        // Tier 0 should produce bows/crossbows
        let mut rng = 42u64;
        let items: Vec<_> = (0..200).map(|_| random_item(0, &mut rng)).collect();
        assert!(items.iter().any(|i| i.kind == ItemKind::RangedWeapon),
            "tier 0 should generate ranged weapons");
        // Tier 1 should produce long bows/heavy crossbows
        rng = 42;
        let items: Vec<_> = (0..200).map(|_| random_item(1, &mut rng)).collect();
        assert!(items.iter().any(|i| i.kind == ItemKind::RangedWeapon),
            "tier 1 should generate ranged weapons");
        // Tier 2 should produce elven bow
        rng = 42;
        let items: Vec<_> = (0..200).map(|_| random_item(2, &mut rng)).collect();
        assert!(items.iter().any(|i| i.name == "Elven Bow"),
            "tier 2 should generate Elven Bow");
    }

    // --- Stamina cost for attacks ---

    #[test]
    fn melee_attack_drains_stamina() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 50, attack: 0, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        let stam_before = g.stamina;
        let cost = g.melee_stamina_cost(); // per-weapon stamina cost
        g.attack_adjacent(gx, gy);
        // Combat turns do NOT regen stamina — only walking does
        assert_eq!(g.stamina, stam_before - cost);
    }

    #[test]
    fn melee_attack_blocked_without_stamina() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 50, attack: 0, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        g.stamina = 0;
        let result = g.attack_adjacent(gx, gy);
        assert!(matches!(result, TurnResult::Blocked));
        assert_eq!(g.enemies[0].hp, 50, "enemy should not take damage when attack blocked");
        assert!(g.messages.iter().any(|m| m.contains("stamina")));
    }

    #[test]
    fn ranged_attack_drains_stamina() {
        let mut map = Map::new_filled(20, 20, Tile::Floor);
        for x in 0..20 { map.set(x, 0, Tile::Wall); map.set(x, 19, Tile::Wall); }
        for y in 0..20 { map.set(0, y, Tile::Wall); map.set(19, y, Tile::Wall); }
        let mut g = Game::new(map);
        g.player_x = 5;
        g.player_y = 5;
        g.player_dexterity = 100;
        g.equipped_weapon = Some(short_bow());
        g.enemies.push(Enemy { x: 8, y: 5, hp: 100, attack: 0, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        let stam_before = g.stamina;
        let cost = g.ranged_stamina_cost(); // per-weapon stamina cost
        g.ranged_attack(8, 5);
        // Combat turns do NOT regen stamina — only walking does
        assert_eq!(g.stamina, stam_before - cost);
    }

    #[test]
    fn ranged_attack_blocked_without_stamina() {
        let mut map = Map::new_filled(20, 20, Tile::Floor);
        for x in 0..20 { map.set(x, 0, Tile::Wall); map.set(x, 19, Tile::Wall); }
        for y in 0..20 { map.set(0, y, Tile::Wall); map.set(19, y, Tile::Wall); }
        let mut g = Game::new(map);
        g.player_x = 5;
        g.player_y = 5;
        g.player_dexterity = 100;
        g.equipped_weapon = Some(short_bow());
        g.enemies.push(Enemy { x: 8, y: 5, hp: 100, attack: 0, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        g.stamina = 0;
        let result = g.ranged_attack(8, 5);
        assert!(matches!(result, TurnResult::Blocked));
        assert_eq!(g.enemies[0].hp, 100, "enemy should not take damage when attack blocked");
        assert!(g.messages.iter().any(|m| m.contains("stamina")));
    }

    #[test]
    fn melee_attack_allowed_with_exact_stamina() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 50, attack: 0, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        g.stamina = g.melee_stamina_cost(); // per-weapon cost
        let result = g.attack_adjacent(gx, gy);
        assert!(!matches!(result, TurnResult::Blocked));
        assert!(g.enemies[0].hp < 50, "attack should land with exact stamina");
    }

    // --- Weapon-based stamina cost ---

    #[test]
    fn unarmed_melee_stamina_cost() {
        let g = test_game();
        // Unarmed: base 6 + weight 0 * 2 = 6
        assert_eq!(g.melee_stamina_cost(), 6);
    }

    #[test]
    fn light_weapon_costs_less_stamina() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        // Iron Dagger: weight 1 → cost = 6 + 1*2 = 8
        g.equipped_weapon = Some(Item {
            kind: ItemKind::Weapon, name: "Iron Dagger", glyph: '/',
            effect: ItemEffect::BuffAttack(2), weight: 1, durability: 200,
        });
        assert_eq!(g.melee_stamina_cost(), 8);
    }

    #[test]
    fn heavy_weapon_costs_more_stamina() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        // Great Hammer: weight 5 → cost = 6 + 5*2 = 16
        g.equipped_weapon = Some(Item {
            kind: ItemKind::Weapon, name: "Great Hammer", glyph: '/',
            effect: ItemEffect::BuffAttack(8), weight: 5, durability: 500,
        });
        assert_eq!(g.melee_stamina_cost(), 16);
    }

    #[test]
    fn ranged_weapon_stamina_cost_uses_weight() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        // Short Bow: weight 2 → cost = 4 + 2 = 6
        g.equipped_weapon = Some(short_bow());
        assert_eq!(g.ranged_stamina_cost(), 6);
        // Heavy Crossbow: weight 4 → cost = 4 + 4 = 8
        g.equipped_weapon = Some(Item {
            kind: ItemKind::RangedWeapon, name: "Heavy Crossbow", glyph: '}',
            effect: ItemEffect::BuffAttack(4), weight: 4, durability: 400,
        });
        assert_eq!(g.ranged_stamina_cost(), 8);
    }

    #[test]
    fn rare_weapons_have_low_stamina_cost() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        // Enchanted Blade: weight 1, +9 ATK → cost = 6 + 1*2 = 8
        g.equipped_weapon = Some(Item {
            kind: ItemKind::Weapon, name: "Enchanted Blade", glyph: '/',
            effect: ItemEffect::BuffAttack(9), weight: 1, durability: 500,
        });
        assert_eq!(g.melee_stamina_cost(), 8);
        // Elven Bow: weight 1, +6 ATK → cost = 4 + 1 = 5
        g.equipped_weapon = Some(elven_bow());
        assert_eq!(g.ranged_stamina_cost(), 5);
    }

    #[test]
    fn heavy_weapon_stamina_cost_in_combat() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        // Equip a heavy weapon (weight 5, cost 16)
        g.equipped_weapon = Some(Item {
            kind: ItemKind::Weapon, name: "Great Axe", glyph: '/',
            effect: ItemEffect::BuffAttack(8), weight: 5, durability: 500,
        });
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 50, attack: 0, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        let stam_before = g.stamina;
        g.attack_adjacent(gx, gy);
        // Combat turns do NOT regen stamina
        assert_eq!(g.stamina, stam_before - 16);
    }

    #[test]
    fn melee_weapons_deal_more_damage_than_ranged_same_tier() {
        // For each tier, melee BuffAttack should be >= ranged BuffAttack
        let mut rng = 42u64;
        for tier in 0..=2 {
            let items: Vec<_> = (0..1000).map(|_| random_item(tier, &mut rng)).collect();
            let melee_atks: Vec<i32> = items.iter()
                .filter(|i| i.kind == ItemKind::Weapon)
                .filter_map(|i| match i.effect { ItemEffect::BuffAttack(n) => Some(n), _ => None })
                .collect();
            let ranged_atks: Vec<i32> = items.iter()
                .filter(|i| i.kind == ItemKind::RangedWeapon)
                .filter_map(|i| match i.effect { ItemEffect::BuffAttack(n) => Some(n), _ => None })
                .collect();
            if !melee_atks.is_empty() && !ranged_atks.is_empty() {
                let avg_melee: f64 = melee_atks.iter().sum::<i32>() as f64 / melee_atks.len() as f64;
                let avg_ranged: f64 = ranged_atks.iter().sum::<i32>() as f64 / ranged_atks.len() as f64;
                assert!(avg_melee > avg_ranged,
                    "tier {tier}: melee avg {avg_melee} should be > ranged avg {avg_ranged}");
            }
        }
    }

    #[test]
    fn ranged_attack_costs_a_turn() {
        let mut map = Map::new_filled(20, 20, Tile::Floor);
        for x in 0..20 { map.set(x, 0, Tile::Wall); map.set(x, 19, Tile::Wall); }
        for y in 0..20 { map.set(0, y, Tile::Wall); map.set(19, y, Tile::Wall); }
        let mut g = Game::new(map);
        g.player_x = 5;
        g.player_y = 5;
        g.player_dexterity = 100;
        g.equipped_weapon = Some(short_bow());
        g.enemies.push(Enemy { x: 8, y: 5, hp: 99, attack: 1, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        let turn_before = g.turn;
        g.ranged_attack(8, 5);
        assert_eq!(g.turn, turn_before + 1, "ranged attack should advance turn counter");
    }

    #[test]
    fn melee_kill_drains_stamina() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        // Enemy with 1 HP — guaranteed one-hit kill
        g.enemies.push(Enemy { x: gx, y: gy, hp: 1, attack: 0, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        let stam_before = g.stamina;
        let cost = g.melee_stamina_cost();
        let result = g.attack_adjacent(gx, gy);
        assert!(matches!(result, TurnResult::Killed { .. }));
        // Combat turns do NOT regen stamina
        assert_eq!(g.stamina, stam_before - cost, "stamina should drain full cost on kill");
    }

    #[test]
    fn ranged_kill_drains_stamina() {
        let mut map = Map::new_filled(20, 20, Tile::Floor);
        for x in 0..20 { map.set(x, 0, Tile::Wall); map.set(x, 19, Tile::Wall); }
        for y in 0..20 { map.set(0, y, Tile::Wall); map.set(19, y, Tile::Wall); }
        let mut g = Game::new(map);
        g.player_x = 5;
        g.player_y = 5;
        g.player_dexterity = 100; // guarantee hit
        g.equipped_weapon = Some(short_bow());
        // Enemy with 1 HP — guaranteed one-hit kill
        g.enemies.push(Enemy { x: 8, y: 5, hp: 1, attack: 0, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        let stam_before = g.stamina;
        let cost = g.ranged_stamina_cost();
        g.ranged_attack(8, 5);
        assert!(g.enemies[0].hp <= 0, "enemy should be dead");
        // Combat turns do NOT regen stamina
        assert_eq!(g.stamina, stam_before - cost, "stamina should drain full cost on ranged kill");
    }

    #[test]
    fn dragon_kill_drains_stamina() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let dx = g.player_x + 1;
        let dy = g.player_y;
        g.enemies.push(Enemy { x: dx, y: dy, hp: 1, attack: 0, glyph: 'D', name: "Dragon", facing_left: false, defense: 0, is_ranged: false });
        let stam_before = g.stamina;
        let cost = g.melee_stamina_cost();
        let result = g.attack_adjacent(dx, dy);
        assert!(matches!(result, TurnResult::Won));
        // Dragon kill early-returns, skipping end_combat_turn (no regen)
        let expected = stam_before - cost;
        assert_eq!(g.stamina, expected, "stamina should drain on dragon kill: before={stam_before}, cost={cost}");
    }
