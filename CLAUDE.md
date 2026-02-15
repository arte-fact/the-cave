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

- **Swipe pathfinding**: During a swipe (touchmove), convert swipe vector to a destination tile, run A* from player to destination, and preview the path on the map (highlighted tiles). On release (touchend), auto-move the player along the path step by step. Requires:
  1. A* pathfinding in `map.rs` (pure logic, TDD)
  2. `touchmove` handling in `input.rs` to emit a preview destination
  3. Path rendering in `renderer.rs` (highlighted tiles from player to target)
  4. Auto-move queue in `game.rs` / `lib.rs` on swipe release
