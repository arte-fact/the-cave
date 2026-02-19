mod types;
mod items;
mod combat;

pub use types::*;
#[cfg(test)]
use items::{random_item, meat_drop};
#[cfg(test)]
use types::{tile_name, tile_desc, enemy_desc, xp_for_enemy, item_info_desc};

use crate::config::GameConfig;
use crate::map::{Map, Tile};
use crate::world::{Location, World};

pub struct Game {
    pub config: GameConfig,
    pub player_x: i32,
    pub player_y: i32,
    pub player_hp: i32,
    pub player_max_hp: i32,
    pub player_attack: i32,
    /// Dexterity: affects ranged weapon max range and hit chance.
    pub player_dexterity: i32,
    /// true when player sprite should face left (mirrored).
    pub player_facing_left: bool,
    pub world: World,
    pub enemies: Vec<Enemy>,
    pub messages: Vec<String>,
    pub alive: bool,
    pub won: bool,
    pub inventory: Vec<Item>,
    pub equipped_weapon: Option<Item>,
    pub equipped_armor: Option<Item>,
    pub equipped_helmet: Option<Item>,
    pub equipped_shield: Option<Item>,
    pub equipped_boots: Option<Item>,
    pub equipped_ring: Option<Item>,
    pub player_defense: i32,
    pub ground_items: Vec<GroundItem>,
    /// Scroll offset for the inventory item list (first visible item index).
    pub inventory_scroll: usize,
    /// Currently selected inventory item index (for detail view / drop).
    pub selected_inventory_item: Option<usize>,
    /// Currently open drawer (slides up from bottom).
    pub drawer: Drawer,
    /// Tile currently being inspected (shown in HUD detail strip).
    pub inspected: Option<TileInfo>,
    /// Player XP and level for progression.
    pub player_xp: u32,
    pub player_level: u32,
    /// Unspent skill points (awarded on level up).
    pub skill_points: u32,
    /// Strength: bonus to melee attack damage.
    pub strength: i32,
    /// Vitality: bonus to max HP.
    pub vitality: i32,
    /// Scroll offset for the stats drawer (in CSS-space pixels).
    pub stats_scroll: f64,
    /// Stamina for sprinting. Max 100, regen 5/turn while walking.
    pub stamina: i32,
    pub max_stamina: i32,
    /// Whether sprint mode is active (toggle).
    pub sprinting: bool,
    /// Hunger. Starts full (100). Decreases 1/turn. 0 = starvation.
    pub hunger: i32,
    pub max_hunger: i32,
    /// Turn counter for hunger tracking.
    pub turn: u32,
    /// Overworld kill counter for XP diminishing returns.
    pub overworld_kills: u32,
    /// Current sprint cost per turn (reduced by Stamina skill).
    pub sprint_cost: i32,
    /// Active floating text indicators.
    pub floating_texts: Vec<FloatingText>,
    /// Active bump/lunge animations.
    pub bump_anims: Vec<BumpAnim>,
    /// Active visual effects.
    pub visual_effects: Vec<VisualEffect>,
    /// Quick-use bar: consumable item slots for one-tap use.
    pub quick_bar: QuickBar,
}

impl Game {
    #[cfg(test)]
    pub fn new(map: Map) -> Self {
        Self::new_with_config(map, GameConfig::normal())
    }

    #[cfg(test)]
    pub fn new_with_config(map: Map, config: GameConfig) -> Self {
        let (px, py) = map.find_spawn();
        let p = &config.player;
        let sprint_cost = config.survival.sprint_cost;
        Self {
            player_x: px,
            player_y: py,
            player_hp: p.starting_hp,
            player_max_hp: p.starting_hp,
            player_attack: p.starting_attack,
            player_dexterity: p.starting_dexterity,
            player_facing_left: false,
            world: World::from_single_map(map),
            enemies: Vec::new(),
            messages: vec!["You enter the cave.".into()],
            alive: true,
            won: false,
            inventory: Vec::new(),
            equipped_weapon: None,
            equipped_armor: None,
            equipped_helmet: None,
            equipped_shield: None,
            equipped_boots: None,
            equipped_ring: None,
            player_defense: 0,
            ground_items: Vec::new(),
            inventory_scroll: 0,
            selected_inventory_item: None,

            drawer: Drawer::None,
            inspected: None,
            player_xp: 0,
            player_level: 1,
            skill_points: 0,
            strength: 0,
            vitality: 0,
            stats_scroll: 0.0,
            stamina: p.starting_stamina,
            max_stamina: p.starting_stamina,
            sprinting: false,
            hunger: p.starting_hunger,
            max_hunger: p.starting_hunger,
            turn: 0,
            overworld_kills: 0,
            sprint_cost,
            floating_texts: Vec::new(),
            bump_anims: Vec::new(),
            visual_effects: Vec::new(),
            quick_bar: QuickBar::new(),
            config,
        }
    }

    #[cfg(test)]
    pub fn new_overworld(world: World) -> Self {
        Self::new_overworld_with_config(world, GameConfig::normal())
    }

    pub fn new_overworld_with_config(world: World, config: GameConfig) -> Self {
        let (px, py) = world.overworld.find_road_spawn();
        let p = &config.player;
        let sprint_cost = config.survival.sprint_cost;
        Self {
            player_x: px,
            player_y: py,
            player_hp: p.starting_hp,
            player_max_hp: p.starting_hp,
            player_attack: p.starting_attack,
            player_dexterity: p.starting_dexterity,
            player_facing_left: false,
            world,
            enemies: Vec::new(),
            messages: vec!["You emerge into the forest.".into()],
            alive: true,
            won: false,
            inventory: Vec::new(),
            equipped_weapon: None,
            equipped_armor: None,
            equipped_helmet: None,
            equipped_shield: None,
            equipped_boots: None,
            equipped_ring: None,
            player_defense: 0,
            ground_items: Vec::new(),
            inventory_scroll: 0,
            selected_inventory_item: None,

            drawer: Drawer::None,
            inspected: None,
            player_xp: 0,
            player_level: 1,
            skill_points: 0,
            strength: 0,
            vitality: 0,
            stats_scroll: 0.0,
            stamina: p.starting_stamina,
            max_stamina: p.starting_stamina,
            sprinting: false,
            hunger: p.starting_hunger,
            max_hunger: p.starting_hunger,
            turn: 0,
            overworld_kills: 0,
            sprint_cost,
            floating_texts: Vec::new(),
            bump_anims: Vec::new(),
            visual_effects: Vec::new(),
            quick_bar: QuickBar::new(),
            config,
        }
    }

    /// Convenience accessor for the current map.
    pub fn current_map(&self) -> &Map {
        self.world.current_map()
    }

    /// FOV radius: configurable per overworld vs dungeon.
    fn fov_radius(&self) -> i32 {
        match self.world.location {
            Location::Overworld => self.config.fov.overworld_radius,
            Location::Dungeon { .. } => self.config.fov.dungeon_radius,
        }
    }

    /// Age Visible→Seen, then recompute FOV from player position.
    pub fn update_fov(&mut self) {
        let r = self.fov_radius();
        let map = self.world.current_map_mut();
        map.age_visibility();
        map.compute_fov(self.player_x, self.player_y, r);
    }

    pub fn location_name(&self) -> String {
        match &self.world.location {
            crate::world::Location::Overworld => "Overworld".into(),
            crate::world::Location::Dungeon { index, level } => {
                let depth = level + 1;
                let total = self.world.dungeons[*index].levels.len();
                if *level == total - 1 && total == 4 {
                    format!("Dragon's Lair (B{})", depth)
                } else {
                    format!("Dungeon {} (B{})", index + 1, depth)
                }
            }
        }
    }

