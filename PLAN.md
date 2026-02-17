# Plan: GameConfig, Menus, and Glyph Mode

## Overview

Centralize all hardcoded game constants into a `GameConfig` struct, add a menu
system (main menu, settings, new game), and a glyph-only rendering mode.

---

## Part 1: `GameConfig` Struct (`src/config.rs`)

A single `GameConfig` struct with nested sub-configs, all with `Default` impls
that produce the current hardcoded values. The game reads from this config
instead of inline constants.

### 1a. `PlayerConfig`

Extracted from `Game::new` / `Game::new_overworld` and survival constants:

```rust
pub struct PlayerConfig {
    pub start_hp: i32,           // 20
    pub start_attack: i32,       // 5
    pub start_defense: i32,      // 0
    pub start_dexterity: i32,    // 3
    pub max_inventory: usize,    // 10
    pub start_stamina: i32,      // 100
    pub max_stamina: i32,        // 100
    pub start_hunger: i32,       // 100
    pub max_hunger: i32,         // 100
}
```

### 1b. `SurvivalConfig`

Extracted from `game.rs` constants and `tick_survival`:

```rust
pub struct SurvivalConfig {
    pub sprint_cost: i32,              // 15
    pub stamina_regen: i32,            // 5
    pub hunger_drain: i32,             // 1
    pub hunger_interval_overworld: u32, // 5
    pub hunger_interval_dungeon: u32,   // 3
    pub hunger_interval_cave: u32,      // 2
    pub starvation_damage: i32,        // 1
    pub regen_hunger_threshold: i32,   // 50
    pub regen_hunger_cost: i32,        // 2
}
```

### 1c. `ProgressionConfig`

Extracted from `check_level_up`, `allocate_skill_point`, `xp_with_diminishing`:

```rust
pub struct ProgressionConfig {
    pub xp_base: f64,                  // 20.0
    pub xp_exponent: f64,              // 1.5
    pub skill_points_per_level: u32,   // 3
    pub hp_per_level: i32,             // 2
    pub level_heal_fraction: f64,      // 0.5 (50% of missing HP)
    pub vitality_hp_bonus: i32,        // 3
    pub max_stamina_per_stamina_skill: i32, // 5
    pub dodge_per_dexterity: i32,      // 2 (percent)
    pub max_dodge_percent: i32,        // 20
    pub overworld_diminish_threshold_1: u32, // 50 kills
    pub overworld_diminish_threshold_2: u32, // 100 kills
}
```

### 1d. `CombatConfig`

Extracted from `move_player`, `ranged_attack`, `enemy_move`:

```rust
pub struct CombatConfig {
    pub base_ranged_range: i32,   // 8
    pub ranged_hit_base: i32,     // 100 (100% at distance 0)
    pub ranged_hit_per_dist: i32, // 10 (lose 10% per tile)
    pub min_ranged_hit: i32,      // 20 (20% floor)
    pub enemy_chase_radius: i32,  // 8
}
```

### 1e. `FovConfig`

```rust
pub struct FovConfig {
    pub overworld_radius: i32,  // 8
    pub dungeon_radius: i32,    // 6
}
```

### 1f. `OverworldConfig`

Extracted from `generate_forest`, `place_dungeons`, `build_roads`:

```rust
pub struct OverworldConfig {
    pub width: i32,                // 200
    pub height: i32,               // 200
    pub tree_fill_percent: u64,    // 55
    pub automata_passes: u32,      // 4
    pub automata_threshold: i32,   // 5 (>=5 neighbors -> tree)
    pub bsp_zone_min_size: i32,    // 30
    pub dungeon_spawn_chance: u64, // 60 (percent)
    pub min_dungeons: usize,       // 3
    pub road_cost_grass: i32,      // 2
    pub road_cost_tree: i32,       // 6
    pub road_cost_road: i32,       // 1
}
```

### 1g. `DungeonConfig`

Extracted from `Dungeon::generate`, `generate_bsp_dungeon`:

```rust
pub struct DungeonConfig {
    pub level_sizes: Vec<(i32, i32)>,  // [(40,30), (50,35), (60,40)]
    pub cave_size: (i32, i32),         // (80, 60)
    pub bsp_min_room: i32,             // 5
    pub cave_wall_percent: u64,        // 45
    pub cave_automata_passes: u32,     // 5
}
```

### 1h. `SpawnConfig`

Extracted from `spawn_enemies`, `spawn_dungeon_enemies`, `spawn_overworld_items`,
`spawn_overworld_food`, `spawn_dungeon_items`:

