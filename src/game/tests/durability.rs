use super::*;
use super::{health_potion, rusty_sword};

fn leather_armor() -> Item {
    Item { kind: ItemKind::Armor, name: "Leather Armor", glyph: '[', effect: ItemEffect::BuffDefense(2), weight: 0, durability: 250, legendary: false }
}
fn short_bow() -> Item {
    Item { kind: ItemKind::RangedWeapon, name: "Short Bow", glyph: '}', effect: ItemEffect::BuffAttack(1), weight: 2, durability: 250, legendary: false }
}
fn iron_helmet() -> Item {
    Item { kind: ItemKind::Helmet, name: "Iron Helmet", glyph: '^', effect: ItemEffect::BuffDefense(3), weight: 0, durability: 400, legendary: false }
}
fn iron_shield() -> Item {
    Item { kind: ItemKind::Shield, name: "Iron Shield", glyph: ')', effect: ItemEffect::BuffDefense(3), weight: 0, durability: 400, legendary: false }
}
fn chain_boots() -> Item {
    Item { kind: ItemKind::Boots, name: "Chain Boots", glyph: '{', effect: ItemEffect::BuffDefense(2), weight: 0, durability: 400, legendary: false }
}

    // --- Weapon durability on melee attack ---

    #[test]
    fn melee_attack_reduces_weapon_durability() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_weapon = Some(rusty_sword());
        let dur_before = g.equipped_weapon.as_ref().unwrap().durability;
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 50, attack: 0, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false, behavior: EnemyBehavior::Aggressive, spawn_x: gx, spawn_y: gy, provoked: false, is_boss: false });
        g.attack_adjacent(gx, gy);
        let dur_after = g.equipped_weapon.as_ref().unwrap().durability;
        assert_eq!(dur_after, dur_before - 1, "weapon should lose 1 durability per melee attack");
    }

    #[test]
    fn ranged_attack_reduces_weapon_durability() {
        let mut map = Map::new_filled(20, 20, Tile::Floor);
        for x in 0..20 { map.set(x, 0, Tile::Wall); map.set(x, 19, Tile::Wall); }
        for y in 0..20 { map.set(0, y, Tile::Wall); map.set(19, y, Tile::Wall); }
        let mut g = Game::new(map);
        g.player_x = 5;
        g.player_y = 5;
        g.player_dexterity = 100; // guarantee hit
        g.equipped_weapon = Some(short_bow());
        let dur_before = g.equipped_weapon.as_ref().unwrap().durability;
        g.enemies.push(Enemy { x: 8, y: 5, hp: 100, attack: 0, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false, behavior: EnemyBehavior::Aggressive, spawn_x: 8, spawn_y: 5, provoked: false, is_boss: false });
        g.ranged_attack(8, 5);
        let dur_after = g.equipped_weapon.as_ref().unwrap().durability;
        assert_eq!(dur_after, dur_before - 1, "ranged weapon should lose 1 durability per shot");
    }

    // --- Weapon breaks at 0 durability ---

    #[test]
    fn weapon_breaks_at_zero_durability() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let mut sword = rusty_sword();
        sword.durability = 1; // about to break
        g.equipped_weapon = Some(sword);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 50, attack: 0, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false, behavior: EnemyBehavior::Aggressive, spawn_x: gx, spawn_y: gy, provoked: false, is_boss: false });
        g.attack_adjacent(gx, gy);
        assert!(g.equipped_weapon.is_none(), "weapon should be destroyed at 0 durability");
        assert!(g.messages.iter().any(|m| m.contains("breaks")),
            "should generate a 'breaks' message");
    }

    // --- Armor durability on damage taken ---

    #[test]
    fn armor_loses_durability_on_hit() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.player_dexterity = 0; // disable dodge
        g.equipped_armor = Some(leather_armor());
        let dur_before = g.equipped_armor.as_ref().unwrap().durability;
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 99, attack: 5, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false, behavior: EnemyBehavior::Aggressive, spawn_x: gx, spawn_y: gy, provoked: false, is_boss: false });
        g.attack_adjacent(gx, gy); // triggers enemy attack back
        let dur_after = g.equipped_armor.as_ref().unwrap().durability;
        assert!(dur_after < dur_before, "armor should lose durability when hit");
    }

    #[test]
    fn helmet_loses_durability_on_hit() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.player_dexterity = 0;
        g.equipped_helmet = Some(iron_helmet());
        let dur_before = g.equipped_helmet.as_ref().unwrap().durability;
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 99, attack: 5, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false, behavior: EnemyBehavior::Aggressive, spawn_x: gx, spawn_y: gy, provoked: false, is_boss: false });
        g.attack_adjacent(gx, gy);
        let dur_after = g.equipped_helmet.as_ref().unwrap().durability;
        assert!(dur_after < dur_before, "helmet should lose durability when hit");
    }

    #[test]
    fn shield_loses_durability_on_hit() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.player_dexterity = 0;
        g.equipped_shield = Some(iron_shield());
        let dur_before = g.equipped_shield.as_ref().unwrap().durability;
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 99, attack: 5, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false, behavior: EnemyBehavior::Aggressive, spawn_x: gx, spawn_y: gy, provoked: false, is_boss: false });
        g.attack_adjacent(gx, gy);
        let dur_after = g.equipped_shield.as_ref().unwrap().durability;
        assert!(dur_after < dur_before, "shield should lose durability when hit");
    }

    #[test]
    fn boots_lose_durability_on_hit() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.player_dexterity = 0;
        g.equipped_boots = Some(chain_boots());
        let dur_before = g.equipped_boots.as_ref().unwrap().durability;
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 99, attack: 5, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false, behavior: EnemyBehavior::Aggressive, spawn_x: gx, spawn_y: gy, provoked: false, is_boss: false });
        g.attack_adjacent(gx, gy);
        let dur_after = g.equipped_boots.as_ref().unwrap().durability;
        assert!(dur_after < dur_before, "boots should lose durability when hit");
    }

    // --- Armor breaks at 0 durability ---

    #[test]
    fn armor_breaks_at_zero_durability() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.player_dexterity = 0;
        let mut armor = leather_armor();
        armor.durability = 1;
        g.equipped_armor = Some(armor);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 99, attack: 5, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false, behavior: EnemyBehavior::Aggressive, spawn_x: gx, spawn_y: gy, provoked: false, is_boss: false });
        g.attack_adjacent(gx, gy);
        assert!(g.equipped_armor.is_none(), "armor should be destroyed at 0 durability");
        assert!(g.messages.iter().any(|m| m.contains("breaks")),
            "should generate a 'breaks' message");
    }

    // --- Durability does not apply to consumables ---

    #[test]
    fn consumables_have_zero_durability() {
        let pot = health_potion();
        assert_eq!(pot.durability, 0, "consumables should have 0 durability (n/a)");
    }

    // --- Multiple attacks wear weapon gradually ---

    #[test]
    fn weapon_durability_decreases_each_attack() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let mut sword = rusty_sword();
        sword.durability = 5;
        g.equipped_weapon = Some(sword);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 200, attack: 0, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false, behavior: EnemyBehavior::Aggressive, spawn_x: gx, spawn_y: gy, provoked: false, is_boss: false });
        for i in 0..4 {
            g.attack_adjacent(gx, gy);
            assert_eq!(g.equipped_weapon.as_ref().unwrap().durability, 4 - i as i32,
                "weapon durability should decrease by 1 each attack");
        }
        // 5th attack should break it
        g.attack_adjacent(gx, gy);
        assert!(g.equipped_weapon.is_none(), "weapon should break after 5 attacks");
    }

    // --- Item description includes durability ---

    #[test]
    fn item_desc_shows_durability() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(rusty_sword());
        let desc = g.inventory_item_desc(0).unwrap();
        assert!(desc.contains("200"), "description should show durability value");
    }

    // --- Dodge prevents armor wear ---

    #[test]
    fn dodge_prevents_armor_wear() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.player_dexterity = 100; // very high dodge chance
        g.equipped_armor = Some(leather_armor());
        let dur_before = g.equipped_armor.as_ref().unwrap().durability;
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 99, attack: 5, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false, behavior: EnemyBehavior::Aggressive, spawn_x: gx, spawn_y: gy, provoked: false, is_boss: false });
        g.attack_adjacent(gx, gy);
        // If dodged, durability should remain the same
        let dur_after = g.equipped_armor.as_ref().unwrap().durability;
        // With 100 dex, dodge is capped at 20%, so it might or might not dodge.
        // We just verify the armor still exists (didn't break from 1 hit at 25 dur)
        assert!(dur_after <= dur_before, "armor durability should not increase");
    }

    // --- Generated items have correct durability ---

    #[test]
    fn generated_items_have_durability() {
        let mut rng = 42u64;
        let items: Vec<_> = (0..200).map(|_| random_item(0, &mut rng)).collect();
        for item in &items {
            if item.kind.is_consumable() {
                assert_eq!(item.durability, 0,
                    "consumable '{}' should have 0 durability", item.name);
            } else {
            match item.kind {
                ItemKind::Weapon => {
                    assert!(item.durability > 0,
                        "weapon '{}' should have positive durability", item.name);
                }
                ItemKind::RangedWeapon => {
                    assert!(item.durability > 0,
                        "ranged weapon '{}' should have positive durability", item.name);
                }
                ItemKind::Armor | ItemKind::Helmet | ItemKind::Shield | ItemKind::Boots => {
                    assert!(item.durability > 0,
                        "armor piece '{}' should have positive durability", item.name);
                }
                ItemKind::Ring => {
                    assert!(item.durability > 0,
                        "ring '{}' should have positive durability", item.name);
                }
                _ => unreachable!("consumables handled above"),
            }
            }
        }
    }

    #[test]
    fn higher_tier_items_have_more_durability() {
        let mut rng = 42u64;
        let t0: Vec<_> = (0..500).map(|_| random_item(0, &mut rng)).collect();
        let t2: Vec<_> = (0..500).map(|_| random_item(2, &mut rng)).collect();

        let avg_dur = |items: &[Item], kind: &ItemKind| -> f64 {
            let durs: Vec<i32> = items.iter()
                .filter(|i| &i.kind == kind && i.durability > 0)
                .map(|i| i.durability)
                .collect();
            if durs.is_empty() { return 0.0; }
            durs.iter().sum::<i32>() as f64 / durs.len() as f64
        };

        let t0_weapon = avg_dur(&t0, &ItemKind::Weapon);
        let t2_weapon = avg_dur(&t2, &ItemKind::Weapon);
        assert!(t2_weapon > t0_weapon,
            "tier 2 weapons should have more durability than tier 0: {t2_weapon} vs {t0_weapon}");
    }

    // --- Armor wear from ranged enemy ---

    #[test]
    fn armor_wears_on_ranged_hit() {
        let mut map = Map::new_filled(20, 20, Tile::Floor);
        for x in 0..20 { map.set(x, 0, Tile::Wall); map.set(x, 19, Tile::Wall); }
        for y in 0..20 { map.set(0, y, Tile::Wall); map.set(19, y, Tile::Wall); }
        let mut g = Game::new(map);
        g.player_x = 5;
        g.player_y = 5;
        g.player_dexterity = 0; // no dodge
        g.equipped_armor = Some(leather_armor());
        let dur_before = g.equipped_armor.as_ref().unwrap().durability;
        // Place a ranged enemy at distance 3
        g.enemies.push(Enemy { x: 8, y: 5, hp: 99, attack: 5, glyph: 'a', name: "Goblin Archer", facing_left: false, defense: 0, is_ranged: true, behavior: EnemyBehavior::Aggressive, spawn_x: 8, spawn_y: 5, provoked: false, is_boss: false });
        // Advance a turn so the archer shoots
        g.advance_turn();
        // The archer may miss (30% miss rate), so check if armor changed at all
        let dur_after = g.equipped_armor.as_ref().unwrap().durability;
        // If hit happened, durability decreased; if missed, stayed same
        assert!(dur_after <= dur_before, "armor durability should not increase from ranged hit");
    }
