use crate::map::{Map, Tile};
use crate::world::{Location, World};

pub const MAX_INVENTORY: usize = 10;

// Survival constants
const SPRINT_COST: i32 = 15;
const STAMINA_REGEN: i32 = 5;
const HUNGER_DRAIN: i32 = 1;
const STARVATION_DAMAGE: i32 = 1;

#[derive(Clone, Debug, PartialEq)]
pub enum ItemKind {
    Potion,
    Scroll,
    Weapon,
    Armor,
    Helmet,
    Shield,
    Boots,
    Food,
    Ring,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ItemEffect {
    Heal(i32),
    DamageAoe(i32),
    BuffAttack(i32),
    BuffDefense(i32),
    /// Restores hunger points.
    Feed(i32),
}

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub kind: ItemKind,
    pub name: &'static str,
    pub glyph: char,
    pub effect: ItemEffect,
}

#[derive(Clone, Debug)]
pub struct GroundItem {
    pub x: i32,
    pub y: i32,
    pub item: Item,
}

#[derive(Clone)]
pub struct Enemy {
    pub x: i32,
    pub y: i32,
    pub hp: i32,
    pub attack: i32,
    pub glyph: char,
    pub name: &'static str,
    /// true when sprite should face left (mirrored).
    pub facing_left: bool,
}

pub enum TurnResult {
    Moved,
    Blocked,
    Attacked { target_name: &'static str, damage: i32 },
    Killed { target_name: &'static str },
    PlayerDied,
    Won,
    /// Player stepped on a transition tile and changed maps.
    MapChanged,
}

/// Structured info about a tile at a specific world position.
#[derive(Debug, Clone, PartialEq)]
pub struct TileInfo {
    pub tile_name: &'static str,
    pub tile_desc: &'static str,
    pub walkable: bool,
    pub enemy: Option<EnemyInfo>,
    pub item: Option<ItemInfo>,
    pub is_player: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnemyInfo {
    pub name: &'static str,
    pub hp: i32,
    pub attack: i32,
    pub desc: &'static str,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ItemInfo {
    pub name: &'static str,
    pub desc: String,
}

fn tile_name(tile: Tile) -> &'static str {
    match tile {
        Tile::Wall => "Stone Wall",
        Tile::Floor => "Stone Floor",
        Tile::Tree => "Dense Tree",
        Tile::Grass => "Grass",
        Tile::Road => "Road",
        Tile::DungeonEntrance => "Dungeon Entrance",
        Tile::StairsDown => "Stairs Down",
        Tile::StairsUp => "Stairs Up",
    }
}

fn tile_desc(tile: Tile) -> &'static str {
    match tile {
        Tile::Wall => "A solid stone wall. Impassable.",
        Tile::Floor => "Rough stone floor. Watch your step.",
        Tile::Tree => "Thick forest. Cannot pass through.",
        Tile::Grass => "An open clearing in the forest.",
        Tile::Road => "A well-worn path between dungeons.",
        Tile::DungeonEntrance => "A dark passage leading underground. Step on to enter.",
        Tile::StairsDown => "Crumbling stairs descending deeper.",
        Tile::StairsUp => "Stairs leading back up.",
    }
}

fn enemy_desc(name: &str) -> &'static str {
    match name {
        // Forest beasts
        "Giant Rat"       => "A disease-carrying rodent the size of a dog.",
        "Giant Bat"       => "A bat with a wingspan wider than a man.",
        "Wolf"            => "A cunning pack hunter. Fast and relentless.",
        "Giant Spider"    => "A venomous arachnid that lurks in the shadows.",
        "Boar"            => "A ferocious wild pig with razor-sharp tusks.",
        "Bear"            => "A massive predator. Top of the forest chain.",
        "Lycanthrope"     => "A cursed shapeshifter. Savage in beast form.",
        // Dungeon — shallow
        "Kobold"          => "A small reptilian scavenger. Cowardly but cunning.",
        "Small Slime"     => "A translucent ooze. Dissolves what it touches.",
        "Goblin"          => "A sneaky green creature. Dangerous in numbers.",
        "Skeleton"        => "Animated bones bound by dark magic.",
        // Dungeon — mid
        "Goblin Archer"   => "A goblin with a crude bow. Deadly at range.",
        "Zombie"          => "A shambling corpse. Slow but relentless.",
        "Skeleton Archer" => "Dead bones with unerring aim.",
        "Big Slime"       => "A massive ooze. Absorbs blows like nothing.",
        "Orc"             => "A fierce tribal warrior. Bred for battle.",
        // Dungeon — deep
        "Ghoul"           => "A ravenous undead. Paralyzes with its claws.",
        "Orc Blademaster" => "An elite orc warrior. Master of the blade.",
        "Wraith"          => "A hateful spirit. Drains the life from victims.",
        "Naga"            => "A serpentine spellcaster. Ancient and cunning.",
        "Troll"           => "A towering brute. Regenerates from any wound.",
        // Cave — boss floor
        "Death Knight"    => "A fallen paladin. Commands undead legions.",
        "Lich"            => "An undead sorcerer of immense power.",
        "Dragon"          => "The cave's ancient guardian. Legendary power.",
        _ => "A mysterious creature.",
    }
}