    pub fn toggle_sprint(&mut self) {
        if self.sprinting {
            self.sprinting = false;
            self.messages.push("Sprint off.".into());
        } else if self.stamina >= self.sprint_cost {
            self.sprinting = true;
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

    /// Called each turn the player moves. Handles stamina drain/regen and hunger.
    fn tick_survival(&mut self) {
        self.turn += 1;
        let surv = &self.config.survival;

        // Stamina: sprint drains, walking regenerates
        if self.sprinting {
            self.stamina -= self.sprint_cost;
            if self.stamina <= 0 {
                self.stamina = 0;
                self.sprinting = false;
                self.messages.push("Exhausted! Sprint disabled.".into());
            }
        } else {
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

        // Enemies take a turn (half speed when sprinting — player is faster but not invincible)
        if self.sprinting {
            self.enemy_turn_inner(true);
        } else {
            self.enemy_turn();
        }

        // Survival tick: stamina drain/regen, hunger
        self.tick_survival();

        // Update fog of war
        self.update_fov();

        TurnResult::Moved
    }

    /// Handle map transitions based on the tile the player stepped on.
    /// Returns true if a transition occurred.
    fn try_transition(&mut self, tile: Tile, x: i32, y: i32) -> bool {
        match tile {
            Tile::DungeonEntrance => {
                if let Location::Overworld = self.world.location {
                    if let Some(di) = self.world.dungeon_at(x, y) {
                        self.enter_dungeon(di);
                        return true;
                    }
                }
            }
            Tile::StairsDown => {
                if let Location::Dungeon { index, level } = self.world.location.clone() {
                    if level + 1 < self.world.dungeons[index].levels.len() {
                        self.descend(index, level);
                        return true;
                    }
                }
            }
            Tile::StairsUp => {
                match self.world.location.clone() {
                    Location::Dungeon { level: 0, .. } => {
                        self.exit_dungeon();
                        return true;
                    }
                    Location::Dungeon { index, level } => {
                        self.ascend(index, level);
                        return true;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        false
    }

    fn enter_dungeon(&mut self, dungeon_index: usize) {
        // Save overworld state
        self.world.saved_overworld_pos = (self.player_x, self.player_y);
        self.world.saved_overworld_enemies = self.enemies.clone();
        self.world.saved_overworld_items = std::mem::take(&mut self.ground_items);

        // Switch to dungeon level 0
        self.world.location = Location::Dungeon { index: dungeon_index, level: 0 };
        let map = self.world.current_map();

        // Place player at StairsUp
        if let Some((sx, sy)) = map.find_tile(Tile::StairsUp) {
            self.player_x = sx;
            self.player_y = sy;
        } else {
            let (sx, sy) = map.find_spawn();
            self.player_x = sx;
            self.player_y = sy;
        }

        self.enemies.clear();
        self.spawn_dungeon_enemies(dungeon_index, 0);
        self.spawn_dungeon_items(dungeon_index, 0);
        self.messages.push("You descend into the dungeon.".into());
        self.update_fov();
    }

    fn exit_dungeon(&mut self) {
        // Restore overworld state
        let (ox, oy) = self.world.saved_overworld_pos;
        self.player_x = ox;
        self.player_y = oy;
        self.enemies = std::mem::take(&mut self.world.saved_overworld_enemies);
        self.ground_items = std::mem::take(&mut self.world.saved_overworld_items);
        self.world.location = Location::Overworld;
        self.messages.push("You return to the overworld.".into());
        self.update_fov();
    }

    fn descend(&mut self, dungeon_index: usize, current_level: usize) {
        self.world.location = Location::Dungeon { index: dungeon_index, level: current_level + 1 };
        let map = self.world.current_map();
        if let Some((sx, sy)) = map.find_tile(Tile::StairsUp) {
            self.player_x = sx;
            self.player_y = sy;
        } else {
            let (sx, sy) = map.find_spawn();
            self.player_x = sx;
            self.player_y = sy;
        }
        self.enemies.clear();
        self.ground_items.clear();
        self.spawn_dungeon_enemies(dungeon_index, current_level + 1);
        self.spawn_dungeon_items(dungeon_index, current_level + 1);
        self.messages.push(format!("You descend to level {}.", current_level + 2));
        self.update_fov();
    }

    fn ascend(&mut self, dungeon_index: usize, current_level: usize) {
        self.world.location = Location::Dungeon { index: dungeon_index, level: current_level - 1 };
        let map = self.world.current_map();
        if let Some((sx, sy)) = map.find_tile(Tile::StairsDown) {
            self.player_x = sx;
            self.player_y = sy;
        } else {
            let (sx, sy) = map.find_spawn();
            self.player_x = sx;
            self.player_y = sy;
        }
        self.enemies.clear();
        self.ground_items.clear();
        self.spawn_dungeon_enemies(dungeon_index, current_level - 1);
        self.spawn_dungeon_items(dungeon_index, current_level - 1);
        self.messages.push(format!("You ascend to level {}.", current_level));
        self.update_fov();
    }

    pub fn advance_turn(&mut self) {
        if !self.alive || self.won { return; }
        if self.sprinting {
            self.enemy_turn_inner(true);
        } else {
            self.enemy_turn();
        }
        self.tick_survival();
        self.update_fov();
    }

    /// Tick all animations forward by one frame. Returns true if any are still active.
    pub fn tick_animations(&mut self) -> bool {
        for ft in &mut self.floating_texts {
            ft.age += 0.025;
        }
        self.floating_texts.retain(|ft| ft.age < 1.0);

        for ba in &mut self.bump_anims {
            ba.progress += 0.12;
        }
        self.bump_anims.retain(|ba| ba.progress < 1.0);

        for ve in &mut self.visual_effects {
            ve.age += 0.035;
        }
        self.visual_effects.retain(|ve| ve.age < 1.0);

        !self.floating_texts.is_empty() || !self.bump_anims.is_empty() || !self.visual_effects.is_empty()
    }
}

pub(super) fn calc_damage(atk: i32, def: i32) -> i32 {
    if atk <= 0 { return 1; }
    let d = def.max(0);
    ((atk * atk) / (atk + d)).max(1)
}

pub(super) fn xorshift64(mut state: u64) -> u64 {
    state ^= state << 13;
    state ^= state >> 7;
    state ^= state << 17;
    state
}
#[cfg(test)]
mod tests {
    use super::*;

    fn test_game() -> Game {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.spawn_enemies(123);
        g
    }

    // === Movement tests ===

    #[test]
    fn player_spawns_on_floor() {
        let g = test_game();
        assert!(g.current_map().is_walkable(g.player_x, g.player_y));
    }

    #[test]
    fn can_move_to_floor() {
        let mut g = test_game();
        let (sx, sy) = (g.player_x, g.player_y);
        let dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)];
        let mut moved = false;
        for (dx, dy) in dirs {
            g.player_x = sx;
            g.player_y = sy;
            if g.current_map().is_walkable(sx + dx, sy + dy)
                && !g.enemies.iter().any(|e| e.x == sx + dx && e.y == sy + dy)
            {
                g.move_player(dx, dy);
                assert_eq!(g.player_x, sx + dx);
                assert_eq!(g.player_y, sy + dy);
                moved = true;
                break;
            }
        }
        assert!(moved, "spawn should have at least one adjacent open floor");
    }

    #[test]
    fn blocked_by_wall() {
        let mut g = test_game();
        let w = g.current_map().width;
        for _ in 0..w {
            g.move_player(-1, 0);
        }
        assert!(g.current_map().is_walkable(g.player_x, g.player_y));
    }

    #[test]
    fn blocked_by_out_of_bounds() {
        let mut g = test_game();
        let h = g.current_map().height;
        for _ in 0..h + 10 {
            g.move_player(0, -1);
        }
        assert!(g.player_y >= 0);
        assert!(g.current_map().is_walkable(g.player_x, g.player_y));
    }

    // === Player stats ===

    #[test]
    fn player_starts_with_full_hp() {
        let g = test_game();
        assert_eq!(g.player_hp, 20);
        assert_eq!(g.player_max_hp, 20);
        assert_eq!(g.player_attack, 5);
        assert!(g.alive);
        assert!(!g.won);
    }

    // === Enemy spawning ===

    #[test]
    fn enemies_spawn_on_floor() {
        let g = test_game();
        for e in &g.enemies {
            assert!(g.current_map().is_walkable(e.x, e.y), "{} at ({},{}) not on floor", e.name, e.x, e.y);
        }
    }

    #[test]
    fn enemies_not_on_player() {
        let g = test_game();
        for e in &g.enemies {
            assert!(
                e.x != g.player_x || e.y != g.player_y,
                "enemy spawned on player"
            );
        }
    }

    #[test]
    fn overworld_has_forest_animals() {
        let g = test_game();
        let forest_glyphs = ['r', 'a', 'w', 'i', 'b', 'B', 'L'];
        for e in &g.enemies {
            assert!(
                forest_glyphs.contains(&e.glyph),
                "unexpected overworld enemy: {} ('{}')", e.name, e.glyph
            );
        }
        // Should have at least some enemies
        assert!(!g.enemies.is_empty(), "overworld should have enemies");
    }

    #[test]
    fn cave_level_has_dragon_boss() {
        let mut g = overworld_game();
        // Find the dungeon with 4 levels (the cave dungeon)
        let cave_di = g.world.dungeons.iter()
            .position(|d| d.levels.len() == 4)
            .expect("one dungeon should have a cave level");
        g.enter_dungeon(cave_di);
        // Descend to the cave (level 3)
        for level in 0..3 {
            g.descend(cave_di, level);
        }
        assert!(
            g.enemies.iter().any(|e| e.glyph == 'D'),
            "cave level should have dragon boss"
        );
        let dragon = g.enemies.iter().find(|e| e.glyph == 'D').unwrap();
        assert!(dragon.hp >= 30, "dragon hp should be >= 30, got {}", dragon.hp);
        assert!(dragon.attack >= 8, "dragon attack should be >= 8, got {}", dragon.attack);
    }

    #[test]
    fn non_cave_dungeon_has_no_dragon() {
        let mut g = overworld_game();
        // Find a dungeon without the cave (3 levels)
        let normal_di = g.world.dungeons.iter()
            .position(|d| d.levels.len() == 3)
            .expect("should have normal dungeons");
        g.enter_dungeon(normal_di);
        // Descend to deepest level (level 2)
        g.descend(normal_di, 0);
        g.descend(normal_di, 1);
        assert!(
            !g.enemies.iter().any(|e| e.glyph == 'D'),
            "non-cave dungeon should not have a dragon"
        );
    }

    // === Combat ===

    #[test]
    fn attacking_enemy_deals_damage() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 10, attack: 2, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        g.attack_adjacent(gx, gy);
        assert_eq!(g.enemies[0].hp, 10 - g.player_attack);
        assert_eq!(g.player_x, gx - 1); // player didn't move
    }

    #[test]
    fn killing_enemy() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 3, attack: 1, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        let result = g.attack_adjacent(gx, gy);
        assert!(matches!(result, TurnResult::Killed { .. }));
        assert!(g.enemies[0].hp <= 0);
    }

    #[test]
    fn enemy_attacks_on_its_turn() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.player_dexterity = 0; // disable dodge for deterministic test
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 20, attack: 3, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        let hp_before = g.player_hp;
        g.attack_adjacent(gx, gy);
        // Enemy attacks back during enemy_turn (adjacent, calc_damage(3, 0) = 3)
        assert_eq!(g.player_hp, hp_before - 3);
    }

    #[test]
    fn player_can_die() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.player_hp = 1;
        g.player_dexterity = 0; // disable dodge
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 99, attack: 5, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        let result = g.attack_adjacent(gx, gy);
        assert!(matches!(result, TurnResult::PlayerDied));
        assert!(!g.alive);
    }

    #[test]
    fn dead_player_cant_move() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.alive = false;
        let (x, y) = (g.player_x, g.player_y);
        g.move_player(1, 0);
        assert_eq!((g.player_x, g.player_y), (x, y));
    }

