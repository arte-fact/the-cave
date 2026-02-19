use crate::map::Tile;
use super::types::*;
use super::{Game, xorshift64};

/// Generate a random item appropriate for the given dungeon tier.
/// Tier 0 = shallow/overworld, 1 = mid, 2+ = deep.
pub(super) fn random_item(tier: usize, rng: &mut u64) -> Item {
    *rng = xorshift64(*rng);
    let roll = *rng % 100;
    // Sub-roll for variant selection within a category
    *rng = xorshift64(*rng);
    let sub = *rng % 3;
    match tier {
        0 => {
            if roll < 28 {
                Item { kind: ItemKind::Potion, name: "Health Potion", glyph: '!', effect: ItemEffect::Heal(5) }
            } else if roll < 40 {
                Item { kind: ItemKind::Scroll, name: "Scroll of Fire", glyph: '?', effect: ItemEffect::DamageAoe(8) }
            } else if roll < 50 {
                match sub {
                    0 => Item { kind: ItemKind::Weapon, name: "Rusty Sword", glyph: '/', effect: ItemEffect::BuffAttack(2) },
                    1 => Item { kind: ItemKind::Weapon, name: "Iron Dagger", glyph: '/', effect: ItemEffect::BuffAttack(1) },
                    _ => Item { kind: ItemKind::Weapon, name: "Wooden Club", glyph: '/', effect: ItemEffect::BuffAttack(2) },
                }
            } else if roll < 56 {
                match sub {
                    0 => Item { kind: ItemKind::RangedWeapon, name: "Short Bow", glyph: '}', effect: ItemEffect::BuffAttack(2) },
                    _ => Item { kind: ItemKind::RangedWeapon, name: "Crossbow", glyph: '}', effect: ItemEffect::BuffAttack(3) },
                }
            } else if roll < 62 {
                Item { kind: ItemKind::Armor, name: "Leather Armor", glyph: '[', effect: ItemEffect::BuffDefense(2) }
            } else if roll < 67 {
                Item { kind: ItemKind::Helmet, name: "Leather Cap", glyph: '^', effect: ItemEffect::BuffDefense(1) }
            } else if roll < 72 {
                Item { kind: ItemKind::Shield, name: "Wooden Shield", glyph: ')', effect: ItemEffect::BuffDefense(1) }
            } else if roll < 77 {
                Item { kind: ItemKind::Boots, name: "Leather Boots", glyph: '{', effect: ItemEffect::BuffDefense(1) }
            } else if roll < 84 {
                Item { kind: ItemKind::Ring, name: "Copper Ring", glyph: '=', effect: ItemEffect::BuffAttack(1) }
            } else {
                match sub {
                    0 => Item { kind: ItemKind::Food, name: "Stale Bread", glyph: '%', effect: ItemEffect::Feed(15, FoodSideEffect::None) },
                    1 => Item { kind: ItemKind::Food, name: "Waterskin", glyph: '~', effect: ItemEffect::Feed(12, FoodSideEffect::None) },
                    _ => Item { kind: ItemKind::Food, name: "Wild Berries", glyph: '%', effect: ItemEffect::Feed(15, FoodSideEffect::Heal(2)) },
                }
            }
        }
        1 => {
            if roll < 24 {
                Item { kind: ItemKind::Potion, name: "Greater Health Potion", glyph: '!', effect: ItemEffect::Heal(10) }
            } else if roll < 36 {
                Item { kind: ItemKind::Scroll, name: "Scroll of Lightning", glyph: '?', effect: ItemEffect::DamageAoe(12) }
            } else if roll < 46 {
                match sub {
                    0 => Item { kind: ItemKind::Weapon, name: "Iron Sword", glyph: '/', effect: ItemEffect::BuffAttack(4) },
                    1 => Item { kind: ItemKind::Weapon, name: "Battle Axe", glyph: '/', effect: ItemEffect::BuffAttack(5) },
                    _ => Item { kind: ItemKind::Weapon, name: "War Hammer", glyph: '/', effect: ItemEffect::BuffAttack(4) },
                }
            } else if roll < 52 {
                match sub {
                    0 => Item { kind: ItemKind::RangedWeapon, name: "Long Bow", glyph: '}', effect: ItemEffect::BuffAttack(4) },
                    _ => Item { kind: ItemKind::RangedWeapon, name: "Heavy Crossbow", glyph: '}', effect: ItemEffect::BuffAttack(5) },
                }
            } else if roll < 58 {
                Item { kind: ItemKind::Armor, name: "Chain Mail", glyph: '[', effect: ItemEffect::BuffDefense(4) }
            } else if roll < 63 {
                Item { kind: ItemKind::Helmet, name: "Iron Helmet", glyph: '^', effect: ItemEffect::BuffDefense(3) }
            } else if roll < 68 {
                Item { kind: ItemKind::Shield, name: "Iron Shield", glyph: ')', effect: ItemEffect::BuffDefense(3) }
            } else if roll < 73 {
                Item { kind: ItemKind::Boots, name: "Chain Boots", glyph: '{', effect: ItemEffect::BuffDefense(2) }
            } else if roll < 80 {
                match sub {
                    0 => Item { kind: ItemKind::Ring, name: "Silver Ring", glyph: '=', effect: ItemEffect::BuffDefense(2) },
                    _ => Item { kind: ItemKind::Ring, name: "Ruby Ring", glyph: '=', effect: ItemEffect::BuffAttack(3) },
                }
            } else {
                match sub {
                    0 => Item { kind: ItemKind::Food, name: "Dried Rations", glyph: '%', effect: ItemEffect::Feed(20, FoodSideEffect::None) },
                    _ => Item { kind: ItemKind::Food, name: "Dwarven Ale", glyph: '~', effect: ItemEffect::Feed(18, FoodSideEffect::Sicken(10)) },
                }
            }
        }
        _ => {
            if roll < 18 {
                Item { kind: ItemKind::Potion, name: "Superior Health Potion", glyph: '!', effect: ItemEffect::Heal(15) }
            } else if roll < 32 {
                Item { kind: ItemKind::Scroll, name: "Scroll of Storm", glyph: '?', effect: ItemEffect::DamageAoe(16) }
            } else if roll < 42 {
                match sub {
                    0 => Item { kind: ItemKind::Weapon, name: "Enchanted Blade", glyph: '/', effect: ItemEffect::BuffAttack(6) },
                    1 => Item { kind: ItemKind::Weapon, name: "Crystal Staff", glyph: '/', effect: ItemEffect::BuffAttack(7) },
                    _ => Item { kind: ItemKind::Weapon, name: "Flame Sword", glyph: '/', effect: ItemEffect::BuffAttack(6) },
                }
            } else if roll < 48 {
                Item { kind: ItemKind::RangedWeapon, name: "Elven Bow", glyph: '}', effect: ItemEffect::BuffAttack(6) }
            } else if roll < 54 {
                Item { kind: ItemKind::Armor, name: "Dragon Scale", glyph: '[', effect: ItemEffect::BuffDefense(6) }
            } else if roll < 59 {
                Item { kind: ItemKind::Helmet, name: "Mithril Helm", glyph: '^', effect: ItemEffect::BuffDefense(5) }
            } else if roll < 64 {
                Item { kind: ItemKind::Shield, name: "Tower Shield", glyph: ')', effect: ItemEffect::BuffDefense(5) }
            } else if roll < 69 {
                Item { kind: ItemKind::Boots, name: "Plate Boots", glyph: '{', effect: ItemEffect::BuffDefense(4) }
            } else if roll < 80 {
                match sub {
                    0 => Item { kind: ItemKind::Ring, name: "Gold Ring", glyph: '=', effect: ItemEffect::BuffAttack(4) },
                    _ => Item { kind: ItemKind::Ring, name: "Diamond Ring", glyph: '=', effect: ItemEffect::BuffDefense(4) },
                }
            } else {
                match sub {
                    0 => Item { kind: ItemKind::Food, name: "Elven Waybread", glyph: '%', effect: ItemEffect::Feed(25, FoodSideEffect::Heal(5)) },
                    _ => Item { kind: ItemKind::Food, name: "Honey Mead", glyph: '~', effect: ItemEffect::Feed(18, FoodSideEffect::Energize(15)) },
                }
            }
        }
    }
}

