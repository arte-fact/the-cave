use crate::world::Location;
use super::Game;

impl Game {
    /// Sprint cost per move: 2× the stamina regen rate.
    pub fn sprint_cost(&self) -> i32 {
        self.config.survival.stamina_regen * 2
    }

    pub fn toggle_sprint(&mut self) {
        if self.sprinting {
            self.sprinting = false;
            self.sprint_skip_turn = false;
            self.messages.push("Sprint off.".into());
        } else if self.stamina >= self.sprint_cost() {
            self.sprinting = true;
            self.sprint_skip_turn = false;
            self.messages.push("Sprint on!".into());
        } else {
            self.messages.push("Too exhausted to sprint.".into());
        }
    }

    /// Hunger drain interval: configurable per location type.
    fn hunger_interval(&self) -> u32 {
        match &self.world.location {
            Location::Overworld => self.config.survival.hunger_interval_overworld,
            Location::Dungeon { index, level } => {
                let total = self.world.dungeons[*index].levels.len();
                let is_cave = total == 4 && *level == 3;
                if is_cave {
                    self.config.survival.hunger_interval_cave
                } else {
                    self.config.survival.hunger_interval_dungeon
                }
            }
        }
    }

    /// Called each turn. Handles stamina regen and hunger.
    /// `regen_stamina`: true on movement turns (walking regens stamina),
    /// false on combat turns (attacking should cost stamina without immediate regen).
    pub(crate) fn tick_survival(&mut self, regen_stamina: bool) {
        self.turn += 1;
        let surv = &self.config.survival;

        // Stamina: sprinting drains 2× regen rate, walking regenerates (combat does not).
        if self.sprinting {
            self.stamina -= self.sprint_cost();
            if self.stamina <= 0 {
                self.stamina = 0;
                self.sprinting = false;
                self.sprint_skip_turn = false;
                self.messages.push("Exhausted! Sprint disabled.".into());
            }
        } else if regen_stamina {
            self.stamina = (self.stamina + surv.stamina_regen).min(self.max_stamina);
        }

        // Hunger: drain rate scales with depth
        let interval = self.hunger_interval();
        if self.turn.is_multiple_of(interval) {
            self.hunger -= surv.hunger_drain;
            if self.hunger < 0 { self.hunger = 0; }
        }

        // Health regen: when well-fed and injured, heal 1 HP per interval, costs food
        if self.turn.is_multiple_of(interval)
            && self.hunger > surv.regen_hunger_threshold
            && self.player_hp < self.player_max_hp
        {
            self.player_hp += 1;
            self.hunger -= surv.regen_hunger_cost;
            if self.hunger < 0 { self.hunger = 0; }
        }

        // Starvation damage
        if self.hunger == 0 {
            self.player_hp -= surv.starvation_damage;
            if self.turn.is_multiple_of(5) {
                self.messages.push("You are starving!".into());
            }
            if self.player_hp <= 0 {
                self.alive = false;
                self.messages.push("You starved to death.".into());
            }
        }
    }
}
