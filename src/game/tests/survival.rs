use super::*;
use super::{test_game, health_potion};

    fn raw_food(amount: i32) -> Item {
        Item { kind: ItemKind::Food, name: "Wild Berries", glyph: '%', effect: ItemEffect::Feed(amount, FoodSideEffect::None), weight: 0, durability: 0, legendary: false }
    }

    // --- Stamina ---

    #[test]
    fn player_starts_with_full_stamina() {
        let g = test_game();
        assert_eq!(g.stamina, 100);
        assert_eq!(g.max_stamina, 100);
        assert!(!g.sprinting);
    }

    #[test]
    fn stamina_regens_on_walk() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.stamina = 50;
        // Find a walkable neighbor
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in dirs {
            let (nx, ny) = (g.player_x + dx, g.player_y + dy);
            if g.current_map().is_walkable(nx, ny) {
                g.move_player(dx, dy);
                // Walking regens STAMINA_REGEN (5)
                assert_eq!(g.stamina, 55, "stamina should regen by 5 on walk");
                return;
            }
        }
        panic!("no walkable neighbor");
    }

    #[test]
    fn stamina_capped_at_max() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.stamina = 98;
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in dirs {
            let (nx, ny) = (g.player_x + dx, g.player_y + dy);
            if g.current_map().is_walkable(nx, ny) {
                g.move_player(dx, dy);
                assert_eq!(g.stamina, 100, "stamina should cap at max");
                return;
            }
        }
    }

    // --- Sprint ---

    #[test]
    fn toggle_sprint_on_and_off() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        assert!(!g.sprinting);
        g.toggle_sprint();
        assert!(g.sprinting);
        g.toggle_sprint();
        assert!(!g.sprinting);
    }

    #[test]
    fn sprint_denied_when_low_stamina() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.stamina = 0;
        g.toggle_sprint();
        assert!(!g.sprinting, "sprint should be denied when stamina too low");
        assert!(g.messages.iter().any(|m| m.contains("exhausted")));
    }

    #[test]
    fn sprint_cost_is_twice_regen_rate() {
        let g = Game::new(Map::generate(30, 20, 42));
        // Default stamina_regen = 5, so sprint cost = 10
        assert_eq!(g.sprint_cost(), 10);
    }

    #[test]
    fn sprint_drains_stamina_on_move() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.sprinting = true;
        let stam_before = g.stamina;
        let cost = g.sprint_cost(); // 10
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in dirs {
            let (nx, ny) = (g.player_x + dx, g.player_y + dy);
            if g.current_map().is_walkable(nx, ny) {
                g.move_player(dx, dy);
                assert_eq!(g.stamina, stam_before - cost, "sprint should drain {cost} stamina");
                return;
            }
        }
    }

    #[test]
    fn sprint_auto_disables_when_exhausted() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.sprinting = true;
        g.stamina = g.sprint_cost(); // exactly one sprint move left
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in dirs {
            let (nx, ny) = (g.player_x + dx, g.player_y + dy);
            if g.current_map().is_walkable(nx, ny) {
                g.move_player(dx, dy);
                assert_eq!(g.stamina, 0);
                assert!(!g.sprinting, "sprint should auto-disable at 0 stamina");
                assert!(g.messages.iter().any(|m| m.contains("Exhausted")));
                return;
            }
        }
    }

    #[test]
    fn sprint_skips_enemy_turn_on_first_move() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.sprinting = true;
        // Place enemy 2 tiles away
        let ex = g.player_x + 2;
        let ey = g.player_y;
        if g.current_map().is_walkable(ex, ey) {
            g.enemies.push(Enemy { x: ex, y: ey, hp: 10, attack: 3, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false, behavior: EnemyBehavior::Aggressive, spawn_x: ex, spawn_y: ey, provoked: false, is_boss: false });
            // Move away from enemy
            if g.current_map().is_walkable(g.player_x, g.player_y + 1) {
                g.move_player(0, 1);
                // First sprint move: enemy should NOT have moved
                assert_eq!(g.enemies[0].x, ex, "enemy should not chase on first sprint move");
                assert_eq!(g.enemies[0].y, ey, "enemy should not chase on first sprint move");
            }
        }
    }

    #[test]
    fn sprint_enemies_act_every_other_move() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.sprinting = true;
        // Place enemy 4 tiles away so it chases but can't attack
        let ex = g.player_x + 4;
        let ey = g.player_y;
        if !g.current_map().is_walkable(ex, ey) { return; }
        g.enemies.push(Enemy { x: ex, y: ey, hp: 10, attack: 3, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false, behavior: EnemyBehavior::Aggressive, spawn_x: ex, spawn_y: ey, provoked: false, is_boss: false });

        // First move: enemies should be skipped
        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let mut moved = false;
        for (dx, dy) in dirs {
            let (nx, ny) = (g.player_x + dx, g.player_y + dy);
            if g.current_map().is_walkable(nx, ny) {
                g.move_player(dx, dy);
                moved = true;
                break;
            }
        }
        if !moved { return; }
        let pos_after_first = (g.enemies[0].x, g.enemies[0].y);
        assert_eq!(pos_after_first, (ex, ey), "enemy should not move on first sprint move");

        // Second move: enemies should act
        moved = false;
        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for (dx, dy) in dirs {
            let (nx, ny) = (g.player_x + dx, g.player_y + dy);
            if g.current_map().is_walkable(nx, ny) {
                g.move_player(dx, dy);
                moved = true;
                break;
            }
        }
        if !moved { return; }
        let pos_after_second = (g.enemies[0].x, g.enemies[0].y);
        assert_ne!(pos_after_second, (ex, ey), "enemy should move on second sprint move");
    }

    #[test]
    fn sprint_toggle_off_resets_skip() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.sprinting = true;
        g.sprint_skip_turn = true;
        g.toggle_sprint();
        assert!(!g.sprinting);
        assert!(!g.sprint_skip_turn, "toggling sprint off should reset skip counter");
    }

    // --- Hunger ---

    #[test]
    fn player_starts_with_full_hunger() {
        let g = test_game();
        assert_eq!(g.hunger, 100);
        assert_eq!(g.max_hunger, 100);
    }

    #[test]
    fn hunger_drains_every_five_moves() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let hunger_before = g.hunger;
        // Move 5 times to trigger hunger drain (every HUNGER_INTERVAL turns)
        let mut moves = 0;
        for _ in 0..10 {
            let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
            for (dx, dy) in dirs {
                let (nx, ny) = (g.player_x + dx, g.player_y + dy);
                if g.current_map().is_walkable(nx, ny) {
                    g.move_player(dx, dy);
                    moves += 1;
                    if moves < 5 {
                        assert_eq!(g.hunger, hunger_before,
                            "hunger should NOT drain before 5 moves (move {moves})");
                    }
                    if moves == 5 {
                        assert_eq!(g.hunger, hunger_before - 1,
                            "hunger should drain 1 after 5 moves");
                        return;
                    }
                    break;
                }
            }
        }
    }

    #[test]
    fn starvation_damages_player() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.hunger = 0; // Already starving
        let hp_before = g.player_hp;
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in dirs {
            let (nx, ny) = (g.player_x + dx, g.player_y + dy);
            if g.current_map().is_walkable(nx, ny) {
                g.move_player(dx, dy);
                assert_eq!(g.hunger, 0);
                assert_eq!(g.player_hp, hp_before - 1, "starvation should deal 1 damage");
                return;
            }
        }
    }

    #[test]
    fn starvation_can_kill_player() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.hunger = 0;
        g.player_hp = 1;
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in dirs {
            let (nx, ny) = (g.player_x + dx, g.player_y + dy);
            if g.current_map().is_walkable(nx, ny) {
                g.move_player(dx, dy);
                assert!(!g.alive, "starvation should kill at 0 HP");
                assert!(g.messages.iter().any(|m| m.contains("starved")));
                return;
            }
        }
    }

    #[test]
    fn hunger_doesnt_go_negative() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.hunger = 0;
        g.player_hp = 20; // Won't die
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in dirs {
            let (nx, ny) = (g.player_x + dx, g.player_y + dy);
            if g.current_map().is_walkable(nx, ny) {
                g.move_player(dx, dy);
                assert_eq!(g.hunger, 0, "hunger should not go below 0");
                return;
            }
        }
    }

    // --- Food ---

    #[test]
    fn eat_food_restores_hunger() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.hunger = 50;
        g.inventory.push(raw_food(20));
        assert!(g.eat_food(0));
        assert_eq!(g.hunger, 70);
        assert!(g.inventory.is_empty());
    }

    #[test]
    fn eat_food_capped_at_max() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.hunger = 90;
        g.inventory.push(raw_food(20));
        g.eat_food(0);
        assert_eq!(g.hunger, 100);
    }

    #[test]
    fn eat_food_message() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.hunger = 80;
        g.inventory.push(raw_food(10));
        g.eat_food(0);
        assert!(g.messages.iter().any(|m| m.contains("Hunger +10")));
    }

    #[test]
    fn eat_non_food_fails() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(health_potion());
        assert!(!g.eat_food(0));
        assert_eq!(g.inventory.len(), 1);
    }

    #[test]
    fn use_item_works_on_food() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.hunger = 50;
        g.inventory.push(raw_food(15));
        assert!(g.use_item(0));
        assert_eq!(g.hunger, 65);
        assert!(g.inventory.is_empty());
    }

    // --- Meat drops ---

    #[test]
    fn killing_wolf_drops_meat() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 1, attack: 0, glyph: 'w', name: "Wolf", facing_left: false, defense: 0, is_ranged: false, behavior: EnemyBehavior::Aggressive, spawn_x: gx, spawn_y: gy, provoked: false, is_boss: false });
        g.attack_adjacent(gx, gy);
        assert!(g.ground_items.iter().any(|gi| gi.item.name == "Wolf Meat"),
            "wolf should drop meat");
    }

    #[test]
    fn killing_boar_drops_meat() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 1, attack: 0, glyph: 'b', name: "Boar", facing_left: false, defense: 0, is_ranged: false, behavior: EnemyBehavior::Aggressive, spawn_x: gx, spawn_y: gy, provoked: false, is_boss: false });
        g.attack_adjacent(gx, gy);
        assert!(g.ground_items.iter().any(|gi| gi.item.name == "Boar Meat"));
    }

    #[test]
    fn killing_bear_drops_meat() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 1, attack: 0, glyph: 'B', name: "Bear", facing_left: false, defense: 0, is_ranged: false, behavior: EnemyBehavior::Aggressive, spawn_x: gx, spawn_y: gy, provoked: false, is_boss: false });
        g.attack_adjacent(gx, gy);
        assert!(g.ground_items.iter().any(|gi| gi.item.name == "Bear Meat"));
    }

    #[test]
    fn killing_rat_drops_rat_meat() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 1, attack: 0, glyph: 'r', name: "Giant Rat", facing_left: false, defense: 0, is_ranged: false, behavior: EnemyBehavior::Aggressive, spawn_x: gx, spawn_y: gy, provoked: false, is_boss: false });
        g.attack_adjacent(gx, gy);
        assert!(g.ground_items.iter().any(|gi| gi.item.name == "Rat Meat"),
            "giant rat should drop rat meat");
    }

    #[test]
    fn killing_goblin_drops_rations() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 1, attack: 0, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false, behavior: EnemyBehavior::Aggressive, spawn_x: gx, spawn_y: gy, provoked: false, is_boss: false });
        g.attack_adjacent(gx, gy);
        assert!(g.ground_items.iter().any(|gi| gi.item.name == "Stolen Rations"),
            "goblin should drop stolen rations");
    }

    #[test]
    fn meat_has_feed_effect() {
        let meat = meat_drop("Wolf").unwrap();
        assert_eq!(meat.kind, ItemKind::Food);
        assert!(matches!(meat.effect, ItemEffect::Feed(_, _)));
    }

    #[test]
    fn bear_meat_restores_more_than_wolf() {
        let wolf = meat_drop("Wolf").unwrap();
        let bear = meat_drop("Bear").unwrap();
        let wolf_feed = match wolf.effect { ItemEffect::Feed(n, _) => n, _ => 0 };
        let bear_feed = match bear.effect { ItemEffect::Feed(n, _) => n, _ => 0 };
        assert!(bear_feed > wolf_feed, "bear meat should restore more hunger");
    }

    // --- Food spawning ---

    #[test]
    fn overworld_has_food_on_grass() {
        let mut g = overworld_game();
        g.spawn_overworld_food(42);
        let food_count = g.ground_items.iter()
            .filter(|gi| gi.item.kind == ItemKind::Food)
            .count();
        assert!(food_count > 0, "overworld should have food items on grass");
        // All food should be on grass
        let map = g.current_map();
        for gi in g.ground_items.iter().filter(|gi| gi.item.kind == ItemKind::Food) {
            assert_eq!(map.get(gi.x, gi.y), Tile::Grass,
                "food should only spawn on grass, found at ({},{})", gi.x, gi.y);
        }
    }

    #[test]
    fn overworld_food_has_variety() {
        let mut g = overworld_game();
        g.spawn_overworld_food(42);
        let food_names: std::collections::HashSet<&str> = g.ground_items.iter()
            .filter(|gi| gi.item.kind == ItemKind::Food)
            .map(|gi| gi.item.name)
            .collect();
        assert!(food_names.len() >= 2, "overworld should have at least 2 food types, got: {:?}", food_names);
    }

    #[test]
    fn large_beasts_drop_food() {
        let beasts = ["Wolf", "Boar", "Bear"];
        for name in beasts {
            assert!(meat_drop(name).is_some(), "{name} should drop food");
        }
    }

    #[test]
    fn inedible_creatures_drop_no_food() {
        let creatures = ["Giant Bat", "Giant Spider"];
        for name in creatures {
            assert!(meat_drop(name).is_none(), "{name} should not drop food");
        }
    }

    #[test]
    fn meat_feed_values_scale_with_beast() {
        let drops: Vec<_> = ["Wolf", "Boar", "Bear"]
            .iter()
            .map(|n| {
                let item = meat_drop(n).unwrap();
                match item.effect { ItemEffect::Feed(v, _) => v, _ => 0 }
            })
            .collect();
        // Bear meat should be the most filling
        assert!(*drops.last().unwrap() > *drops.first().unwrap(),
            "larger beasts should drop more filling food");
    }

    #[test]
    fn dungeon_food_includes_drinks() {
        let mut rng = 42u64;
        let items: Vec<_> = (0..500).map(|_| random_item(1, &mut rng)).collect();
        let drink_names = ["Dwarven Ale"];
        assert!(items.iter().any(|i| drink_names.contains(&i.name)),
            "dungeon tier 1 should produce drinks");
    }

    #[test]
    fn deep_dungeon_food_better_than_shallow() {
        let mut rng = 42u64;
        let t0_food: Vec<_> = (0..500)
            .map(|_| random_item(0, &mut rng))
            .filter(|i| i.kind == ItemKind::Food)
            .collect();
        rng = 42;
        let t2_food: Vec<_> = (0..500)
            .map(|_| random_item(2, &mut rng))
            .filter(|i| i.kind == ItemKind::Food)
            .collect();
        let avg_t0: f64 = t0_food.iter().map(|i| match i.effect { ItemEffect::Feed(v, _) => v as f64, _ => 0.0 }).sum::<f64>() / t0_food.len() as f64;
        let avg_t2: f64 = t2_food.iter().map(|i| match i.effect { ItemEffect::Feed(v, _) => v as f64, _ => 0.0 }).sum::<f64>() / t2_food.len() as f64;
        assert!(avg_t2 > avg_t0, "deep dungeon food should be more filling: t0_avg={avg_t0}, t2_avg={avg_t2}");
    }

    #[test]
    fn random_item_includes_food() {
        let mut rng = 42u64;
        let items: Vec<_> = (0..200).map(|_| random_item(0, &mut rng)).collect();
        assert!(items.iter().any(|i| i.kind == ItemKind::Food),
            "tier 0 random_item should sometimes produce food");
    }

    #[test]
    fn dungeon_random_item_includes_rations() {
        let mut rng = 42u64;
        let items: Vec<_> = (0..200).map(|_| random_item(1, &mut rng)).collect();
        assert!(items.iter().any(|i| i.name == "Dried Rations"),
            "dungeon tier should produce rations");
    }

    // --- Turn counter ---

    #[test]
    fn turn_counter_increments_on_move() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        assert_eq!(g.turn, 0);
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in dirs {
            let (nx, ny) = (g.player_x + dx, g.player_y + dy);
            if g.current_map().is_walkable(nx, ny) {
                g.move_player(dx, dy);
                assert_eq!(g.turn, 1);
                return;
            }
        }
    }

    // --- Item info for food ---

    #[test]
    fn food_item_info_desc() {
        let food = raw_food(15);
        let desc = item_info_desc(&food);
        assert!(desc.contains("Restores 15 hunger"), "food desc: {desc}");
    }

    // --- Rings ---

    #[test]
    fn equip_ring() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(Item {
            kind: ItemKind::Ring, name: "Gold Ring", glyph: '=', effect: ItemEffect::BuffAttack(4), weight: 0, durability: 300, legendary: false,
        });
        assert!(g.equip_item(0));
        assert!(g.inventory.is_empty());
        assert!(g.equipped_ring.is_some());
        assert_eq!(g.equipped_ring.as_ref().unwrap().name, "Gold Ring");
    }

    #[test]
    fn ring_boosts_attack() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let base_atk = g.effective_attack();
        g.equipped_ring = Some(Item {
            kind: ItemKind::Ring, name: "Gold Ring", glyph: '=', effect: ItemEffect::BuffAttack(4), weight: 0, durability: 300, legendary: false,
        });
        assert_eq!(g.effective_attack(), base_atk + 4);
    }

    #[test]
    fn ring_boosts_defense() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let base_def = g.effective_defense();
        g.equipped_ring = Some(Item {
            kind: ItemKind::Ring, name: "Diamond Ring", glyph: '=', effect: ItemEffect::BuffDefense(4), weight: 0, durability: 300, legendary: false,
        });
        assert_eq!(g.effective_defense(), base_def + 4);
    }

    #[test]
    fn ring_swaps_to_inventory() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_ring = Some(Item {
            kind: ItemKind::Ring, name: "Copper Ring", glyph: '=', effect: ItemEffect::BuffAttack(1), weight: 0, durability: 300, legendary: false,
        });
        g.inventory.push(Item {
            kind: ItemKind::Ring, name: "Gold Ring", glyph: '=', effect: ItemEffect::BuffAttack(4), weight: 0, durability: 300, legendary: false,
        });
        g.equip_item(0);
        assert_eq!(g.equipped_ring.as_ref().unwrap().name, "Gold Ring");
        assert_eq!(g.inventory[0].name, "Copper Ring");
    }

    // --- Helmet, Shield, Boots ---

    #[test]
    fn equip_helmet() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(Item {
            kind: ItemKind::Helmet, name: "Leather Cap", glyph: '^', effect: ItemEffect::BuffDefense(1), weight: 0, durability: 250, legendary: false,
        });
        assert!(g.equip_item(0));
        assert!(g.inventory.is_empty());
        assert_eq!(g.equipped_helmet.as_ref().unwrap().name, "Leather Cap");
        assert_eq!(g.effective_defense(), 1);
    }

    #[test]
    fn equip_shield() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(Item {
            kind: ItemKind::Shield, name: "Wooden Shield", glyph: ')', effect: ItemEffect::BuffDefense(1), weight: 0, durability: 250, legendary: false,
        });
        assert!(g.equip_item(0));
        assert!(g.inventory.is_empty());
        assert_eq!(g.equipped_shield.as_ref().unwrap().name, "Wooden Shield");
        assert_eq!(g.effective_defense(), 1);
    }

    #[test]
    fn equip_boots() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(Item {
            kind: ItemKind::Boots, name: "Leather Boots", glyph: '{', effect: ItemEffect::BuffDefense(1), weight: 0, durability: 250, legendary: false,
        });
        assert!(g.equip_item(0));
        assert!(g.inventory.is_empty());
        assert_eq!(g.equipped_boots.as_ref().unwrap().name, "Leather Boots");
        assert_eq!(g.effective_defense(), 1);
    }

    #[test]
    fn helmet_swaps_to_inventory() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_helmet = Some(Item {
            kind: ItemKind::Helmet, name: "Leather Cap", glyph: '^', effect: ItemEffect::BuffDefense(1), weight: 0, durability: 250, legendary: false,
        });
        g.inventory.push(Item {
            kind: ItemKind::Helmet, name: "Iron Helmet", glyph: '^', effect: ItemEffect::BuffDefense(3), weight: 0, durability: 400, legendary: false,
        });
        g.equip_item(0);
        assert_eq!(g.equipped_helmet.as_ref().unwrap().name, "Iron Helmet");
        assert_eq!(g.inventory[0].name, "Leather Cap");
    }

    #[test]
    fn shield_swaps_to_inventory() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_shield = Some(Item {
            kind: ItemKind::Shield, name: "Wooden Shield", glyph: ')', effect: ItemEffect::BuffDefense(1), weight: 0, durability: 250, legendary: false,
        });
        g.inventory.push(Item {
            kind: ItemKind::Shield, name: "Iron Shield", glyph: ')', effect: ItemEffect::BuffDefense(3), weight: 0, durability: 400, legendary: false,
        });
        g.equip_item(0);
        assert_eq!(g.equipped_shield.as_ref().unwrap().name, "Iron Shield");
        assert_eq!(g.inventory[0].name, "Wooden Shield");
    }

    #[test]
    fn boots_swaps_to_inventory() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_boots = Some(Item {
            kind: ItemKind::Boots, name: "Leather Boots", glyph: '{', effect: ItemEffect::BuffDefense(1), weight: 0, durability: 250, legendary: false,
        });
        g.inventory.push(Item {
            kind: ItemKind::Boots, name: "Plate Boots", glyph: '{', effect: ItemEffect::BuffDefense(4), weight: 0, durability: 600, legendary: false,
        });
        g.equip_item(0);
        assert_eq!(g.equipped_boots.as_ref().unwrap().name, "Plate Boots");
        assert_eq!(g.inventory[0].name, "Leather Boots");
    }

    #[test]
    fn full_defense_stacks_all_slots() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_armor = Some(Item {
            kind: ItemKind::Armor, name: "Chain Mail", glyph: '[', effect: ItemEffect::BuffDefense(4), weight: 0, durability: 400, legendary: false,
        });
        g.equipped_helmet = Some(Item {
            kind: ItemKind::Helmet, name: "Iron Helmet", glyph: '^', effect: ItemEffect::BuffDefense(3), weight: 0, durability: 400, legendary: false,
        });
        g.equipped_shield = Some(Item {
            kind: ItemKind::Shield, name: "Iron Shield", glyph: ')', effect: ItemEffect::BuffDefense(3), weight: 0, durability: 400, legendary: false,
        });
        g.equipped_boots = Some(Item {
            kind: ItemKind::Boots, name: "Chain Boots", glyph: '{', effect: ItemEffect::BuffDefense(2), weight: 0, durability: 400, legendary: false,
        });
        g.equipped_ring = Some(Item {
            kind: ItemKind::Ring, name: "Diamond Ring", glyph: '=', effect: ItemEffect::BuffDefense(4), weight: 0, durability: 300, legendary: false,
        });
        // base 0 + armor 4 + helmet 3 + shield 3 + boots 2 + ring 4 = 16
        assert_eq!(g.effective_defense(), 16);
    }

    // --- Item variety ---

    #[test]
    fn random_item_produces_variety() {
        let mut rng = 42u64;
        let items: Vec<_> = (0..500).map(|_| random_item(1, &mut rng)).collect();
        // Should produce all equippable kinds plus consumables
        assert!(items.iter().any(|i| i.kind == ItemKind::Weapon), "should have weapons");
        assert!(items.iter().any(|i| i.kind == ItemKind::Armor), "should have armor");
        assert!(items.iter().any(|i| i.kind == ItemKind::Helmet), "should have helmets");
        assert!(items.iter().any(|i| i.kind == ItemKind::Shield), "should have shields");
        assert!(items.iter().any(|i| i.kind == ItemKind::Boots), "should have boots");
        assert!(items.iter().any(|i| i.kind == ItemKind::Ring), "should have rings");
        assert!(items.iter().any(|i| i.kind == ItemKind::Food), "should have food");
        assert!(items.iter().any(|i| i.kind == ItemKind::Potion), "should have potions");
        assert!(items.iter().any(|i| i.kind == ItemKind::Scroll), "should have scrolls");
    }

    #[test]
    fn weapon_names_vary() {
        let mut rng = 42u64;
        let weapons: Vec<_> = (0..500)
            .map(|_| random_item(0, &mut rng))
            .filter(|i| i.kind == ItemKind::Weapon)
            .collect();
        let names: std::collections::HashSet<&str> = weapons.iter().map(|i| i.name).collect();
        assert!(names.len() >= 2, "should have at least 2 weapon variants, got: {:?}", names);
    }