/// Returns a meat/food item if the killed enemy is a beast.
pub(super) fn meat_drop(enemy_name: &str) -> Option<Item> {
    match enemy_name {
        "Giant Rat" => Some(Item {
            kind: ItemKind::Food, name: "Rat Meat", glyph: '%',
            effect: ItemEffect::Feed(10, FoodSideEffect::Sicken(5)),
        }),
        "Wolf" => Some(Item {
            kind: ItemKind::Food, name: "Wolf Meat", glyph: '%',
            effect: ItemEffect::Feed(20, FoodSideEffect::Energize(10)),
        }),
        "Boar" => Some(Item {
            kind: ItemKind::Food, name: "Boar Meat", glyph: '%',
            effect: ItemEffect::Feed(25, FoodSideEffect::Heal(3)),
        }),
        "Bear" => Some(Item {
            kind: ItemKind::Food, name: "Bear Meat", glyph: '%',
            effect: ItemEffect::Feed(25, FoodSideEffect::Heal(3)),
        }),
        "Goblin" | "Goblin Archer" => {
            // 20% chance to drop stolen rations (handled by caller via RNG,
            // but for simplicity we always drop for goblins here)
            Some(Item {
                kind: ItemKind::Food, name: "Stolen Rations", glyph: '%',
                effect: ItemEffect::Feed(18, FoodSideEffect::None),
            })
        }
        _ => None,
    }
}