```rust
pub struct SpawnConfig {
    pub overworld_enemy_chance: u64,   // 3 (percent per walkable)
    pub dungeon_enemy_chance: u64,     // 10 (percent per floor)
    pub cave_enemy_chance: u64,        // 6
    pub overworld_item_road_chance: u64,  // 3 (per mille)
    pub overworld_item_grass_chance: u64, // 1
    pub overworld_food_chance: u64,       // 8 (per mille)
    pub dungeon_item_chance: u64,         // 2 (percent)
    pub cave_item_chance: u64,            // 1
}
```

### 1i. `EnemyTemplate` and `EnemyTable`

Replace the inline match arms with data-driven templates:

```rust
pub struct EnemyTemplate {
    pub name: &'static str,
    pub glyph: char,
    pub hp: i32,
    pub attack: i32,
    pub defense: i32,
    pub is_ranged: bool,
    pub xp: u32,
    pub weight: u32,   // relative spawn weight (replaces roll ranges)
    pub desc: &'static str,
}

pub struct EnemyTable {
    pub overworld: Vec<EnemyTemplate>,
    pub dungeon_l0: Vec<EnemyTemplate>,
    pub dungeon_l1: Vec<EnemyTemplate>,
    pub dungeon_l2: Vec<EnemyTemplate>,
    pub cave: Vec<EnemyTemplate>,
}
```

Default impl populates with all current enemy definitions (Giant Rat through
Dragon). Spawn functions use weighted random selection from the appropriate
table instead of hardcoded if/else chains.

### 1j. `ItemTable`

Similarly, replace `random_item()` tiers with data-driven tables:

```rust
pub struct ItemTemplate {
    pub kind: ItemKind,
    pub name: &'static str,
    pub glyph: char,
    pub effect: ItemEffect,
    pub weight: u32,  // relative drop weight within tier
}

pub struct ItemTable {
    pub tier0: Vec<ItemTemplate>,
    pub tier1: Vec<ItemTemplate>,
    pub tier2: Vec<ItemTemplate>,
}
```

### 1k. `FoodTable`

Same pattern for overworld food and meat drops:

```rust
pub struct FoodTemplate {
    pub name: &'static str,
    pub glyph: char,
    pub feed: i32,
    pub side_effect: FoodSideEffect,
    pub weight: u32,
}

pub struct FoodTable {
    pub overworld: Vec<FoodTemplate>,
    pub meat_drops: Vec<(&'static str, FoodTemplate)>, // (enemy_name, drop)
}
```

### 1l. `RenderConfig`

```rust
pub struct RenderConfig {
    pub viewport_tiles_wide: f64,   // 15.0
    pub glyph_mode: bool,          // false (sprites by default)
    pub top_bar_base: f64,          // 52.0
    pub detail_strip_base: f64,     // 52.0
    pub bottom_bar_base: f64,       // 48.0
    pub msg_area_base: f64,         // 42.0
    pub drawer_anim_speed: f64,     // 0.15
}
```

### 1m. `CameraConfig`

```rust
pub struct CameraConfig {
    pub lerp_min: f64,    // 0.15
    pub lerp_max: f64,    // 0.4
    pub lerp_dist_factor: f64, // 0.06
    pub snap_threshold: f64,   // 0.1
}
```

### 1n. Top-level `GameConfig`

```rust
pub struct GameConfig {
    pub player: PlayerConfig,
    pub survival: SurvivalConfig,
    pub progression: ProgressionConfig,
    pub combat: CombatConfig,
    pub fov: FovConfig,
    pub overworld: OverworldConfig,
    pub dungeon: DungeonConfig,
    pub spawn: SpawnConfig,
    pub enemies: EnemyTable,
    pub items: ItemTable,
    pub food: FoodTable,
    pub render: RenderConfig,
    pub camera: CameraConfig,
}
```

`GameConfig::default()` produces all current hardcoded values -- zero behavior
change.

### 1o. Wiring

- `GameConfig` is constructed once at startup (in `lib.rs`).
- Passed as `&GameConfig` to `new_game()`, `Game::new_overworld()`, and stored
  in `Game` as a reference or `Rc<GameConfig>`.
- Each module reads values from config instead of module-level `const`s.
- Camera, Renderer, and Map functions accept relevant config sub-structs.
- The old `const` lines in `game.rs`, `camera.rs`, `renderer.rs`, `map.rs`
  are removed.

---

## Part 2: Game State Machine & Menu System

### 2a. `AppState` enum

Currently the game starts directly into gameplay. Add a state machine:

