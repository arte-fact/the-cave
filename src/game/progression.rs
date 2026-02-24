use crate::world::Location;
use super::types::*;
use super::Game;

impl Game {
    /// Apply diminishing XP returns for overworld kills. Dungeon kills unaffected.
    fn xp_with_diminishing(&self, enemy_name: &str) -> u32 {
        let base = crate::config::xp_for_enemy(enemy_name);
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
        let prog = &self.config.progression;
        let sp = prog.skill_points_per_level;
        let hp_per = prog.hp_per_level;
        let heal_pct = prog.levelup_heal_missing_pct;
        let heal_min = prog.levelup_heal_min;
        while self.player_xp >= self.xp_to_next_level() {
            self.player_xp -= self.xp_to_next_level();
            self.player_level += 1;
            self.skill_points += sp;
            // Small base HP bump on level up + partial heal
            self.player_max_hp += hp_per;
            let missing = self.player_max_hp - self.player_hp;
            self.player_hp += missing * heal_pct / 100 + heal_min;
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
        let prog = &self.config.progression;
        let combat = &self.config.combat;
        match skill {
            SkillKind::Strength => {
                self.strength += 1;
                self.messages.push(format!("Strength increased to {}.", self.strength));
            }
            SkillKind::Vitality => {
                let hp_gain = prog.vitality_hp_per_point;
                self.vitality += 1;
                self.player_max_hp += hp_gain;
                self.player_hp = (self.player_hp + hp_gain).min(self.player_max_hp);
                self.messages.push(format!("Vitality increased to {}. Max HP +{}.", self.vitality, hp_gain));
            }
            SkillKind::Dexterity => {
                self.player_dexterity += 1;
                let dodge = (self.player_dexterity * combat.dodge_pct_per_dex).min(combat.dodge_cap_pct);
                self.messages.push(format!("Dexterity increased to {}. Dodge {}%.", self.player_dexterity, dodge));
            }
            SkillKind::Stamina => {
                let stam_gain = prog.stamina_per_point;
                self.max_stamina += stam_gain;
                self.stamina = (self.stamina + stam_gain).min(self.max_stamina);
                self.messages.push(format!("Max stamina increased to {}.", self.max_stamina));
            }
        }
        true
    }

    /// Handle kill rewards: XP, floating text, meat drop, dragon check.
    /// Returns TurnResult::Won if the dragon was killed, otherwise None.
    pub(super) fn handle_kill(&mut self, enemy_idx: usize) -> Option<TurnResult> {
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
        if super::spawning::is_rare_monster(name, self.config.spawn_tables.rare_monster_names) {
            let mut loot_rng = (ex as u64).wrapping_mul(31).wrapping_add(ey as u64).wrapping_mul(6364136223846793005);
            if let Some(loot) = super::items::monster_loot_drop(name, &mut loot_rng, self.config.spawn_tables.monster_loot_tiers) {
                let loot_name = loot.name;
                self.ground_items.push(GroundItem { x: ex, y: ey, item: loot });
                self.messages.push(format!("The {name} dropped {loot_name}!"));
            }
        }
        if self.enemies[enemy_idx].glyph == 'D' {
            self.won = true;
            self.messages.push("You conquered the cave!".into());
            return Some(TurnResult::Won);
        }
        None
    }
}