impl Game {
    /// Use a consumable item from inventory. Returns true if used successfully.
    pub fn use_item(&mut self, index: usize) -> bool {
        if index >= self.inventory.len() {
            return false;
        }
        let item = &self.inventory[index];
        match item.kind {
            ItemKind::Potion => {
                if let ItemEffect::Heal(amount) = item.effect {
                    let old_hp = self.player_hp;
                    self.player_hp = (self.player_hp + amount).min(self.player_max_hp);
                    let healed = self.player_hp - old_hp;
                    let name = item.name;
                    self.messages.push(format!("You drink {name}. Healed {healed} HP."));
                    self.floating_texts.push(FloatingText {
                        world_x: self.player_x, world_y: self.player_y,
                        text: format!("+{healed} HP"), color: "#4f4", age: 0.0,
                    });
                    self.visual_effects.push(VisualEffect {
                        kind: EffectKind::HealGlow,
                        x: self.player_x, y: self.player_y, age: 0.0,
                    });
                    self.inventory.remove(index);
                    self.quick_bar.on_item_removed(index);
                    self.clamp_inventory_scroll();
                    return true;
                }
                false
            }
            ItemKind::Scroll => {
                if let ItemEffect::DamageAoe(damage) = item.effect {
                    let name = item.name;
                    self.messages.push(format!("You read {name}!"));
                    self.inventory.remove(index);
                    self.quick_bar.on_item_removed(index);
                    self.clamp_inventory_scroll();
                    let px = self.player_x;
                    let py = self.player_y;
                    // AOE blast effect
                    self.visual_effects.push(VisualEffect {
                        kind: EffectKind::AoeBlast,
                        x: px, y: py, age: 0.0,
                    });
                    // Damage all enemies within 3 tiles
                    for enemy in &mut self.enemies {
                        if enemy.hp <= 0 { continue; }
                        let dist = (enemy.x - px).abs() + (enemy.y - py).abs();
                        if dist <= 3 {
                            enemy.hp -= damage;
                            self.floating_texts.push(FloatingText {
                                world_x: enemy.x, world_y: enemy.y,
                                text: format!("-{damage}"), color: "#f84", age: 0.0,
                            });
                            if enemy.hp <= 0 {
                                self.messages.push(format!("{} is destroyed!", enemy.name));
                            }
                        }
                    }
                    return true;
                }
                false
            }
            ItemKind::Food => self.eat_food(index),
            _ => false, // Weapons/Armor/RangedWeapons should be equipped, not used
        }
    }

    /// Equip an item from inventory into its matching slot. Returns true if equipped.
    /// If a slot is already occupied, the old item goes back to inventory.
    pub fn equip_item(&mut self, index: usize) -> bool {
        if index >= self.inventory.len() {
            return false;
        }
        let slot = match self.inventory[index].kind {
            ItemKind::Weapon | ItemKind::RangedWeapon => &mut self.equipped_weapon,
            ItemKind::Armor   => &mut self.equipped_armor,
            ItemKind::Helmet  => &mut self.equipped_helmet,
            ItemKind::Shield  => &mut self.equipped_shield,
            ItemKind::Boots   => &mut self.equipped_boots,
            ItemKind::Ring    => &mut self.equipped_ring,
            _ => return false, // Potions/Scrolls/Food should be used, not equipped
        };
        let new_item = self.inventory.remove(index);
        self.quick_bar.on_item_removed(index);
        let name = new_item.name;
        if let Some(old) = slot.replace(new_item) {
            self.messages.push(format!("You swap {} for {name}.", old.name));
            self.inventory.push(old);
        } else {
            self.messages.push(format!("You equip {name}."));
        }
        self.clamp_inventory_scroll();
        true
    }

