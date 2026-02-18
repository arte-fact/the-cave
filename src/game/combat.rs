use crate::map::Tile;
use crate::world::Location;
use super::types::*;
use super::{Game, xorshift64, calc_damage};

impl Game {
    /// Player's total attack: base + strength + weapon bonus.
    pub fn effective_attack(&self) -> i32 {
        let mut total = self.player_attack + self.strength;
        if let Some(item) = &self.equipped_weapon {
            if let ItemEffect::BuffAttack(bonus) = item.effect { total += bonus; }
        }
        if let Some(ring) = &self.equipped_ring {
            if let ItemEffect::BuffAttack(bonus) = ring.effect { total += bonus; }
        }
        total
    }

    /// Player's total defense: base + armor + helmet + shield + boots + ring.
    pub fn effective_defense(&self) -> i32 {
        let mut total = self.player_defense;
        for slot in [&self.equipped_armor, &self.equipped_helmet, &self.equipped_shield, &self.equipped_boots] {
            if let Some(item) = slot {
                if let ItemEffect::BuffDefense(bonus) = item.effect { total += bonus; }
            }
        }
        if let Some(ring) = &self.equipped_ring {
            if let ItemEffect::BuffDefense(bonus) = ring.effect { total += bonus; }
        }
        total
    }

    pub(super) fn enemy_turn(&mut self) {
        self.enemy_turn_inner(false);
    }

