# The Cave

Rust/WASM roguelike. Mobile-first.

## Dev workflow

- **TDD**: Write tests first, see them fail, then implement.
- Pure logic modules (`map`, `game`) are tested with `cargo test`.
- WASM/browser modules (`renderer`, `input`, `lib`) are integration-tested in the browser.

## Commands

- `cargo test` — run unit tests
- `wasm-pack build --target web --out-dir web/pkg --no-typescript` — build WASM

## Roadmap

### Done

- **Swipe pathfinding**: A* pathfinding, swipe preview, auto-move along path.

### Phase 1: Camera & Viewport

The map is about to get much bigger. Before generating the world, the renderer
needs a camera that follows the player and only draws visible tiles.

1. **Camera struct in `renderer.rs`** — `camera_x/y` (float, world units). Viewport = visible tile window derived from canvas size ÷ cell size. Clamp at world edges.
2. **Render only visible tiles** — Loop over `camera_x..camera_x+viewport_w` instead of `0..map.width`. Offset all draw calls by camera position.
3. **Camera follow** — After every player move (including auto-move steps), lerp camera toward player. Snap when close enough (< 0.1 tile).
4. **Update coordinate conversions** — `css_to_grid` and swipe preview must account for camera offset. Touch coords → world coords = screen coords + camera offset.
5. **Tests**: Camera clamping at edges, viewport size calculation, coordinate round-trips.

### Phase 2: World Generation — Forest + Dungeons

Large overworld (~200×200). Cellular automata forest with BSP-placed dungeon complexes connected by A*-generated roads.

#### 2a: Tile system expansion

6. **Extend `Tile` enum** — `Wall`, `Floor`, `Tree`, `Grass`, `Road`, `DungeonEntrance`, `StairsDown`, `StairsUp`. Each has walkability and render properties.
7. **Tile metadata helper** — `tile.is_walkable()`, `tile.glyph()`, `tile.color()` methods so renderer and game logic stay decoupled from enum variants.
8. **Tests**: Walkability truth table for every tile type.

#### 2b: Forest generation (cellular automata)

9. **Forest automata in `map.rs`** — New `generate_forest(width, height, seed)`. Fill ~55% trees, smooth 4 passes. Produces organic forest with clearings (Grass tiles). Border = dense trees.
10. **Clearing detection** — Flood-fill to tag connected grass regions. Ensure the largest region is reachable (fill small isolated pockets with trees).
11. **Tests**: At least 30% grass, all grass reachable from spawn, border is trees, deterministic.

#### 2c: BSP dungeon placement on the overworld

12. **BSP zone partitioning** — Subdivide overworld into ~6–10 rectangular zones (min 30×30). Each zone *may* contain a dungeon entrance based on seed + probability.
13. **Dungeon footprint** — For each selected zone, carve a rectangular dungeon footprint (~20×20) into the forest. Place `DungeonEntrance` tile at an edge of the footprint facing a clearing.
14. **Tests**: Dungeon footprints don't overlap, every entrance is adjacent to walkable grass, at least 3 dungeons placed.

#### 2d: Road generation (A*)

15. **Road network** — Build a minimum spanning tree across all dungeon entrances (Euclidean edge weights). For each MST edge, run A* on the overworld (grass and trees walkable for road carving). Carve path tiles as `Road`.
16. **Road tile weight** — A* cost: grass=1, tree=3, existing road=0.5. This makes roads prefer clearings and reuse existing roads, creating natural-looking paths.
17. **Player spawn** — On a road tile near the center of the map.
18. **Tests**: Every dungeon entrance reachable from spawn via roads, no road on border, road count reasonable.

#### 2e: BSP dungeon interiors (per-dungeon)

19. **`Dungeon` struct** — Separate data structure: `levels: Vec<Map>`, `depth: usize`. Each level is a self-contained `Map` with `StairsDown`/`StairsUp` tiles.
20. **BSP room generation per level** — Recursive BSP split (min room 5×5). Leaf nodes = rooms (Floor). Connect sibling rooms with L-shaped corridors. Walls everywhere else.
21. **Depth scaling** — Deeper levels: more rooms, tighter corridors, fewer clearings. Level 1 = 40×30, level 2 = 50×35, level 3+ = 60×40.
22. **Stairs placement** — `StairsDown` in a random room far from `StairsUp`. Level 1 `StairsUp` = dungeon entrance (leads back to overworld).
23. **Tests**: Every room reachable from stairs, stairs exist on every level, BSP splits produce valid rooms, no overlap.

### Phase 3: Dungeon Traversal & World Switching

Wire the multi-layer world into the game loop.

24. **`World` struct in new `world.rs`** — Holds the overworld `Map` + `Vec<Dungeon>`. Tracks which map the player is currently on (`overworld` or `dungeon[i].level[j]`).
25. **Enter/exit dungeons** — Step on `DungeonEntrance` → prompt or auto-enter dungeon level 1. Step on `StairsUp` at level 1 → return to overworld. `StairsDown`/`StairsUp` within dungeon → change level.
26. **Camera reset on transition** — Snap camera to player on map change. Different maps may have different sizes.
27. **Enemy scoping** — Enemies belong to a specific map. Only active enemies are on the current map. Overworld: forest creatures. Dungeons: goblins, deeper = dragons.
28. **Tests**: Enter and exit dungeon round-trip preserves player position on overworld, stairs connect correct levels.

### Phase 4: Fog of War & Exploration

29. **Visibility grid** — Per-map `Vec<TileVisibility>` where `TileVisibility = Hidden | Seen | Visible`. Reset `Visible→Seen` each turn, then recompute FOV.
30. **Shadowcasting FOV in `map.rs`** — Recursive shadowcasting, radius 8 (overworld) / 6 (dungeon). Pure logic, no renderer dependency.
31. **Renderer integration** — `Hidden` = don't draw. `Seen` = dimmed (50% opacity). `Visible` = full brightness. Enemies/items only drawn if `Visible`.
32. **Tests**: Player tile always visible, walls block LOS, FOV radius respected, seen tiles persist.

### Phase 5: Items, Inventory & Equipment

33. **Item data model** — `Item { kind, name, glyph, effect }`. Kinds: `Potion`, `Scroll`, `Weapon`, `Armor`. Effects: heal, damage-aoe, buff attack, buff defense.
34. **Item spawning** — Dungeon rooms have % chance to contain items. Deeper = better loot. Overworld = rare, only near roads.
35. **Inventory UI** — Touch-friendly overlay panel. Tap item to use/equip. Max 10 slots.
36. **Equipment slots** — Weapon + Armor. Affect `player_attack` and a new `player_defense` stat. Damage formula: `max(1, attack - defense)`.
37. **Tests**: Pickup, use, equip, drop, inventory full, stat effects.

### Phase 6: Modern UI Polish

38. **Smooth camera** — Lerp with configurable speed. Ease-out curve for satisfying feel.
39. **Mini-map** — Corner overlay showing explored overworld tiles. Player blip, dungeon entrance markers, fog.
40. **Tile rendering upgrade** — Color palette per biome (forest greens, dungeon grays, road browns). Subtle shade variation per tile (seeded noise) for visual richness.
41. **HUD improvements** — Depth indicator, dungeon name, inventory quick-bar at bottom, XP/level display.
42. **Tap-to-move** — Tap any visible tile: A* pathfind there, preview, auto-move. Complement to swipe.
43. **Transition animations** — Fade-to-black on dungeon enter/exit. Damage flash on hit. Pickup sparkle.