    /// Drop an item from inventory onto the ground. Returns true if dropped.
    pub fn drop_item(&mut self, index: usize) -> bool {
        if index >= self.inventory.len() {
            return false;
        }
        let item = self.inventory.remove(index);
        self.quick_bar.on_item_removed(index);
        let name = item.name;
        self.messages.push(format!("You drop {name}."));
        self.ground_items.push(GroundItem {
            x: self.player_x,
            y: self.player_y,
            item,
        });
        self.clamp_inventory_scroll();
        true
    }

    /// Eat food from inventory. Returns true if eaten.
    pub fn eat_food(&mut self, index: usize) -> bool {
        if index >= self.inventory.len() {
            return false;
        }
        if self.inventory[index].kind != ItemKind::Food {
            return false;
        }
        if let ItemEffect::Feed(amount, side_effect) = self.inventory[index].effect {
            let old = self.hunger;
            self.hunger = (self.hunger + amount).min(self.max_hunger);
            let gained = self.hunger - old;
            let name = self.inventory[index].name;
            self.messages.push(format!("You eat {name}. Hunger +{gained}."));

            // Apply side effect
            let fx = self.player_x;
            let fy = self.player_y;
            match side_effect {
                FoodSideEffect::None => {}
                FoodSideEffect::Heal(hp) => {
                    self.player_hp = (self.player_hp + hp).min(self.player_max_hp);
                    self.messages.push(format!("You feel revitalized. +{hp} HP."));
                    self.floating_texts.push(FloatingText {
                        world_x: fx, world_y: fy,
                        text: format!("+{hp} HP"), color: "#4f4", age: 0.0,
                    });
                    self.visual_effects.push(VisualEffect {
                        kind: EffectKind::HealGlow, x: fx, y: fy, age: 0.0,
                    });
                }
                FoodSideEffect::Poison(dmg) => {
                    self.player_hp -= dmg;
                    self.messages.push(format!("Your stomach churns! -{dmg} HP."));
                    self.floating_texts.push(FloatingText {
                        world_x: fx, world_y: fy,
                        text: format!("-{dmg} HP"), color: "#a4f", age: 0.0,
                    });
                    self.visual_effects.push(VisualEffect {
                        kind: EffectKind::PoisonCloud, x: fx, y: fy, age: 0.0,
                    });
                    if self.player_hp <= 0 {
                        self.alive = false;
                        self.messages.push("You died from food poisoning.".into());
                    }
                }
                FoodSideEffect::Energize(stam) => {
                    self.stamina = (self.stamina + stam).min(self.max_stamina);
                    self.messages.push(format!("You feel energized. +{stam} stamina."));
                    self.floating_texts.push(FloatingText {
                        world_x: fx, world_y: fy,
                        text: format!("+{stam} STA"), color: "#4ef", age: 0.0,
                    });
                    self.visual_effects.push(VisualEffect {
                        kind: EffectKind::EnergizeEffect, x: fx, y: fy, age: 0.0,
                    });
                }
                FoodSideEffect::Sicken(stam) => {
                    self.stamina = (self.stamina - stam).max(0);
                    self.messages.push(format!("You feel nauseous. -{stam} stamina."));
                    self.floating_texts.push(FloatingText {
                        world_x: fx, world_y: fy,
                        text: format!("-{stam} STA"), color: "#a4f", age: 0.0,
                    });
                }
            }

            self.inventory.remove(index);
            self.quick_bar.on_item_removed(index);
            self.clamp_inventory_scroll();
            true
        } else {
            false
        }
    }

    /// Clamp inventory scroll so it never exceeds the item count.
    pub fn clamp_inventory_scroll(&mut self) {
        let len = self.inventory.len();
        if len == 0 {
            self.inventory_scroll = 0;
        } else if self.inventory_scroll >= len {
            self.inventory_scroll = len - 1;
        }
        // Also clamp selection
        if let Some(sel) = self.selected_inventory_item {
            if sel >= len {
                self.selected_inventory_item = None;
            }
        }
    }