fn xp_for_enemy(name: &str) -> u32 {
    match name {
        // Forest
        "Giant Rat" => 3,
        "Giant Bat" => 4,
        "Wolf" => 5,
        "Giant Spider" => 6,
        "Boar" => 7,
        "Bear" => 12,
        "Lycanthrope" => 18,
        // Dungeon — shallow
        "Kobold" => 3,
        "Small Slime" => 3,
        "Goblin" => 4,
        "Skeleton" => 6,
        // Dungeon — mid
        "Goblin Archer" => 5,
        "Zombie" => 6,
        "Skeleton Archer" => 7,
        "Big Slime" => 7,
        "Orc" => 10,
        // Dungeon — deep
        "Ghoul" => 11,
        "Orc Blademaster" => 14,
        "Wraith" => 13,
        "Naga" => 16,
        "Troll" => 15,
        // Cave
        "Death Knight" => 22,
        "Lich" => 25,
        "Dragon" => 100,
        _ => 3,
    }
}

fn item_info_desc(item: &Item) -> String {
    let effect = match &item.effect {
        ItemEffect::Heal(n) => format!("Restores {} HP", n),
        ItemEffect::DamageAoe(n) => format!("Deals {} damage in area", n),
        ItemEffect::BuffAttack(n) => format!("+{} Attack", n),
        ItemEffect::BuffDefense(n) => format!("+{} Defense", n),
        ItemEffect::Feed(n) => format!("Restores {} hunger", n),
    };
    format!("{} — {}", item.name, effect)
}

/// Which bottom drawer is currently open.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Drawer {
    None,
    Inventory,
    Stats,
}

pub struct Game {
    pub player_x: i32,
    pub player_y: i32,
    pub player_hp: i32,
    pub player_max_hp: i32,
    pub player_attack: i32,
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
    pub inventory_open: bool,
    /// Currently open drawer (slides up from bottom).
    pub drawer: Drawer,
    /// Tile currently being inspected (shown in HUD detail strip).
    pub inspected: Option<TileInfo>,
    /// Player XP and level for progression.
    pub player_xp: u32,
    pub player_level: u32,
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
}

impl Game {
    pub fn new(map: Map) -> Self {
        let (px, py) = map.find_spawn();
        Self {
            player_x: px,
            player_y: py,
            player_hp: 20,
            player_max_hp: 20,
            player_attack: 5,
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
            inventory_open: false,
            drawer: Drawer::None,
            inspected: None,
            player_xp: 0,
            player_level: 1,
            stamina: 100,
            max_stamina: 100,
            sprinting: false,
            hunger: 100,
            max_hunger: 100,
            turn: 0,
        }
    }

    pub fn new_overworld(world: World) -> Self {
        let (px, py) = world.overworld.find_road_spawn();
        Self {
            player_x: px,
            player_y: py,
            player_hp: 20,
            player_max_hp: 20,
            player_attack: 5,
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
            inventory_open: false,
            drawer: Drawer::None,
            inspected: None,
            player_xp: 0,
            player_level: 1,
            stamina: 100,
            max_stamina: 100,
            sprinting: false,
            hunger: 100,
            max_hunger: 100,
            turn: 0,
        }
    }

    /// Convenience accessor for the current map.
    pub fn current_map(&self) -> &Map {
        self.world.current_map()
    }

    /// FOV radius: 8 on overworld, 6 in dungeons.
    fn fov_radius(&self) -> i32 {
        match self.world.location {
            Location::Overworld => 8,
            Location::Dungeon { .. } => 6,
        }
    }

    /// Age Visible→Seen, then recompute FOV from player position.
    pub fn update_fov(&mut self) {
        let r = self.fov_radius();
        let map = self.world.current_map_mut();
        map.age_visibility();
        map.compute_fov(self.player_x, self.player_y, r);
    }