    /// Core enemy AI. If `half_speed` is true, only odd-indexed enemies act (sprint mode).
    pub(super) fn enemy_turn_inner(&mut self, half_speed: bool) {
        let px = self.player_x;
        let py = self.player_y;
        let pdef = self.effective_defense();
        let p_dex = self.player_dexterity;

        for i in 0..self.enemies.len() {
            if self.enemies[i].hp <= 0 {
                continue;
            }
            // Half speed: skip even-indexed enemies
            if half_speed && i % 2 == 0 {
                continue;
            }
            let ex = self.enemies[i].x;
            let ey = self.enemies[i].y;
            let dist = (ex - px).abs() + (ey - py).abs();

            // Ranged enemies: shoot if within 2-4 tiles and have line of sight
            if self.enemies[i].is_ranged && dist >= 2 && dist <= 4 {
                if self.world.current_map().has_line_of_sight(ex, ey, px, py) {
                    let raw = self.enemies[i].attack;
                    let dmg = calc_damage(raw, pdef);
                    // Ranged hit chance: 70% base, miss chance exists
                    let seed = self.turn as u64 * 13 + i as u64 * 7 + ex as u64 * 31 + 337;
                    let roll = xorshift64(seed) % 100;
                    let name = self.enemies[i].name;
                    if roll < 70 {
                        // Dodge check: 2% per DEX point, capped at 20%
                        let dodge_chance = (p_dex * 2).min(20) as u64;
                        let dodge_roll = xorshift64(seed.wrapping_add(17)) % 100;
                        if dodge_roll < dodge_chance {
                            self.messages.push(format!("You dodge {name}'s arrow!"));
                            self.floating_texts.push(FloatingText {
                                world_x: px, world_y: py,
                                text: "DODGE".into(), color: "#4ef", age: 0.0,
                            });
                            continue;
                        }
                        self.player_hp -= dmg;
                        self.messages.push(format!("{name} shoots you for {dmg} damage."));
                        self.floating_texts.push(FloatingText {
                            world_x: px, world_y: py,
                            text: format!("-{dmg}"), color: "#f44", age: 0.0,
                        });
                        // Player recoils from hit
                        let rdx = (px - ex) as f64;
                        let rdy = (py - ey) as f64;
                        let rlen = (rdx * rdx + rdy * rdy).sqrt().max(1.0);
                        self.bump_anims.push(BumpAnim {
                            is_player: true, enemy_idx: 0,
                            dx: rdx / rlen * 0.15, dy: rdy / rlen * 0.15,
                            progress: 0.0,
                        });
                        if self.player_hp <= 0 {
                            self.alive = false;
                            self.messages.push("You died.".into());
                        }
                    } else {
                        self.messages.push(format!("{name}'s arrow misses!"));
                        self.floating_texts.push(FloatingText {
                            world_x: ex, world_y: ey,
                            text: "MISS".into(), color: "#888", age: 0.0,
                        });
                    }
                    continue; // Ranged enemies don't also chase this turn
                }
            }

            // Adjacent (Chebyshev): attack the player directly (includes diagonals)
            let chebyshev = (ex - px).abs().max((ey - py).abs());
            if chebyshev == 1 {
                let raw = self.enemies[i].attack;
                let dmg = calc_damage(raw, pdef);
                let name = self.enemies[i].name;
                // Dodge check: 2% per DEX point, capped at 20%
                let dodge_chance = (p_dex * 2).min(20) as u64;
                let dodge_seed = self.turn as u64 * 7 + i as u64 * 13 + 997;
                let dodge_roll = xorshift64(dodge_seed) % 100;
                if dodge_roll < dodge_chance {
                    self.messages.push(format!("You dodge {name}'s attack!"));
                    self.floating_texts.push(FloatingText {
                        world_x: px, world_y: py,
                        text: "DODGE".into(), color: "#4ef", age: 0.0,
                    });
                } else {
                    self.player_hp -= dmg;
                    self.messages.push(format!("{name} hits you for {dmg} damage."));
                    self.floating_texts.push(FloatingText {
                        world_x: px, world_y: py,
                        text: format!("-{dmg}"), color: "#f44", age: 0.0,
                    });
                    // Enemy lunges at player
                    let ldx = (px - ex) as f64;
                    let ldy = (py - ey) as f64;
                    self.bump_anims.push(BumpAnim {
                        is_player: false, enemy_idx: i,
                        dx: ldx * 0.25, dy: ldy * 0.25,
                        progress: 0.0,
                    });
                    // Player recoils
                    self.bump_anims.push(BumpAnim {
                        is_player: true, enemy_idx: 0,
                        dx: -ldx * 0.12, dy: -ldy * 0.12,
                        progress: 0.0,
                    });
                    if self.player_hp <= 0 {
                        self.alive = false;
                        self.messages.push("You died.".into());
                    }
                }
            } else if dist <= 8 {
                // Chase if within 8 tiles (matches overworld FOV)
                let dx = (px - ex).signum();
                let dy = (py - ey).signum();
                // Diagonal first if both axes differ, then cardinal fallbacks
                let seed = self.turn as u64 * 11 + i as u64 * 3 + 127;
                let mut cands: Vec<(i32, i32)> = Vec::new();
                // Diagonal candidate (only if both axes differ)
                if dx != 0 && dy != 0 {
                    // Check corner-cut: both adjacent cardinal tiles must be walkable
                    let map = self.world.current_map();
                    if map.is_walkable(ex + dx, ey) && map.is_walkable(ex, ey + dy) {
                        cands.push((ex + dx, ey + dy));
                    }
                }
                // Cardinal fallbacks (randomized order)
                if xorshift64(seed) % 2 == 0 {
                    if dx != 0 { cands.push((ex + dx, ey)); }
                    if dy != 0 { cands.push((ex, ey + dy)); }
                } else {
                    if dy != 0 { cands.push((ex, ey + dy)); }
                    if dx != 0 { cands.push((ex + dx, ey)); }
                }
                for (cx, cy) in cands {
                    if cx == px && cy == py {
                        // Melee attack — uses ratio-based damage formula
                        let raw = self.enemies[i].attack;
                        let dmg = calc_damage(raw, pdef);
                        let name = self.enemies[i].name;
                        // Dodge check: 2% per DEX point, capped at 20%
                        let dodge_chance = (p_dex * 2).min(20) as u64;
                        let dodge_seed = self.turn as u64 * 7 + i as u64 * 13 + 997;
                        let dodge_roll = xorshift64(dodge_seed) % 100;
                        if dodge_roll < dodge_chance {
                            self.messages.push(format!("You dodge {name}'s attack!"));
                            self.floating_texts.push(FloatingText {
                                world_x: px, world_y: py,
                                text: "DODGE".into(), color: "#4ef", age: 0.0,
                            });
                            break;
                        }
                        self.player_hp -= dmg;
                        self.messages.push(format!("{name} hits you for {dmg} damage."));
                        self.floating_texts.push(FloatingText {
                            world_x: px, world_y: py,
                            text: format!("-{dmg}"), color: "#f44", age: 0.0,
                        });
                        let ldx = (px - ex) as f64;
                        let ldy = (py - ey) as f64;
                        self.bump_anims.push(BumpAnim {
                            is_player: false, enemy_idx: i,
                            dx: ldx * 0.25, dy: ldy * 0.25,
                            progress: 0.0,
                        });
                        self.bump_anims.push(BumpAnim {
                            is_player: true, enemy_idx: 0,
                            dx: -ldx * 0.12, dy: -ldy * 0.12,
                            progress: 0.0,
                        });
                        if self.player_hp <= 0 {
                            self.alive = false;
                            self.messages.push("You died.".into());
                        }
                        break;
                    }
                    if self.world.current_map().is_walkable(cx, cy)
                        && !self.enemies.iter().any(|e| e.hp > 0 && e.x == cx && e.y == cy)
                    {
                        let move_dx = cx - self.enemies[i].x;
                        if move_dx < 0 { self.enemies[i].facing_left = true; }
                        if move_dx > 0 { self.enemies[i].facing_left = false; }
                        self.enemies[i].x = cx;
                        self.enemies[i].y = cy;
                        break;
                    }
                }
            }
        }
    }

