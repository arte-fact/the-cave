use super::types::*;
use super::{Game, calc_damage};
use rand::SeedableRng;
use rand::Rng;
use rand_chacha::ChaCha8Rng;

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

    /// Stamina cost for the player's current melee attack.
    /// Based on equipped weapon weight: heavier weapons cost more stamina.
    pub fn melee_stamina_cost(&self) -> i32 {
        let c = &self.config.combat;
        let (kind, weight) = match &self.equipped_weapon {
            Some(item) if item.kind == ItemKind::Weapon => (&item.kind, item.weight),
            _ => (&ItemKind::Weapon, 0),
        };
        weapon_stamina_cost(kind, weight, c.melee_stamina_base, c.melee_stamina_weight_mult, c.ranged_stamina_base, c.ranged_stamina_weight_mult)
    }

    /// Stamina cost for the player's current ranged attack.
    /// Based on equipped ranged weapon weight.
    pub fn ranged_stamina_cost(&self) -> i32 {
        let c = &self.config.combat;
        let (kind, weight) = match &self.equipped_weapon {
            Some(item) if item.kind == ItemKind::RangedWeapon => (&item.kind, item.weight),
            _ => (&ItemKind::RangedWeapon, 0),
        };
        weapon_stamina_cost(kind, weight, c.melee_stamina_base, c.melee_stamina_weight_mult, c.ranged_stamina_base, c.ranged_stamina_weight_mult)
    }

    /// Returns true if the player has all 4 legendary set pieces equipped
    /// (helmet, armor, shield, boots all with `legendary: true`).
    pub fn has_legendary_set(&self) -> bool {
        [&self.equipped_helmet, &self.equipped_armor, &self.equipped_shield, &self.equipped_boots]
            .iter()
            .all(|slot| slot.as_ref().is_some_and(|item| item.legendary))
    }

    /// Player's total defense: base + armor + helmet + shield + boots + ring + set bonus.
    pub fn effective_defense(&self) -> i32 {
        let mut total = self.player_defense;
        for item in [&self.equipped_armor, &self.equipped_helmet, &self.equipped_shield, &self.equipped_boots].into_iter().flatten() {
            if let ItemEffect::BuffDefense(bonus) = item.effect { total += bonus; }
        }
        if let Some(ring) = &self.equipped_ring {
            if let ItemEffect::BuffDefense(bonus) = ring.effect { total += bonus; }
        }
        if self.has_legendary_set() {
            total += self.config.combat.legendary_set_defense_bonus;
        }
        total
    }

    /// Roll a dodge check: dodge_pct_per_dex % per DEX point, capped at dodge_cap_pct.
    /// Returns true if the player dodges.
    fn roll_dodge(&mut self, dodge_seed: u64, attacker_name: &str, label: &str) -> bool {
        let c = &self.config.combat;
        let dodge_chance = (self.player_dexterity * c.dodge_pct_per_dex).min(c.dodge_cap_pct) as u64;
        let dodge_roll = ChaCha8Rng::seed_from_u64(dodge_seed).gen_range(0u64..100);
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
        self.wear_armor();
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
        self.wear_armor();
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

    /// Wear the equipped weapon by 1 durability. Destroys it if durability hits 0.
    fn wear_weapon(&mut self) {
        let should_break = if let Some(ref mut w) = self.equipped_weapon {
            if w.durability > 0 {
                w.durability -= 1;
                w.durability == 0
            } else {
                false
            }
        } else {
            false
        };
        if should_break {
            let name = self.equipped_weapon.as_ref().unwrap().name;
            self.messages.push(format!("Your {name} breaks!"));
            self.equipped_weapon = None;
        }
    }

    /// Wear all equipped armor pieces by 1 durability each. Destroys any that hit 0.
    fn wear_armor(&mut self) {
        // Decrement each slot and collect names of broken items
        let mut broken: Vec<&'static str> = Vec::new();

        for slot in [&mut self.equipped_armor, &mut self.equipped_helmet,
                     &mut self.equipped_shield, &mut self.equipped_boots] {
            let should_break = if let Some(ref mut item) = slot {
                if item.durability > 0 {
                    item.durability -= 1;
                    item.durability == 0
                } else {
                    false
                }
            } else {
                false
            };
            if should_break {
                broken.push(slot.as_ref().unwrap().name);
                *slot = None;
            }
        }

        for name in broken {
            self.messages.push(format!("Your {name} breaks!"));
        }
    }

    /// End a combat turn: enemies act, survival ticks, FOV updates.
    /// Combat turns do NOT regen stamina — only walking does.
    fn end_combat_turn(&mut self) {
        if self.sprinting {
            self.sprint_skip_turn = !self.sprint_skip_turn;
            if !self.sprint_skip_turn {
                self.enemy_turn();
            }
        } else {
            self.enemy_turn();
        }
        self.tick_survival(false);
        self.update_fov();
    }

    /// Core enemy AI — dispatches to behavior-specific handlers.
    pub(super) fn enemy_turn(&mut self) {
        use crate::config::EnemyBehavior::*;
        for i in 0..self.enemies.len() {
            if self.enemies[i].hp <= 0 { continue; }
            match self.enemies[i].behavior {
                Passive     => self.ai_passive(i),
                Timid       => self.ai_timid(i),
                Territorial => self.ai_territorial(i),
                Aggressive  => self.ai_aggressive(i),
                Stalker     => self.ai_stalker(i),
            }
        }
    }

    // ── Behavior implementations ─────────────────────────────────────

    /// Passive: do nothing unless provoked, then flee.
    fn ai_passive(&mut self, i: usize) {
        if !self.enemies[i].provoked { return; }
        let dist = self.manhattan_to_player(i);
        if dist <= self.config.combat.passive_flee_range {
            self.enemy_flee(i);
        }
    }

    /// Timid: flee from player. Fight back only when cornered (adjacent + provoked).
    fn ai_timid(&mut self, i: usize) {
        let chebyshev = (self.enemies[i].x - self.player_x).abs()
            .max((self.enemies[i].y - self.player_y).abs());
        if chebyshev == 1 && self.enemies[i].provoked {
            let pdef = self.effective_defense();
            self.enemy_melee_attack(i, pdef);
            return;
        }
        let dist = self.manhattan_to_player(i);
        if dist <= self.config.combat.timid_flee_range {
            self.enemy_flee(i);
        }
    }

    /// Territorial: leashed to spawn. Engages when close or provoked.
    fn ai_territorial(&mut self, i: usize) {
        let c = &self.config.combat;
        let dist_from_spawn = (self.enemies[i].x - self.enemies[i].spawn_x).abs()
            + (self.enemies[i].y - self.enemies[i].spawn_y).abs();
        // Beyond leash → return home
        if dist_from_spawn > c.territorial_leash_range {
            let sx = self.enemies[i].spawn_x;
            let sy = self.enemies[i].spawn_y;
            self.enemy_move_toward(i, sx, sy);
            return;
        }
        let dist = self.manhattan_to_player(i);
        if self.enemies[i].provoked || dist <= c.territorial_alert_range {
            self.ai_standard_combat(i, c.enemy_chase_range);
        }
    }

    /// Aggressive: standard chase-and-attack.
    fn ai_aggressive(&mut self, i: usize) {
        let chase = self.config.combat.enemy_chase_range;
        self.ai_standard_combat(i, chase);
    }

    /// Stalker: motionless until proximity trigger, then relentless extended chase.
    fn ai_stalker(&mut self, i: usize) {
        let c = &self.config.combat;
        if !self.enemies[i].provoked {
            let dist = self.manhattan_to_player(i);
            if dist <= c.stalker_activation_range {
                self.enemies[i].provoked = true;
            } else {
                return; // motionless
            }
        }
        self.ai_standard_combat(i, c.stalker_chase_range);
    }

    /// Shared combat logic: ranged → melee → chase with configurable range.
    fn ai_standard_combat(&mut self, i: usize, chase_range: i32) {
        let px = self.player_x;
        let py = self.player_y;
        let pdef = self.effective_defense();
        let ex = self.enemies[i].x;
        let ey = self.enemies[i].y;
        let dist = (ex - px).abs() + (ey - py).abs();

        let c = &self.config.combat;
        if self.enemies[i].is_ranged
            && (c.enemy_ranged_min..=c.enemy_ranged_max).contains(&dist)
            && self.world.current_map().has_line_of_sight(ex, ey, px, py)
        {
            self.enemy_ranged_attack(i, pdef);
            return;
        }

        let chebyshev = (ex - px).abs().max((ey - py).abs());
        if chebyshev == 1 {
            self.enemy_melee_attack(i, pdef);
            return;
        }

        if dist <= chase_range {
            self.enemy_smart_chase(i, px, py, pdef);
        }
    }

    fn manhattan_to_player(&self, i: usize) -> i32 {
        (self.enemies[i].x - self.player_x).abs() + (self.enemies[i].y - self.player_y).abs()
    }

    /// Enemy ranged attack logic.
    fn enemy_ranged_attack(&mut self, i: usize, pdef: i32) {
        let ex = self.enemies[i].x;
        let ey = self.enemies[i].y;
        let raw = self.enemies[i].attack;
        let dmg = calc_damage(raw, pdef);
        let seed = self.turn as u64 * 13 + i as u64 * 7 + ex as u64 * 31 + 337;
        let roll = ChaCha8Rng::seed_from_u64(seed).gen_range(0u64..100);
        let name = self.enemies[i].name;

        if roll >= self.config.combat.enemy_ranged_miss_threshold {
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
        let mut dmg = calc_damage(raw, pdef);
        let name = self.enemies[i].name;
        let dodge_seed = self.turn as u64 * 7 + i as u64 * 13 + 997;

        if self.roll_dodge(dodge_seed, name, "attack") {
            return;
        }

        // Dragon damage reduction when player has full legendary set
        if self.enemies[i].glyph == 'D' && self.has_legendary_set() {
            dmg = (dmg * self.config.combat.legendary_dragon_damage_pct / 100).max(1);
        }

        self.apply_enemy_melee_hit(i, dmg, format!("{name} hits you for {dmg} damage."));
    }

    // ── Movement helpers ────────────────────────────────────────────

    /// Move enemy away from the player.
    fn enemy_flee(&mut self, i: usize) {
        let ex = self.enemies[i].x;
        let ey = self.enemies[i].y;
        let px = self.player_x;
        let py = self.player_y;
        let fdx = (ex - px).signum();
        let fdy = (ey - py).signum();
        let seed = self.turn as u64 * 11 + i as u64 * 3 + 127;

        let mut cands: Vec<(i32, i32)> = Vec::new();
        if fdx != 0 && fdy != 0 { cands.push((ex + fdx, ey + fdy)); }
        if ChaCha8Rng::seed_from_u64(seed).gen_range(0u64..2) == 0 {
            if fdx != 0 { cands.push((ex + fdx, ey)); }
            if fdy != 0 { cands.push((ex, ey + fdy)); }
        } else {
            if fdy != 0 { cands.push((ex, ey + fdy)); }
            if fdx != 0 { cands.push((ex + fdx, ey)); }
        }
        // Perpendicular escapes
        if fdy != 0 { cands.push((ex + 1, ey)); cands.push((ex - 1, ey)); }
        if fdx != 0 { cands.push((ex, ey + 1)); cands.push((ex, ey - 1)); }

        for (cx, cy) in cands {
            if cx == px && cy == py { continue; }
            if self.world.current_map().is_walkable(cx, cy)
                && !self.enemies.iter().any(|e| e.hp > 0 && e.x == cx && e.y == cy)
            {
                let move_dx = cx - ex;
                if move_dx < 0 { self.enemies[i].facing_left = true; }
                if move_dx > 0 { self.enemies[i].facing_left = false; }
                self.enemies[i].x = cx;
                self.enemies[i].y = cy;
                return;
            }
        }
    }

    /// Smart chase: try greedy first; if stuck and smart enemy, use A*.
    fn enemy_smart_chase(&mut self, i: usize, px: i32, py: i32, pdef: i32) {
        if self.try_greedy_chase(i, px, py, pdef) {
            return;
        }
        let c = &self.config.combat;
        let dist = (self.enemies[i].x - px).abs() + (self.enemies[i].y - py).abs();
        if dist <= c.smart_pathfind_range
            && self.config.spawn_tables.smart_enemy_names.contains(&self.enemies[i].name)
        {
            let start = (self.enemies[i].x, self.enemies[i].y);
            let goal = (px, py);
            let path = self.world.current_map().find_path(start, goal);
            if path.len() >= 2 {
                let (nx, ny) = path[1];
                if nx == px && ny == py {
                    self.enemy_melee_attack(i, pdef);
                } else if !self.enemies.iter().any(|e| e.hp > 0 && e.x == nx && e.y == ny) {
                    let move_dx = nx - self.enemies[i].x;
                    if move_dx < 0 { self.enemies[i].facing_left = true; }
                    if move_dx > 0 { self.enemies[i].facing_left = false; }
                    self.enemies[i].x = nx;
                    self.enemies[i].y = ny;
                }
            }
        }
    }

    /// Greedy signum-based chase. Returns true if the enemy moved or attacked.
    fn try_greedy_chase(&mut self, i: usize, px: i32, py: i32, pdef: i32) -> bool {
        let ex = self.enemies[i].x;
        let ey = self.enemies[i].y;
        let dx = (px - ex).signum();
        let dy = (py - ey).signum();
        let seed = self.turn as u64 * 11 + i as u64 * 3 + 127;

        let mut cands: Vec<(i32, i32)> = Vec::new();
        if dx != 0 && dy != 0 {
            let map = self.world.current_map();
            if map.is_walkable(ex + dx, ey) && map.is_walkable(ex, ey + dy) {
                cands.push((ex + dx, ey + dy));
            }
        }
        if ChaCha8Rng::seed_from_u64(seed).gen_range(0u64..2) == 0 {
            if dx != 0 { cands.push((ex + dx, ey)); }
            if dy != 0 { cands.push((ex, ey + dy)); }
        } else {
            if dy != 0 { cands.push((ex, ey + dy)); }
            if dx != 0 { cands.push((ex + dx, ey)); }
        }

        for (cx, cy) in cands {
            if cx == px && cy == py {
                self.enemy_melee_attack(i, pdef);
                return true;
            }
            if self.world.current_map().is_walkable(cx, cy)
                && !self.enemies.iter().any(|e| e.hp > 0 && e.x == cx && e.y == cy)
            {
                let move_dx = cx - self.enemies[i].x;
                if move_dx < 0 { self.enemies[i].facing_left = true; }
                if move_dx > 0 { self.enemies[i].facing_left = false; }
                self.enemies[i].x = cx;
                self.enemies[i].y = cy;
                return true;
            }
        }
        false
    }

    /// Greedy move toward an arbitrary target (for territorial return-to-spawn).
    fn enemy_move_toward(&mut self, i: usize, tx: i32, ty: i32) {
        let ex = self.enemies[i].x;
        let ey = self.enemies[i].y;
        let dx = (tx - ex).signum();
        let dy = (ty - ey).signum();
        let seed = self.turn as u64 * 11 + i as u64 * 3 + 127;

        let mut cands: Vec<(i32, i32)> = Vec::new();
        if dx != 0 && dy != 0 {
            let map = self.world.current_map();
            if map.is_walkable(ex + dx, ey) && map.is_walkable(ex, ey + dy) {
                cands.push((ex + dx, ey + dy));
            }
        }
        if ChaCha8Rng::seed_from_u64(seed).gen_range(0u64..2) == 0 {
            if dx != 0 { cands.push((ex + dx, ey)); }
            if dy != 0 { cands.push((ex, ey + dy)); }
        } else {
            if dy != 0 { cands.push((ex, ey + dy)); }
            if dx != 0 { cands.push((ex + dx, ey)); }
        }

        let px = self.player_x;
        let py = self.player_y;
        for (cx, cy) in cands {
            if cx == px && cy == py { continue; }
            if self.world.current_map().is_walkable(cx, cy)
                && !self.enemies.iter().any(|e| e.hp > 0 && e.x == cx && e.y == cy)
            {
                let move_dx = cx - ex;
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

        let cost = self.melee_stamina_cost();
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
        self.enemies[idx].provoked = true;
        let name = self.enemies[idx].name;

        // Wear weapon on every melee attack
        self.wear_weapon();

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
        let it = &self.config.item_tables;
        if let Some(name) = self.equipped_weapon.as_ref().map(|w| w.name) {
            for &(weapon_name, range) in it.ranged_base_ranges {
                if weapon_name == name {
                    return range;
                }
            }
        }
        it.ranged_default_range
    }

    /// Max range for the equipped ranged weapon, factoring in dexterity.
    pub fn ranged_max_range(&self) -> i32 {
        self.ranged_weapon_base_range() + self.player_dexterity / self.config.combat.ranged_range_dex_divisor
    }

    /// Hit chance (0–cap) for a ranged attack at the given distance.
    /// Higher dexterity = better accuracy. Chance drops with distance.
    pub fn ranged_hit_chance(&self, distance: i32) -> i32 {
        let c = &self.config.combat;
        let max_range = self.ranged_max_range();
        if distance <= 0 || distance > max_range {
            return 0;
        }
        let base = (c.ranged_hit_ceiling - distance * c.ranged_hit_falloff / max_range).max(c.ranged_hit_floor);
        (base + self.player_dexterity * c.ranged_accuracy_per_dex).min(c.ranged_hit_cap)
    }

    /// Ranged damage: base attack + distance bonus + DEX bonus, reduced by enemy defense.
    fn ranged_damage(&self, enemy_defense: i32, distance: i32) -> i32 {
        let c = &self.config.combat;
        let atk = self.effective_attack() + distance / c.ranged_dist_bonus_divisor + self.player_dexterity / c.ranged_dex_bonus_divisor;
        calc_damage(atk, enemy_defense)
    }

    /// Fire the equipped ranged weapon at target tile (tx, ty).
    /// Consumes a turn: enemies move, survival ticks, FOV updates.
    pub fn ranged_attack(&mut self, tx: i32, ty: i32) -> TurnResult {
        if !self.alive || self.won || !self.has_ranged_weapon() {
            return TurnResult::Blocked;
        }

        let cost = self.ranged_stamina_cost();
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
        let roll = ChaCha8Rng::seed_from_u64(seed).gen_range(0i32..100);
        let name = self.enemies[idx].name;

        // Wear ranged weapon on every shot (hit or miss)
        self.wear_weapon();

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
            self.enemies[idx].provoked = true;
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