    /// Player's total attack: base + weapon bonus.
    pub fn effective_attack(&self) -> i32 {
        let mut total = self.player_attack;
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

    pub fn toggle_inventory(&mut self) {
        self.inventory_open = !self.inventory_open;
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

    /// Returns a description of the current location for the HUD.
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
                    self.inventory.remove(index);
                    return true;
                }
                false
            }
            ItemKind::Scroll => {
                if let ItemEffect::DamageAoe(damage) = item.effect {
                    let name = item.name;
                    self.messages.push(format!("You read {name}!"));
                    self.inventory.remove(index);
                    // Damage all enemies within 3 tiles
                    let px = self.player_x;
                    let py = self.player_y;
                    for enemy in &mut self.enemies {
                        if enemy.hp <= 0 { continue; }
                        let dist = (enemy.x - px).abs() + (enemy.y - py).abs();
                        if dist <= 3 {
                            enemy.hp -= damage;
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
            _ => false, // Weapons/Armor should be equipped, not used
        }
    }

    /// Equip an item from inventory into its matching slot. Returns true if equipped.
    /// If a slot is already occupied, the old item goes back to inventory.
    pub fn equip_item(&mut self, index: usize) -> bool {
        if index >= self.inventory.len() {
            return false;
        }
        let slot = match self.inventory[index].kind {
            ItemKind::Weapon  => &mut self.equipped_weapon,
            ItemKind::Armor   => &mut self.equipped_armor,
            ItemKind::Helmet  => &mut self.equipped_helmet,
            ItemKind::Shield  => &mut self.equipped_shield,
            ItemKind::Boots   => &mut self.equipped_boots,
            ItemKind::Ring    => &mut self.equipped_ring,
            _ => return false, // Potions/Scrolls/Food should be used, not equipped
        };
        let new_item = self.inventory.remove(index);
        let name = new_item.name;
        if let Some(old) = slot.replace(new_item) {
            self.messages.push(format!("You swap {} for {name}.", old.name));
            self.inventory.push(old);
        } else {
            self.messages.push(format!("You equip {name}."));
        }
        true
    }

    /// Drop an item from inventory onto the ground. Returns true if dropped.
    pub fn drop_item(&mut self, index: usize) -> bool {
        if index >= self.inventory.len() {
            return false;
        }
        let item = self.inventory.remove(index);
        let name = item.name;
        self.messages.push(format!("You drop {name}."));
        self.ground_items.push(GroundItem {
            x: self.player_x,
            y: self.player_y,
            item,
        });
        true
    }

    /// Auto-pickup items at the player's position.
    fn pickup_items(&mut self) {
        let px = self.player_x;
        let py = self.player_y;
        let mut i = 0;
        while i < self.ground_items.len() {
            if self.ground_items[i].x == px && self.ground_items[i].y == py {
                if self.inventory.len() >= MAX_INVENTORY {
                    self.messages.push("Inventory full!".into());
                    break;
                }
                let gi = self.ground_items.remove(i);
                self.messages.push(format!("Picked up {}.", gi.item.name));
                self.inventory.push(gi.item);
            } else {
                i += 1;
            }
        }
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
                // ~0.3% chance on roads, ~0.1% on grass
                let threshold = if tile == Tile::Road { 3 } else { 1 };
                if rng % 1000 >= threshold {
                    continue;
                }
                rng = xorshift64(rng);
                let item = random_item(0, &mut rng);
                self.ground_items.push(GroundItem { x, y, item });
            }
        }
    }