    /// Explicitly attack an enemy at the given position (must be adjacent).
    pub fn attack_adjacent(&mut self, tx: i32, ty: i32) -> TurnResult {
        if !self.alive || self.won {
            return TurnResult::Blocked;
        }

        // Chebyshev distance: adjacent includes diagonals
        let cdist = (tx - self.player_x).abs().max((ty - self.player_y).abs());
        if cdist != 1 {
            return TurnResult::Blocked;
        }

        // Face the target
        let dx = tx - self.player_x;
        let dy = ty - self.player_y;
        if dx < 0 { self.player_facing_left = true; }
        if dx > 0 { self.player_facing_left = false; }

        if let Some(idx) = self.enemies.iter().position(|e| e.x == tx && e.y == ty && e.hp > 0) {
            let atk = self.effective_attack();
            let edef = self.enemies[idx].defense;
            let dmg = calc_damage(atk, edef);
            self.enemies[idx].hp -= dmg;
            let name = self.enemies[idx].name;

            // Player lunges toward enemy
            self.bump_anims.push(BumpAnim {
                is_player: true, enemy_idx: 0,
                dx: dx as f64 * 0.3, dy: dy as f64 * 0.3,
                progress: 0.0,
            });
            // Enemy recoils
            self.bump_anims.push(BumpAnim {
                is_player: false, enemy_idx: idx,
                dx: dx as f64 * 0.15, dy: dy as f64 * 0.15,
                progress: 0.0,
            });

            let mut result;
            if self.enemies[idx].hp <= 0 {
                let xp = self.xp_with_diminishing(name);
                let ex = self.enemies[idx].x;
                let ey = self.enemies[idx].y;
                self.player_xp += xp;
                self.check_level_up();
                self.messages.push(format!("You slay the {name}! (+{xp} XP)"));
                // Floating XP text
                self.floating_texts.push(FloatingText {
                    world_x: ex, world_y: ey,
                    text: format!("+{xp} XP"), color: "#ff0", age: 0.0,
                });
                if self.world.location == Location::Overworld {
                    self.overworld_kills += 1;
                }
                if let Some(meat) = super::items::meat_drop(name) {
                    self.ground_items.push(GroundItem { x: ex, y: ey, item: meat });
                    self.messages.push("It dropped some meat.".into());
                }
                if self.enemies[idx].glyph == 'D' {
                    self.won = true;
                    self.messages.push("You conquered the cave!".into());
                    return TurnResult::Won;
                }
                result = TurnResult::Killed { target_name: name };
            } else {
                self.messages.push(format!("You hit {name} for {dmg} damage."));
                // Floating damage text on enemy
                self.floating_texts.push(FloatingText {
                    world_x: tx, world_y: ty,
                    text: format!("-{dmg}"), color: "#f44", age: 0.0,
                });
                result = TurnResult::Attacked { target_name: name, damage: dmg };
            }

            if self.sprinting {
                self.enemy_turn_inner(true);
            } else {
                self.enemy_turn();
            }
            self.tick_survival();
            self.update_fov();

            if !self.alive {
                result = TurnResult::PlayerDied;
            }

            return result;
        }

        TurnResult::Blocked
    }

