# The Cave — WASM Migration Roadmap

> Migrating from a server-rendered Tokio TCP game to a fully client-side
> WebAssembly game running on GitHub Pages.

## New Stack

| Layer       | Technology                          |
|-------------|-------------------------------------|
| Language    | Rust → WebAssembly                  |
| Bindings    | `wasm-bindgen` + `web-sys` + `js-sys` |
| Rendering   | HTML5 Canvas 2D                     |
| RNG         | `getrandom` (wasm) + `rand`        |
| Build       | `wasm-pack build --target web`      |
| Deploy      | GitHub Actions → GitHub Pages       |
| Multiplayer | None — single-player, all state in browser |

## Migration Phases

### Phase 0 — Scaffold & Proof of Concept ✅
- [x] Strip server code (tokio, TCP, HTTP, sessions)
- [x] Set up `wasm-pack` project with `wasm-bindgen`, `web-sys`
- [x] Create `index.html` with `<canvas>` and JS bootstrap
- [x] Draw `@` symbol on canvas from Rust/WASM
- [x] GitHub Actions: `wasm-pack build` + deploy to GitHub Pages

### Phase 1 — Canvas Rendering Engine
- [ ] Implement `Renderer` struct wrapping `CanvasRenderingContext2d`
- [ ] Port tile system to canvas draw calls (text glyphs / emoji)
- [ ] Implement camera/viewport (centered on player)
- [ ] Fog-of-war via alpha compositing
- [ ] Background color per biome

### Phase 2 — Game Loop & Input
- [ ] `requestAnimationFrame`-based game loop via `wasm-bindgen`
- [ ] Keyboard input handling (WASD / HJKL / arrows)
- [ ] Touch input support
- [ ] Turn-based tick: input → game.tick() → render

### Phase 3 — Port Game Logic
- [ ] Port `Position`, `Direction`, movement
- [ ] Port map generation (Poisson disk sampling)
- [ ] Port biome system & tile definitions
- [ ] Port enemy spawning, AI, and combat
- [ ] Port item system & player stats
- [ ] Port event log / message display

### Phase 4 — UI & Polish
- [ ] HUD overlay: health, attack, defense
- [ ] Event log display (canvas or HTML overlay)
- [ ] Victory/death screens
- [ ] Start screen / instructions
- [ ] Responsive canvas sizing

### Phase 5 — Ship It
- [ ] Final GitHub Pages deploy workflow
- [ ] Update README with gameplay instructions
- [ ] Clean up dead code from server era
- [ ] Tag v1.0.0 release

## Breaking Changes

This migration is a **full rewrite**. The following are removed entirely:

- `tokio` async runtime & TCP server
- HTTP request/response handling (`server/`)
- Session/cookie-based multiplayer
- Server-side HTML rendering (`pages/`, `html/`)
- `Dockerfile` & docker-compose

All game state now lives in the browser. The Rust code compiles to WASM
and runs client-side. The "server" is just a static file host (GitHub Pages).