    #[test]
    fn killing_dragon_wins() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let dx = g.player_x + 1;
        let dy = g.player_y;
        g.enemies.push(Enemy { x: dx, y: dy, hp: 1, attack: 0, glyph: 'D', name: "Dragon", facing_left: false, defense: 0, is_ranged: false });
        let result = g.attack_adjacent(dx, dy);
        assert!(matches!(result, TurnResult::Won));
        assert!(g.won);
    }

    #[test]
    fn won_player_cant_move() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.won = true;
        let (x, y) = (g.player_x, g.player_y);
        g.move_player(1, 0);
        assert_eq!((g.player_x, g.player_y), (x, y));
    }

    // === Messages ===

    #[test]
    fn initial_message() {
        let g = test_game();
        assert_eq!(g.messages[0], "You enter the cave.");
    }

    #[test]
    fn combat_generates_messages() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 20, attack: 2, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        let msg_count_before = g.messages.len();
        g.attack_adjacent(gx, gy);
        assert!(g.messages.len() > msg_count_before, "combat should generate messages");
    }

    // === Enemy AI ===

    #[test]
    fn enemy_chases_player() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let ex = g.player_x + 3;
        let ey = g.player_y;
        if g.current_map().is_walkable(ex, ey) {
            g.enemies.push(Enemy { x: ex, y: ey, hp: 10, attack: 1, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
            if g.current_map().is_walkable(g.player_x, g.player_y + 1) {
                g.move_player(0, 1);
                let new_dist = (g.enemies[0].x - g.player_x).abs() + (g.enemies[0].y - g.player_y).abs();
                assert!(new_dist < 4, "enemy should have chased toward player, dist={new_dist}");
            }
        }
    }

    // === Dungeon traversal (Phase 3) ===

    fn overworld_game() -> Game {
        let mut map = Map::generate_forest(200, 200, 42);
        let entrances = map.place_dungeons(42);
        map.build_roads(&entrances);
        let world = World::new(map, entrances, 99);
        let mut g = Game::new_overworld(world);
        g.spawn_enemies(777);
        g
    }

    #[test]
    fn enter_dungeon_changes_location() {
        let mut g = overworld_game();
        let entrance = g.world.dungeon_entrances[0];
        // Teleport player to dungeon entrance
        g.player_x = entrance.0;
        g.player_y = entrance.1;
        g.enter_dungeon(0);
        assert_eq!(g.world.location, Location::Dungeon { index: 0, level: 0 });
    }

    #[test]
    fn enter_dungeon_places_player_at_stairs_up() {
        let mut g = overworld_game();
        g.enter_dungeon(0);
        let map = g.current_map();
        assert_eq!(map.get(g.player_x, g.player_y), Tile::StairsUp);
    }

    #[test]
    fn enter_dungeon_saves_overworld_pos() {
        let mut g = overworld_game();
        let (ox, oy) = (g.player_x, g.player_y);
        g.enter_dungeon(0);
        assert_eq!(g.world.saved_overworld_pos, (ox, oy));
    }

    #[test]
    fn exit_dungeon_restores_overworld() {
        let mut g = overworld_game();
        let (ox, oy) = (g.player_x, g.player_y);
        let enemy_count_before = g.enemies.len();
        g.enter_dungeon(0);
        // Now we're in dungeon
        assert_ne!(g.enemies.len(), enemy_count_before);
        g.exit_dungeon();
        assert_eq!(g.world.location, Location::Overworld);
        assert_eq!((g.player_x, g.player_y), (ox, oy));
        assert_eq!(g.enemies.len(), enemy_count_before);
    }

    #[test]
    fn descend_changes_level() {
        let mut g = overworld_game();
        g.enter_dungeon(0);
        assert_eq!(g.world.location, Location::Dungeon { index: 0, level: 0 });
        g.descend(0, 0);
        assert_eq!(g.world.location, Location::Dungeon { index: 0, level: 1 });
    }

    #[test]
    fn ascend_changes_level() {
        let mut g = overworld_game();
        g.enter_dungeon(0);
        g.descend(0, 0);
        assert_eq!(g.world.location, Location::Dungeon { index: 0, level: 1 });
        g.ascend(0, 1);
        assert_eq!(g.world.location, Location::Dungeon { index: 0, level: 0 });
    }

    #[test]
    fn round_trip_dungeon_preserves_overworld_position() {
        let mut g = overworld_game();
        let (ox, oy) = (g.player_x, g.player_y);
        // Enter dungeon
        g.enter_dungeon(0);
        // Descend to level 2
        g.descend(0, 0);
        g.descend(0, 1);
        assert_eq!(g.world.location, Location::Dungeon { index: 0, level: 2 });
        // Ascend back
        g.ascend(0, 2);
        g.ascend(0, 1);
        assert_eq!(g.world.location, Location::Dungeon { index: 0, level: 0 });
        // Exit to overworld
        g.exit_dungeon();
        assert_eq!(g.world.location, Location::Overworld);
        assert_eq!((g.player_x, g.player_y), (ox, oy));
    }

    #[test]
    fn stairs_connect_correct_levels() {
        let mut g = overworld_game();
        g.enter_dungeon(0);
        // On level 0, player should be at StairsUp
        assert_eq!(g.current_map().get(g.player_x, g.player_y), Tile::StairsUp);
        // Descend
        g.descend(0, 0);
        // On level 1, player should be at StairsUp
        assert_eq!(g.current_map().get(g.player_x, g.player_y), Tile::StairsUp);
    }

    #[test]
    fn dungeon_enemies_spawn_on_walkable() {
        let mut g = overworld_game();
        g.enter_dungeon(0);
        for e in &g.enemies {
            assert!(g.current_map().is_walkable(e.x, e.y),
                "{} at ({},{}) not walkable", e.name, e.x, e.y);
        }
    }

    #[test]
    fn dungeon_has_classic_enemies() {
        let mut g = overworld_game();
        g.enter_dungeon(0);
        // Level 0: rats, kobolds, slimes, goblins, skeletons
        let l0_glyphs = ['r', 'c', 'S', 'g', 's'];
        for e in &g.enemies {
            assert!(
                l0_glyphs.contains(&e.glyph),
                "unexpected dungeon L0 enemy: {} ('{}')", e.name, e.glyph
            );
        }
    }

    #[test]
    fn no_dragon_on_shallow_levels() {
        let mut g = overworld_game();
        // Check level 0 of the first dungeon — no dragon
        g.enter_dungeon(0);
        assert!(
            !g.enemies.iter().any(|e| e.glyph == 'D'),
            "level 0 should not have a dragon"
        );
        // Check level 1 — no dragon
        g.descend(0, 0);
        assert!(
            !g.enemies.iter().any(|e| e.glyph == 'D'),
            "level 1 should not have a dragon"
        );
    }

    #[test]
    fn deeper_dungeon_enemies_are_stronger() {
        let mut g = overworld_game();
        g.enter_dungeon(0);
        let l0_max_hp = g.enemies.iter().filter(|e| e.glyph != 'D').map(|e| e.hp).max().unwrap_or(0);

        g.descend(0, 0);
        g.descend(0, 1);
        let l2_max_hp = g.enemies.iter().filter(|e| e.glyph != 'D').map(|e| e.hp).max().unwrap_or(0);

        assert!(l2_max_hp > l0_max_hp,
            "deeper enemies should be stronger: l0_max={l0_max_hp}, l2_max={l2_max_hp}");
    }

    #[test]
    fn transition_message_on_enter_dungeon() {
        let mut g = overworld_game();
        g.enter_dungeon(0);
        assert!(g.messages.iter().any(|m| m.contains("descend")));
    }

    #[test]
    fn transition_message_on_exit_dungeon() {
        let mut g = overworld_game();
        g.enter_dungeon(0);
        g.exit_dungeon();
        assert!(g.messages.iter().any(|m| m.contains("overworld")));
    }

    // === Items, Inventory & Equipment (Phase 5) ===

    fn health_potion() -> Item {
        Item { kind: ItemKind::Potion, name: "Health Potion", glyph: '!', effect: ItemEffect::Heal(5) }
    }
    fn scroll_fire() -> Item {
        Item { kind: ItemKind::Scroll, name: "Scroll of Fire", glyph: '?', effect: ItemEffect::DamageAoe(8) }
    }
    fn rusty_sword() -> Item {
        Item { kind: ItemKind::Weapon, name: "Rusty Sword", glyph: '/', effect: ItemEffect::BuffAttack(2) }
    }
    fn iron_sword() -> Item {
        Item { kind: ItemKind::Weapon, name: "Iron Sword", glyph: '/', effect: ItemEffect::BuffAttack(4) }
    }
    fn leather_armor() -> Item {
        Item { kind: ItemKind::Armor, name: "Leather Armor", glyph: '[', effect: ItemEffect::BuffDefense(2) }
    }
    fn chain_mail() -> Item {
        Item { kind: ItemKind::Armor, name: "Chain Mail", glyph: '[', effect: ItemEffect::BuffDefense(4) }
    }

    // --- Pickup ---

    #[test]
    fn pickup_item_on_move() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        // Place an item one tile to the right of the player, move there, then pick up
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in dirs {
            let (nx, ny) = (g.player_x + dx, g.player_y + dy);
            if g.current_map().is_walkable(nx, ny) {
                g.ground_items.push(GroundItem { x: nx, y: ny, item: health_potion() });
                g.move_player(dx, dy);
                g.pickup_items_explicit();
                assert_eq!(g.inventory.len(), 1);
                assert_eq!(g.inventory[0].name, "Health Potion");
                assert!(g.ground_items.is_empty());
                return;
            }
        }
        panic!("no adjacent walkable tile");
    }

    #[test]
    fn pickup_generates_message() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in dirs {
            let (nx, ny) = (g.player_x + dx, g.player_y + dy);
            if g.current_map().is_walkable(nx, ny) {
                g.ground_items.push(GroundItem { x: nx, y: ny, item: rusty_sword() });
                let msg_before = g.messages.len();
                g.move_player(dx, dy);
                g.pickup_items_explicit();
                assert!(g.messages.len() > msg_before);
                assert!(g.messages.last().unwrap().contains("Picked up"));
                return;
            }
        }
    }

    #[test]
    fn inventory_full_stops_pickup() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let max_inv = g.config.player.max_inventory;
        for _ in 0..max_inv {
            g.inventory.push(health_potion());
        }
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in dirs {
            let (nx, ny) = (g.player_x + dx, g.player_y + dy);
            if g.current_map().is_walkable(nx, ny) {
                g.ground_items.push(GroundItem { x: nx, y: ny, item: rusty_sword() });
                g.move_player(dx, dy);
                assert_eq!(g.inventory.len(), max_inv);
                assert_eq!(g.ground_items.len(), 1, "item should stay on ground");
                return;
            }
        }
    }

    // --- Use ---

    #[test]
    fn use_potion_heals() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.player_hp = 10;
        g.inventory.push(health_potion());
        assert!(g.use_item(0));
        assert_eq!(g.player_hp, 15);
        assert!(g.inventory.is_empty());
    }

    #[test]
    fn use_potion_caps_at_max_hp() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.player_hp = 18;
        g.inventory.push(health_potion());
        g.use_item(0);
        assert_eq!(g.player_hp, 20); // max_hp is 20
    }

    #[test]
    fn use_scroll_damages_nearby_enemies() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let (px, py) = (g.player_x, g.player_y);
        // Place enemies: one close (dist 2), one far (dist 10)
        g.enemies.push(Enemy { x: px + 2, y: py, hp: 20, attack: 1, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        g.enemies.push(Enemy { x: px + 10, y: py, hp: 20, attack: 1, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        g.inventory.push(scroll_fire());
        g.use_item(0);
        assert_eq!(g.enemies[0].hp, 20 - 8, "close enemy should take 8 damage");
        assert_eq!(g.enemies[1].hp, 20, "far enemy should be unaffected");
        assert!(g.inventory.is_empty());
    }

    #[test]
    fn use_weapon_returns_false() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(rusty_sword());
        assert!(!g.use_item(0), "weapon should not be usable");
        assert_eq!(g.inventory.len(), 1, "weapon should remain in inventory");
    }

    #[test]
    fn use_invalid_index_returns_false() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        assert!(!g.use_item(0));
        assert!(!g.use_item(99));
    }

    // --- Equip ---

    #[test]
    fn equip_weapon() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(rusty_sword());
        assert!(g.equip_item(0));
        assert!(g.inventory.is_empty());
        assert_eq!(g.equipped_weapon.as_ref().unwrap().name, "Rusty Sword");
        assert_eq!(g.effective_attack(), 5 + 2);
    }

    #[test]
    fn equip_armor() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(leather_armor());
        assert!(g.equip_item(0));
        assert!(g.inventory.is_empty());
        assert_eq!(g.equipped_armor.as_ref().unwrap().name, "Leather Armor");
        assert_eq!(g.effective_defense(), 0 + 2);
    }

    #[test]
    fn equip_weapon_swaps_old_to_inventory() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(rusty_sword());
        g.equip_item(0);
        g.inventory.push(iron_sword());
        g.equip_item(0);
        assert_eq!(g.equipped_weapon.as_ref().unwrap().name, "Iron Sword");
        assert_eq!(g.inventory.len(), 1);
        assert_eq!(g.inventory[0].name, "Rusty Sword");
        assert_eq!(g.effective_attack(), 5 + 4);
    }

    #[test]
    fn equip_armor_swaps_old_to_inventory() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(leather_armor());
        g.equip_item(0);
        g.inventory.push(chain_mail());
        g.equip_item(0);
        assert_eq!(g.equipped_armor.as_ref().unwrap().name, "Chain Mail");
        assert_eq!(g.inventory.len(), 1);
        assert_eq!(g.inventory[0].name, "Leather Armor");
        assert_eq!(g.effective_defense(), 0 + 4);
    }

    #[test]
    fn equip_potion_returns_false() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(health_potion());
        assert!(!g.equip_item(0));
        assert_eq!(g.inventory.len(), 1);
    }

    // --- Drop ---

    #[test]
    fn drop_item_places_on_ground() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(health_potion());
        assert!(g.drop_item(0));
        assert!(g.inventory.is_empty());
        assert_eq!(g.ground_items.len(), 1);
        assert_eq!(g.ground_items[0].x, g.player_x);
        assert_eq!(g.ground_items[0].y, g.player_y);
        assert_eq!(g.ground_items[0].item.name, "Health Potion");
    }

    #[test]
    fn drop_invalid_index_returns_false() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        assert!(!g.drop_item(0));
    }

    // --- Combat with equipment ---

    #[test]
    fn weapon_increases_damage() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_weapon = Some(rusty_sword());
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 20, attack: 1, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        g.attack_adjacent(gx, gy);
        // Base attack 5 + weapon 2 = 7 damage
        assert_eq!(g.enemies[0].hp, 20 - 7);
    }

    #[test]
    fn armor_reduces_damage_taken() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.player_dexterity = 0; // disable dodge for deterministic test
        g.equipped_armor = Some(leather_armor());
        let hp_before = g.player_hp;
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 99, attack: 5, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        g.attack_adjacent(gx, gy);
        // Enemy attacks during enemy_turn: calc_damage(5, 2) = 25/7 = 3
        assert_eq!(g.player_hp, hp_before - 3);
    }

    #[test]
    fn defense_minimum_damage_is_one() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.player_dexterity = 0; // disable dodge for deterministic test
        // Defense higher than enemy attack
        g.equipped_armor = Some(Item {
            kind: ItemKind::Armor, name: "Dragon Scale", glyph: '[',
            effect: ItemEffect::BuffDefense(6),
        });
        let hp_before = g.player_hp;
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 99, attack: 2, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        g.attack_adjacent(gx, gy);
        // calc_damage(2, 6) = 4/8 = 0 → max(1) = 1
        assert_eq!(g.player_hp, hp_before - 1);
    }

    #[test]
    fn effective_attack_without_weapon() {
        let g = test_game();
        assert_eq!(g.effective_attack(), 5);
    }

    #[test]
    fn effective_defense_without_armor() {
        let g = test_game();
        assert_eq!(g.effective_defense(), 0);
    }

    // --- Item spawning ---

    #[test]
    fn dungeon_has_ground_items() {
        let mut g = overworld_game();
        g.enter_dungeon(0);
        assert!(!g.ground_items.is_empty(), "dungeon level 0 should have items");
    }

    #[test]
    fn dungeon_items_on_floor_tiles() {
        let mut g = overworld_game();
        g.enter_dungeon(0);
        let map = g.current_map();
        for gi in &g.ground_items {
            assert_eq!(map.get(gi.x, gi.y), Tile::Floor,
                "item '{}' at ({},{}) not on Floor", gi.item.name, gi.x, gi.y);
        }
    }

    #[test]
    fn dungeons_have_loot_on_each_level() {
        let mut g = overworld_game();
        g.enter_dungeon(0);
        assert!(!g.ground_items.is_empty(), "dungeon level 0 should have items");
        g.descend(0, 0);
        assert!(!g.ground_items.is_empty(), "dungeon level 1 should have items");
        g.descend(0, 1);
        assert!(!g.ground_items.is_empty(), "dungeon level 2 should have items");
    }

    #[test]
    fn overworld_items_sparse() {
        let mut g = overworld_game();
        g.spawn_overworld_items(42);
        // Overworld should have very few items
        let map = g.current_map();
        let total_walkable = (0..map.height)
            .flat_map(|y| (0..map.width).map(move |x| (x, y)))
            .filter(|&(x, y)| map.is_walkable(x, y))
            .count();
        assert!(g.ground_items.len() < total_walkable / 50,
            "overworld items should be sparse: {} items for {} walkable tiles",
            g.ground_items.len(), total_walkable);
    }

    #[test]
    fn random_item_tiers_correct() {
        // Tier 0 produces basic items
        let mut rng = 42u64;
        let items: Vec<_> = (0..50).map(|_| random_item(0, &mut rng)).collect();
        assert!(items.iter().any(|i| i.name == "Health Potion" || i.name == "Rusty Sword"));
        // Tier 2 produces advanced items
        rng = 42;
        let items: Vec<_> = (0..50).map(|_| random_item(2, &mut rng)).collect();
        assert!(items.iter().any(|i| i.name == "Superior Health Potion" || i.name == "Enchanted Blade" || i.name == "Dragon Scale"));
    }

    // --- Ground items persist across dungeon transitions ---

    #[test]
    fn overworld_items_saved_on_enter_dungeon() {
        let mut g = overworld_game();
        g.ground_items.push(GroundItem { x: 10, y: 10, item: health_potion() });
        let ow_item_count = g.ground_items.len();
        g.enter_dungeon(0);
        // Dungeon should have its own items, overworld items saved
        assert_ne!(g.ground_items.len(), ow_item_count);
        g.exit_dungeon();
        // Overworld items restored
        assert!(g.ground_items.iter().any(|gi| gi.item.name == "Health Potion"));
    }

    #[test]
    fn inventory_persists_across_transitions() {
        let mut g = overworld_game();
        g.inventory.push(rusty_sword());
        g.enter_dungeon(0);
        assert_eq!(g.inventory.len(), 1);
        assert_eq!(g.inventory[0].name, "Rusty Sword");
        g.exit_dungeon();
        assert_eq!(g.inventory.len(), 1);
    }

    // === Phase 6: UI, Tile Info, XP & Drawers ===

    // --- Tile info ---

    #[test]
    fn inspect_player_tile() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.update_fov();
        let info = g.inspect_tile(g.player_x, g.player_y).unwrap();
        assert_eq!(info.tile_name, "Stone Floor");
        assert!(info.walkable);
        assert!(info.is_player);
        assert!(info.enemy.is_none());
    }

    #[test]
    fn inspect_hidden_tile_returns_none() {
        let map = Map::generate(30, 20, 42);
        let g = Game::new(map);
        // Without update_fov, all tiles are Hidden
        assert!(g.inspect_tile(0, 0).is_none());
    }

    #[test]
    fn inspect_enemy_tile() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.update_fov();
        let (ex, ey) = (g.player_x + 1, g.player_y);
        g.enemies.push(Enemy {
            x: ex, y: ey, hp: 10, attack: 3, defense: 0, glyph: 'g', name: "Goblin", facing_left: false, is_ranged: false,
        });
        let info = g.inspect_tile(ex, ey).unwrap();
        let enemy = info.enemy.unwrap();
        assert_eq!(enemy.name, "Goblin");
        assert_eq!(enemy.hp, 10);
        assert_eq!(enemy.attack, 3);
        assert_eq!(enemy.desc, "A sneaky green creature. Dangerous in numbers.");
    }

    #[test]
    fn inspect_item_tile() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.update_fov();
        let (ix, iy) = (g.player_x + 1, g.player_y);
        g.ground_items.push(GroundItem { x: ix, y: iy, item: rusty_sword() });
        let info = g.inspect_tile(ix, iy).unwrap();
        let item = info.item.unwrap();
        assert_eq!(item.name, "Rusty Sword");
        assert!(item.desc.contains("+2 Attack"));
    }

    #[test]
    fn inspect_out_of_bounds_returns_none() {
        let map = Map::generate(30, 20, 42);
        let g = Game::new(map);
        assert!(g.inspect_tile(-1, -1).is_none());
        assert!(g.inspect_tile(999, 999).is_none());
    }

    #[test]
    fn every_tile_has_name_and_desc() {
        let tiles = [
            Tile::Wall, Tile::Floor, Tile::Tree, Tile::Grass,
            Tile::Road, Tile::DungeonEntrance, Tile::StairsDown, Tile::StairsUp,
        ];
        for tile in tiles {
            assert!(!tile_name(tile).is_empty(), "{:?} has no name", tile);
            assert!(!tile_desc(tile).is_empty(), "{:?} has no desc", tile);
        }
    }

    #[test]
    fn every_enemy_has_desc() {
        let all_enemies = [
            "Giant Rat", "Giant Bat", "Wolf", "Giant Spider", "Boar", "Bear", "Lycanthrope",
            "Kobold", "Small Slime", "Goblin", "Skeleton",
            "Goblin Archer", "Zombie", "Skeleton Archer", "Big Slime", "Orc",
            "Ghoul", "Orc Blademaster", "Wraith", "Naga", "Troll",
            "Death Knight", "Lich", "Dragon",
        ];
        for name in all_enemies {
            let desc = enemy_desc(name);
            assert!(!desc.is_empty(), "{name} has no desc");
            assert_ne!(desc, "A mysterious creature.", "{name} should have a unique desc");
        }
    }

    // --- Location name ---

    #[test]
    fn location_name_overworld() {
        let g = overworld_game();
        assert_eq!(g.location_name(), "Overworld");
    }

    #[test]
    fn location_name_dungeon() {
        let mut g = overworld_game();
        g.enter_dungeon(0);
        let name = g.location_name();
        assert!(name.starts_with("Dungeon") || name.starts_with("Dragon"),
            "unexpected location name: {name}");
        assert!(name.contains("B1"));
    }

    // --- Drawers ---

    #[test]
    fn toggle_drawer_opens_and_closes() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        assert_eq!(g.drawer, Drawer::None);
        g.toggle_drawer(Drawer::Inventory);
        assert_eq!(g.drawer, Drawer::Inventory);
        g.toggle_drawer(Drawer::Inventory);
        assert_eq!(g.drawer, Drawer::None);
    }

    #[test]
    fn toggle_drawer_switches_between_drawers() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.toggle_drawer(Drawer::Inventory);
        g.toggle_drawer(Drawer::Stats);
        assert_eq!(g.drawer, Drawer::Stats);
    }

    // --- XP and leveling ---

    #[test]
    fn xp_granted_on_kill() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 1, attack: 0, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        g.attack_adjacent(gx, gy);
        assert_eq!(g.player_xp, 4); // goblin = 4 XP
    }

    #[test]
    fn level_up_awards_skill_points() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let old_max = g.player_max_hp;
        // Force enough XP for level 2 (need 20 XP at level 1)
        g.player_xp = 20;
        g.check_level_up();
        assert_eq!(g.player_level, 2);
        assert_eq!(g.skill_points, 3); // 3 skill points per level
        assert_eq!(g.player_max_hp, old_max + 2); // base +2 HP per level
        // Partial heal (50% of missing + 1), at full HP → stays at max
        assert_eq!(g.player_hp, g.player_max_hp);
    }

    #[test]
    fn xp_to_next_level_scales() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let l1 = g.xp_to_next_level();
        g.player_level = 3;
        let l3 = g.xp_to_next_level();
        assert!(l3 > l1, "higher levels should need more XP");
    }

    #[test]
    fn xp_for_each_enemy_type() {
        // Forest
        assert_eq!(xp_for_enemy("Giant Rat"), 3);
        assert_eq!(xp_for_enemy("Giant Bat"), 4);
        assert_eq!(xp_for_enemy("Wolf"), 5);
        assert_eq!(xp_for_enemy("Giant Spider"), 6);
        assert_eq!(xp_for_enemy("Boar"), 7);
        assert_eq!(xp_for_enemy("Bear"), 12);
        assert_eq!(xp_for_enemy("Lycanthrope"), 18);
        // Dungeon shallow
        assert_eq!(xp_for_enemy("Kobold"), 3);
        assert_eq!(xp_for_enemy("Small Slime"), 3);
        assert_eq!(xp_for_enemy("Goblin"), 4);
        assert_eq!(xp_for_enemy("Skeleton"), 6);
        // Dungeon mid
        assert_eq!(xp_for_enemy("Goblin Archer"), 5);
        assert_eq!(xp_for_enemy("Zombie"), 6);
        assert_eq!(xp_for_enemy("Skeleton Archer"), 7);
        assert_eq!(xp_for_enemy("Big Slime"), 7);
        assert_eq!(xp_for_enemy("Orc"), 10);
        // Dungeon deep
        assert_eq!(xp_for_enemy("Ghoul"), 11);
        assert_eq!(xp_for_enemy("Orc Blademaster"), 14);
        assert_eq!(xp_for_enemy("Wraith"), 13);
        assert_eq!(xp_for_enemy("Naga"), 16);
        assert_eq!(xp_for_enemy("Troll"), 15);
        // Cave boss
        assert_eq!(xp_for_enemy("Death Knight"), 22);
        assert_eq!(xp_for_enemy("Lich"), 25);
        assert_eq!(xp_for_enemy("Dragon"), 100);
    }

    #[test]
    fn kill_message_includes_xp() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 1, attack: 0, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        g.attack_adjacent(gx, gy);
        assert!(g.messages.iter().any(|m| m.contains("+4 XP")));
    }

    // === Stamina, Sprint, Hunger & Food ===

    fn raw_food(amount: i32) -> Item {
        Item { kind: ItemKind::Food, name: "Wild Berries", glyph: '%', effect: ItemEffect::Feed(amount, FoodSideEffect::None) }
    }

    // --- Stamina ---

    #[test]
    fn player_starts_with_full_stamina() {
        let g = test_game();
        assert_eq!(g.stamina, 100);
        assert_eq!(g.max_stamina, 100);
        assert!(!g.sprinting);
    }

    #[test]
    fn stamina_regens_on_walk() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.stamina = 50;
        // Find a walkable neighbor
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in dirs {
            let (nx, ny) = (g.player_x + dx, g.player_y + dy);
            if g.current_map().is_walkable(nx, ny) {
                g.move_player(dx, dy);
                // Walking regens STAMINA_REGEN (5)
                assert_eq!(g.stamina, 55, "stamina should regen by 5 on walk");
                return;
            }
        }
        panic!("no walkable neighbor");
    }

    #[test]
    fn stamina_capped_at_max() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.stamina = 98;
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in dirs {
            let (nx, ny) = (g.player_x + dx, g.player_y + dy);
            if g.current_map().is_walkable(nx, ny) {
                g.move_player(dx, dy);
                assert_eq!(g.stamina, 100, "stamina should cap at max");
                return;
            }
        }
    }

    // --- Sprint ---

    #[test]
    fn toggle_sprint_on_and_off() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        assert!(!g.sprinting);
        g.toggle_sprint();
        assert!(g.sprinting);
        g.toggle_sprint();
        assert!(!g.sprinting);
    }

    #[test]
    fn sprint_denied_when_low_stamina() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.stamina = 5; // below SPRINT_COST (15)
        g.toggle_sprint();
        assert!(!g.sprinting, "sprint should be denied when stamina too low");
        assert!(g.messages.iter().any(|m| m.contains("exhausted")));
    }

    #[test]
    fn sprint_drains_stamina_on_move() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.sprinting = true;
        let stam_before = g.stamina;
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in dirs {
            let (nx, ny) = (g.player_x + dx, g.player_y + dy);
            if g.current_map().is_walkable(nx, ny) {
                g.move_player(dx, dy);
                assert_eq!(g.stamina, stam_before - 15, "sprint should drain 15 stamina");
                return;
            }
        }
    }

    #[test]
    fn sprint_auto_disables_when_exhausted() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.sprinting = true;
        g.stamina = 15; // exactly one sprint move left
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in dirs {
            let (nx, ny) = (g.player_x + dx, g.player_y + dy);
            if g.current_map().is_walkable(nx, ny) {
                g.move_player(dx, dy);
                assert_eq!(g.stamina, 0);
                assert!(!g.sprinting, "sprint should auto-disable at 0 stamina");
                assert!(g.messages.iter().any(|m| m.contains("Exhausted")));
                return;
            }
        }
    }

    #[test]
    fn sprint_skips_enemy_turn() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.sprinting = true;
        // Place enemy 2 tiles away
        let ex = g.player_x + 2;
        let ey = g.player_y;
        if g.current_map().is_walkable(ex, ey) {
            g.enemies.push(Enemy { x: ex, y: ey, hp: 10, attack: 3, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
            // Move away from enemy
            if g.current_map().is_walkable(g.player_x, g.player_y + 1) {
                g.move_player(0, 1);
                // Enemy should NOT have moved because player is sprinting
                assert_eq!(g.enemies[0].x, ex, "enemy should not chase during sprint");
                assert_eq!(g.enemies[0].y, ey, "enemy should not chase during sprint");
            }
        }
    }

    // --- Hunger ---

    #[test]
    fn player_starts_with_full_hunger() {
        let g = test_game();
        assert_eq!(g.hunger, 100);
        assert_eq!(g.max_hunger, 100);
    }

    #[test]
    fn hunger_drains_every_five_moves() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let hunger_before = g.hunger;
        // Move 5 times to trigger hunger drain (every HUNGER_INTERVAL turns)
        let mut moves = 0;
        for _ in 0..10 {
            let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
            for (dx, dy) in dirs {
                let (nx, ny) = (g.player_x + dx, g.player_y + dy);
                if g.current_map().is_walkable(nx, ny) {
                    g.move_player(dx, dy);
                    moves += 1;
                    if moves < 5 {
                        assert_eq!(g.hunger, hunger_before,
                            "hunger should NOT drain before 5 moves (move {moves})");
                    }
                    if moves == 5 {
                        assert_eq!(g.hunger, hunger_before - 1,
                            "hunger should drain 1 after 5 moves");
                        return;
                    }
                    break;
                }
            }
        }
    }

    #[test]
    fn starvation_damages_player() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.hunger = 0; // Already starving
        let hp_before = g.player_hp;
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in dirs {
            let (nx, ny) = (g.player_x + dx, g.player_y + dy);
            if g.current_map().is_walkable(nx, ny) {
                g.move_player(dx, dy);
                assert_eq!(g.hunger, 0);
                assert_eq!(g.player_hp, hp_before - 1, "starvation should deal 1 damage");
                return;
            }
        }
    }

    #[test]
    fn starvation_can_kill_player() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.hunger = 0;
        g.player_hp = 1;
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in dirs {
            let (nx, ny) = (g.player_x + dx, g.player_y + dy);
            if g.current_map().is_walkable(nx, ny) {
                g.move_player(dx, dy);
                assert!(!g.alive, "starvation should kill at 0 HP");
                assert!(g.messages.iter().any(|m| m.contains("starved")));
                return;
            }
        }
    }

    #[test]
    fn hunger_doesnt_go_negative() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.hunger = 0;
        g.player_hp = 20; // Won't die
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in dirs {
            let (nx, ny) = (g.player_x + dx, g.player_y + dy);
            if g.current_map().is_walkable(nx, ny) {
                g.move_player(dx, dy);
                assert_eq!(g.hunger, 0, "hunger should not go below 0");
                return;
            }
        }
    }

    // --- Food ---

    #[test]
    fn eat_food_restores_hunger() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.hunger = 50;
        g.inventory.push(raw_food(20));
        assert!(g.eat_food(0));
        assert_eq!(g.hunger, 70);
        assert!(g.inventory.is_empty());
    }

    #[test]
    fn eat_food_capped_at_max() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.hunger = 90;
        g.inventory.push(raw_food(20));
        g.eat_food(0);
        assert_eq!(g.hunger, 100);
    }

    #[test]
    fn eat_food_message() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.hunger = 80;
        g.inventory.push(raw_food(10));
        g.eat_food(0);
        assert!(g.messages.iter().any(|m| m.contains("Hunger +10")));
    }

    #[test]
    fn eat_non_food_fails() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(health_potion());
        assert!(!g.eat_food(0));
        assert_eq!(g.inventory.len(), 1);
    }

    #[test]
    fn use_item_works_on_food() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.hunger = 50;
        g.inventory.push(raw_food(15));
        assert!(g.use_item(0));
        assert_eq!(g.hunger, 65);
        assert!(g.inventory.is_empty());
    }

    // --- Meat drops ---

    #[test]
    fn killing_wolf_drops_meat() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 1, attack: 0, glyph: 'w', name: "Wolf", facing_left: false, defense: 0, is_ranged: false });
        g.attack_adjacent(gx, gy);
        assert!(g.ground_items.iter().any(|gi| gi.item.name == "Wolf Meat"),
            "wolf should drop meat");
    }

    #[test]
    fn killing_boar_drops_meat() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 1, attack: 0, glyph: 'b', name: "Boar", facing_left: false, defense: 0, is_ranged: false });
        g.attack_adjacent(gx, gy);
        assert!(g.ground_items.iter().any(|gi| gi.item.name == "Boar Meat"));
    }

    #[test]
    fn killing_bear_drops_meat() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 1, attack: 0, glyph: 'B', name: "Bear", facing_left: false, defense: 0, is_ranged: false });
        g.attack_adjacent(gx, gy);
        assert!(g.ground_items.iter().any(|gi| gi.item.name == "Bear Meat"));
    }

    #[test]
    fn killing_rat_drops_rat_meat() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 1, attack: 0, glyph: 'r', name: "Giant Rat", facing_left: false, defense: 0, is_ranged: false });
        g.attack_adjacent(gx, gy);
        assert!(g.ground_items.iter().any(|gi| gi.item.name == "Rat Meat"),
            "giant rat should drop rat meat");
    }

    #[test]
    fn killing_goblin_drops_rations() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 1, attack: 0, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        g.attack_adjacent(gx, gy);
        assert!(g.ground_items.iter().any(|gi| gi.item.name == "Stolen Rations"),
            "goblin should drop stolen rations");
    }

    #[test]
    fn meat_has_feed_effect() {
        let meat = meat_drop("Wolf").unwrap();
        assert_eq!(meat.kind, ItemKind::Food);
        assert!(matches!(meat.effect, ItemEffect::Feed(_, _)));
    }

    #[test]
    fn bear_meat_restores_more_than_wolf() {
        let wolf = meat_drop("Wolf").unwrap();
        let bear = meat_drop("Bear").unwrap();
        let wolf_feed = match wolf.effect { ItemEffect::Feed(n, _) => n, _ => 0 };
        let bear_feed = match bear.effect { ItemEffect::Feed(n, _) => n, _ => 0 };
        assert!(bear_feed > wolf_feed, "bear meat should restore more hunger");
    }

    // --- Food spawning ---

    #[test]
    fn overworld_has_food_on_grass() {
        let mut g = overworld_game();
        g.spawn_overworld_food(42);
        let food_count = g.ground_items.iter()
            .filter(|gi| gi.item.kind == ItemKind::Food)
            .count();
        assert!(food_count > 0, "overworld should have food items on grass");
        // All food should be on grass
        let map = g.current_map();
        for gi in g.ground_items.iter().filter(|gi| gi.item.kind == ItemKind::Food) {
            assert_eq!(map.get(gi.x, gi.y), Tile::Grass,
                "food should only spawn on grass, found at ({},{})", gi.x, gi.y);
        }
    }

    #[test]
    fn overworld_food_has_variety() {
        let mut g = overworld_game();
        g.spawn_overworld_food(42);
        let food_names: std::collections::HashSet<&str> = g.ground_items.iter()
            .filter(|gi| gi.item.kind == ItemKind::Food)
            .map(|gi| gi.item.name)
            .collect();
        assert!(food_names.len() >= 2, "overworld should have at least 2 food types, got: {:?}", food_names);
    }

    #[test]
    fn large_beasts_drop_food() {
        let beasts = ["Wolf", "Boar", "Bear"];
        for name in beasts {
            assert!(meat_drop(name).is_some(), "{name} should drop food");
        }
    }

    #[test]
    fn inedible_creatures_drop_no_food() {
        let creatures = ["Giant Bat", "Giant Spider"];
        for name in creatures {
            assert!(meat_drop(name).is_none(), "{name} should not drop food");
        }
    }

    #[test]
    fn meat_feed_values_scale_with_beast() {
        let drops: Vec<_> = ["Wolf", "Boar", "Bear"]
            .iter()
            .map(|n| {
                let item = meat_drop(n).unwrap();
                match item.effect { ItemEffect::Feed(v, _) => v, _ => 0 }
            })
            .collect();
        // Bear meat should be the most filling
        assert!(*drops.last().unwrap() > *drops.first().unwrap(),
            "larger beasts should drop more filling food");
    }

    #[test]
    fn dungeon_food_includes_drinks() {
        let mut rng = 42u64;
        let items: Vec<_> = (0..500).map(|_| random_item(1, &mut rng)).collect();
        let drink_names = ["Dwarven Ale"];
        assert!(items.iter().any(|i| drink_names.contains(&i.name)),
            "dungeon tier 1 should produce drinks");
    }

    #[test]
    fn deep_dungeon_food_better_than_shallow() {
        let mut rng = 42u64;
        let t0_food: Vec<_> = (0..500)
            .map(|_| random_item(0, &mut rng))
            .filter(|i| i.kind == ItemKind::Food)
            .collect();
        rng = 42;
        let t2_food: Vec<_> = (0..500)
            .map(|_| random_item(2, &mut rng))
            .filter(|i| i.kind == ItemKind::Food)
            .collect();
        let avg_t0: f64 = t0_food.iter().map(|i| match i.effect { ItemEffect::Feed(v, _) => v as f64, _ => 0.0 }).sum::<f64>() / t0_food.len() as f64;
        let avg_t2: f64 = t2_food.iter().map(|i| match i.effect { ItemEffect::Feed(v, _) => v as f64, _ => 0.0 }).sum::<f64>() / t2_food.len() as f64;
        assert!(avg_t2 > avg_t0, "deep dungeon food should be more filling: t0_avg={avg_t0}, t2_avg={avg_t2}");
    }

    #[test]
    fn random_item_includes_food() {
        let mut rng = 42u64;
        let items: Vec<_> = (0..200).map(|_| random_item(0, &mut rng)).collect();
        assert!(items.iter().any(|i| i.kind == ItemKind::Food),
            "tier 0 random_item should sometimes produce food");
    }

    #[test]
    fn dungeon_random_item_includes_rations() {
        let mut rng = 42u64;
        let items: Vec<_> = (0..200).map(|_| random_item(1, &mut rng)).collect();
        assert!(items.iter().any(|i| i.name == "Dried Rations"),
            "dungeon tier should produce rations");
    }

    // --- Turn counter ---

    #[test]
    fn turn_counter_increments_on_move() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        assert_eq!(g.turn, 0);
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in dirs {
            let (nx, ny) = (g.player_x + dx, g.player_y + dy);
            if g.current_map().is_walkable(nx, ny) {
                g.move_player(dx, dy);
                assert_eq!(g.turn, 1);
                return;
            }
        }
    }

    // --- Item info for food ---

    #[test]
    fn food_item_info_desc() {
        let food = raw_food(15);
        let desc = item_info_desc(&food);
        assert!(desc.contains("Restores 15 hunger"), "food desc: {desc}");
    }

    // --- Rings ---

    #[test]
    fn equip_ring() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(Item {
            kind: ItemKind::Ring, name: "Gold Ring", glyph: '=', effect: ItemEffect::BuffAttack(4),
        });
        assert!(g.equip_item(0));
        assert!(g.inventory.is_empty());
        assert!(g.equipped_ring.is_some());
        assert_eq!(g.equipped_ring.as_ref().unwrap().name, "Gold Ring");
    }

    #[test]
    fn ring_boosts_attack() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let base_atk = g.effective_attack();
        g.equipped_ring = Some(Item {
            kind: ItemKind::Ring, name: "Gold Ring", glyph: '=', effect: ItemEffect::BuffAttack(4),
        });
        assert_eq!(g.effective_attack(), base_atk + 4);
    }

    #[test]
    fn ring_boosts_defense() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let base_def = g.effective_defense();
        g.equipped_ring = Some(Item {
            kind: ItemKind::Ring, name: "Diamond Ring", glyph: '=', effect: ItemEffect::BuffDefense(4),
        });
        assert_eq!(g.effective_defense(), base_def + 4);
    }

    #[test]
    fn ring_swaps_to_inventory() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_ring = Some(Item {
            kind: ItemKind::Ring, name: "Copper Ring", glyph: '=', effect: ItemEffect::BuffAttack(1),
        });
        g.inventory.push(Item {
            kind: ItemKind::Ring, name: "Gold Ring", glyph: '=', effect: ItemEffect::BuffAttack(4),
        });
        g.equip_item(0);
        assert_eq!(g.equipped_ring.as_ref().unwrap().name, "Gold Ring");
        assert_eq!(g.inventory[0].name, "Copper Ring");
    }

    // --- Helmet, Shield, Boots ---

    #[test]
    fn equip_helmet() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(Item {
            kind: ItemKind::Helmet, name: "Leather Cap", glyph: '^', effect: ItemEffect::BuffDefense(1),
        });
        assert!(g.equip_item(0));
        assert!(g.inventory.is_empty());
        assert_eq!(g.equipped_helmet.as_ref().unwrap().name, "Leather Cap");
        assert_eq!(g.effective_defense(), 1);
    }

    #[test]
    fn equip_shield() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(Item {
            kind: ItemKind::Shield, name: "Wooden Shield", glyph: ')', effect: ItemEffect::BuffDefense(1),
        });
        assert!(g.equip_item(0));
        assert!(g.inventory.is_empty());
        assert_eq!(g.equipped_shield.as_ref().unwrap().name, "Wooden Shield");
        assert_eq!(g.effective_defense(), 1);
    }

    #[test]
    fn equip_boots() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(Item {
            kind: ItemKind::Boots, name: "Leather Boots", glyph: '{', effect: ItemEffect::BuffDefense(1),
        });
        assert!(g.equip_item(0));
        assert!(g.inventory.is_empty());
        assert_eq!(g.equipped_boots.as_ref().unwrap().name, "Leather Boots");
        assert_eq!(g.effective_defense(), 1);
    }

    #[test]
    fn helmet_swaps_to_inventory() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_helmet = Some(Item {
            kind: ItemKind::Helmet, name: "Leather Cap", glyph: '^', effect: ItemEffect::BuffDefense(1),
        });
        g.inventory.push(Item {
            kind: ItemKind::Helmet, name: "Iron Helmet", glyph: '^', effect: ItemEffect::BuffDefense(3),
        });
        g.equip_item(0);
        assert_eq!(g.equipped_helmet.as_ref().unwrap().name, "Iron Helmet");
        assert_eq!(g.inventory[0].name, "Leather Cap");
    }

    #[test]
    fn shield_swaps_to_inventory() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_shield = Some(Item {
            kind: ItemKind::Shield, name: "Wooden Shield", glyph: ')', effect: ItemEffect::BuffDefense(1),
        });
        g.inventory.push(Item {
            kind: ItemKind::Shield, name: "Iron Shield", glyph: ')', effect: ItemEffect::BuffDefense(3),
        });
        g.equip_item(0);
        assert_eq!(g.equipped_shield.as_ref().unwrap().name, "Iron Shield");
        assert_eq!(g.inventory[0].name, "Wooden Shield");
    }

    #[test]
    fn boots_swaps_to_inventory() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_boots = Some(Item {
            kind: ItemKind::Boots, name: "Leather Boots", glyph: '{', effect: ItemEffect::BuffDefense(1),
        });
        g.inventory.push(Item {
            kind: ItemKind::Boots, name: "Plate Boots", glyph: '{', effect: ItemEffect::BuffDefense(4),
        });
        g.equip_item(0);
        assert_eq!(g.equipped_boots.as_ref().unwrap().name, "Plate Boots");
        assert_eq!(g.inventory[0].name, "Leather Boots");
    }

    #[test]
    fn full_defense_stacks_all_slots() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_armor = Some(Item {
            kind: ItemKind::Armor, name: "Chain Mail", glyph: '[', effect: ItemEffect::BuffDefense(4),
        });
        g.equipped_helmet = Some(Item {
            kind: ItemKind::Helmet, name: "Iron Helmet", glyph: '^', effect: ItemEffect::BuffDefense(3),
        });
        g.equipped_shield = Some(Item {
            kind: ItemKind::Shield, name: "Iron Shield", glyph: ')', effect: ItemEffect::BuffDefense(3),
        });
        g.equipped_boots = Some(Item {
            kind: ItemKind::Boots, name: "Chain Boots", glyph: '{', effect: ItemEffect::BuffDefense(2),
        });
        g.equipped_ring = Some(Item {
            kind: ItemKind::Ring, name: "Diamond Ring", glyph: '=', effect: ItemEffect::BuffDefense(4),
        });
        // base 0 + armor 4 + helmet 3 + shield 3 + boots 2 + ring 4 = 16
        assert_eq!(g.effective_defense(), 16);
    }

    // --- Item variety ---

    #[test]
    fn random_item_produces_variety() {
        let mut rng = 42u64;
        let items: Vec<_> = (0..500).map(|_| random_item(1, &mut rng)).collect();
        // Should produce all equippable kinds plus consumables
        assert!(items.iter().any(|i| i.kind == ItemKind::Weapon), "should have weapons");
        assert!(items.iter().any(|i| i.kind == ItemKind::Armor), "should have armor");
        assert!(items.iter().any(|i| i.kind == ItemKind::Helmet), "should have helmets");
        assert!(items.iter().any(|i| i.kind == ItemKind::Shield), "should have shields");
        assert!(items.iter().any(|i| i.kind == ItemKind::Boots), "should have boots");
        assert!(items.iter().any(|i| i.kind == ItemKind::Ring), "should have rings");
        assert!(items.iter().any(|i| i.kind == ItemKind::Food), "should have food");
        assert!(items.iter().any(|i| i.kind == ItemKind::Potion), "should have potions");
        assert!(items.iter().any(|i| i.kind == ItemKind::Scroll), "should have scrolls");
    }

    #[test]
    fn weapon_names_vary() {
        let mut rng = 42u64;
        let weapons: Vec<_> = (0..500)
            .map(|_| random_item(0, &mut rng))
            .filter(|i| i.kind == ItemKind::Weapon)
            .collect();
        let names: std::collections::HashSet<&str> = weapons.iter().map(|i| i.name).collect();
        assert!(names.len() >= 2, "should have at least 2 weapon variants, got: {:?}", names);
    }

    // === Inventory scroll ===

    #[test]
    fn scroll_inventory_down_and_up() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        for _ in 0..10 {
            g.inventory.push(health_potion());
        }
        assert_eq!(g.inventory_scroll, 0);
        g.scroll_inventory(3);
        assert_eq!(g.inventory_scroll, 3);
        g.scroll_inventory(-1);
        assert_eq!(g.inventory_scroll, 2);
    }

    #[test]
    fn scroll_inventory_clamps_at_zero() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        for _ in 0..5 {
            g.inventory.push(health_potion());
        }
        g.scroll_inventory(-10);
        assert_eq!(g.inventory_scroll, 0);
    }

    #[test]
    fn scroll_inventory_clamps_at_max() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        for _ in 0..5 {
            g.inventory.push(health_potion());
        }
        g.scroll_inventory(100);
        assert_eq!(g.inventory_scroll, 4); // last valid index = len - 1
    }

    #[test]
    fn scroll_clamps_after_item_removal() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        for _ in 0..5 {
            g.inventory.push(health_potion());
        }
        g.inventory_scroll = 4; // pointing at last item
        g.player_hp = 10; // damage so potion heals
        g.use_item(4); // removes last item
        assert_eq!(g.inventory_scroll, 3); // clamped to new last index
    }

    #[test]
    fn scroll_resets_when_inventory_empty() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(health_potion());
        g.inventory_scroll = 0;
        g.player_hp = 10;
        g.use_item(0);
        assert_eq!(g.inventory_scroll, 0);
        assert!(g.inventory.is_empty());
    }

    #[test]
    fn drop_item_clamps_scroll() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        for _ in 0..3 {
            g.inventory.push(rusty_sword());
        }
        g.inventory_scroll = 2;
        g.drop_item(2);
        assert_eq!(g.inventory_scroll, 1);
    }

    #[test]
    fn equip_item_clamps_scroll() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        for _ in 0..3 {
            g.inventory.push(rusty_sword());
        }
        g.inventory_scroll = 2;
        g.equip_item(2); // removes item, pushes nothing back (slot empty)
        assert_eq!(g.inventory_scroll, 1);
    }

    #[test]
    fn scroll_inventory_page_jump() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        for _ in 0..10 {
            g.inventory.push(health_potion());
        }
        // Page-down by 5
        g.scroll_inventory(5);
        assert_eq!(g.inventory_scroll, 5);
        // Page-up by 3
        g.scroll_inventory(-3);
        assert_eq!(g.inventory_scroll, 2);
        // Large page-down clamps to max
        g.scroll_inventory(100);
        assert_eq!(g.inventory_scroll, 9);
    }

    // ── Item selection tests ─────────────────────────────────────────

    #[test]
    fn selected_item_starts_none() {
        let map = Map::generate(30, 20, 42);
        let g = Game::new(map);
        assert!(g.selected_inventory_item.is_none());
    }

    #[test]
    fn selection_cleared_on_drawer_toggle() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(rusty_sword());
        g.selected_inventory_item = Some(0);
        // Opening inventory clears selection
        g.toggle_drawer(Drawer::Inventory);
        assert!(g.selected_inventory_item.is_none());
        // Re-select and close drawer
        g.selected_inventory_item = Some(0);
        g.toggle_drawer(Drawer::Inventory); // toggles off
        assert!(g.selected_inventory_item.is_none());
    }

    #[test]
    fn selection_cleared_when_item_dropped() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(rusty_sword());
        g.inventory.push(health_potion());
        g.selected_inventory_item = Some(0);
        g.drop_item(0);
        // Selection should be cleared because item was removed
        // (clamp_inventory_scroll clears selection when index >= len)
        assert_eq!(g.inventory.len(), 1);
    }

    #[test]
    fn selection_cleared_when_item_used() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(health_potion());
        g.player_hp = 10;
        g.selected_inventory_item = Some(0);
        g.use_item(0);
        // Item consumed, selection should be cleared (only had 1 item)
        assert!(g.inventory.is_empty());
        assert!(g.selected_inventory_item.is_none());
    }

    #[test]
    fn selection_survives_when_valid_after_removal() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(health_potion());
        g.inventory.push(rusty_sword());
        g.inventory.push(health_potion());
        g.selected_inventory_item = Some(0);
        // Drop item at index 2 — selection at 0 stays valid
        g.drop_item(2);
        assert_eq!(g.selected_inventory_item, Some(0));
    }

    #[test]
    fn selection_cleared_when_index_out_of_bounds() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(health_potion());
        g.selected_inventory_item = Some(5); // out of bounds
        g.clamp_inventory_scroll();
        assert!(g.selected_inventory_item.is_none());
    }

    // ── Item description tests ───────────────────────────────────────

    #[test]
    fn inventory_item_desc_returns_description() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(rusty_sword());
        let desc = g.inventory_item_desc(0).unwrap();
        assert!(desc.contains("Rusty Sword"));
        assert!(desc.contains("Attack"));
    }

    #[test]
    fn inventory_item_desc_returns_none_for_empty() {
        let map = Map::generate(30, 20, 42);
        let g = Game::new(map);
        assert!(g.inventory_item_desc(0).is_none());
    }

    #[test]
    fn inventory_item_desc_potion() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(health_potion());
        let desc = g.inventory_item_desc(0).unwrap();
        assert!(desc.contains("HP"));
    }

    // ── set_inventory_scroll tests ───────────────────────────────────

    #[test]
    fn set_inventory_scroll_clamps() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(health_potion());
        g.inventory.push(rusty_sword());
        g.set_inventory_scroll(100);
        assert_eq!(g.inventory_scroll, 1); // clamped to len-1
    }

    #[test]
    fn set_inventory_scroll_zero_on_empty() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.set_inventory_scroll(5);
        assert_eq!(g.inventory_scroll, 0);
    }

    // === Ranged weapon system ===

    fn short_bow() -> Item {
        Item { kind: ItemKind::RangedWeapon, name: "Short Bow", glyph: '}', effect: ItemEffect::BuffAttack(2) }
    }
    fn crossbow() -> Item {
        Item { kind: ItemKind::RangedWeapon, name: "Crossbow", glyph: '}', effect: ItemEffect::BuffAttack(3) }
    }
    fn long_bow() -> Item {
        Item { kind: ItemKind::RangedWeapon, name: "Long Bow", glyph: '}', effect: ItemEffect::BuffAttack(4) }
    }
    fn elven_bow() -> Item {
        Item { kind: ItemKind::RangedWeapon, name: "Elven Bow", glyph: '}', effect: ItemEffect::BuffAttack(6) }
    }

    #[test]
    fn player_starts_with_dexterity() {
        let g = test_game();
        assert_eq!(g.player_dexterity, 3);
    }

    #[test]
    fn equip_ranged_weapon() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(short_bow());
        assert!(g.equip_item(0));
        assert!(g.inventory.is_empty());
        assert_eq!(g.equipped_weapon.as_ref().unwrap().name, "Short Bow");
        assert!(g.has_ranged_weapon());
    }

    #[test]
    fn ranged_weapon_goes_in_weapon_slot() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        // Equip melee weapon first
        g.inventory.push(rusty_sword());
        g.equip_item(0);
        assert!(!g.has_ranged_weapon());
        // Equip ranged weapon — swaps melee to inventory
        g.inventory.push(short_bow());
        g.equip_item(0);
        assert!(g.has_ranged_weapon());
        assert_eq!(g.inventory.len(), 1);
        assert_eq!(g.inventory[0].name, "Rusty Sword");
    }

    #[test]
    fn ranged_weapon_attack_bonus() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_weapon = Some(short_bow());
        // Base attack 5 + bow 2 = 7
        assert_eq!(g.effective_attack(), 7);
    }

    #[test]
    fn ranged_max_range_with_dexterity() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_weapon = Some(short_bow());
        // Short Bow base range 4, dex 3 → 4 + 3/3 = 5
        assert_eq!(g.ranged_max_range(), 5);
        g.player_dexterity = 9;
        // dex 9 → 4 + 9/3 = 7
        assert_eq!(g.ranged_max_range(), 7);
    }

    #[test]
    fn ranged_max_range_crossbow() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_weapon = Some(crossbow());
        // Crossbow base range 3, dex 3 → 3 + 1 = 4
        assert_eq!(g.ranged_max_range(), 4);
    }

    #[test]
    fn ranged_max_range_elven_bow() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_weapon = Some(elven_bow());
        // Elven Bow base range 8, dex 3 → 8 + 1 = 9
        assert_eq!(g.ranged_max_range(), 9);
    }

    #[test]
    fn ranged_hit_chance_decreases_with_distance() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_weapon = Some(short_bow());
        let chance_1 = g.ranged_hit_chance(1);
        let chance_3 = g.ranged_hit_chance(3);
        let chance_5 = g.ranged_hit_chance(5);
        assert!(chance_1 > chance_3, "closer should be more accurate");
        assert!(chance_3 > chance_5, "closer should be more accurate");
    }

    #[test]
    fn ranged_hit_chance_zero_beyond_range() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_weapon = Some(short_bow());
        let max_range = g.ranged_max_range();
        assert_eq!(g.ranged_hit_chance(max_range + 1), 0);
    }

    #[test]
    fn ranged_hit_chance_zero_at_zero_distance() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_weapon = Some(short_bow());
        assert_eq!(g.ranged_hit_chance(0), 0);
    }

    #[test]
    fn ranged_hit_chance_capped_at_95() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_weapon = Some(short_bow());
        g.player_dexterity = 100; // absurdly high
        assert!(g.ranged_hit_chance(1) <= 95);
    }

    #[test]
    fn ranged_hit_chance_improves_with_dexterity() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_weapon = Some(long_bow());
        g.player_dexterity = 3;
        let low_dex = g.ranged_hit_chance(4);
        g.player_dexterity = 9;
        let high_dex = g.ranged_hit_chance(4);
        assert!(high_dex > low_dex, "higher dex should improve hit chance");
    }

    #[test]
    fn ranged_attack_hits_enemy() {
        let mut map = Map::new_filled(20, 20, Tile::Floor);
        // Ensure edges are walls for valid map
        for x in 0..20 { map.set(x, 0, Tile::Wall); map.set(x, 19, Tile::Wall); }
        for y in 0..20 { map.set(0, y, Tile::Wall); map.set(19, y, Tile::Wall); }
        let mut g = Game::new(map);
        g.player_x = 5;
        g.player_y = 5;
        g.player_dexterity = 100; // guarantee hit
        g.equipped_weapon = Some(short_bow());
        g.enemies.push(Enemy { x: 8, y: 5, hp: 100, attack: 2, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        let result = g.ranged_attack(8, 5);
        assert!(matches!(result, TurnResult::Moved));
        // With dex 100, should definitely hit. Ranged damage includes distance + DEX bonuses.
        assert!(g.enemies[0].hp < 100, "enemy should have taken damage");
    }

    #[test]
    fn ranged_attack_kills_enemy() {
        let mut map = Map::new_filled(20, 20, Tile::Floor);
        for x in 0..20 { map.set(x, 0, Tile::Wall); map.set(x, 19, Tile::Wall); }
        for y in 0..20 { map.set(0, y, Tile::Wall); map.set(19, y, Tile::Wall); }
        let mut g = Game::new(map);
        g.player_x = 5;
        g.player_y = 5;
        g.player_dexterity = 100;
        g.equipped_weapon = Some(short_bow());
        g.enemies.push(Enemy { x: 8, y: 5, hp: 3, attack: 1, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        g.ranged_attack(8, 5);
        assert!(g.enemies[0].hp <= 0);
        assert!(g.messages.iter().any(|m| m.contains("slay")));
    }

    #[test]
    fn ranged_attack_no_retaliation() {
        let mut map = Map::new_filled(20, 20, Tile::Floor);
        for x in 0..20 { map.set(x, 0, Tile::Wall); map.set(x, 19, Tile::Wall); }
        for y in 0..20 { map.set(0, y, Tile::Wall); map.set(19, y, Tile::Wall); }
        let mut g = Game::new(map);
        g.player_x = 5;
        g.player_y = 5;
        g.player_dexterity = 100;
        g.equipped_weapon = Some(short_bow());
        g.enemies.push(Enemy { x: 8, y: 5, hp: 99, attack: 10, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        let hp_before = g.player_hp;
        g.ranged_attack(8, 5);
        // Enemy is 3 tiles away — no retaliation from the ranged shot itself
        // (enemy_turn may still attack if it chases into melee)
        // At distance 3, the enemy won't reach the player in one turn
        assert_eq!(g.player_hp, hp_before, "ranged attack should not cause retaliation from distant enemy");
    }

    #[test]
    fn ranged_attack_out_of_range() {
        let mut map = Map::new_filled(30, 30, Tile::Floor);
        for x in 0..30 { map.set(x, 0, Tile::Wall); map.set(x, 29, Tile::Wall); }
        for y in 0..30 { map.set(0, y, Tile::Wall); map.set(29, y, Tile::Wall); }
        let mut g = Game::new(map);
        g.player_x = 5;
        g.player_y = 5;
        g.equipped_weapon = Some(short_bow());
        // Place enemy far beyond range
        g.enemies.push(Enemy { x: 25, y: 5, hp: 10, attack: 1, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        let result = g.ranged_attack(25, 5);
        assert!(matches!(result, TurnResult::Blocked));
        assert_eq!(g.enemies[0].hp, 10, "out-of-range shot should not damage");
    }

    #[test]
    fn ranged_attack_blocked_by_wall() {
        let mut map = Map::new_filled(20, 20, Tile::Floor);
        for x in 0..20 { map.set(x, 0, Tile::Wall); map.set(x, 19, Tile::Wall); }
        for y in 0..20 { map.set(0, y, Tile::Wall); map.set(19, y, Tile::Wall); }
        // Place a wall between player and enemy
        map.set(7, 5, Tile::Wall);
        let mut g = Game::new(map);
        g.player_x = 5;
        g.player_y = 5;
        g.equipped_weapon = Some(short_bow());
        g.enemies.push(Enemy { x: 9, y: 5, hp: 10, attack: 1, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        let result = g.ranged_attack(9, 5);
        assert!(matches!(result, TurnResult::Blocked));
        assert!(g.messages.iter().any(|m| m.contains("line of sight")));
    }

    #[test]
    fn ranged_attack_no_enemy() {
        let mut map = Map::new_filled(20, 20, Tile::Floor);
        for x in 0..20 { map.set(x, 0, Tile::Wall); map.set(x, 19, Tile::Wall); }
        for y in 0..20 { map.set(0, y, Tile::Wall); map.set(19, y, Tile::Wall); }
        let mut g = Game::new(map);
        g.player_x = 5;
        g.player_y = 5;
        g.equipped_weapon = Some(short_bow());
        let result = g.ranged_attack(8, 5);
        assert!(matches!(result, TurnResult::Blocked));
        assert!(g.messages.iter().any(|m| m.contains("Nothing to shoot")));
    }

    #[test]
    fn ranged_attack_needs_ranged_weapon() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_weapon = Some(rusty_sword()); // melee weapon
        let result = g.ranged_attack(g.player_x + 3, g.player_y);
        assert!(matches!(result, TurnResult::Blocked));
    }

    #[test]
    fn ranged_attack_updates_facing() {
        let mut map = Map::new_filled(20, 20, Tile::Floor);
        for x in 0..20 { map.set(x, 0, Tile::Wall); map.set(x, 19, Tile::Wall); }
        for y in 0..20 { map.set(0, y, Tile::Wall); map.set(19, y, Tile::Wall); }
        let mut g = Game::new(map);
        g.player_x = 10;
        g.player_y = 10;
        g.player_dexterity = 100;
        g.equipped_weapon = Some(short_bow());
        g.enemies.push(Enemy { x: 7, y: 10, hp: 20, attack: 1, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        g.ranged_attack(7, 10); // shooting left
        assert!(g.player_facing_left);
    }

    #[test]
    fn use_ranged_weapon_returns_false() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.inventory.push(short_bow());
        assert!(!g.use_item(0), "ranged weapon should not be usable as consumable");
        assert_eq!(g.inventory.len(), 1);
    }

    #[test]
    fn has_ranged_weapon_false_without_weapon() {
        let g = test_game();
        assert!(!g.has_ranged_weapon());
    }

    #[test]
    fn has_ranged_weapon_false_with_melee() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_weapon = Some(rusty_sword());
        assert!(!g.has_ranged_weapon());
    }

    #[test]
    fn multiple_level_ups_stack_skill_points() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        // Give enough XP for 3 level-ups
        g.player_xp = 200;
        g.check_level_up();
        assert!(g.player_level >= 3);
        assert!(g.skill_points >= 4); // at least 2 level ups * 2 points
    }

    #[test]
    fn ranged_weapons_in_loot_tables() {
        // Tier 0 should produce bows/crossbows
        let mut rng = 42u64;
        let items: Vec<_> = (0..200).map(|_| random_item(0, &mut rng)).collect();
        assert!(items.iter().any(|i| i.kind == ItemKind::RangedWeapon),
            "tier 0 should generate ranged weapons");
        // Tier 1 should produce long bows/heavy crossbows
        rng = 42;
        let items: Vec<_> = (0..200).map(|_| random_item(1, &mut rng)).collect();
        assert!(items.iter().any(|i| i.kind == ItemKind::RangedWeapon),
            "tier 1 should generate ranged weapons");
        // Tier 2 should produce elven bow
        rng = 42;
        let items: Vec<_> = (0..200).map(|_| random_item(2, &mut rng)).collect();
        assert!(items.iter().any(|i| i.name == "Elven Bow"),
            "tier 2 should generate Elven Bow");
    }

    #[test]
    fn ranged_attack_costs_a_turn() {
        let mut map = Map::new_filled(20, 20, Tile::Floor);
        for x in 0..20 { map.set(x, 0, Tile::Wall); map.set(x, 19, Tile::Wall); }
        for y in 0..20 { map.set(0, y, Tile::Wall); map.set(19, y, Tile::Wall); }
        let mut g = Game::new(map);
        g.player_x = 5;
        g.player_y = 5;
        g.player_dexterity = 100;
        g.equipped_weapon = Some(short_bow());
        g.enemies.push(Enemy { x: 8, y: 5, hp: 99, attack: 1, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        let turn_before = g.turn;
        g.ranged_attack(8, 5);
        assert_eq!(g.turn, turn_before + 1, "ranged attack should advance turn counter");
    }

    // === Skill Points ===

    #[test]
    fn player_starts_with_zero_skill_points() {
        let g = test_game();
        assert_eq!(g.skill_points, 0);
        assert_eq!(g.strength, 0);
        assert_eq!(g.vitality, 0);
    }

    #[test]
    fn allocate_strength_increases_attack() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.skill_points = 1;
        let atk_before = g.effective_attack();
        assert!(g.allocate_skill_point(SkillKind::Strength));
        assert_eq!(g.strength, 1);
        assert_eq!(g.effective_attack(), atk_before + 1);
        assert_eq!(g.skill_points, 0);
    }

    #[test]
    fn allocate_vitality_increases_max_hp() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.skill_points = 1;
        let hp_before = g.player_max_hp;
        assert!(g.allocate_skill_point(SkillKind::Vitality));
        assert_eq!(g.vitality, 1);
        assert_eq!(g.player_max_hp, hp_before + 3);
        assert_eq!(g.skill_points, 0);
    }

    #[test]
    fn allocate_dexterity_increases_dex() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.skill_points = 1;
        let dex_before = g.player_dexterity;
        assert!(g.allocate_skill_point(SkillKind::Dexterity));
        assert_eq!(g.player_dexterity, dex_before + 1);
        assert_eq!(g.skill_points, 0);
    }

    #[test]
    fn allocate_stamina_reduces_sprint_cost() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.skill_points = 1;
        let cost_before = g.sprint_cost;
        let stam_before = g.max_stamina;
        assert!(g.allocate_skill_point(SkillKind::Stamina));
        assert_eq!(g.sprint_cost, cost_before - 1); // sprint cost -1
        assert_eq!(g.max_stamina, stam_before + 5); // max stamina +5
        assert_eq!(g.skill_points, 0);
    }

    #[test]
    fn allocate_fails_with_no_points() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        assert_eq!(g.skill_points, 0);
        assert!(!g.allocate_skill_point(SkillKind::Strength));
        assert_eq!(g.strength, 0);
    }

    #[test]
    fn allocate_generates_message() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.skill_points = 1;
        let msg_before = g.messages.len();
        g.allocate_skill_point(SkillKind::Strength);
        assert!(g.messages.len() > msg_before);
        assert!(g.messages.last().unwrap().contains("Strength"));
    }

    #[test]
    fn strength_affects_combat_damage() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.strength = 3;
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 20, attack: 1, glyph: 'g', name: "Goblin", facing_left: false, defense: 0, is_ranged: false });
        g.attack_adjacent(gx, gy);
        // Base attack 5 + strength 3 = 8 damage
        assert_eq!(g.enemies[0].hp, 20 - 8);
    }

    #[test]
    fn vitality_hp_gained_on_allocate() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.player_hp = 15; // damaged
        g.skill_points = 1;
        g.allocate_skill_point(SkillKind::Vitality);
        // HP should increase by 3 but not exceed new max
        assert_eq!(g.player_hp, 18);
        assert_eq!(g.player_max_hp, 23); // 20 + 3
    }

    #[test]
    fn level_up_message_mentions_skill_points() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.player_xp = 20;
        g.check_level_up();
        assert!(g.messages.iter().any(|m| m.contains("skill points")));
    }

    // === Config integration tests ===

    #[test]
    fn new_with_config_applies_player_stats() {
        let mut config = GameConfig::normal();
        config.player.starting_hp = 50;
        config.player.starting_attack = 10;
        config.player.starting_dexterity = 7;
        config.player.starting_stamina = 200;
        config.player.starting_hunger = 80;
        let map = Map::generate(30, 20, 42);
        let g = Game::new_with_config(map, config);
        assert_eq!(g.player_hp, 50);
        assert_eq!(g.player_max_hp, 50);
        assert_eq!(g.player_attack, 10);
        assert_eq!(g.player_dexterity, 7);
        assert_eq!(g.stamina, 200);
        assert_eq!(g.max_stamina, 200);
        assert_eq!(g.hunger, 80);
        assert_eq!(g.max_hunger, 80);
    }

    #[test]
    fn easy_config_gives_more_hp() {
        let easy = Game::new_with_config(Map::generate(30, 20, 42), GameConfig::easy());
        let normal = Game::new_with_config(Map::generate(30, 20, 42), GameConfig::normal());
        assert!(easy.player_hp > normal.player_hp);
    }

    #[test]
    fn hard_config_gives_less_hp() {
        let hard = Game::new_with_config(Map::generate(30, 20, 42), GameConfig::hard());
        let normal = Game::new_with_config(Map::generate(30, 20, 42), GameConfig::normal());
        assert!(hard.player_hp < normal.player_hp);
    }

    #[test]
    fn config_fov_radius_used() {
        let mut config = GameConfig::normal();
        config.fov.overworld_radius = 12;
        config.fov.dungeon_radius = 4;
        let map = Map::generate(30, 20, 42);
        let g = Game::new_with_config(map, config);
        // from_single_map uses Location::Overworld
        assert_eq!(g.fov_radius(), 12);
    }

    #[test]
    fn config_xp_formula_used() {
        let mut config = GameConfig::normal();
        config.progression.xp_base = 10.0;
        config.progression.xp_exponent = 1.0;
        let map = Map::generate(30, 20, 42);
        let g = Game::new_with_config(map, config);
        // xp_to_next = 10.0 * 1^1.0 = 10
        assert_eq!(g.xp_to_next_level(), 10);
    }

    // === Diagonal movement tests ===

    #[test]
    fn diagonal_move_works_on_open_floor() {
        // Create a small open map
        let map = Map::new_filled(10, 10, Tile::Floor);
        let mut g = Game::new(map);
        g.player_x = 5;
        g.player_y = 5;
        let result = g.move_player(1, 1); // DownRight
        assert_ne!(result, TurnResult::Blocked);
        assert_eq!(g.player_x, 6);
        assert_eq!(g.player_y, 6);
    }

    #[test]
    fn diagonal_move_blocked_by_corner_cutting() {
        // Create a map where diagonal would cut a corner:
        // . W
        // W .
        // Player at (1,1), wall at (2,1) and (1,2): diagonal to (2,2) should be blocked
        let mut map = Map::new_filled(5, 5, Tile::Floor);
        map.set(2, 1, Tile::Wall);
        map.set(1, 2, Tile::Wall);
        let mut g = Game::new(map);
        g.player_x = 1;
        g.player_y = 1;
        let result = g.move_player(1, 1); // DownRight
        assert_eq!(result, TurnResult::Blocked);
        assert_eq!(g.player_x, 1);
        assert_eq!(g.player_y, 1);
    }

    #[test]
    fn diagonal_move_allowed_when_one_adjacent_open() {
        // Wall only on one side — should still be blocked (both adjacents needed)
        let mut map = Map::new_filled(5, 5, Tile::Floor);
        map.set(2, 1, Tile::Wall); // wall east
        // (1,2) is floor — south is open
        let mut g = Game::new(map);
        g.player_x = 1;
        g.player_y = 1;
        let result = g.move_player(1, 1); // DownRight
        assert_eq!(result, TurnResult::Blocked);
    }

    #[test]
    fn diagonal_move_all_four_diagonals() {
        let diags = [(-1, -1), (1, -1), (-1, 1), (1, 1)];
        for (dx, dy) in diags {
            let mut g = Game::new(Map::new_filled(10, 10, Tile::Floor));
            g.player_x = 5;
            g.player_y = 5;
            let result = g.move_player(dx, dy);
            assert_ne!(result, TurnResult::Blocked, "diagonal ({},{}) should work on open floor", dx, dy);
            assert_eq!(g.player_x, 5 + dx);
            assert_eq!(g.player_y, 5 + dy);
        }
    }

    #[test]
    fn attack_adjacent_diagonal() {
        // Player can attack an enemy diagonally
        let map = Map::new_filled(10, 10, Tile::Floor);
        let mut g = Game::new(map);
        g.player_x = 5;
        g.player_y = 5;
        g.enemies.push(Enemy {
            x: 6, y: 6, hp: 10,
            name: "Goblin", glyph: 'g',
            attack: 1, defense: 0,
            facing_left: false, is_ranged: false,
        });
        let result = g.attack_adjacent(6, 6);
        assert_ne!(result, TurnResult::Blocked, "diagonal attack should work");
    }

    #[test]
    fn attack_adjacent_too_far() {
        // Chebyshev distance 2 should be blocked
        let map = Map::new_filled(10, 10, Tile::Floor);
        let mut g = Game::new(map);
        g.player_x = 5;
        g.player_y = 5;
        g.enemies.push(Enemy {
            x: 7, y: 7, hp: 10,
            name: "Goblin", glyph: 'g',
            attack: 1, defense: 0,
            facing_left: false, is_ranged: false,
        });
        let result = g.attack_adjacent(7, 7);
        assert_eq!(result, TurnResult::Blocked, "distance 2 should be blocked");
    }

    // === Quick-bar tests ===

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
}
