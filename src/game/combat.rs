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

    /// Enemy ranged attack logic.
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

        let cost = self.config.combat.melee_stamina_cost;
        if self.stamina < cost {
            self.messages.push("Too exhausted to attack! No stamina.".into());
            return TurnResult::Blocked;
        }

        let dx = tx - self.player_x;
        let dy = ty - self.player_y;
        if dx < 0 { self.player_facing_left = true; }
        if dx > 0 { self.player_facing_left = false; }

        let Some(idx) = self.enemies.iter().position(|e| e.x == tx && e.y == ty && e.hp > 0) else {
            return TurnResult::Blocked;
        };

        self.stamina -= cost;

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

        let cost = self.config.combat.ranged_stamina_cost;
        if self.stamina < cost {
            self.messages.push("Too exhausted to shoot! No stamina.".into());
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

        self.stamina -= cost;

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
}