    // === Ranged weapon system ===

    /// Returns true if the player has a ranged weapon (bow/crossbow) equipped.
    pub fn has_ranged_weapon(&self) -> bool {
        matches!(
            self.equipped_weapon,
            Some(Item { kind: ItemKind::RangedWeapon, .. })
        )
    }

    /// Base range for the equipped ranged weapon (before dexterity bonus).
    fn ranged_weapon_base_range(&self) -> i32 {
        match self.equipped_weapon.as_ref().map(|w| w.name) {
            Some("Short Bow") => 4,
            Some("Crossbow") => 3,
            Some("Long Bow") => 6,
            Some("Heavy Crossbow") => 4,
            Some("Elven Bow") => 8,
            _ => 4,
        }
    }

    /// Max range for the equipped ranged weapon, factoring in dexterity.
    pub fn ranged_max_range(&self) -> i32 {
        self.ranged_weapon_base_range() + self.player_dexterity / 3
    }

    /// Hit chance (0–95) for a ranged attack at the given distance.
    /// Higher dexterity = better accuracy (+3% per DEX). Chance drops with distance.
    pub fn ranged_hit_chance(&self, distance: i32) -> i32 {
        let max_range = self.ranged_max_range();
        if distance <= 0 || distance > max_range {
            return 0;
        }
        let base = (90 - distance * 70 / max_range).max(20);
        (base + self.player_dexterity * 3).min(95)
    }

    /// Ranged damage: base attack + distance/2 bonus + DEX/2 bonus, reduced by enemy defense.
    fn ranged_damage(&self, enemy_defense: i32, distance: i32) -> i32 {
        let atk = self.effective_attack() + distance / 2 + self.player_dexterity / 2;
        calc_damage(atk, enemy_defense)
    }

