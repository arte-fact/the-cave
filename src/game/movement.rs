use crate::map::Tile;
use crate::world::Location;
use super::types::*;
use super::Game;

impl Game {
    pub fn move_player(&mut self, dx: i32, dy: i32) -> TurnResult {
        if !self.alive || self.won {
            return TurnResult::Blocked;
        }

        // Update facing direction on horizontal movement
        if dx < 0 { self.player_facing_left = true; }
        if dx > 0 { self.player_facing_left = false; }

        let nx = self.player_x + dx;
        let ny = self.player_y + dy;

        // Enemies block movement — attack must be explicit (tap the enemy)
        if self.enemies.iter().any(|e| e.x == nx && e.y == ny && e.hp > 0) {
            return TurnResult::Blocked;
        }

        // Corner-cutting prevention: diagonal moves require both adjacent cardinal tiles walkable
        if dx != 0 && dy != 0 {
            let map = self.world.current_map();
            if !map.is_walkable(self.player_x + dx, self.player_y)
                || !map.is_walkable(self.player_x, self.player_y + dy)
            {
                return TurnResult::Blocked;
            }
        }

        if !self.world.current_map().is_walkable(nx, ny) {
            return TurnResult::Blocked;
        }

        self.player_x = nx;
        self.player_y = ny;

        // Auto-pickup items on the tile we moved to
        self.pickup_items_explicit();

        // Check for map transitions
        let tile = self.world.current_map().get(nx, ny);
        if self.try_transition(tile, nx, ny) {
            return TurnResult::MapChanged;
        }

        // Enemies take a turn (when sprinting, enemies only act every other move)
        if self.sprinting {
            self.sprint_skip_turn = !self.sprint_skip_turn;
            if !self.sprint_skip_turn {
                self.enemy_turn();
            }
        } else {
            self.enemy_turn();
        }

        // Survival tick: stamina regen, hunger
        self.tick_survival(true);

        // Update fog of war
        self.update_fov();

        TurnResult::Moved
    }

    /// Handle map transitions based on the tile the player stepped on.
    /// Returns true if a transition occurred.
    pub(crate) fn try_transition(&mut self, tile: Tile, x: i32, y: i32) -> bool {
        match tile {
            Tile::DungeonEntrance if self.world.location == Location::Overworld => {
                let Some(di) = self.world.dungeon_at(x, y) else { return false };
                self.enter_dungeon(di);
                true
            }
            Tile::StairsDown => {
                let Location::Dungeon { index, level } = self.world.location else { return false };
                if level + 1 >= self.world.dungeons[index].levels.len() { return false; }
                self.descend(index, level);
                true
            }
            Tile::StairsUp => {
                let Location::Dungeon { index, level } = self.world.location else { return false };
                if level == 0 {
                    self.exit_dungeon();
                } else {
                    self.ascend(index, level);
                }
                true
            }
            _ => false,
        }
    }

    pub(crate) fn enter_dungeon(&mut self, dungeon_index: usize) {
        self.world.saved_overworld_pos = (self.player_x, self.player_y);
        self.world.saved_overworld_enemies = self.enemies.clone();
        self.world.saved_overworld_items = std::mem::take(&mut self.ground_items);

        self.world.location = Location::Dungeon { index: dungeon_index, level: 0 };
        let map = self.world.current_map();
        let (sx, sy) = map.find_tile(Tile::StairsUp).unwrap_or_else(|| map.find_spawn());
        self.player_x = sx;
        self.player_y = sy;

        self.enemies.clear();
        self.spawn_dungeon_enemies(dungeon_index, 0);
        self.spawn_dungeon_items(dungeon_index, 0);
        self.messages.push("You descend into the dungeon.".into());
        self.update_fov();
    }

    pub(crate) fn exit_dungeon(&mut self) {
        let (ox, oy) = self.world.saved_overworld_pos;
        self.player_x = ox;
        self.player_y = oy;
        self.enemies = std::mem::take(&mut self.world.saved_overworld_enemies);
        self.ground_items = std::mem::take(&mut self.world.saved_overworld_items);
        self.world.location = Location::Overworld;
        self.messages.push("You return to the overworld.".into());
        self.update_fov();
    }

    pub(crate) fn descend(&mut self, dungeon_index: usize, current_level: usize) {
        self.world.location = Location::Dungeon { index: dungeon_index, level: current_level + 1 };
        let map = self.world.current_map();
        let (sx, sy) = map.find_tile(Tile::StairsUp).unwrap_or_else(|| map.find_spawn());
        self.player_x = sx;
        self.player_y = sy;
        self.enemies.clear();
        self.ground_items.clear();
        self.spawn_dungeon_enemies(dungeon_index, current_level + 1);
        self.spawn_dungeon_items(dungeon_index, current_level + 1);
        self.messages.push(format!("You descend to level {}.", current_level + 2));
        self.update_fov();
    }

    pub(crate) fn ascend(&mut self, dungeon_index: usize, current_level: usize) {
        self.world.location = Location::Dungeon { index: dungeon_index, level: current_level - 1 };
        let map = self.world.current_map();
        let (sx, sy) = map.find_tile(Tile::StairsDown).unwrap_or_else(|| map.find_spawn());
        self.player_x = sx;
        self.player_y = sy;
        self.enemies.clear();
        self.ground_items.clear();
        self.spawn_dungeon_enemies(dungeon_index, current_level - 1);
        self.spawn_dungeon_items(dungeon_index, current_level - 1);
        self.messages.push(format!("You ascend to level {}.", current_level));
        self.update_fov();
    }
}