    /// Scroll the inventory list by `delta` items (positive = down, negative = up).
    pub fn scroll_inventory(&mut self, delta: i32) {
        let new = self.inventory_scroll as i32 + delta;
        self.inventory_scroll = new.max(0) as usize;
        self.clamp_inventory_scroll();
    }

    /// Set the inventory scroll position absolutely (clamped).
    pub fn set_inventory_scroll(&mut self, pos: usize) {
        self.inventory_scroll = pos;
        self.clamp_inventory_scroll();
    }

    /// Get a description string for an inventory item.
    pub fn inventory_item_desc(&self, index: usize) -> Option<String> {
        self.inventory.get(index).map(item_info_desc)
    }

    pub fn toggle_drawer(&mut self, drawer: Drawer) {
        if self.drawer == drawer {
            self.drawer = Drawer::None;
        } else {
            self.drawer = drawer;
        }
        self.selected_inventory_item = None;
    }

    /// Inspect a world tile and return structured info for the HUD.
    /// Returns None if the tile is not visible (Hidden).
    pub fn inspect_tile(&self, x: i32, y: i32) -> Option<TileInfo> {
        let map = self.current_map();
        if x < 0 || y < 0 || x >= map.width || y >= map.height {
            return None;
        }
        let vis = map.get_visibility(x, y);
        if vis == crate::map::Visibility::Hidden {
            return None;
        }

        let tile = map.get(x, y);
        let mut info = TileInfo {
            tile_name: tile_name(tile),
            tile_desc: tile_desc(tile),
            walkable: tile.is_walkable(),
            enemy: None,
            item: None,
            is_player: x == self.player_x && y == self.player_y,
        };

        // Only show entities on currently visible tiles
        if vis == crate::map::Visibility::Visible {
            if let Some(e) = self.enemies.iter().find(|e| e.x == x && e.y == y && e.hp > 0) {
                info.enemy = Some(EnemyInfo {
                    name: e.name,
                    hp: e.hp,
                    attack: e.attack,
                    defense: e.defense,
                    desc: enemy_desc(e.name),
                });
            }
            if let Some(gi) = self.ground_items.iter().find(|gi| gi.x == x && gi.y == y) {
                info.item = Some(ItemInfo {
                    name: gi.item.name,
                    desc: item_info_desc(&gi.item),
                });
            }
        }

        Some(info)
    }

    /// Pick up items at the player's position explicitly. Returns true if any picked up.
    pub fn pickup_items_explicit(&mut self) -> bool {
        let px = self.player_x;
        let py = self.player_y;
        let mut picked = false;
        let mut i = 0;
        while i < self.ground_items.len() {
            if self.ground_items[i].x == px && self.ground_items[i].y == py {
                if self.inventory.len() >= self.config.player.max_inventory {
                    self.messages.push("Inventory full!".into());
                    break;
                }
                let gi = self.ground_items.remove(i);
                self.messages.push(format!("Picked up {}.", gi.item.name));
                self.inventory.push(gi.item);
                picked = true;
            } else {
                i += 1;
            }
        }
        picked
    }

    /// Spawn items on the overworld (rare, near roads).
    pub fn spawn_overworld_items(&mut self, seed: u64) {
        let map = self.world.current_map();
        let mut rng = seed;
        for y in 2..map.height - 2 {
            for x in 2..map.width - 2 {
                let tile = map.get(x, y);
                if tile != Tile::Road && tile != Tile::Grass {
                    continue;
                }
                rng = xorshift64(rng);
                // Configurable chance on roads vs grass
                let threshold = if tile == Tile::Road {
                    self.config.spawn.overworld_item_road_pct
                } else {
                    self.config.spawn.overworld_item_grass_pct
                };
                if rng % 1000 >= threshold {
                    continue;
                }
                rng = xorshift64(rng);
                let item = random_item(0, &mut rng);
                self.ground_items.push(GroundItem { x, y, item });
            }
        }
    }