    /// Fire the equipped ranged weapon at target tile (tx, ty).
    /// Consumes a turn: enemies move, survival ticks, FOV updates.
    /// Returns a TurnResult describing what happened.
    pub fn ranged_attack(&mut self, tx: i32, ty: i32) -> TurnResult {
        if !self.alive || self.won {
            return TurnResult::Blocked;
        }
        if !self.has_ranged_weapon() {
            return TurnResult::Blocked;
        }

        // Update facing direction toward target
        if tx < self.player_x { self.player_facing_left = true; }
        if tx > self.player_x { self.player_facing_left = false; }

        let map = self.world.current_map();
        let distance = ((tx - self.player_x).abs()).max((ty - self.player_y).abs());
        let max_range = self.ranged_max_range();
        let weapon_name = self.equipped_weapon.as_ref().map(|w| w.name).unwrap_or("bow");

        // Range check
        if distance > max_range || distance <= 0 {
            self.messages.push(format!("Out of range! Max range: {max_range}."));
            return TurnResult::Blocked;
        }

        // Line of sight check
        if !map.has_line_of_sight(self.player_x, self.player_y, tx, ty) {
            self.messages.push("No line of sight!".into());
            return TurnResult::Blocked;
        }

        // Find enemy at target
        let enemy_idx = self.enemies.iter().position(|e| e.x == tx && e.y == ty && e.hp > 0);
        if enemy_idx.is_none() {
            self.messages.push("Nothing to shoot at.".into());
            return TurnResult::Blocked;
        }
        let idx = enemy_idx.unwrap();

        // Roll hit chance
        let hit_chance = self.ranged_hit_chance(distance);
        let seed = self.turn as u64 * 7 + self.player_x as u64 * 31 + self.player_y as u64 * 17;
        let roll = (xorshift64(seed) % 100) as i32;
        let name = self.enemies[idx].name;

        if roll >= hit_chance {
            // Miss
            self.messages.push(format!(
                "Your {} misses the {name}! ({hit_chance}% chance)",
                weapon_name,
            ));
            self.floating_texts.push(FloatingText {
                world_x: tx, world_y: ty,
                text: "MISS".into(), color: "#888", age: 0.0,
            });
        } else {
            // Hit — ranged damage includes distance bonus
            let edef = self.enemies[idx].defense;
            let dmg = self.ranged_damage(edef, distance);
            self.enemies[idx].hp -= dmg;
            self.messages.push(format!(
                "Your {} hits {name} for {dmg} damage!",
                weapon_name,
            ));
            self.floating_texts.push(FloatingText {
                world_x: tx, world_y: ty,
                text: format!("-{dmg}"), color: "#f44", age: 0.0,
            });
            // Enemy recoils from hit
            let rdx = (tx - self.player_x) as f64;
            let rdy = (ty - self.player_y) as f64;
            let rlen = (rdx * rdx + rdy * rdy).sqrt().max(1.0);
            self.bump_anims.push(BumpAnim {
                is_player: false, enemy_idx: idx,
                dx: rdx / rlen * 0.15, dy: rdy / rlen * 0.15,
                progress: 0.0,
            });

            if self.enemies[idx].hp <= 0 {
                let xp = self.xp_with_diminishing(name);
                let ex = self.enemies[idx].x;
                let ey = self.enemies[idx].y;
                self.player_xp += xp;
                self.check_level_up();
                self.messages.push(format!("You slay the {name}! (+{xp} XP)"));
                self.floating_texts.push(FloatingText {
                    world_x: ex, world_y: ey,
                    text: format!("+{xp} XP"), color: "#ff0", age: 0.0,
                });
                if self.world.location == Location::Overworld {
                    self.overworld_kills += 1;
                }
                if let Some(meat) = super::items::meat_drop(name) {
                    self.ground_items.push(GroundItem { x: ex, y: ey, item: meat });
                    self.messages.push("It dropped some meat.".into());
                }
                if self.enemies[idx].glyph == 'D' {
                    self.won = true;
                    self.messages.push("You conquered the cave!".into());
                    return TurnResult::Won;
                }
            }
        }

        // Ranged attack costs a turn: enemies move (half speed when sprinting), survival ticks
        if self.sprinting {
            self.enemy_turn_inner(true);
        } else {
            self.enemy_turn();
        }
        self.tick_survival();
        self.update_fov();

        TurnResult::Moved
    }

    /// Spawn forest animals on the overworld: wolves, boars, bears.
    pub fn spawn_enemies(&mut self, seed: u64) {
        let map = self.world.current_map();
        let mut rng = seed;
        for y in 2..map.height - 2 {
            for x in 2..map.width - 2 {
                if !map.is_walkable(x, y) {
                    continue;
                }
                if x == self.player_x && y == self.player_y {
                    continue;
                }
                rng = xorshift64(rng);
                // Configurable chance per walkable tile (forest is sparse)
                if rng % 100 < self.config.spawn.overworld_enemy_pct {
                    rng = xorshift64(rng);
                    let roll = rng % 100;
                    let (hp, attack, def, glyph, name) = if roll < 20 {
                        (3, 1, 0, 'r', "Giant Rat")
                    } else if roll < 35 {
                        (4, 2, 0, 'a', "Giant Bat")
                    } else if roll < 60 {
                        (5, 2, 1, 'w', "Wolf")
                    } else if roll < 75 {
                        (6, 3, 0, 'i', "Giant Spider")
                    } else if roll < 87 {
                        (8, 2, 2, 'b', "Boar")
                    } else if roll < 95 {
                        (12, 4, 2, 'B', "Bear")
                    } else {
                        (14, 5, 3, 'L', "Lycanthrope")
                    };
                    self.enemies.push(Enemy { x, y, hp, attack, defense: def, glyph, name, facing_left: false, is_ranged: false });
                }
            }
        }
    }