```rust
pub enum AppState {
    MainMenu,
    NewGameMenu,
    SettingsMenu,
    Playing,
}
```

This lives in `lib.rs` alongside the game loop. The game loop checks `AppState`
and delegates to the appropriate render/input handler.

### 2b. Main Menu

Rendered in `renderer.rs` as a new `draw_main_menu()` method:

- Game title "THE CAVE" in large text, centered
- Three buttons (touch-friendly, full-width):
  1. **New Game** -> transitions to `NewGameMenu`
  2. **Settings** -> transitions to `SettingsMenu`
  3. **Continue** (only shown if a game is in progress) -> transitions to
     `Playing`
- Dark background with subtle styling

Input: tap on a button -> change `AppState`.

### 2c. New Game Menu

Rendered as `draw_new_game_menu()`:

- **Seed input**: Shows a randomly generated seed, with a "Randomize" button.
  Tap to edit (prompt via browser `window.prompt()` for text input on mobile).
- **Difficulty preset** (3 buttons, mutually exclusive):
  - **Easy**: More HP, more food, fewer enemies, wider FOV
  - **Normal**: Default config values
  - **Hard**: Less HP, less food, more enemies, faster hunger
- **Start** button -> creates `GameConfig` (modified by difficulty), calls
  `new_game(config, seed)`, transitions to `Playing`
- **Back** button -> returns to `MainMenu`

Difficulty presets are implemented as functions that return a modified
`GameConfig`:

```rust
impl GameConfig {
    pub fn easy() -> Self { /* adjusted values */ }
    pub fn normal() -> Self { Self::default() }
    pub fn hard() -> Self { /* adjusted values */ }
}
```

### 2d. Settings Menu

Rendered as `draw_settings_menu()`:

- **Glyph Mode** toggle: ON/OFF -> sets `config.render.glyph_mode`
- **Viewport Zoom** slider (or -/+ buttons): Adjusts
  `config.render.viewport_tiles_wide` (range 10-25)
- **Back** button -> returns to `MainMenu`

Settings are persisted to `localStorage` via `web_sys` so they survive page
reloads.

### 2e. Input routing

The input handler in `lib.rs` checks `AppState`:
- `MainMenu` / `NewGameMenu` / `SettingsMenu`: Route taps to menu button
  hit-tests.
- `Playing`: Existing swipe/tap/keyboard logic.

Keyboard shortcut: `Escape` during gameplay -> `MainMenu` (pause).

---

## Part 3: Glyph Mode

A rendering mode that skips all sprite sheets and draws pure ASCII characters
with colored backgrounds -- classic roguelike style.

### 3a. Renderer changes

Add a branch in `draw_world()`:

```rust
if self.glyph_mode {
    self.draw_tile_glyph(tile, vis, px, py, cell);
} else {
    // existing sprite path
}
```

`draw_tile_glyph()` draws:
- Background fill with `tile.color()` (the existing color method)
- Foreground glyph character (`tile.glyph()`) centered in the cell
- For `Seen` tiles: dimmed with semi-transparent black overlay (same as now)

Entity rendering in glyph mode:
- Player: `@` in white
- Enemies: `enemy.glyph` in red (ranged: yellow)
- Items: `item.glyph` in yellow/green

This reuses the existing `draw_tile_fallback` logic but as the primary render
path when glyph mode is active.

### 3b. Config integration

`config.render.glyph_mode` controls which path the renderer takes.
Toggling it in the settings menu takes effect immediately (next frame).

### 3c. Performance note

Glyph mode is lighter than sprites (no image decoding). Useful for low-end
devices or players who prefer the classic look. The game can start rendering
immediately without waiting for sprite sheets to load.

---

## Implementation Order

1. **config.rs**: Define all config structs with Default impls.
2. **Wire config**: Thread `GameConfig` through `new_game()`, `Game`,
   `Renderer`, `Camera`, `Map` generation functions. Replace all `const`
   and hardcoded values with config reads. Ensure `cargo test` still passes.
3. **Data-driven enemies/items**: Replace match-arm spawn tables with
   `EnemyTable`/`ItemTable` iteration. Ensure `cargo test` still passes.
4. **AppState + menu rendering**: Add state machine, draw menus, route input.
5. **New Game menu logic**: Difficulty presets, seed input, game creation.
6. **Settings menu + localStorage**: Glyph toggle, zoom slider, persistence.
7. **Glyph mode renderer**: Implement the ASCII rendering path.

Each step is independently testable. Steps 1-3 are pure refactoring (no
behavior change, all tests must keep passing). Steps 4-7 add new features.