    /// Spawn food on the overworld: berries, mushrooms, plants, water on grass tiles.
    pub fn spawn_overworld_food(&mut self, seed: u64) {
        let map = self.world.current_map();
        let mut rng = seed;
        for y in 2..map.height - 2 {
            for x in 2..map.width - 2 {
                let tile = map.get(x, y);
                if tile != Tile::Grass {
                    continue;
                }
                rng = xorshift64(rng);
                // Configurable chance per grass tile
                if rng % 1000 >= self.config.spawn.overworld_food_pct {
                    continue;
                }
                rng = xorshift64(rng);
                let roll = rng % 100;
                let food = if roll < 12 {
                    Item { kind: ItemKind::Food, name: "Wild Berries", glyph: '%',
                        effect: ItemEffect::Feed(14, FoodSideEffect::Heal(2)) }
                } else if roll < 22 {
                    Item { kind: ItemKind::Food, name: "Wild Mushrooms", glyph: '%',
                        effect: ItemEffect::Feed(16, FoodSideEffect::Poison(2)) }
                } else if roll < 30 {
                    Item { kind: ItemKind::Food, name: "Clean Water", glyph: '~',
                        effect: ItemEffect::Feed(8, FoodSideEffect::None) }
                } else if roll < 40 {
                    Item { kind: ItemKind::Food, name: "Wild Wheat", glyph: '%',
                        effect: ItemEffect::Feed(10, FoodSideEffect::None) }
                } else if roll < 48 {
                    Item { kind: ItemKind::Food, name: "Wild Rice", glyph: '%',
                        effect: ItemEffect::Feed(8, FoodSideEffect::None) }
                } else if roll < 56 {
                    Item { kind: ItemKind::Food, name: "Wild Corn", glyph: '%',
                        effect: ItemEffect::Feed(14, FoodSideEffect::Energize(5)) }
                } else if roll < 64 {
                    Item { kind: ItemKind::Food, name: "Quinoa Seeds", glyph: '%',
                        effect: ItemEffect::Feed(12, FoodSideEffect::Heal(2)) }
                } else if roll < 72 {
                    Item { kind: ItemKind::Food, name: "Amaranth", glyph: '%',
                        effect: ItemEffect::Feed(10, FoodSideEffect::Heal(1)) }
                } else if roll < 80 {
                    Item { kind: ItemKind::Food, name: "Red Spinach", glyph: '%',
                        effect: ItemEffect::Feed(8, FoodSideEffect::Energize(3)) }
                } else if roll < 87 {
                    Item { kind: ItemKind::Food, name: "Bitter Vetch", glyph: '%',
                        effect: ItemEffect::Feed(8, FoodSideEffect::Poison(3)) }
                } else if roll < 93 {
                    Item { kind: ItemKind::Food, name: "Sorghum", glyph: '%',
                        effect: ItemEffect::Feed(8, FoodSideEffect::None) }
                } else {
                    Item { kind: ItemKind::Food, name: "Buckwheat", glyph: '%',
                        effect: ItemEffect::Feed(8, FoodSideEffect::None) }
                };
                self.ground_items.push(GroundItem { x, y, item: food });
            }
        }
    }

    /// Spawn items on a dungeon level. Deeper = better loot.
    pub(super) fn spawn_dungeon_items(&mut self, dungeon_index: usize, level: usize) {
        let total_levels = self.world.dungeons[dungeon_index].levels.len();
        let is_cave = total_levels == 4 && level == 3;

        let map = self.world.current_map();
        let seed = (dungeon_index as u64)
            .wrapping_mul(37)
            .wrapping_add(level as u64)
            .wrapping_mul(2654435761);
        let mut rng = seed;
        for y in 1..map.height - 1 {
            for x in 1..map.width - 1 {
                if map.get(x, y) != Tile::Floor {
                    continue;
                }
                rng = xorshift64(rng);
                // Configurable chance per floor tile in dungeons vs cave
                let threshold = if is_cave {
                    self.config.spawn.cave_item_pct
                } else {
                    self.config.spawn.dungeon_item_pct
                };
                if rng % 100 >= threshold {
                    continue;
                }
                rng = xorshift64(rng);
                // Tier bleed: 20% chance one tier higher, 10% one tier lower
                let base_tier = if is_cave { 2 } else { level };
                rng = xorshift64(rng);
                let bleed_roll = rng % 100;
                let tier = if bleed_roll < 20 && base_tier < 2 {
                    base_tier + 1 // Lucky find
                } else if bleed_roll < 30 && base_tier > 0 {
                    base_tier - 1 // Tough luck
                } else {
                    base_tier
                };
                let item = random_item(tier, &mut rng);
                self.ground_items.push(GroundItem { x, y, item });
            }
        }
    }
}