    /// Spawn enemies appropriate for a dungeon level.
    /// L0: rats, kobolds, slimes, goblins, skeletons.
    /// L1: goblin archers, zombies, skeleton archers, big slimes, orcs.
    /// L2+: ghouls, orc blademasters, wraiths, nagas, trolls.
    /// Cave (L3, dragon dungeon only): death knights, trolls, liches + dragon boss.
    pub(super) fn spawn_dungeon_enemies(&mut self, dungeon_index: usize, level: usize) {
        let total_levels = self.world.dungeons[dungeon_index].levels.len();
        let is_cave = total_levels == 4 && level == 3;

        let map = self.world.current_map();
        let seed = (dungeon_index as u64)
            .wrapping_mul(31)
            .wrapping_add(level as u64)
            .wrapping_mul(6364136223846793005);
        let mut rng = seed;
        for y in 1..map.height - 1 {
            for x in 1..map.width - 1 {
                if !map.is_walkable(x, y) {
                    continue;
                }
                if x == self.player_x && y == self.player_y {
                    continue;
                }
                let tile = map.get(x, y);
                if tile == Tile::StairsUp || tile == Tile::StairsDown {
                    continue;
                }
                rng = xorshift64(rng);
                let spawn_chance = if is_cave {
                    self.config.spawn.cave_enemy_pct
                } else {
                    self.config.spawn.dungeon_enemy_pct
                };
                if rng % 100 < spawn_chance {
                    rng = xorshift64(rng);
                    let roll = rng % 100;
                    // (hp, attack, defense, glyph, name, is_ranged)
                    let (hp, attack, def, glyph, name, ranged) = if is_cave {
                        if roll < 40 {
                            (20, 7, 5, 'K', "Death Knight", false)
                        } else if roll < 70 {
                            (16, 5, 3, 'T', "Troll", false)
                        } else {
                            (15, 8, 2, 'l', "Lich", false)
                        }
                    } else {
                        match level {
                            0 => {
                                if roll < 25 {
                                    (3, 1, 0, 'r', "Giant Rat", false)
                                } else if roll < 40 {
                                    (4, 2, 1, 'c', "Kobold", false)
                                } else if roll < 55 {
                                    (4, 1, 0, 'S', "Small Slime", false)
                                } else if roll < 80 {
                                    (5, 2, 1, 'g', "Goblin", false)
                                } else {
                                    (6, 3, 2, 's', "Skeleton", false)
                                }
                            }
                            1 => {
                                if roll < 20 {
                                    (6, 3, 1, 'G', "Goblin Archer", true)
                                } else if roll < 40 {
                                    (10, 2, 1, 'z', "Zombie", false)
                                } else if roll < 55 {
                                    (7, 4, 2, 'k', "Skeleton Archer", true)
                                } else if roll < 70 {
                                    (10, 2, 0, 'm', "Big Slime", false)
                                } else {
                                    (10, 4, 3, 'o', "Orc", false)
                                }
                            }
                            _ => {
                                if roll < 20 {
                                    (10, 5, 2, 'u', "Ghoul", false)
                                } else if roll < 40 {
                                    (14, 5, 4, 'O', "Orc Blademaster", false)
                                } else if roll < 55 {
                                    (8, 6, 0, 'W', "Wraith", false)
                                } else if roll < 70 {
                                    (12, 6, 3, 'N', "Naga", false)
                                } else {
                                    (16, 5, 3, 'T', "Troll", false)
                                }
                            }
                        }
                    };
                    self.enemies.push(Enemy { x, y, hp, attack, defense: def, glyph, name, facing_left: false, is_ranged: ranged });
                }
            }
        }

        // Place unique dragon boss only in the cave level
        if is_cave {
            let map = self.world.current_map();
            for y in (1..map.height - 1).rev() {
                for x in (1..map.width - 1).rev() {
                    if map.is_walkable(x, y)
                        && map.get(x, y) == Tile::Floor
                        && (x - self.player_x).abs() + (y - self.player_y).abs() > 5
                        && !self.enemies.iter().any(|e| e.x == x && e.y == y)
                    {
                        self.enemies.push(Enemy {
                            x, y, hp: 40, attack: 10, defense: 6, glyph: 'D', name: "Dragon", facing_left: false, is_ranged: false,
                        });
                        return;
                    }
                }
            }
        }
    }

