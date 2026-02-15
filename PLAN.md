# Plan: Unique Dragon + Cave Layer Under Dungeons

## Problem
- Dragon spawns on the deepest level of **every** dungeon — not unique at all
- All dungeon levels use BSP room generation — no variety

## Proposal: Rock Cave as Dragon's Lair

Add a **4th level** to exactly **one** dungeon: a cellular automata rock cave
(like `Map::generate` but larger). This is the Dragon's Lair — the only place
the dragon exists.

### Structure

```
Overworld (forest)
  └── Dungeon 1 (BSP, 3 levels)
  └── Dungeon 2 (BSP, 3 levels) ← chosen by seed
  │     └── Level 0: goblins + skeletons
  │     └── Level 1: goblins + skeletons + orcs
  │     └── Level 2: skeletons + orcs + trolls
  │     └── Level 3: THE CAVE (cellular automata, unique dragon boss)
  └── Dungeon 3 (BSP, 3 levels)
```

### Changes

1. **`map.rs`** — New `Map::generate_cave(width, height, seed)`:
   - Reuses existing `Map::generate` cellular automata (Wall/Floor)
   - Larger size: 80×60
   - Places `StairsUp` in a connected floor region near (1,1)
   - No `StairsDown` — this is the deepest level

2. **`map.rs` / `Dungeon::generate`** — One dungeon gets a 4th cave level:
   - Accept a `has_cave: bool` parameter
   - If true, append a cave level after the 3 BSP levels
   - Level 2 of that dungeon still gets `StairsDown` (connecting to the cave)

3. **`world.rs` / `World::new`** — Pick which dungeon gets the cave:
   - Use `seed % dungeon_count` to deterministically select one
   - Pass `has_cave: true` only for that dungeon

4. **`game.rs` / `spawn_dungeon_enemies`** — Dragon only in the cave:
   - Remove current "place dragon on deepest level" logic
   - Add detection: if current level is a cave (level == 3 on the chosen
     dungeon), spawn the dragon boss (hp=30, atk=8) plus a few trolls as
     minions
   - Other deepest levels (level 2 in non-cave dungeons) get no dragon

5. **Tests**:
   - Exactly one dungeon has 4 levels, the rest have 3
   - Cave level has no `StairsDown`, has `StairsUp`
   - Dragon spawns only in the cave, nowhere else
   - Cave floor connectivity (player can reach all areas from stairs)
   - Existing dungeon traversal tests still pass
