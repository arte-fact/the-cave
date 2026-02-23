use super::*;
use super::{test_game, health_potion, rusty_sword};

fn stamina_potion() -> Item {
    Item { kind: ItemKind::Potion, name: "Stamina Potion", glyph: '!', effect: ItemEffect::RestoreStamina(40), weight: 0, durability: 0 }
}
fn scroll_fire() -> Item {
    Item { kind: ItemKind::Scroll, name: "Scroll of Fire", glyph: '?', effect: ItemEffect::DamageAoe(8), weight: 0, durability: 0 }
}
fn iron_sword() -> Item {
    Item { kind: ItemKind::Weapon, name: "Iron Sword", glyph: '/', effect: ItemEffect::BuffAttack(5), weight: 2, durability: 350 }
}
fn leather_armor() -> Item {
    Item { kind: ItemKind::Armor, name: "Leather Armor", glyph: '[', effect: ItemEffect::BuffDefense(2), weight: 0, durability: 250 }
}
fn chain_mail() -> Item {
    Item { kind: ItemKind::Armor, name: "Chain Mail", glyph: '[', effect: ItemEffect::BuffDefense(4), weight: 0, durability: 400 }
}

    // --- Pickup ---

    #[test]
    fn pickup_item_on_move() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        // Place an item one tile to the right of the player, move there, then pick up
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in dirs {
            let (nx, ny) = (g.player_x + dx, g.player_y + dy);
            if g.current_map().is_walkable(nx, ny) {
                g.ground_items.push(GroundItem { x: nx, y: ny, item: health_potion() });
                g.move_player(dx, dy);
                g.pickup_items_explicit();
                assert_eq!(g.inventory.len(), 1);
                assert_eq!(g.inventory[0].name, "Health Potion");
                assert!(g.ground_items.is_empty());
                return;
            }
        }
        panic!("no adjacent walkable tile");
    }

    #[test]
    fn pickup_generates_message() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in dirs {
            let (nx, ny) = (g.player_x + dx, g.player_y + dy);
            if g.current_map().is_walkable(nx, ny) {
                g.ground_items.push(GroundItem { x: nx, y: ny, item: rusty_sword() });
                let msg_before = g.messages.len();
                g.move_player(dx, dy);
                g.pickup_items_explicit();
                assert!(g.messages.len() > msg_before);
                assert!(g.messages.last().unwrap().contains("Picked up"));
                return;
            }
        }
    }

    #[test]
    fn inventory_full_stops_pickup() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let max_inv = g.config.player.max_inventory;
        for _ in 0..max_inv {
            g.inventory.push(health_potion());
        }
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in dirs {
            let (nx, ny) = (g.player_x + dx, g.player_y + dy);
            if g.current_map().is_walkable(nx, ny) {
                g.ground_items.push(GroundItem { x: nx, y: ny, item: rusty_sword() });
                g.move_player(dx, dy);
                assert_eq!(g.inventory.len(), max_inv);
                assert_eq!(g.ground_items.len(), 1, "item should stay on ground");
                return;
            }
        }
    }

    // --- Use ---

    #[test]
    fn use_potion_heals() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.player_hp = 10;
        g.inventory.push(health_potion());
        assert!(g.use_item(0));
        assert_eq!(g.player_hp, 15);
        assert!(g.inventory.is_empty());
    }

    #[test]
    fn use_potion_caps_at_max_hp() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.player_hp = 18;
        g.inventory.push(health_potion());
        g.use_item(0);
        assert_eq!(g.player_hp, 20); // max_hp is 20
    }

    #[test]
    fn use_stamina_potion_restores_stamina() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.stamina = 30;
        g.max_stamina = 100;
        g.inventory.push(stamina_potion());
        assert!(g.use_item(0));
        assert_eq!(g.stamina, 70); // 30 + 40
        assert!(g.inventory.is_empty());
    }

    #[test]
    fn use_stamina_potion_caps_at_max() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.stamina = 80;
        g.max_stamina = 100;
        g.inventory.push(stamina_potion());
        g.use_item(0);
        assert_eq!(g.stamina, 100); // capped at max_stamina
    }

    #[test]
    fn use_scroll_damages_nearby_enemies() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let (px, py) = (g.player_x, g.player_y);
        // Place enemies: one close (dist 2), one far (dist 10)
        g.enemies.push(Enemy { x: px + 2, y: py, hp: 20, attack: 1, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        g.enemies.push(Enemy { x: px + 10, y: py, hp: 20, attack: 1, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        g.inventory.push(scroll_fire());
        g.use_item(0);
        assert_eq!(g.enemies[0].hp, 20 - 8, "close enemy should take 8 damage");
        assert_eq!(g.enemies[1].hp, 20, "far enemy should be unaffected");
        assert!(g.inventory.is_empty());
    }

    #[test]
    fn use_weapon_returns_false() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(rusty_sword());
        assert!(!g.use_item(0), "weapon should not be usable");
        assert_eq!(g.inventory.len(), 1, "weapon should remain in inventory");
    }

    #[test]
    fn use_invalid_index_returns_false() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        assert!(!g.use_item(0));
        assert!(!g.use_item(99));
    }

    // --- Equip ---

    #[test]
    fn equip_weapon() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(rusty_sword());
        assert!(g.equip_item(0));
        assert!(g.inventory.is_empty());
        assert_eq!(g.equipped_weapon.as_ref().unwrap().name, "Rusty Sword");
        assert_eq!(g.effective_attack(), 5 + 3); // Rusty Sword: +3 ATK
    }

    #[test]
    fn equip_armor() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(leather_armor());
        assert!(g.equip_item(0));
        assert!(g.inventory.is_empty());
        assert_eq!(g.equipped_armor.as_ref().unwrap().name, "Leather Armor");
        assert_eq!(g.effective_defense(), 2);
    }

    #[test]
    fn equip_weapon_swaps_old_to_inventory() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(rusty_sword());
        g.equip_item(0);
        g.inventory.push(iron_sword());
        g.equip_item(0);
        assert_eq!(g.equipped_weapon.as_ref().unwrap().name, "Iron Sword");
        assert_eq!(g.inventory.len(), 1);
        assert_eq!(g.inventory[0].name, "Rusty Sword");
        assert_eq!(g.effective_attack(), 5 + 5); // Iron Sword: +5 ATK
    }

    #[test]
    fn equip_armor_swaps_old_to_inventory() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(leather_armor());
        g.equip_item(0);
        g.inventory.push(chain_mail());
        g.equip_item(0);
        assert_eq!(g.equipped_armor.as_ref().unwrap().name, "Chain Mail");
        assert_eq!(g.inventory.len(), 1);
        assert_eq!(g.inventory[0].name, "Leather Armor");
        assert_eq!(g.effective_defense(), 4);
    }

    #[test]
    fn equip_potion_returns_false() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(health_potion());
        assert!(!g.equip_item(0));
        assert_eq!(g.inventory.len(), 1);
    }

    // --- Drop ---

    #[test]
    fn drop_item_places_on_ground() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(health_potion());
        assert!(g.drop_item(0));
        assert!(g.inventory.is_empty());
        assert_eq!(g.ground_items.len(), 1);
        assert_eq!(g.ground_items[0].x, g.player_x);
        assert_eq!(g.ground_items[0].y, g.player_y);
        assert_eq!(g.ground_items[0].item.name, "Health Potion");
    }

    #[test]
    fn drop_invalid_index_returns_false() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        assert!(!g.drop_item(0));
    }

    // --- Combat with equipment ---

    #[test]
    fn weapon_increases_damage() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_weapon = Some(rusty_sword());
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 20, attack: 1, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        g.attack_adjacent(gx, gy);
        // Base attack 5 + weapon 3 = 8 damage (calc_damage(8, 0) = 8)
        assert_eq!(g.enemies[0].hp, 20 - 8);
    }

    #[test]
    fn armor_reduces_damage_taken() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.player_dexterity = 0; // disable dodge for deterministic test
        g.equipped_armor = Some(leather_armor());
        let hp_before = g.player_hp;
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 99, attack: 5, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        g.attack_adjacent(gx, gy);
        // Enemy attacks during enemy_turn: calc_damage(5, 2) = 25/7 = 3
        assert_eq!(g.player_hp, hp_before - 3);
    }

    #[test]
    fn defense_minimum_damage_is_one() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.player_dexterity = 0; // disable dodge for deterministic test
        // Defense higher than enemy attack
        g.equipped_armor = Some(Item {
            kind: ItemKind::Armor, name: "Dragon Scale", glyph: '[',
            effect: ItemEffect::BuffDefense(6), weight: 0, durability: 600,
        });
        let hp_before = g.player_hp;
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 99, attack: 2, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        g.attack_adjacent(gx, gy);
        // calc_damage(2, 6) = 4/8 = 0 → max(1) = 1
        assert_eq!(g.player_hp, hp_before - 1);
    }

    #[test]
    fn effective_attack_without_weapon() {
        let g = test_game();
        assert_eq!(g.effective_attack(), 5);
    }

    #[test]
    fn effective_defense_without_armor() {
        let g = test_game();
        assert_eq!(g.effective_defense(), 0);
    }

    // --- Item spawning ---

    #[test]
    fn dungeon_has_ground_items() {
        let mut g = overworld_game();
        g.enter_dungeon(0);
        assert!(!g.ground_items.is_empty(), "dungeon level 0 should have items");
    }

    #[test]
    fn dungeon_items_on_floor_tiles() {
        let mut g = overworld_game();
        g.enter_dungeon(0);
        let map = g.current_map();
        for gi in &g.ground_items {
            assert_eq!(map.get(gi.x, gi.y), Tile::Floor,
                "item '{}' at ({},{}) not on Floor", gi.item.name, gi.x, gi.y);
        }
    }

    #[test]
    fn dungeons_have_loot_on_each_level() {
        let mut g = overworld_game();
        g.enter_dungeon(0);
        assert!(!g.ground_items.is_empty(), "dungeon level 0 should have items");
        g.descend(0, 0);
        assert!(!g.ground_items.is_empty(), "dungeon level 1 should have items");
        g.descend(0, 1);
        assert!(!g.ground_items.is_empty(), "dungeon level 2 should have items");
    }

    #[test]
    fn overworld_items_sparse() {
        let mut g = overworld_game();
        g.spawn_overworld_items(42);
        // Overworld should have very few items
        let map = g.current_map();
        let total_walkable = (0..map.height)
            .flat_map(|y| (0..map.width).map(move |x| (x, y)))
            .filter(|&(x, y)| map.is_walkable(x, y))
            .count();
        assert!(g.ground_items.len() < total_walkable / 50,
            "overworld items should be sparse: {} items for {} walkable tiles",
            g.ground_items.len(), total_walkable);
    }

    #[test]
    fn random_item_tiers_correct() {
        // Tier 0 produces basic items
        let mut rng = 42u64;
        let items: Vec<_> = (0..50).map(|_| random_item(0, &mut rng)).collect();
        assert!(items.iter().any(|i| i.name == "Health Potion" || i.name == "Rusty Sword"));
        // Tier 2 produces advanced items
        rng = 42;
        let items: Vec<_> = (0..50).map(|_| random_item(2, &mut rng)).collect();
        assert!(items.iter().any(|i| i.name == "Superior Health Potion" || i.name == "Enchanted Blade" || i.name == "Dragon Scale"));
    }

    // --- Ground items persist across dungeon transitions ---

    #[test]
    fn overworld_items_saved_on_enter_dungeon() {
        let mut g = overworld_game();
        g.ground_items.push(GroundItem { x: 10, y: 10, item: health_potion() });
        let ow_item_count = g.ground_items.len();
        g.enter_dungeon(0);
        // Dungeon should have its own items, overworld items saved
        assert_ne!(g.ground_items.len(), ow_item_count);
        g.exit_dungeon();
        // Overworld items restored
        assert!(g.ground_items.iter().any(|gi| gi.item.name == "Health Potion"));
    }

    #[test]
    fn inventory_persists_across_transitions() {
        let mut g = overworld_game();
        g.inventory.push(rusty_sword());
        g.enter_dungeon(0);
        assert_eq!(g.inventory.len(), 1);
        assert_eq!(g.inventory[0].name, "Rusty Sword");
        g.exit_dungeon();
        assert_eq!(g.inventory.len(), 1);
    }

    #[test]
    fn scroll_inventory_down_and_up() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        for _ in 0..10 {
            g.inventory.push(health_potion());
        }
        assert_eq!(g.ui.inventory_scroll, 0);
        g.scroll_inventory(3);
        assert_eq!(g.ui.inventory_scroll, 3);
        g.scroll_inventory(-1);
        assert_eq!(g.ui.inventory_scroll, 2);
    }

    #[test]
    fn scroll_inventory_clamps_at_zero() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        for _ in 0..5 {
            g.inventory.push(health_potion());
        }
        g.scroll_inventory(-10);
        assert_eq!(g.ui.inventory_scroll, 0);
    }

    #[test]
    fn scroll_inventory_clamps_at_max() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        for _ in 0..5 {
            g.inventory.push(health_potion());
        }
        g.scroll_inventory(100);
        assert_eq!(g.ui.inventory_scroll, 4); // last valid index = len - 1
    }

    #[test]
    fn scroll_clamps_after_item_removal() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        for _ in 0..5 {
            g.inventory.push(health_potion());
        }
        g.ui.inventory_scroll = 4; // pointing at last item
        g.player_hp = 10; // damage so potion heals
        g.use_item(4); // removes last item
        assert_eq!(g.ui.inventory_scroll, 3); // clamped to new last index
    }

    #[test]
    fn scroll_resets_when_inventory_empty() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(health_potion());
        g.ui.inventory_scroll = 0;
        g.player_hp = 10;
        g.use_item(0);
        assert_eq!(g.ui.inventory_scroll, 0);
        assert!(g.inventory.is_empty());
    }

    #[test]
    fn drop_item_clamps_scroll() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        for _ in 0..3 {
            g.inventory.push(rusty_sword());
        }
        g.ui.inventory_scroll = 2;
        g.drop_item(2);
        assert_eq!(g.ui.inventory_scroll, 1);
    }

    #[test]
    fn equip_item_clamps_scroll() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        for _ in 0..3 {
            g.inventory.push(rusty_sword());
        }
        g.ui.inventory_scroll = 2;
        g.equip_item(2); // removes item, pushes nothing back (slot empty)
        assert_eq!(g.ui.inventory_scroll, 1);
    }

    #[test]
    fn scroll_inventory_page_jump() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        for _ in 0..10 {
            g.inventory.push(health_potion());
        }
        // Page-down by 5
        g.scroll_inventory(5);
        assert_eq!(g.ui.inventory_scroll, 5);
        // Page-up by 3
        g.scroll_inventory(-3);
        assert_eq!(g.ui.inventory_scroll, 2);
        // Large page-down clamps to max
        g.scroll_inventory(100);
        assert_eq!(g.ui.inventory_scroll, 9);
    }

    // ── Item selection tests ─────────────────────────────────────────

    #[test]
    fn selected_item_starts_none() {
        let map = Map::generate(30, 20, 42);
        let g = Game::new(map);
        assert!(g.ui.selected_inventory_item.is_none());
    }

    #[test]
    fn selection_cleared_on_drawer_toggle() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(rusty_sword());
        g.ui.selected_inventory_item = Some(0);
        // Opening inventory clears selection
        g.toggle_drawer(Drawer::Inventory);
        assert!(g.ui.selected_inventory_item.is_none());
        // Re-select and close drawer
        g.ui.selected_inventory_item = Some(0);
        g.toggle_drawer(Drawer::Inventory); // toggles off
        assert!(g.ui.selected_inventory_item.is_none());
    }

    #[test]
    fn selection_cleared_when_item_dropped() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(rusty_sword());
        g.inventory.push(health_potion());
        g.ui.selected_inventory_item = Some(0);
        g.drop_item(0);
        // Selection should be cleared because item was removed
        // (clamp_inventory_scroll clears selection when index >= len)
        assert_eq!(g.inventory.len(), 1);
    }

    #[test]
    fn selection_cleared_when_item_used() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(health_potion());
        g.player_hp = 10;
        g.ui.selected_inventory_item = Some(0);
        g.use_item(0);
        // Item consumed, selection should be cleared (only had 1 item)
        assert!(g.inventory.is_empty());
        assert!(g.ui.selected_inventory_item.is_none());
    }

    #[test]
    fn selection_survives_when_valid_after_removal() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(health_potion());
        g.inventory.push(rusty_sword());
        g.inventory.push(health_potion());
        g.ui.selected_inventory_item = Some(0);
        // Drop item at index 2 — selection at 0 stays valid
        g.drop_item(2);
        assert_eq!(g.ui.selected_inventory_item, Some(0));
    }

    #[test]
    fn selection_cleared_when_index_out_of_bounds() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(health_potion());
        g.ui.selected_inventory_item = Some(5); // out of bounds
        g.clamp_inventory_scroll();
        assert!(g.ui.selected_inventory_item.is_none());
    }

    // ── Item description tests ───────────────────────────────────────

    #[test]
    fn inventory_item_desc_returns_description() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(rusty_sword());
        let desc = g.inventory_item_desc(0).unwrap();
        assert!(desc.contains("Rusty Sword"));
        assert!(desc.contains("Attack"));
    }

    #[test]
    fn inventory_item_desc_returns_none_for_empty() {
        let map = Map::generate(30, 20, 42);
        let g = Game::new(map);
        assert!(g.inventory_item_desc(0).is_none());
    }

    #[test]
    fn inventory_item_desc_potion() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(health_potion());
        let desc = g.inventory_item_desc(0).unwrap();
        assert!(desc.contains("HP"));
    }

    // ── set_inventory_scroll tests ───────────────────────────────────

    #[test]
    fn set_inventory_scroll_clamps() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(health_potion());
        g.inventory.push(rusty_sword());
        g.set_inventory_scroll(100);
        assert_eq!(g.ui.inventory_scroll, 1); // clamped to len-1
    }

    #[test]
    fn set_inventory_scroll_zero_on_empty() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.set_inventory_scroll(5);
        assert_eq!(g.ui.inventory_scroll, 0);
    }

    // ── Starting equipment tests ─────────────────────────────────────

    #[test]
    fn overworld_spawn_has_starting_weapon() {
        let g = overworld_game();
        let weapon = g.equipped_weapon.as_ref().expect("should spawn with weapon");
        assert_eq!(weapon.name, "Iron Dagger");
        assert_eq!(weapon.kind, ItemKind::Weapon);
        if let ItemEffect::BuffAttack(bonus) = weapon.effect {
            assert_eq!(bonus, 2);
        } else {
            panic!("weapon should have BuffAttack effect");
        }
    }

    #[test]
    fn overworld_spawn_has_starting_armor() {
        let g = overworld_game();
        let armor = g.equipped_armor.as_ref().expect("should spawn with armor");
        assert_eq!(armor.name, "Cloth Armor");
        assert_eq!(armor.kind, ItemKind::Armor);
        if let ItemEffect::BuffDefense(bonus) = armor.effect {
            assert_eq!(bonus, 1);
        } else {
            panic!("armor should have BuffDefense effect");
        }
    }

    #[test]
    fn overworld_spawn_has_starting_helmet() {
        let g = overworld_game();
        let helmet = g.equipped_helmet.as_ref().expect("should spawn with helmet");
        assert_eq!(helmet.name, "Cloth Hood");
        assert_eq!(helmet.kind, ItemKind::Helmet);
        if let ItemEffect::BuffDefense(bonus) = helmet.effect {
            assert_eq!(bonus, 1);
        } else {
            panic!("helmet should have BuffDefense effect");
        }
    }

    #[test]
    fn overworld_spawn_has_starting_boots() {
        let g = overworld_game();
        let boots = g.equipped_boots.as_ref().expect("should spawn with boots");
        assert_eq!(boots.name, "Shoes");
        assert_eq!(boots.kind, ItemKind::Boots);
        if let ItemEffect::BuffDefense(bonus) = boots.effect {
            assert_eq!(bonus, 1);
        } else {
            panic!("boots should have BuffDefense effect");
        }
    }

    #[test]
    fn overworld_spawn_effective_stats_include_equipment() {
        let g = overworld_game();
        // Base attack 5 + Iron Dagger +2 = 7
        assert_eq!(g.effective_attack(), 7);
        // Cloth Armor +1, Cloth Hood +1, Shoes +1 = 3
        assert_eq!(g.effective_defense(), 3);
    }

    #[test]
    fn overworld_spawn_inventory_empty() {
        let g = overworld_game();
        assert!(g.inventory.is_empty(), "starting equipment should be equipped, not in inventory");
    }

    #[test]
    fn starting_equipment_has_durability() {
        let g = overworld_game();
        let weapon = g.equipped_weapon.as_ref().unwrap();
        assert!(weapon.durability > 0, "starting weapon should have durability");
        let armor = g.equipped_armor.as_ref().unwrap();
        assert!(armor.durability > 0, "starting armor should have durability");
        let helmet = g.equipped_helmet.as_ref().unwrap();
        assert!(helmet.durability > 0, "starting helmet should have durability");
        let boots = g.equipped_boots.as_ref().unwrap();
        assert!(boots.durability > 0, "starting boots should have durability");
    }
