use super::*;
use super::test_game;

    fn potion() -> Item {
        Item { kind: ItemKind::Potion, name: "Health Potion", glyph: '!', effect: ItemEffect::Heal(10) }
    }
    fn scroll() -> Item {
        Item { kind: ItemKind::Scroll, name: "Scroll of Fire", glyph: '?', effect: ItemEffect::DamageAoe(5) }
    }
    fn food() -> Item {
        Item { kind: ItemKind::Food, name: "Bread", glyph: '%', effect: ItemEffect::Feed(20, FoodSideEffect::None) }
    }
    fn sword() -> Item {
        Item { kind: ItemKind::Weapon, name: "Iron Sword", glyph: '/', effect: ItemEffect::BuffAttack(3) }
    }
    fn armor() -> Item {
        Item { kind: ItemKind::Armor, name: "Chain Mail", glyph: '[', effect: ItemEffect::BuffDefense(2) }
    }

    #[test]
    fn quickbar_new_is_all_empty() {
        let qb = QuickBar::new();
        assert!(qb.slots.iter().all(|s| s.is_none()));
    }

    #[test]
    fn quickbar_assign_potion() {
        let mut qb = QuickBar::new();
        assert!(qb.assign(0, 3, &potion()));
        assert_eq!(qb.slots[0], Some(3));
    }

    #[test]
    fn quickbar_assign_scroll() {
        let mut qb = QuickBar::new();
        assert!(qb.assign(1, 0, &scroll()));
        assert_eq!(qb.slots[1], Some(0));
    }

    #[test]
    fn quickbar_assign_food() {
        let mut qb = QuickBar::new();
        assert!(qb.assign(2, 5, &food()));
        assert_eq!(qb.slots[2], Some(5));
    }

    #[test]
    fn quickbar_rejects_weapon() {
        let mut qb = QuickBar::new();
        assert!(!qb.assign(0, 0, &sword()));
        assert_eq!(qb.slots[0], None);
    }

    #[test]
    fn quickbar_rejects_armor() {
        let mut qb = QuickBar::new();
        assert!(!qb.assign(0, 0, &armor()));
        assert_eq!(qb.slots[0], None);
    }

    #[test]
    fn quickbar_rejects_out_of_bounds_slot() {
        let mut qb = QuickBar::new();
        assert!(!qb.assign(QUICKBAR_SLOTS, 0, &potion()));
        assert!(!qb.assign(99, 0, &potion()));
    }

    #[test]
    fn quickbar_assign_same_item_clears_previous_slot() {
        let mut qb = QuickBar::new();
        qb.assign(0, 2, &potion());
        qb.assign(3, 2, &potion()); // same inv_index=2 in different slot
        assert_eq!(qb.slots[0], None, "old slot should be cleared");
        assert_eq!(qb.slots[3], Some(2));
    }

    #[test]
    fn quickbar_assign_replaces_existing_slot_content() {
        let mut qb = QuickBar::new();
        qb.assign(0, 1, &potion());
        qb.assign(0, 5, &scroll()); // overwrite slot 0
        assert_eq!(qb.slots[0], Some(5));
    }

    #[test]
    fn quickbar_clear() {
        let mut qb = QuickBar::new();
        qb.assign(1, 4, &potion());
        qb.clear(1);
        assert_eq!(qb.slots[1], None);
    }

    #[test]
    fn quickbar_clear_out_of_bounds_no_panic() {
        let mut qb = QuickBar::new();
        qb.clear(99); // should not panic
    }

    #[test]
    fn quickbar_on_item_removed_clears_matching_slot() {
        let mut qb = QuickBar::new();
        qb.assign(0, 2, &potion());
        qb.assign(1, 5, &scroll());
        qb.on_item_removed(2);
        assert_eq!(qb.slots[0], None, "slot pointing to removed index should clear");
        assert_eq!(qb.slots[1], Some(4), "slot pointing to higher index should decrement");
    }

    #[test]
    fn quickbar_on_item_removed_decrements_higher_indices() {
        let mut qb = QuickBar::new();
        qb.assign(0, 0, &potion());
        qb.assign(1, 3, &scroll());
        qb.assign(2, 7, &food());
        qb.on_item_removed(2);
        assert_eq!(qb.slots[0], Some(0), "index below removed stays unchanged");
        assert_eq!(qb.slots[1], Some(2), "index 3 -> 2 after removing index 2");
        assert_eq!(qb.slots[2], Some(6), "index 7 -> 6 after removing index 2");
    }

    #[test]
    fn quickbar_on_item_removed_empty_bar_no_panic() {
        let mut qb = QuickBar::new();
        qb.on_item_removed(0); // should not panic
        qb.on_item_removed(99);
    }

    #[test]
    fn quickbar_on_item_removed_index_zero() {
        let mut qb = QuickBar::new();
        qb.assign(0, 0, &potion());
        qb.assign(1, 1, &scroll());
        qb.assign(2, 2, &food());
        qb.on_item_removed(0);
        assert_eq!(qb.slots[0], None);
        assert_eq!(qb.slots[1], Some(0));
        assert_eq!(qb.slots[2], Some(1));
    }

    #[test]
    fn quickbar_swap() {
        let mut qb = QuickBar::new();
        qb.assign(0, 1, &potion());
        qb.assign(2, 4, &scroll());
        qb.swap(0, 2);
        assert_eq!(qb.slots[0], Some(4));
        assert_eq!(qb.slots[2], Some(1));
    }

    #[test]
    fn quickbar_swap_with_empty() {
        let mut qb = QuickBar::new();
        qb.assign(0, 3, &potion());
        qb.swap(0, 1);
        assert_eq!(qb.slots[0], None);
        assert_eq!(qb.slots[1], Some(3));
    }

    #[test]
    fn quickbar_swap_out_of_bounds_no_panic() {
        let mut qb = QuickBar::new();
        qb.assign(0, 1, &potion());
        qb.swap(0, 99); // should not panic, no change
        assert_eq!(qb.slots[0], Some(1));
    }

    #[test]
    fn quickbar_game_initialized_empty() {
        let g = test_game();
        assert!(g.quick_bar.slots.iter().all(|s| s.is_none()));
    }

    #[test]
    fn quickbar_use_item_clears_slot() {
        let mut g = test_game();
        g.inventory.push(potion());
        g.inventory.push(scroll());
        g.quick_bar.assign(0, 0, &g.inventory[0].clone());
        g.quick_bar.assign(1, 1, &g.inventory[1].clone());
        // Use item at index 0 (potion) — on_item_removed is called internally
        g.player_hp = g.player_max_hp - 5; // ensure heal has effect
        g.use_item(0);
        assert_eq!(g.quick_bar.slots[0], None, "used item slot should clear");
        assert_eq!(g.quick_bar.slots[1], Some(0), "scroll shifted from index 1 to 0");
    }

    #[test]
    fn quickbar_equip_item_clears_slot() {
        let mut g = test_game();
        g.inventory.push(potion());
        g.inventory.push(sword());
        g.quick_bar.assign(0, 0, &g.inventory[0].clone());
        // Equip sword at index 1 — not a consumable, but on_item_removed still adjusts indices
        g.equip_item(1);
        // Potion was at index 0, sword was removed at index 1 — potion index unchanged
        assert_eq!(g.quick_bar.slots[0], Some(0));
    }

    #[test]
    fn quickbar_drop_item_clears_slot() {
        let mut g = test_game();
        g.inventory.push(potion());
        g.inventory.push(scroll());
        g.quick_bar.assign(0, 0, &g.inventory[0].clone());
        g.quick_bar.assign(1, 1, &g.inventory[1].clone());
        // Drop the potion at index 0
        g.drop_item(0);
        assert_eq!(g.quick_bar.slots[0], None, "dropped item slot should clear");
        assert_eq!(g.quick_bar.slots[1], Some(0), "scroll shifted from 1 to 0");
    }

    #[test]
    fn quickbar_eat_food_clears_slot() {
        let mut g = test_game();
        g.inventory.push(food());
        g.quick_bar.assign(2, 0, &g.inventory[0].clone());
        g.eat_food(0);
        assert_eq!(g.quick_bar.slots[2], None, "eaten food slot should clear");
    }

    #[test]
    fn quickbar_use_scroll_clears_slot() {
        let mut g = test_game();
        g.inventory.push(scroll());
        g.quick_bar.assign(3, 0, &g.inventory[0].clone());
        g.use_item(0);
        assert_eq!(g.quick_bar.slots[3], None, "used scroll slot should clear");
    }

    #[test]
    fn quickbar_multiple_removes_sequential() {
        let mut qb = QuickBar::new();
        // Simulate inventory of 5 items, slots assigned to indices 1, 3
        qb.assign(0, 1, &potion());
        qb.assign(1, 3, &scroll());
        // Remove index 1 (the potion)
        qb.on_item_removed(1);
        assert_eq!(qb.slots[0], None);
        assert_eq!(qb.slots[1], Some(2), "3 -> 2 after removing 1");
        // Now remove index 2 (was the scroll, shifted from 3)
        qb.on_item_removed(2);
        assert_eq!(qb.slots[1], None);
    }
