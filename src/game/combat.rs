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
        for item in [&self.equipped_armor, &self.equipped_helmet, &self.equipped_shield, &self.equipped_boots].into_iter().flatten() {
            if let ItemEffect::BuffDefense(bonus) = item.effect { total += bonus; }
        }
        if let Some(ring) = &self.equipped_ring {
            if let ItemEffect::BuffDefense(bonus) = ring.effect { total += bonus; }
        }
        total
    }

    /// Roll a dodge check: 2% per DEX point, capped at 20%.
    /// Returns true if the player dodges.
    fn roll_dodge(&mut self, dodge_seed: u64, attacker_name: &str, label: &str) -> bool {
        let dodge_chance = (self.player_dexterity * 2).min(20) as u64;
        let dodge_roll = xorshift64(dodge_seed) % 100;
        if dodge_roll < dodge_chance {
            self.messages.push(format!("You dodge {attacker_name}'s {label}!"));
            self.floating_texts.push(FloatingText {
                world_x: self.player_x, world_y: self.player_y,
                text: "DODGE".into(), color: "#4ef", age: 0.0,
            });
            return true;
        }
        false
    }

    /// Apply melee damage from an enemy to the player, with bump animations.
    fn apply_enemy_melee_hit(&mut self, enemy_idx: usize, dmg: i32, msg: String) {
        let px = self.player_x;
        let py = self.player_y;
        let ex = self.enemies[enemy_idx].x;
        let ey = self.enemies[enemy_idx].y;
        self.player_hp -= dmg;
        self.messages.push(msg);
        self.floating_texts.push(FloatingText {
            world_x: px, world_y: py,
            text: format!("-{dmg}"), color: "#f44", age: 0.0,
        });
        let ldx = (px - ex) as f64;
        let ldy = (py - ey) as f64;
        self.bump_anims.push(BumpAnim {
            is_player: false, enemy_idx,
            dx: ldx * 0.25, dy: ldy * 0.25,
            progress: 0.0,
        });
        self.bump_anims.push(BumpAnim {
            is_player: true, enemy_idx: 0,
            dx: -ldx * 0.12, dy: -ldy * 0.12,
            progress: 0.0,
        });
        self.check_player_death();
    }

    /// Apply ranged damage recoil from an enemy to the player.
    fn apply_enemy_ranged_hit(&mut self, enemy_idx: usize, dmg: i32) {
        let px = self.player_x;
        let py = self.player_y;
        let ex = self.enemies[enemy_idx].x;
        let ey = self.enemies[enemy_idx].y;
        let name = self.enemies[enemy_idx].name;
        self.player_hp -= dmg;
        self.messages.push(format!("{name} shoots you for {dmg} damage."));
        self.floating_texts.push(FloatingText {
            world_x: px, world_y: py,
            text: format!("-{dmg}"), color: "#f44", age: 0.0,
        });
        let rdx = (px - ex) as f64;
        let rdy = (py - ey) as f64;
        let rlen = (rdx * rdx + rdy * rdy).sqrt().max(1.0);
        self.bump_anims.push(BumpAnim {
            is_player: true, enemy_idx: 0,
            dx: rdx / rlen * 0.15, dy: rdy / rlen * 0.15,
            progress: 0.0,
        });
        self.check_player_death();
    }

    fn check_player_death(&mut self) {
        if self.player_hp <= 0 {
            self.alive = false;
            self.messages.push("You died.".into());
        }
    }

    /// Handle kill rewards: XP, floating text, meat drop, dragon check.
    /// Returns TurnResult::Won if the dragon was killed, otherwise None.
    fn handle_kill(&mut self, enemy_idx: usize) -> Option<TurnResult> {
        let name = self.enemies[enemy_idx].name;
        let ex = self.enemies[enemy_idx].x;
        let ey = self.enemies[enemy_idx].y;
        let xp = self.xp_with_diminishing(name);
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
        if self.enemies[enemy_idx].glyph == 'D' {
            self.won = true;
            self.messages.push("You conquered the cave!".into());
            return Some(TurnResult::Won);
        }
        None
    }

    /// End a combat turn: enemies act, survival ticks, FOV updates.
    fn end_combat_turn(&mut self) {
        if self.sprinting {
            self.enemy_turn_inner(true);
        } else {
            self.enemy_turn();
        }
        self.tick_survival();
        self.update_fov();
    }

    pub(super) fn enemy_turn(&mut self) {
        self.enemy_turn_inner(false);
    }

    /// Core enemy AI. If `half_speed` is true, only odd-indexed enemies act (sprint mode).
    pub(super) fn enemy_turn_inner(&mut self, half_speed: bool) {
        let px = self.player_x;
        let py = self.player_y;
        let pdef = self.effective_defense();

        for i in 0..self.enemies.len() {
            if self.enemies[i].hp <= 0 { continue; }
            if half_speed && i % 2 == 0 { continue; }

            let ex = self.enemies[i].x;
            let ey = self.enemies[i].y;
            let dist = (ex - px).abs() + (ey - py).abs();

            // Ranged enemies: shoot if within 2-4 tiles and have line of sight
            if self.enemies[i].is_ranged && (2..=4).contains(&dist)
                && self.world.current_map().has_line_of_sight(ex, ey, px, py)
            {
                self.enemy_ranged_attack(i, pdef);
                continue;
            }

            // Adjacent (Chebyshev): attack the player
            let chebyshev = (ex - px).abs().max((ey - py).abs());
            if chebyshev == 1 {
                self.enemy_melee_attack(i, pdef);
                continue;
            }

            // Chase if within 8 tiles
            if dist <= 8 {
                self.enemy_chase(i, px, py, pdef);
            }
        }
    }

    /// Enemy ranged attack logic (extracted from enemy_turn_inner).
    fn enemy_ranged_attack(&mut self, i: usize, pdef: i32) {
        let ex = self.enemies[i].x;
        let ey = self.enemies[i].y;
        let raw = self.enemies[i].attack;
        let dmg = calc_damage(raw, pdef);
        let seed = self.turn as u64 * 13 + i as u64 * 7 + ex as u64 * 31 + 337;
        let roll = xorshift64(seed) % 100;
        let name = self.enemies[i].name;

        if roll >= 70 {
            self.messages.push(format!("{name}'s arrow misses!"));
            self.floating_texts.push(FloatingText {
                world_x: ex, world_y: ey,
                text: "MISS".into(), color: "#888", age: 0.0,
            });
            return;
        }

        if self.roll_dodge(seed.wrapping_add(17), name, "arrow") {
            return;
        }
        self.apply_enemy_ranged_hit(i, dmg);
    }

    /// Enemy melee attack (adjacent to player).
    fn enemy_melee_attack(&mut self, i: usize, pdef: i32) {
        let raw = self.enemies[i].attack;
        let dmg = calc_damage(raw, pdef);
        let name = self.enemies[i].name;
        let dodge_seed = self.turn as u64 * 7 + i as u64 * 13 + 997;

        if self.roll_dodge(dodge_seed, name, "attack") {
            return;
        }
        self.apply_enemy_melee_hit(i, dmg, format!("{name} hits you for {dmg} damage."));
    }

    /// Enemy chase logic: move toward player or attack if bumping into them.
    fn enemy_chase(&mut self, i: usize, px: i32, py: i32, pdef: i32) {
        let ex = self.enemies[i].x;
        let ey = self.enemies[i].y;
        let dx = (px - ex).signum();
        let dy = (py - ey).signum();
        let seed = self.turn as u64 * 11 + i as u64 * 3 + 127;

        let mut cands: Vec<(i32, i32)> = Vec::new();
        // Diagonal candidate (only if both axes differ)
        if dx != 0 && dy != 0 {
            let map = self.world.current_map();
            if map.is_walkable(ex + dx, ey) && map.is_walkable(ex, ey + dy) {
                cands.push((ex + dx, ey + dy));
            }
        }
        // Cardinal fallbacks (randomized order)
        if xorshift64(seed).is_multiple_of(2) {
            if dx != 0 { cands.push((ex + dx, ey)); }
            if dy != 0 { cands.push((ex, ey + dy)); }
        } else {
            if dy != 0 { cands.push((ex, ey + dy)); }
            if dx != 0 { cands.push((ex + dx, ey)); }
        }

        for (cx, cy) in cands {
            if cx == px && cy == py {
                self.enemy_melee_attack(i, pdef);
                return;
            }
            if self.world.current_map().is_walkable(cx, cy)
                && !self.enemies.iter().any(|e| e.hp > 0 && e.x == cx && e.y == cy)
            {
                let move_dx = cx - self.enemies[i].x;
                if move_dx < 0 { self.enemies[i].facing_left = true; }
                if move_dx > 0 { self.enemies[i].facing_left = false; }
                self.enemies[i].x = cx;
                self.enemies[i].y = cy;
                return;
            }
        }
    }

    /// Explicitly attack an enemy at the given position (must be adjacent).
    pub fn attack_adjacent(&mut self, tx: i32, ty: i32) -> TurnResult {
        if !self.alive || self.won {
            return TurnResult::Blocked;
        }

        let cdist = (tx - self.player_x).abs().max((ty - self.player_y).abs());
        if cdist != 1 {
            return TurnResult::Blocked;
        }

        let dx = tx - self.player_x;
        let dy = ty - self.player_y;
        if dx < 0 { self.player_facing_left = true; }
        if dx > 0 { self.player_facing_left = false; }

        let Some(idx) = self.enemies.iter().position(|e| e.x == tx && e.y == ty && e.hp > 0) else {
            return TurnResult::Blocked;
        };

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

        let mut result = if self.enemies[idx].hp <= 0 {
            if let Some(won) = self.handle_kill(idx) {
                return won;
            }
            TurnResult::Killed { target_name: name }
        } else {
            self.messages.push(format!("You hit {name} for {dmg} damage."));
            self.floating_texts.push(FloatingText {
                world_x: tx, world_y: ty,
                text: format!("-{dmg}"), color: "#f44", age: 0.0,
            });
            TurnResult::Attacked { target_name: name, damage: dmg }
        };

        self.end_combat_turn();
        if !self.alive { result = TurnResult::PlayerDied; }
        result
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
    pub fn ranged_attack(&mut self, tx: i32, ty: i32) -> TurnResult {
        if !self.alive || self.won || !self.has_ranged_weapon() {
            return TurnResult::Blocked;
        }

        if tx < self.player_x { self.player_facing_left = true; }
        if tx > self.player_x { self.player_facing_left = false; }

        let distance = ((tx - self.player_x).abs()).max((ty - self.player_y).abs());
        let max_range = self.ranged_max_range();
        let weapon_name = self.equipped_weapon.as_ref().map(|w| w.name).unwrap_or("bow");

        if distance > max_range || distance <= 0 {
            self.messages.push(format!("Out of range! Max range: {max_range}."));
            return TurnResult::Blocked;
        }
        if !self.world.current_map().has_line_of_sight(self.player_x, self.player_y, tx, ty) {
            self.messages.push("No line of sight!".into());
            return TurnResult::Blocked;
        }

        let Some(idx) = self.enemies.iter().position(|e| e.x == tx && e.y == ty && e.hp > 0) else {
            self.messages.push("Nothing to shoot at.".into());
            return TurnResult::Blocked;
        };

        let hit_chance = self.ranged_hit_chance(distance);
        let seed = self.turn as u64 * 7 + self.player_x as u64 * 31 + self.player_y as u64 * 17;
        let roll = (xorshift64(seed) % 100) as i32;
        let name = self.enemies[idx].name;

        if roll >= hit_chance {
            self.messages.push(format!("Your {weapon_name} misses the {name}! ({hit_chance}% chance)"));
            self.floating_texts.push(FloatingText {
                world_x: tx, world_y: ty,
                text: "MISS".into(), color: "#888", age: 0.0,
            });
        } else {
            let edef = self.enemies[idx].defense;
            let dmg = self.ranged_damage(edef, distance);
            self.enemies[idx].hp -= dmg;
            self.messages.push(format!("Your {weapon_name} hits {name} for {dmg} damage!"));
            self.floating_texts.push(FloatingText {
                world_x: tx, world_y: ty,
                text: format!("-{dmg}"), color: "#f44", age: 0.0,
            });
            // Enemy recoils
            let rdx = (tx - self.player_x) as f64;
            let rdy = (ty - self.player_y) as f64;
            let rlen = (rdx * rdx + rdy * rdy).sqrt().max(1.0);
            self.bump_anims.push(BumpAnim {
                is_player: false, enemy_idx: idx,
                dx: rdx / rlen * 0.15, dy: rdy / rlen * 0.15,
                progress: 0.0,
            });

            if self.enemies[idx].hp <= 0 {
                if let Some(won) = self.handle_kill(idx) {
                    return won;
                }
            }
        }

        self.end_combat_turn();
        TurnResult::Moved
    }

    /// Spawn forest creatures on the overworld with expanded variety.
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
                    // (hp, atk, def, glyph, name)
                    let (hp, attack, def, glyph, name) = if roll < 12 {
                        (3, 1, 0, 'r', "Giant Rat")
                    } else if roll < 20 {
                        (4, 2, 0, 'a', "Giant Bat")
                    } else if roll < 30 {
                        (4, 2, 1, 'f', "Fox")
                    } else if roll < 40 {
                        (5, 3, 0, 'n', "Viper")
                    } else if roll < 55 {
                        (5, 2, 1, 'w', "Wolf")
                    } else if roll < 65 {
                        (6, 3, 0, 'i', "Giant Spider")
                    } else if roll < 72 {
                        (5, 3, 1, 'j', "Badger")
                    } else if roll < 80 {
                        (8, 2, 2, 'b', "Boar")
                    } else if roll < 87 {
                        (9, 4, 2, 'h', "Cougar")
                    } else if roll < 94 {
                        (12, 4, 2, 'B', "Bear")
                    } else {
                        (14, 5, 3, 'L', "Lycanthrope")
                    };
                    self.enemies.push(Enemy { x, y, hp, attack, defense: def, glyph, name, facing_left: false, is_ranged: false });
                }
            }
        }
    }

    /// Spawn enemies appropriate for a dungeon level with expanded variety.
    /// L0: rats, kobolds, slimes, goblins, skeletons, centipedes, myconids.
    /// L1: goblin archers, zombies, skeleton archers, big slimes, orcs, ants, mages, hags.
    /// L2+: ghouls, blademasters, wraiths, nagas, trolls, ettins, golems, minotaurs, medusas, banshees.
    /// Cave: death knights, liches, drakes, basilisks, imps, manticores, reapers + dragon boss.
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
                        if roll < 20 {
                            (20, 7, 5, 'K', "Death Knight", false)
                        } else if roll < 35 {
                            (16, 5, 3, 'T', "Troll", false)
                        } else if roll < 50 {
                            (15, 8, 2, 'l', "Lich", false)
                        } else if roll < 60 {
                            (14, 6, 3, 'd', "Drake", false)
                        } else if roll < 70 {
                            (16, 7, 4, 'C', "Basilisk", false)
                        } else if roll < 80 {
                            (10, 6, 1, 'I', "Imp", false)
                        } else if roll < 90 {
                            (18, 7, 4, 'X', "Manticore", false)
                        } else {
                            (20, 9, 3, 'V', "Reaper", false)
                        }
                    } else {
                        match level {
                            0 => {
                                if roll < 15 {
                                    (3, 1, 0, 'r', "Giant Rat", false)
                                } else if roll < 25 {
                                    (4, 2, 1, 'c', "Kobold", false)
                                } else if roll < 35 {
                                    (4, 1, 0, 'S', "Small Slime", false)
                                } else if roll < 50 {
                                    (5, 2, 1, 'g', "Goblin", false)
                                } else if roll < 62 {
                                    (4, 2, 0, 'e', "Giant Centipede", false)
                                } else if roll < 75 {
                                    (3, 1, 1, 'p', "Myconid", false)
                                } else {
                                    (6, 3, 2, 's', "Skeleton", false)
                                }
                            }
                            1 => {
                                if roll < 12 {
                                    (6, 3, 1, 'G', "Goblin Archer", true)
                                } else if roll < 24 {
                                    (10, 2, 1, 'z', "Zombie", false)
                                } else if roll < 36 {
                                    (7, 4, 2, 'k', "Skeleton Archer", true)
                                } else if roll < 48 {
                                    (10, 2, 0, 'm', "Big Slime", false)
                                } else if roll < 60 {
                                    (10, 4, 3, 'o', "Orc", false)
                                } else if roll < 72 {
                                    (8, 3, 2, 'A', "Giant Ant", false)
                                } else if roll < 84 {
                                    (7, 5, 1, 'M', "Goblin Mage", true)
                                } else {
                                    (9, 4, 1, 'H', "Hag", false)
                                }
                            }
                            _ => {
                                if roll < 10 {
                                    (10, 5, 2, 'u', "Ghoul", false)
                                } else if roll < 20 {
                                    (14, 5, 4, 'O', "Orc Blademaster", false)
                                } else if roll < 30 {
                                    (8, 6, 0, 'W', "Wraith", false)
                                } else if roll < 40 {
                                    (12, 6, 3, 'N', "Naga", false)
                                } else if roll < 50 {
                                    (16, 5, 3, 'T', "Troll", false)
                                } else if roll < 60 {
                                    (18, 6, 4, 'E', "Ettin", false)
                                } else if roll < 70 {
                                    (22, 5, 6, 'R', "Rock Golem", false)
                                } else if roll < 80 {
                                    (16, 7, 3, 'Y', "Minotaur", false)
                                } else if roll < 90 {
                                    (12, 7, 2, 'P', "Medusa", false)
                                } else {
                                    (10, 6, 1, 'Q', "Banshee", false)
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
        // Check thresholds from highest to lowest
        match self.overworld_kills {
            k if k >= combat.xp_diminish_quarter => (base / 4).max(1),
            k if k >= combat.xp_diminish_half => (base / 2).max(1),
            _ => base,
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