    /// Spawn food on the overworld: berries, mushrooms, herbs, water on grass tiles.
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
                // ~0.8% chance per grass tile
                if rng % 1000 >= 8 {
                    continue;
                }
                rng = xorshift64(rng);
                let roll = rng % 100;
                let food = if roll < 35 {
                    Item { kind: ItemKind::Food, name: "Wild Berries", glyph: '%', effect: ItemEffect::Feed(8) }
                } else if roll < 60 {
                    Item { kind: ItemKind::Food, name: "Mushrooms", glyph: '%', effect: ItemEffect::Feed(10) }
                } else if roll < 80 {
                    Item { kind: ItemKind::Food, name: "Fresh Herbs", glyph: '%', effect: ItemEffect::Feed(6) }
                } else {
                    Item { kind: ItemKind::Food, name: "Clean Water", glyph: '~', effect: ItemEffect::Feed(5) }
                };
                self.ground_items.push(GroundItem { x, y, item: food });
            }
        }
    }

    /// Spawn items on a dungeon level. Deeper = better loot.
    fn spawn_dungeon_items(&mut self, dungeon_index: usize, level: usize) {
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
                // ~5% chance per floor tile in dungeons, ~3% in cave
                let threshold = if is_cave { 3 } else { 5 };
                if rng % 100 >= threshold {
                    continue;
                }
                rng = xorshift64(rng);
                let tier = if is_cave { 2 } else { level };
                let item = random_item(tier, &mut rng);
                self.ground_items.push(GroundItem { x, y, item });
            }
        }
    }

    pub fn toggle_sprint(&mut self) {
        if self.sprinting {
            self.sprinting = false;
            self.messages.push("Sprint off.".into());
        } else if self.stamina >= SPRINT_COST {
            self.sprinting = true;
            self.messages.push("Sprint on!".into());
        } else {
            self.messages.push("Too exhausted to sprint.".into());
        }
    }

    /// Called each turn the player moves. Handles stamina drain/regen and hunger.
    fn tick_survival(&mut self) {
        self.turn += 1;

        // Stamina: sprint drains, walking regenerates
        if self.sprinting {
            self.stamina -= SPRINT_COST;
            if self.stamina <= 0 {
                self.stamina = 0;
                self.sprinting = false;
                self.messages.push("Exhausted! Sprint disabled.".into());
            }
        } else {
            self.stamina = (self.stamina + STAMINA_REGEN).min(self.max_stamina);
        }

        // Hunger: decrease every turn
        self.hunger -= HUNGER_DRAIN;
        if self.hunger < 0 { self.hunger = 0; }

        // Starvation damage
        if self.hunger == 0 {
            self.player_hp -= STARVATION_DAMAGE;
            if self.turn % 5 == 0 {
                self.messages.push("You are starving!".into());
            }
            if self.player_hp <= 0 {
                self.alive = false;
                self.messages.push("You starved to death.".into());
            }
        }
    }

    /// Eat food from inventory. Returns true if eaten.
    pub fn eat_food(&mut self, index: usize) -> bool {
        if index >= self.inventory.len() {
            return false;
        }
        if self.inventory[index].kind != ItemKind::Food {
            return false;
        }
        if let ItemEffect::Feed(amount) = self.inventory[index].effect {
            let old = self.hunger;
            self.hunger = (self.hunger + amount).min(self.max_hunger);
            let gained = self.hunger - old;
            let name = self.inventory[index].name;
            self.messages.push(format!("You eat {name}. Hunger +{gained}."));
            self.inventory.remove(index);
            true
        } else {
            false
        }
    }

    pub fn toggle_drawer(&mut self, drawer: Drawer) {
        if self.drawer == drawer {
            self.drawer = Drawer::None;
        } else {
            self.drawer = drawer;
        }
    }

    /// XP required to reach next level: 20 * current_level^1.5 (rounded).
    pub fn xp_to_next_level(&self) -> u32 {
        (20.0 * (self.player_level as f64).powf(1.5)).round() as u32
    }

    fn check_level_up(&mut self) {
        while self.player_xp >= self.xp_to_next_level() {
            self.player_xp -= self.xp_to_next_level();
            self.player_level += 1;
            self.player_max_hp += 3;
            self.player_hp = self.player_max_hp;
            self.player_attack += 1;
            self.messages.push(format!(
                "Level up! You are now level {}. HP+3, ATK+1.",
                self.player_level
            ));
        }
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
                // ~3% chance per walkable tile (forest is sparse)
                if rng % 100 < 3 {
                    rng = xorshift64(rng);
                    let roll = rng % 100;
                    let (hp, attack, glyph, name) = if roll < 20 {
                        (3, 1, 'r', "Giant Rat")
                    } else if roll < 35 {
                        (4, 2, 'a', "Giant Bat")
                    } else if roll < 60 {
                        (5, 2, 'w', "Wolf")
                    } else if roll < 75 {
                        (6, 3, 'i', "Giant Spider")
                    } else if roll < 87 {
                        (8, 2, 'b', "Boar")
                    } else if roll < 95 {
                        (12, 4, 'B', "Bear")
                    } else {
                        (14, 5, 'L', "Lycanthrope")
                    };
                    self.enemies.push(Enemy { x, y, hp, attack, glyph, name, facing_left: false });
                }
            }
        }
    }

    /// Spawn enemies appropriate for a dungeon level.
    /// L0: rats, kobolds, slimes, goblins, skeletons.
    /// L1: goblin archers, zombies, skeleton archers, big slimes, orcs.
    /// L2+: ghouls, orc blademasters, wraiths, nagas, trolls.
    /// Cave (L3, dragon dungeon only): death knights, trolls, liches + dragon boss.
    fn spawn_dungeon_enemies(&mut self, dungeon_index: usize, level: usize) {
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
                let spawn_chance = if is_cave { 6 } else { 10 };
                if rng % 100 < spawn_chance {
                    rng = xorshift64(rng);
                    let roll = rng % 100;
                    let (hp, attack, glyph, name) = if is_cave {
                        if roll < 40 {
                            (20, 7, 'K', "Death Knight")
                        } else if roll < 70 {
                            (16, 5, 'T', "Troll")
                        } else {
                            (15, 8, 'l', "Lich")
                        }
                    } else {
                        match level {
                            0 => {
                                if roll < 25 {
                                    (3, 1, 'r', "Giant Rat")
                                } else if roll < 40 {
                                    (4, 2, 'c', "Kobold")
                                } else if roll < 55 {
                                    (4, 1, 'S', "Small Slime")
                                } else if roll < 80 {
                                    (5, 2, 'g', "Goblin")
                                } else {
                                    (6, 3, 's', "Skeleton")
                                }
                            }
                            1 => {
                                if roll < 20 {
                                    (6, 3, 'G', "Goblin Archer")
                                } else if roll < 40 {
                                    (10, 2, 'z', "Zombie")
                                } else if roll < 55 {
                                    (7, 4, 'k', "Skeleton Archer")
                                } else if roll < 70 {
                                    (10, 2, 'm', "Big Slime")
                                } else {
                                    (10, 4, 'o', "Orc")
                                }
                            }
                            _ => {
                                if roll < 20 {
                                    (10, 5, 'u', "Ghoul")
                                } else if roll < 40 {
                                    (14, 5, 'O', "Orc Blademaster")
                                } else if roll < 55 {
                                    (8, 6, 'W', "Wraith")
                                } else if roll < 70 {
                                    (12, 6, 'N', "Naga")
                                } else {
                                    (16, 5, 'T', "Troll")
                                }
                            }
                        }
                    };
                    self.enemies.push(Enemy { x, y, hp, attack, glyph, name, facing_left: false });
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
                            x, y, hp: 30, attack: 8, glyph: 'D', name: "Dragon", facing_left: false,
                        });
                        return;
                    }
                }
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

        // Check for enemy at target
        if let Some(idx) = self.enemies.iter().position(|e| e.x == nx && e.y == ny && e.hp > 0) {
            let dmg = self.effective_attack();
            self.enemies[idx].hp -= dmg;
            let name = self.enemies[idx].name;

            if self.enemies[idx].hp <= 0 {
                let xp = xp_for_enemy(name);
                let ex = self.enemies[idx].x;
                let ey = self.enemies[idx].y;
                self.player_xp += xp;
                self.check_level_up();
                self.messages.push(format!("You slay the {name}! (+{xp} XP)"));
                // Animals drop meat
                if let Some(meat) = meat_drop(name) {
                    self.ground_items.push(GroundItem { x: ex, y: ey, item: meat });
                    self.messages.push("It dropped some meat.".into());
                }
                // Check win: dragon killed
                if self.enemies[idx].glyph == 'D' {
                    self.won = true;
                    self.messages.push("You conquered the cave!".into());
                    return TurnResult::Won;
                }
                return TurnResult::Killed { target_name: name };
            }
            self.messages.push(format!("You hit {name} for {dmg} damage."));

            // Enemy retaliates — reduced by player defense
            let raw = self.enemies[idx].attack;
            let retaliation = (raw - self.effective_defense()).max(1);
            self.player_hp -= retaliation;
            self.messages.push(format!("{name} hits you for {retaliation} damage."));
            if self.player_hp <= 0 {
                self.alive = false;
                self.messages.push("You died.".into());
                return TurnResult::PlayerDied;
            }

            return TurnResult::Attacked { target_name: name, damage: dmg };
        }

        if !self.world.current_map().is_walkable(nx, ny) {
            return TurnResult::Blocked;
        }

        self.player_x = nx;
        self.player_y = ny;

        // Auto-pickup items at new position
        self.pickup_items();

        // Check for map transitions
        let tile = self.world.current_map().get(nx, ny);
        if self.try_transition(tile, nx, ny) {
            return TurnResult::MapChanged;
        }

        // Enemies take a turn (skip if sprinting — player outruns them)
        if !self.sprinting {
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

    fn enemy_turn(&mut self) {
        let px = self.player_x;
        let py = self.player_y;

        for i in 0..self.enemies.len() {
            if self.enemies[i].hp <= 0 {
                continue;
            }
            let ex = self.enemies[i].x;
            let ey = self.enemies[i].y;
            let dist = (ex - px).abs() + (ey - py).abs();

            // Chase if within 5 tiles
            if dist <= 5 && dist > 1 {
                let dx = (px - ex).signum();
                let dy = (py - ey).signum();
                let candidates = [(ex + dx, ey), (ex, ey + dy)];
                for (cx, cy) in candidates {
                    if cx == px && cy == py {
                        // Attack player — reduced by defense
                        let raw = self.enemies[i].attack;
                        let atk = (raw - self.effective_defense()).max(1);
                        let name = self.enemies[i].name;
                        self.player_hp -= atk;
                        self.messages.push(format!("{name} hits you for {atk} damage."));
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
}

/// Generate a random item appropriate for the given dungeon tier.
/// Tier 0 = shallow/overworld, 1 = mid, 2+ = deep.
fn random_item(tier: usize, rng: &mut u64) -> Item {
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
            } else if roll < 52 {
                match sub {
                    0 => Item { kind: ItemKind::Weapon, name: "Rusty Sword", glyph: '/', effect: ItemEffect::BuffAttack(2) },
                    1 => Item { kind: ItemKind::Weapon, name: "Iron Dagger", glyph: '/', effect: ItemEffect::BuffAttack(1) },
                    _ => Item { kind: ItemKind::Weapon, name: "Wooden Club", glyph: '/', effect: ItemEffect::BuffAttack(2) },
                }
            } else if roll < 58 {
                Item { kind: ItemKind::Armor, name: "Leather Armor", glyph: '[', effect: ItemEffect::BuffDefense(2) }
            } else if roll < 63 {
                Item { kind: ItemKind::Helmet, name: "Leather Cap", glyph: '^', effect: ItemEffect::BuffDefense(1) }
            } else if roll < 68 {
                Item { kind: ItemKind::Shield, name: "Wooden Shield", glyph: ')', effect: ItemEffect::BuffDefense(1) }
            } else if roll < 73 {
                Item { kind: ItemKind::Boots, name: "Leather Boots", glyph: '{', effect: ItemEffect::BuffDefense(1) }
            } else if roll < 82 {
                Item { kind: ItemKind::Ring, name: "Copper Ring", glyph: '=', effect: ItemEffect::BuffAttack(1) }
            } else {
                match sub {
                    0 => Item { kind: ItemKind::Food, name: "Stale Bread", glyph: '%', effect: ItemEffect::Feed(8) },
                    1 => Item { kind: ItemKind::Food, name: "Waterskin", glyph: '~', effect: ItemEffect::Feed(6) },
                    _ => Item { kind: ItemKind::Food, name: "Wild Berries", glyph: '%', effect: ItemEffect::Feed(8) },
                }
            }
        }
        1 => {
            if roll < 24 {
                Item { kind: ItemKind::Potion, name: "Greater Health Potion", glyph: '!', effect: ItemEffect::Heal(10) }
            } else if roll < 36 {
                Item { kind: ItemKind::Scroll, name: "Scroll of Lightning", glyph: '?', effect: ItemEffect::DamageAoe(12) }
            } else if roll < 48 {
                match sub {
                    0 => Item { kind: ItemKind::Weapon, name: "Iron Sword", glyph: '/', effect: ItemEffect::BuffAttack(4) },
                    1 => Item { kind: ItemKind::Weapon, name: "Battle Axe", glyph: '/', effect: ItemEffect::BuffAttack(5) },
                    _ => Item { kind: ItemKind::Weapon, name: "War Hammer", glyph: '/', effect: ItemEffect::BuffAttack(4) },
                }
            } else if roll < 54 {
                Item { kind: ItemKind::Armor, name: "Chain Mail", glyph: '[', effect: ItemEffect::BuffDefense(4) }
            } else if roll < 59 {
                Item { kind: ItemKind::Helmet, name: "Iron Helmet", glyph: '^', effect: ItemEffect::BuffDefense(3) }
            } else if roll < 64 {
                Item { kind: ItemKind::Shield, name: "Iron Shield", glyph: ')', effect: ItemEffect::BuffDefense(3) }
            } else if roll < 69 {
                Item { kind: ItemKind::Boots, name: "Chain Boots", glyph: '{', effect: ItemEffect::BuffDefense(2) }
            } else if roll < 78 {
                match sub {
                    0 => Item { kind: ItemKind::Ring, name: "Silver Ring", glyph: '=', effect: ItemEffect::BuffDefense(2) },
                    _ => Item { kind: ItemKind::Ring, name: "Ruby Ring", glyph: '=', effect: ItemEffect::BuffAttack(3) },
                }
            } else {
                match sub {
                    0 => Item { kind: ItemKind::Food, name: "Dried Rations", glyph: '%', effect: ItemEffect::Feed(15) },
                    _ => Item { kind: ItemKind::Food, name: "Dwarven Ale", glyph: '~', effect: ItemEffect::Feed(12) },
                }
            }
        }
        _ => {
            if roll < 18 {
                Item { kind: ItemKind::Potion, name: "Superior Health Potion", glyph: '!', effect: ItemEffect::Heal(15) }
            } else if roll < 32 {
                Item { kind: ItemKind::Scroll, name: "Scroll of Storm", glyph: '?', effect: ItemEffect::DamageAoe(16) }
            } else if roll < 46 {
                match sub {
                    0 => Item { kind: ItemKind::Weapon, name: "Enchanted Blade", glyph: '/', effect: ItemEffect::BuffAttack(6) },
                    1 => Item { kind: ItemKind::Weapon, name: "Crystal Staff", glyph: '/', effect: ItemEffect::BuffAttack(7) },
                    _ => Item { kind: ItemKind::Weapon, name: "Flame Sword", glyph: '/', effect: ItemEffect::BuffAttack(6) },
                }
            } else if roll < 52 {
                Item { kind: ItemKind::Armor, name: "Dragon Scale", glyph: '[', effect: ItemEffect::BuffDefense(6) }
            } else if roll < 57 {
                Item { kind: ItemKind::Helmet, name: "Mithril Helm", glyph: '^', effect: ItemEffect::BuffDefense(5) }
            } else if roll < 62 {
                Item { kind: ItemKind::Shield, name: "Tower Shield", glyph: ')', effect: ItemEffect::BuffDefense(5) }
            } else if roll < 67 {
                Item { kind: ItemKind::Boots, name: "Plate Boots", glyph: '{', effect: ItemEffect::BuffDefense(4) }
            } else if roll < 80 {
                match sub {
                    0 => Item { kind: ItemKind::Ring, name: "Gold Ring", glyph: '=', effect: ItemEffect::BuffAttack(4) },
                    _ => Item { kind: ItemKind::Ring, name: "Diamond Ring", glyph: '=', effect: ItemEffect::BuffDefense(4) },
                }
            } else {
                match sub {
                    0 => Item { kind: ItemKind::Food, name: "Elven Waybread", glyph: '%', effect: ItemEffect::Feed(25) },
                    _ => Item { kind: ItemKind::Food, name: "Honey Mead", glyph: '~', effect: ItemEffect::Feed(18) },
                }
            }
        }
    }
}

/// Returns a meat/food item if the killed enemy is a beast.
fn meat_drop(enemy_name: &str) -> Option<Item> {
    match enemy_name {
        "Giant Rat" => Some(Item {
            kind: ItemKind::Food, name: "Rat Meat", glyph: '%',
            effect: ItemEffect::Feed(5),
        }),
        "Giant Bat" => Some(Item {
            kind: ItemKind::Food, name: "Bat Wing", glyph: '%',
            effect: ItemEffect::Feed(4),
        }),
        "Wolf" => Some(Item {
            kind: ItemKind::Food, name: "Wolf Meat", glyph: '%',
            effect: ItemEffect::Feed(15),
        }),
        "Giant Spider" => Some(Item {
            kind: ItemKind::Food, name: "Spider Leg", glyph: '%',
            effect: ItemEffect::Feed(8),
        }),
        "Boar" => Some(Item {
            kind: ItemKind::Food, name: "Boar Meat", glyph: '%',
            effect: ItemEffect::Feed(25),
        }),
        "Bear" => Some(Item {
            kind: ItemKind::Food, name: "Bear Meat", glyph: '%',
            effect: ItemEffect::Feed(35),
        }),
        _ => None,
    }
}

fn xorshift64(mut state: u64) -> u64 {
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
        g.enemies.push(Enemy { x: gx, y: gy, hp: 10, attack: 2, glyph: 'g', name: "Goblin", facing_left: false });
        g.move_player(1, 0);
        assert_eq!(g.enemies[0].hp, 10 - g.player_attack);
        assert_eq!(g.player_x, gx - 1);
    }

    #[test]
    fn killing_enemy() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 3, attack: 1, glyph: 'g', name: "Goblin", facing_left: false });
        let result = g.move_player(1, 0);
        assert!(matches!(result, TurnResult::Killed { .. }));
        assert!(g.enemies[0].hp <= 0);
    }

    #[test]
    fn enemy_retaliates() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 20, attack: 3, glyph: 'g', name: "Goblin", facing_left: false });
        let hp_before = g.player_hp;
        g.move_player(1, 0);
        assert_eq!(g.player_hp, hp_before - 3);
    }

    #[test]
    fn player_can_die() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.player_hp = 1;
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 99, attack: 5, glyph: 'g', name: "Goblin", facing_left: false });
        let result = g.move_player(1, 0);
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
        g.enemies.push(Enemy { x: dx, y: dy, hp: 1, attack: 0, glyph: 'D', name: "Dragon", facing_left: false });
        let result = g.move_player(1, 0);
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
        g.enemies.push(Enemy { x: gx, y: gy, hp: 20, attack: 2, glyph: 'g', name: "Goblin", facing_left: false });
        let msg_count_before = g.messages.len();
        g.move_player(1, 0);
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
            g.enemies.push(Enemy { x: ex, y: ey, hp: 10, attack: 1, glyph: 'g', name: "Goblin", facing_left: false });
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
        // Place an item one tile to the right of the player
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in dirs {
            let (nx, ny) = (g.player_x + dx, g.player_y + dy);
            if g.current_map().is_walkable(nx, ny) {
                g.ground_items.push(GroundItem { x: nx, y: ny, item: health_potion() });
                g.move_player(dx, dy);
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
        for _ in 0..MAX_INVENTORY {
            g.inventory.push(health_potion());
        }
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in dirs {
            let (nx, ny) = (g.player_x + dx, g.player_y + dy);
            if g.current_map().is_walkable(nx, ny) {
                g.ground_items.push(GroundItem { x: nx, y: ny, item: rusty_sword() });
                g.move_player(dx, dy);
                assert_eq!(g.inventory.len(), MAX_INVENTORY);
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
        g.enemies.push(Enemy { x: px + 2, y: py, hp: 20, attack: 1, glyph: 'g', name: "Goblin", facing_left: false });
        g.enemies.push(Enemy { x: px + 10, y: py, hp: 20, attack: 1, glyph: 'g', name: "Goblin", facing_left: false });
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
        g.enemies.push(Enemy { x: gx, y: gy, hp: 20, attack: 1, glyph: 'g', name: "Goblin", facing_left: false });
        g.move_player(1, 0);
        // Base attack 5 + weapon 2 = 7 damage
        assert_eq!(g.enemies[0].hp, 20 - 7);
    }

    #[test]
    fn armor_reduces_damage_taken() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.equipped_armor = Some(leather_armor());
        let hp_before = g.player_hp;
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 99, attack: 5, glyph: 'g', name: "Goblin", facing_left: false });
        g.move_player(1, 0);
        // Enemy attack 5 - defense 2 = 3 damage
        assert_eq!(g.player_hp, hp_before - 3);
    }

    #[test]
    fn defense_minimum_damage_is_one() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        // Defense higher than enemy attack
        g.equipped_armor = Some(Item {
            kind: ItemKind::Armor, name: "Dragon Scale", glyph: '[',
            effect: ItemEffect::BuffDefense(6),
        });
        let hp_before = g.player_hp;
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 99, attack: 2, glyph: 'g', name: "Goblin", facing_left: false });
        g.move_player(1, 0);
        // Attack 2 - defense 6 = max(1, -4) = 1
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
    fn deeper_dungeon_has_better_loot() {
        let mut g = overworld_game();
        g.enter_dungeon(0);
        let l0_items: Vec<_> = g.ground_items.iter().map(|gi| gi.item.name).collect();
        g.descend(0, 0);
        g.descend(0, 1);
        let l2_items: Vec<_> = g.ground_items.iter().map(|gi| gi.item.name).collect();
        // Level 2 should have higher-tier items
        let l0_has_basic = l0_items.iter().any(|n| *n == "Health Potion" || *n == "Rusty Sword");
        let l2_has_advanced = l2_items.iter().any(|n|
            *n == "Superior Health Potion" || *n == "Enchanted Blade" || *n == "Dragon Scale" || *n == "Scroll of Storm"
        );
        assert!(l0_has_basic || l0_items.is_empty(), "level 0 should have basic items");
        assert!(l2_has_advanced || l2_items.is_empty(), "level 2 should have advanced items");
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

    // --- Inventory toggle ---

    #[test]
    fn toggle_inventory() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        assert!(!g.inventory_open);
        g.toggle_inventory();
        assert!(g.inventory_open);
        g.toggle_inventory();
        assert!(!g.inventory_open);
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
            x: ex, y: ey, hp: 10, attack: 3, glyph: 'g', name: "Goblin", facing_left: false,
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
        g.enemies.push(Enemy { x: gx, y: gy, hp: 1, attack: 0, glyph: 'g', name: "Goblin", facing_left: false });
        g.move_player(1, 0);
        assert_eq!(g.player_xp, 4); // goblin = 4 XP
    }

    #[test]
    fn level_up_increases_stats() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let old_max = g.player_max_hp;
        let old_atk = g.player_attack;
        // Force enough XP for level 2 (need 20 XP at level 1)
        g.player_xp = 20;
        g.check_level_up();
        assert_eq!(g.player_level, 2);
        assert_eq!(g.player_max_hp, old_max + 3);
        assert_eq!(g.player_attack, old_atk + 1);
        assert_eq!(g.player_hp, g.player_max_hp); // full heal on level up
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
        g.enemies.push(Enemy { x: gx, y: gy, hp: 1, attack: 0, glyph: 'g', name: "Goblin", facing_left: false });
        g.move_player(1, 0);
        assert!(g.messages.iter().any(|m| m.contains("+4 XP")));
    }

    // === Stamina, Sprint, Hunger & Food ===

    fn raw_food(amount: i32) -> Item {
        Item { kind: ItemKind::Food, name: "Wild Berries", glyph: '%', effect: ItemEffect::Feed(amount) }
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
            g.enemies.push(Enemy { x: ex, y: ey, hp: 10, attack: 3, glyph: 'g', name: "Goblin", facing_left: false });
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
    fn hunger_drains_on_move() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let hunger_before = g.hunger;
        let dirs = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        for (dx, dy) in dirs {
            let (nx, ny) = (g.player_x + dx, g.player_y + dy);
            if g.current_map().is_walkable(nx, ny) {
                g.move_player(dx, dy);
                assert_eq!(g.hunger, hunger_before - 1, "hunger should drain 1 per move");
                return;
            }
        }
    }

    #[test]
    fn starvation_damages_player() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        g.hunger = 1; // Will reach 0 after one move
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
        g.hunger = 1;
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
        g.enemies.push(Enemy { x: gx, y: gy, hp: 1, attack: 0, glyph: 'w', name: "Wolf", facing_left: false });
        g.move_player(1, 0);
        assert!(g.ground_items.iter().any(|gi| gi.item.name == "Wolf Meat"),
            "wolf should drop meat");
    }

    #[test]
    fn killing_boar_drops_meat() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 1, attack: 0, glyph: 'b', name: "Boar", facing_left: false });
        g.move_player(1, 0);
        assert!(g.ground_items.iter().any(|gi| gi.item.name == "Boar Meat"));
    }

    #[test]
    fn killing_bear_drops_meat() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 1, attack: 0, glyph: 'B', name: "Bear", facing_left: false });
        g.move_player(1, 0);
        assert!(g.ground_items.iter().any(|gi| gi.item.name == "Bear Meat"));
    }

    #[test]
    fn killing_rat_drops_meat() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 1, attack: 0, glyph: 'r', name: "Giant Rat", facing_left: false });
        g.move_player(1, 0);
        assert!(g.ground_items.iter().any(|gi| gi.item.name == "Rat Meat"),
            "giant rat should drop rat meat");
    }

    #[test]
    fn killing_bat_drops_meat() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 1, attack: 0, glyph: 'a', name: "Giant Bat", facing_left: false });
        g.move_player(1, 0);
        assert!(g.ground_items.iter().any(|gi| gi.item.name == "Bat Wing"),
            "giant bat should drop bat wing");
    }

    #[test]
    fn killing_spider_drops_meat() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 1, attack: 0, glyph: 'i', name: "Giant Spider", facing_left: false });
        g.move_player(1, 0);
        assert!(g.ground_items.iter().any(|gi| gi.item.name == "Spider Leg"),
            "giant spider should drop spider leg");
    }

    #[test]
    fn killing_goblin_drops_no_meat() {
        let map = Map::generate(30, 20, 42);
        let mut g = Game::new(map);
        let gx = g.player_x + 1;
        let gy = g.player_y;
        g.enemies.push(Enemy { x: gx, y: gy, hp: 1, attack: 0, glyph: 'g', name: "Goblin", facing_left: false });
        g.move_player(1, 0);
        assert!(g.ground_items.is_empty(), "goblin should not drop meat");
    }

    #[test]
    fn meat_has_feed_effect() {
        let meat = meat_drop("Wolf").unwrap();
        assert_eq!(meat.kind, ItemKind::Food);
        assert!(matches!(meat.effect, ItemEffect::Feed(_)));
    }

    #[test]
    fn bear_meat_restores_more_than_wolf() {
        let wolf = meat_drop("Wolf").unwrap();
        let bear = meat_drop("Bear").unwrap();
        let wolf_feed = match wolf.effect { ItemEffect::Feed(n) => n, _ => 0 };
        let bear_feed = match bear.effect { ItemEffect::Feed(n) => n, _ => 0 };
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
    fn all_beasts_drop_food() {
        let beasts = ["Giant Rat", "Giant Bat", "Wolf", "Giant Spider", "Boar", "Bear"];
        for name in beasts {
            assert!(meat_drop(name).is_some(), "{name} should drop food");
        }
    }

    #[test]
    fn meat_feed_values_scale_with_beast() {
        let drops: Vec<_> = ["Giant Rat", "Giant Bat", "Wolf", "Giant Spider", "Boar", "Bear"]
            .iter()
            .map(|n| {
                let item = meat_drop(n).unwrap();
                match item.effect { ItemEffect::Feed(v) => v, _ => 0 }
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
        let avg_t0: f64 = t0_food.iter().map(|i| match i.effect { ItemEffect::Feed(v) => v as f64, _ => 0.0 }).sum::<f64>() / t0_food.len() as f64;
        let avg_t2: f64 = t2_food.iter().map(|i| match i.effect { ItemEffect::Feed(v) => v as f64, _ => 0.0 }).sum::<f64>() / t2_food.len() as f64;
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
}