    /// Apply diminishing XP returns for overworld kills. Dungeon kills unaffected.
    fn xp_with_diminishing(&self, enemy_name: &str) -> u32 {
        let base = super::types::xp_for_enemy(enemy_name);
        if self.world.location != Location::Overworld {
            return base;
        }
        let combat = &self.config.combat;
        if self.overworld_kills >= combat.xp_diminish_quarter {
            (base / 4).max(1)
        } else if self.overworld_kills >= combat.xp_diminish_half {
            (base / 2).max(1)
        } else {
            base
        }
    }

    /// XP required to reach next level: xp_base * current_level^xp_exponent (rounded).
    pub fn xp_to_next_level(&self) -> u32 {
        let prog = &self.config.progression;
        (prog.xp_base * (self.player_level as f64).powf(prog.xp_exponent)).round() as u32
    }

    pub(super) fn check_level_up(&mut self) {
        let sp = self.config.progression.skill_points_per_level;
        let hp_per = self.config.progression.hp_per_level;
        while self.player_xp >= self.xp_to_next_level() {
            self.player_xp -= self.xp_to_next_level();
            self.player_level += 1;
            self.skill_points += sp;
            // Small base HP bump on level up + partial heal (50% of missing HP)
            self.player_max_hp += hp_per;
            let missing = self.player_max_hp - self.player_hp;
            self.player_hp += missing / 2 + 1; // +1 so you always heal at least 1
            self.player_hp = self.player_hp.min(self.player_max_hp);
            self.messages.push(format!(
                "Level up! You are now level {}. +{} skill points!",
                self.player_level, sp,
            ));
        }
    }

    /// Allocate one skill point into the given attribute.
    /// Returns true if successful, false if no points available.
    pub fn allocate_skill_point(&mut self, skill: SkillKind) -> bool {
        if self.skill_points == 0 {
            return false;
        }
        self.skill_points -= 1;
        match skill {
            SkillKind::Strength => {
                self.strength += 1;
                self.messages.push(format!("Strength increased to {}.", self.strength));
            }
            SkillKind::Vitality => {
                self.vitality += 1;
                self.player_max_hp += 3;
                self.player_hp = (self.player_hp + 3).min(self.player_max_hp);
                self.messages.push(format!("Vitality increased to {}. Max HP +3.", self.vitality));
            }
            SkillKind::Dexterity => {
                self.player_dexterity += 1;
                let dodge = (self.player_dexterity * 2).min(20);
                self.messages.push(format!("Dexterity increased to {}. Dodge {}%.", self.player_dexterity, dodge));
            }
            SkillKind::Stamina => {
                self.sprint_cost = (self.sprint_cost - 1).max(5);
                self.max_stamina += 5;
                self.stamina = (self.stamina + 5).min(self.max_stamina);
                self.messages.push(format!("Sprint cost reduced to {}. Max stamina {}.", self.sprint_cost, self.max_stamina));
            }
        }
        true
    }
}
