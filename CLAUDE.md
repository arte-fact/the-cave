# The Cave

Rust/WASM roguelike. Mobile-first.

## Dev workflow

- **TDD**: Write tests first, see them fail, then implement.
- Pure logic modules (`map`, `game`) are tested with `cargo test`.
- WASM/browser modules (`renderer`, `input`, `lib`) are integration-tested in the browser.

## Commands

- `cargo test` — run unit tests
- `wasm-pack build --target web --out-dir web/pkg --no-typescript` — build WASM
