mod camera;
mod config;
mod errors;
mod game;
mod hit_test;
mod input;
mod map;
mod renderer;
mod sprite_atlas;
mod sprites;
mod world;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, HtmlImageElement};

use config::{Difficulty, GameConfig};
use game::{Drawer, Game, TurnResult};
use hit_test::*;
use input::{Input, InputAction};
use map::{bresenham_line, Map};
use renderer::{Renderer, SpriteSheets};
use world::World;

/// Long-press frame threshold for drag (~300ms at 60fps).
const DRAG_LONG_PRESS_FRAMES: u32 = 18;
/// Auto-move step interval in frames (~7.5 tiles/sec at 60fps).
const AUTO_MOVE_INTERVAL: u32 = 8;

/// Type alias for the animation frame closure pattern.
type AnimFrameClosure = Rc<RefCell<Option<Closure<dyn FnMut()>>>>;

/// Drag-and-drop state for quick-bar item assignment.
#[derive(Clone, Debug)]
enum DragState {
    /// No drag in progress.
    Idle,
    /// Finger is down on an inventory item; waiting for long-press timer.
    Pending {
        inv_index: usize,
        start_x: f64,
        start_y: f64,
        /// Frame counter when the press started.
        start_frame: u32,
        /// If dragging from a quick-bar slot (for reorder), which slot.
        from_quickbar_slot: Option<usize>,
    },
    /// Long-press confirmed — item is being dragged.
    Dragging {
        inv_index: usize,
        /// Current finger position (CSS pixels).
        touch_x: f64,
        touch_y: f64,
        /// Source: true if dragging from a quick-bar slot (reorder), false from inventory.
        from_quickbar_slot: Option<usize>,
    },
}

/// Application state machine: menus vs playing.
#[derive(Clone, Debug)]
enum AppState {
    MainMenu,
    NewGame {
        selected_difficulty: usize, // 0=Easy, 1=Normal, 2=Hard
        seed: u64,
    },
    Settings,
    Playing,
}

/// Menu state held in an Rc<RefCell<>> alongside the game.
#[derive(Clone, Debug)]
struct MenuState {
    app_state: AppState,
    has_save: bool,
}


/// Get localStorage handle, returning None if unavailable.
fn get_local_storage() -> Option<web_sys::Storage> {
    web_sys::window().and_then(|w| w.local_storage().ok().flatten())
}

/// Load glyph_mode setting from localStorage.
fn load_glyph_mode() -> bool {
    get_local_storage()
        .and_then(|s| s.get_item("glyph_mode").ok().flatten())
        .map(|v| v == "true")
        .unwrap_or(false)
}

/// Save glyph_mode setting to localStorage.
fn save_glyph_mode(enabled: bool) {
    if let Some(storage) = get_local_storage() {
        let _ = storage.set_item("glyph_mode", if enabled { "true" } else { "false" });
    }
}

/// Load last-used difficulty from localStorage. Defaults to Normal (1).
fn load_difficulty() -> usize {
    get_local_storage()
        .and_then(|s| s.get_item("difficulty").ok().flatten())
        .and_then(|v| v.parse().ok())
        .unwrap_or(1)
}

/// Save difficulty selection to localStorage.
fn save_difficulty(idx: usize) {
    if let Some(storage) = get_local_storage() {
        let _ = storage.set_item("difficulty", &idx.to_string());
    }
}

/// Handle a tap event within menu screens. Modifies menu state and
/// may transition to Playing state (creating a new game).
fn handle_menu_tap(
    ms: &mut MenuState,
    game: &Rc<RefCell<Game>>,
    renderer: &Rc<RefCell<Renderer>>,
    auto_path: &Rc<RefCell<Vec<(i32, i32)>>>,
    area: &CssHitArea,
    dpr: f64,
) {
    // Convert to canvas-pixel coords for hit testing
    let cx = area.x * dpr;
    let cy = area.y * dpr;
    let cw = area.w * dpr;
    let ch = area.h * dpr;
    let compact = ch < cw;
    let ref_dim = cw.min(ch);

    match &ms.app_state {
        AppState::MainMenu => {
            // Button layout matches draw_main_menu
            let btn_w = (ref_dim * 0.5).min(280.0 * dpr);
            let btn_h = if compact { 36.0 * dpr } else { 44.0 * dpr };
            let gap = if compact { 10.0 * dpr } else { 16.0 * dpr };
            let start_y = if compact { ch * 0.38 } else { ch * 0.45 };
            let btn_x = (cw - btn_w) / 2.0;

            // New Game
            if cx >= btn_x && cx <= btn_x + btn_w
                && cy >= start_y && cy <= start_y + btn_h
            {
                ms.app_state = AppState::NewGame {
                    selected_difficulty: load_difficulty(),
                    seed: js_sys::Date::now() as u64 ^ 0xDEAD_BEEF,
                };
                return;
            }

            // Continue
            let continue_y = start_y + btn_h + gap;
            if ms.has_save
                && cx >= btn_x && cx <= btn_x + btn_w
                && cy >= continue_y && cy <= continue_y + btn_h
            {
                ms.app_state = AppState::Playing;
                return;
            }

            // Settings
            let settings_y = continue_y + btn_h + gap;
            if cx >= btn_x && cx <= btn_x + btn_w
                && cy >= settings_y && cy <= settings_y + btn_h
            {
                ms.app_state = AppState::Settings;
            }
        }
        AppState::NewGame { selected_difficulty, seed } => {
            let selected_difficulty = *selected_difficulty;
            let seed = *seed;

            // Back button (top-left area)
            if cx < 100.0 * dpr && cy < 40.0 * dpr {
                ms.app_state = AppState::MainMenu;
                return;
            }

            // Difficulty buttons
            let btn_w = (ref_dim * 0.7).min(300.0 * dpr);
            let btn_h = if compact { 36.0 * dpr } else { 52.0 * dpr };
            let gap = if compact { 6.0 * dpr } else { 10.0 * dpr };
            let btn_x = (cw - btn_w) / 2.0;
            let section_y = if compact { ch * 0.16 } else { ch * 0.22 };
            let list_y = section_y + if compact { 18.0 * dpr } else { 24.0 * dpr };

            for i in 0..3 {
                let y = list_y + (btn_h + gap) * i as f64;
                if cx >= btn_x && cx <= btn_x + btn_w
                    && cy >= y && cy <= y + btn_h
                {
                    ms.app_state = AppState::NewGame {
                        selected_difficulty: i,
                        seed,
                    };
                    return;
                }
            }

            // Seed area tap — randomize seed
            let seed_gap = if compact { 6.0 * dpr } else { 10.0 * dpr };
            let seed_y = list_y + (btn_h + gap) * 3.0 + seed_gap;
            if cy >= seed_y - 20.0 * dpr && cy <= seed_y + 30.0 * dpr
                && cx >= cw * 0.2 && cx <= cw * 0.8
            {
                ms.app_state = AppState::NewGame {
                    selected_difficulty,
                    seed: js_sys::Date::now() as u64 ^ 0xBEEF_CAFE,
                };
                return;
            }

            // Start button
            let start_btn_gap = if compact { 28.0 * dpr } else { 44.0 * dpr };
            let start_y_btn = seed_y + start_btn_gap;
            let start_w = (ref_dim * 0.5).min(220.0 * dpr);
            let start_h = if compact { 38.0 * dpr } else { 48.0 * dpr };
            let start_x = (cw - start_w) / 2.0;
            if cx >= start_x && cx <= start_x + start_w
                && cy >= start_y_btn && cy <= start_y_btn + start_h
            {
                // Create game with selected difficulty
                let difficulties = [Difficulty::Easy, Difficulty::Normal, Difficulty::Hard];
                let diff = difficulties[selected_difficulty];
                save_difficulty(selected_difficulty);
                let config = GameConfig::from_difficulty(diff);
                let new = new_game_with_config(seed, config);
                *game.borrow_mut() = new;
                {
                    let gm = game.borrow();
                    let mut r = renderer.borrow_mut();
                    let map = gm.current_map();
                    r.camera.snap(gm.player_x as f64, gm.player_y as f64, map.width, map.height);
                }
                auto_path.borrow_mut().clear();
                ms.app_state = AppState::Playing;
                ms.has_save = true;
            }
        }
        AppState::Settings => {
            // Back button
            if cx < 100.0 * dpr && cy < 40.0 * dpr {
                ms.app_state = AppState::MainMenu;
                return;
            }

            // Glyph Mode toggle — same layout as draw_settings_menu
            let row_w = (ref_dim * 0.8).min(340.0 * dpr);
            let row_h = if compact { 38.0 * dpr } else { 44.0 * dpr };
            let row_x = (cw - row_w) / 2.0;
            let pad = 14.0 * dpr;
            let row_y = if compact { ch * 0.18 } else { ch * 0.25 };

            let toggle_w = 60.0 * dpr;
            let toggle_h = 28.0 * dpr;
            let toggle_x = row_x + row_w - pad - toggle_w;
            let toggle_y = row_y + (row_h - toggle_h) / 2.0;

            if cx >= toggle_x && cx <= toggle_x + toggle_w
                && cy >= toggle_y && cy <= toggle_y + toggle_h
            {
                let mut r = renderer.borrow_mut();
                r.glyph_mode = !r.glyph_mode;
                save_glyph_mode(r.glyph_mode);
            }
        }
        AppState::Playing => {} // shouldn't be called
    }
}

/// Handle a drawer tap result — shared between portrait and landscape modes.
fn handle_drawer_tap(
    gm: &mut Game,
    renderer: &Rc<RefCell<Renderer>>,
    go_to_menu: &mut bool,
    dtap: DrawerTap,
) {
    match dtap {
        DrawerTap::InventoryItem(idx) => {
            if gm.ui.selected_inventory_item == Some(idx) {
                gm.ui.selected_inventory_item = None;
            } else {
                gm.ui.selected_inventory_item = Some(idx);
            }
            gm.selected_equipment_slot = None;
        }
        DrawerTap::EquipmentSlot(slot) => {
            if gm.selected_equipment_slot == Some(slot) {
                gm.selected_equipment_slot = None;
            } else if gm.equipment_slot_item(slot).is_some() {
                gm.selected_equipment_slot = Some(slot);
            } else {
                gm.selected_equipment_slot = None;
            }
            gm.ui.selected_inventory_item = None;
        }
        DrawerTap::Unequip(slot) => {
            gm.unequip_item(slot);
            gm.selected_equipment_slot = None;
        }
        DrawerTap::UseEquip(idx) => {
            if idx < gm.inventory.len() {
                if gm.inventory[idx].kind.is_consumable() {
                    gm.use_item(idx);
                    gm.ui.drawer = Drawer::None;
                    gm.advance_turn();
                } else {
                    gm.equip_item(idx);
                }
                gm.ui.selected_inventory_item = None;
            }
        }
        DrawerTap::Drop(idx) => {
            gm.drop_item(idx);
            gm.ui.selected_inventory_item = None;
        }
        DrawerTap::ScrollUp => {
            gm.scroll_inventory(-1);
        }
        DrawerTap::ScrollDown => {
            gm.scroll_inventory(1);
        }
        DrawerTap::StatsAllocate(skill) => {
            gm.allocate_skill_point(skill);
        }
        DrawerTap::ToggleGlyphMode => {
            let mut r = renderer.borrow_mut();
            r.glyph_mode = !r.glyph_mode;
            save_glyph_mode(r.glyph_mode);
        }
        DrawerTap::MainMenu => {
            gm.ui.drawer = Drawer::None;
            *go_to_menu = true;
        }
        DrawerTap::Consumed => {}
    }
}

/// Try to use the item in the given quick-bar slot. Returns true if an item was used.
fn use_quick_bar_slot(gm: &mut Game, renderer: &Rc<RefCell<Renderer>>, slot: usize) -> bool {
    if let Some(inv_idx) = gm.quick_bar.slots[slot] {
        if inv_idx < gm.inventory.len() && gm.inventory[inv_idx].kind.is_consumable() {
            gm.use_item(inv_idx);
            renderer.borrow_mut().flash_quickbar_slot(slot);
            return true;
        }
    }
    false
}

fn request_animation_frame(window: &web_sys::Window, f: &Closure<dyn FnMut()>) {
    if let Err(e) = window.request_animation_frame(f.as_ref().unchecked_ref()) {
        errors::report_error(&format!("requestAnimationFrame failed: {:?}", e));
    }
}

/// Get window CSS dimensions, returning fallback (320, 480) on failure.
fn window_css_size() -> (f64, f64) {
    let w = web_sys::window().expect("no global window");
    let css_w = w.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(320.0);
    let css_h = w.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(480.0);
    (css_w, css_h)
}

fn fit_canvas(canvas: &HtmlCanvasElement) -> (f64, f64) {
    let window = web_sys::window().expect("no global window");
    let dpr = window.device_pixel_ratio();
    let (css_w, css_h) = window_css_size();
    let px_w = (css_w * dpr).round();
    let px_h = (css_h * dpr).round();
    canvas.set_width(px_w as u32);
    canvas.set_height(px_h as u32);
    (px_w, px_h)
}

fn new_game_with_config(seed: u64, config: GameConfig) -> Game {
    let mg = &config.mapgen;
    let mut map = Map::generate_forest(mg.overworld_width, mg.overworld_height, seed, mg);
    let entrances = map.place_dungeons(seed.wrapping_add(1), mg);
    map.build_roads(&entrances, mg);
    let world = World::new(map, entrances, seed.wrapping_add(2), mg);
    let mut game = Game::new_overworld_with_config(world, config);
    game.spawn_enemies(seed.wrapping_mul(6364136223846793005));
    game.spawn_overworld_items(seed.wrapping_add(3));
    game.spawn_overworld_food(seed.wrapping_add(4));
    game.update_fov();
    game
}

fn new_game() -> Game {
    let seed = js_sys::Date::now() as u64 ^ 0xDEAD_BEEF;
    new_game_with_config(seed, GameConfig::normal())
}

/// Load an image from a URL and call `on_load` when ready.
/// Reports load failures to the error overlay.
fn load_image(src: &str, on_load: impl FnMut() + 'static) -> HtmlImageElement {
    let img = HtmlImageElement::new().expect("failed to create HtmlImageElement");
    let cb = Closure::<dyn FnMut()>::new(on_load);
    img.set_onload(Some(cb.as_ref().unchecked_ref()));
    cb.forget();
    // Report image load errors to the DOM overlay
    let src_owned = src.to_string();
    let err_cb = Closure::<dyn FnMut()>::new(move || {
        errors::report_error(&format!("failed to load sprite sheet: {}", src_owned));
    });
    img.set_onerror(Some(err_cb.as_ref().unchecked_ref()));
    err_cb.forget();
    img.set_src(src);
    img
}

/// Load an optional image — calls `on_load` with the element if it succeeds,
/// silently ignores failures (no error overlay).
fn load_image_optional(src: &str, mut on_load: impl FnMut(HtmlImageElement) + 'static) -> HtmlImageElement {
    let img = HtmlImageElement::new().expect("failed to create HtmlImageElement");
    let img_clone = img.clone();
    let cb = Closure::<dyn FnMut()>::new(move || {
        on_load(img_clone.clone());
    });
    img.set_onload(Some(cb.as_ref().unchecked_ref()));
    cb.forget();
    // Silently ignore load errors for optional sheets
    img.set_src(src);
    img
}

/// Handle a tap in the game area (portrait or landscape).
/// Dispatches to bottom bar, quick bar, drawer, or world tile hit-tests.
fn handle_game_tap(
    gm: &mut Game,
    renderer: &Rc<RefCell<Renderer>>,
    css_x: f64,
    css_y: f64,
    dpr: f64,
    go_to_menu: &mut bool,
) {
    let (css_w, css_h) = window_css_size();
    let is_landscape = renderer.borrow().landscape;
    let panel_w = renderer.borrow().side_panel_css_w();

    if is_landscape {
        let panel_x = css_w - panel_w;
        if css_x >= panel_x {
            // Tap in side panel — check buttons first, then drawer
            if let Some(tap) = hit_test_side_panel_buttons(css_x, css_y, css_w, panel_w) {
                match tap {
                    BarTap::OpenDrawer(drawer) => gm.toggle_drawer(drawer),
                    BarTap::Sprint => gm.toggle_sprint(),
                }
            } else if let Some(slot) = hit_test_side_panel_quickbar(css_x, css_y, css_w, panel_w, gm.ui.inspected.is_some()) {
                if gm.ui.drawer != Drawer::Inventory {
                    use_quick_bar_slot(gm, renderer, slot);
                }
            } else if gm.ui.drawer != Drawer::None {
                if let Some(dtap) = hit_test_side_panel_drawer(
                    &CssHitArea { x: css_x, y: css_y, w: css_w, h: css_h },
                    panel_w,
                    &DrawerHitState { drawer: gm.ui.drawer, item_count: gm.inventory.len(), inventory_scroll: gm.ui.inventory_scroll, skill_points: gm.skill_points, selected_eq_slot: gm.selected_equipment_slot },
                ) {
                    handle_drawer_tap(gm, renderer, go_to_menu, dtap);
                }
            }
        } else {
            // Tap in game area (left of panel)
            let r = renderer.borrow();
            let (wx, wy) = r.camera.screen_to_world(css_x * dpr, css_y * dpr);
            drop(r);
            let dist = (wx - gm.player_x).abs() + (wy - gm.player_y).abs();
            if dist == 1 && gm.has_enemy_at(wx, wy) {
                gm.attack_adjacent(wx, wy);
            } else if dist == 0 {
                gm.pickup_items_explicit();
            }
            gm.ui.inspected = gm.inspect_tile(wx, wy);
        }
    } else {
        // Portrait mode: bottom bar + quick bar + drawer
        let r_borrow = renderer.borrow();
        let bar_h_css = r_borrow.bottom_bar_height() / dpr;
        let qbar_h_css = r_borrow.quickbar_height() / dpr;
        drop(r_borrow);
        let bottom_region_css = bar_h_css + qbar_h_css;

        if let Some(tap) = hit_test_bottom_bar(css_x, css_y, css_w, css_h, bar_h_css) {
            match tap {
                BarTap::OpenDrawer(drawer) => gm.toggle_drawer(drawer),
                BarTap::Sprint => gm.toggle_sprint(),
            }
        } else if let Some(slot) = hit_test_quick_bar(css_x, css_y, css_w, css_h, bar_h_css, qbar_h_css) {
            if gm.ui.drawer != Drawer::Inventory {
                use_quick_bar_slot(gm, renderer, slot);
            }
        } else if let Some(dtap) = hit_test_drawer(
            &CssHitArea { x: css_x, y: css_y, w: css_w, h: css_h },
            bottom_region_css,
            &DrawerHitState { drawer: gm.ui.drawer, item_count: gm.inventory.len(), inventory_scroll: gm.ui.inventory_scroll, skill_points: gm.skill_points, selected_eq_slot: gm.selected_equipment_slot },
            gm.ui.selected_inventory_item, gm.ui.stats_scroll, gm.has_ranged_weapon(),
        ) {
            handle_drawer_tap(gm, renderer, go_to_menu, dtap);
        } else if css_y < css_h - bottom_region_css {
            // Tap in game area
            let r = renderer.borrow();
            let (wx, wy) = r.camera.screen_to_world(css_x * dpr, css_y * dpr);
            drop(r);
            let dist = (wx - gm.player_x).abs() + (wy - gm.player_y).abs();
            if dist == 1 && gm.has_enemy_at(wx, wy) {
                gm.attack_adjacent(wx, wy);
            } else if dist == 0 {
                gm.pickup_items_explicit();
            }
            gm.ui.inspected = gm.inspect_tile(wx, wy);
        }
    }
}

/// Process the drag-and-drop state machine for quick-bar item assignment.
fn update_drag_state(
    ds: &mut DragState,
    game: &Rc<RefCell<Game>>,
    renderer: &Rc<RefCell<Renderer>>,
    td: Option<input::TouchDown>,
    frame_counter: u32,
    is_landscape: bool,
    dpr: f64,
) {
    match *ds {
        DragState::Idle => {
            if let Some(td) = td {
                let (css_w, css_h) = window_css_size();
                if game.borrow().ui.drawer == Drawer::Inventory {
                    // Finger down while inventory is open — check if on an item row
                    let inv_idx = if is_landscape {
                        let panel_css_w = renderer.borrow().side_panel_css_w();
                        hit_test_inventory_item_row_landscape(td.start_x, td.start_y, css_w, css_h, panel_css_w,
                            game.borrow().inventory.len(), game.borrow().ui.inventory_scroll)
                    } else {
                        let bar_h_css = renderer.borrow().bottom_bar_height() / dpr;
                        let qbar_h_css = renderer.borrow().quickbar_height() / dpr;
                        let bottom_region = bar_h_css + qbar_h_css;
                        hit_test_inventory_item_row(td.start_x, td.start_y, css_w, css_h, bottom_region,
                            game.borrow().inventory.len(), game.borrow().ui.inventory_scroll)
                    };
                    if let Some(idx) = inv_idx {
                        let gm = game.borrow();
                        if idx < gm.inventory.len() && gm.inventory[idx].kind.is_consumable() {
                            *ds = DragState::Pending {
                                inv_index: idx,
                                start_x: td.start_x,
                                start_y: td.start_y,
                                start_frame: frame_counter,
                                from_quickbar_slot: None,
                            };
                        }
                    } else {
                        // Check quick-bar slots — allow dragging back to unassign
                        let qbar_slot = if is_landscape {
                            let panel_css_w = renderer.borrow().side_panel_css_w();
                            hit_test_side_panel_quickbar(td.start_x, td.start_y, css_w, panel_css_w, game.borrow().ui.inspected.is_some())
                        } else {
                            let bar_h_css = renderer.borrow().bottom_bar_height() / dpr;
                            let qbar_h_css = renderer.borrow().quickbar_height() / dpr;
                            hit_test_quick_bar(td.start_x, td.start_y, css_w, css_h, bar_h_css, qbar_h_css)
                        };
                        if let Some(slot) = qbar_slot {
                            if let Some(inv_index) = game.borrow().quick_bar.slots[slot] {
                                *ds = DragState::Pending {
                                    inv_index,
                                    start_x: td.start_x,
                                    start_y: td.start_y,
                                    start_frame: frame_counter,
                                    from_quickbar_slot: Some(slot),
                                };
                            }
                        }
                    }
                }
            }
        }
        DragState::Pending { inv_index, start_x, start_y, start_frame, from_quickbar_slot } => {
            if let Some(td) = td {
                let dx = td.current_x - start_x;
                let dy = td.current_y - start_y;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist > 12.0 || dy.abs() > 6.0 {
                    *ds = DragState::Idle;
                } else if frame_counter.wrapping_sub(start_frame) >= DRAG_LONG_PRESS_FRAMES {
                    *ds = DragState::Dragging {
                        inv_index,
                        touch_x: td.current_x,
                        touch_y: td.current_y,
                        from_quickbar_slot,
                    };
                }
            } else {
                *ds = DragState::Idle;
            }
        }
        DragState::Dragging { inv_index, ref mut touch_x, ref mut touch_y, from_quickbar_slot } => {
            if let Some(td) = td {
                *touch_x = td.current_x;
                *touch_y = td.current_y;
            } else {
                // Finger lifted — drop!
                let (css_w, css_h) = window_css_size();
                let mut gm = game.borrow_mut();

                let drop_slot = if is_landscape {
                    let panel_css_w = renderer.borrow().side_panel_css_w();
                    hit_test_side_panel_quickbar(*touch_x, *touch_y, css_w, panel_css_w, gm.ui.inspected.is_some())
                } else {
                    let bar_h_css = renderer.borrow().bottom_bar_height() / dpr;
                    let qbar_h_css = renderer.borrow().quickbar_height() / dpr;
                    hit_test_quick_bar(*touch_x, *touch_y, css_w, css_h, bar_h_css, qbar_h_css)
                };

                if let Some(slot) = drop_slot {
                    if inv_index < gm.inventory.len() {
                        let item = gm.inventory[inv_index].clone();
                        if let Some(from_slot) = from_quickbar_slot {
                            gm.quick_bar.swap(from_slot, slot);
                        } else {
                            gm.quick_bar.assign(slot, inv_index, &item);
                        }
                    }
                } else if let Some(from_slot) = from_quickbar_slot {
                    gm.quick_bar.clear(from_slot);
                }

                drop(gm);
                *ds = DragState::Idle;
            }
        }
    }
}

/// Process one step of the auto-move queue. Returns true if a map change occurred.
fn process_auto_move(
    game: &Rc<RefCell<Game>>,
    auto_path: &mut Vec<(i32, i32)>,
    tick: &mut u32,
) -> bool {
    if auto_path.is_empty() {
        return false;
    }
    *tick += 1;
    if *tick < AUTO_MOVE_INTERVAL {
        return false;
    }
    *tick = 0;
    let mut gm = game.borrow_mut();
    if !gm.alive || gm.won {
        auto_path.clear();
        return false;
    }
    let (nx, ny) = auto_path[0];
    let dx = nx - gm.player_x;
    let dy = ny - gm.player_y;
    let result = gm.move_player(dx, dy);
    auto_path.remove(0);
    if matches!(result, TurnResult::Blocked) {
        if gm.has_enemy_at(nx, ny) {
            gm.attack_adjacent(nx, ny);
        }
        auto_path.clear();
        return false;
    }
    if gm.player_x != nx || gm.player_y != ny || matches!(result, TurnResult::MapChanged) {
        auto_path.clear();
    }
    matches!(result, TurnResult::MapChanged)
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    errors::install_panic_hook();

    let window = web_sys::window().expect("no global window");
    let document = window.document().expect("window has no document");

    let canvas = document
        .get_element_by_id("canvas")
        .ok_or_else(|| JsValue::from_str("canvas element not found"))?
        .dyn_into::<HtmlCanvasElement>()?;

    let ctx = canvas
        .get_context("2d")?
        .ok_or_else(|| JsValue::from_str("failed to get 2d context"))?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let game = Rc::new(RefCell::new(new_game()));
    let mut rend = Renderer::new(ctx);
    rend.glyph_mode = load_glyph_mode();
    let renderer = Rc::new(RefCell::new(rend));
    let input = Rc::new(Input::new(&canvas));
    let canvas = Rc::new(canvas);
    let menu_state = Rc::new(RefCell::new(MenuState {
        app_state: AppState::MainMenu,
        has_save: false,
    }));

    // Auto-move queue: steps remaining from a pathfind swipe
    let auto_path: Rc<RefCell<Vec<(i32, i32)>>> = Rc::new(RefCell::new(Vec::new()));
    // Preview path: computed during live swipe for rendering
    let preview_path: Rc<RefCell<Vec<(i32, i32)>>> = Rc::new(RefCell::new(Vec::new()));
    // Frame counter for throttling auto-move speed
    let auto_move_tick: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    // Drawer swipe-scroll base: (base_scroll, start_y) when swipe began
    let drawer_swipe_base: Rc<RefCell<Option<(usize, f64)>>> = Rc::new(RefCell::new(None));
    // Drag-and-drop state for quick-bar item assignment
    let drag_state: Rc<RefCell<DragState>> = Rc::new(RefCell::new(DragState::Idle));
    // Frame counter (used for drag long-press timing)
    let frame_counter: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));

    // Load sprite sheets asynchronously (4 core + optional animals)
    {
        let loaded_count: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
        let renderer_for_load = Rc::clone(&renderer);

        let tiles_img: Rc<RefCell<Option<HtmlImageElement>>> = Rc::new(RefCell::new(None));
        let monsters_img: Rc<RefCell<Option<HtmlImageElement>>> = Rc::new(RefCell::new(None));
        let rogues_img: Rc<RefCell<Option<HtmlImageElement>>> = Rc::new(RefCell::new(None));
        let items_img: Rc<RefCell<Option<HtmlImageElement>>> = Rc::new(RefCell::new(None));

        let make_on_load = |slot: Rc<RefCell<Option<HtmlImageElement>>>,
                            loaded: Rc<RefCell<u32>>,
                            rend: Rc<RefCell<Renderer>>,
                            t: Rc<RefCell<Option<HtmlImageElement>>>,
                            m: Rc<RefCell<Option<HtmlImageElement>>>,
                            r: Rc<RefCell<Option<HtmlImageElement>>>,
                            i: Rc<RefCell<Option<HtmlImageElement>>>| {
            move || {
                let _ = slot.borrow();
                let mut count = loaded.borrow_mut();
                *count += 1;
                if *count == 4 {
                    let tiles = t.borrow_mut().take().expect("tiles sprite sheet missing");
                    let monsters = m.borrow_mut().take().expect("monsters sprite sheet missing");
                    let rogues = r.borrow_mut().take().expect("rogues sprite sheet missing");
                    let items = i.borrow_mut().take().expect("items sprite sheet missing");
                    rend.borrow_mut().set_sheets(SpriteSheets { tiles, monsters, rogues, items, animals: None });
                }
            }
        };

        let img = load_image(
            "assets/tiles.png",
            make_on_load(
                Rc::clone(&tiles_img), Rc::clone(&loaded_count), Rc::clone(&renderer_for_load),
                Rc::clone(&tiles_img), Rc::clone(&monsters_img), Rc::clone(&rogues_img), Rc::clone(&items_img),
            ),
        );
        *tiles_img.borrow_mut() = Some(img);

        let img = load_image(
            "assets/monsters.png",
            make_on_load(
                Rc::clone(&monsters_img), Rc::clone(&loaded_count), Rc::clone(&renderer_for_load),
                Rc::clone(&tiles_img), Rc::clone(&monsters_img), Rc::clone(&rogues_img), Rc::clone(&items_img),
            ),
        );
        *monsters_img.borrow_mut() = Some(img);

        let img = load_image(
            "assets/rogues.png",
            make_on_load(
                Rc::clone(&rogues_img), Rc::clone(&loaded_count), Rc::clone(&renderer_for_load),
                Rc::clone(&tiles_img), Rc::clone(&monsters_img), Rc::clone(&rogues_img), Rc::clone(&items_img),
            ),
        );
        *rogues_img.borrow_mut() = Some(img);

        let img = load_image(
            "assets/items.png",
            make_on_load(
                Rc::clone(&items_img), Rc::clone(&loaded_count), Rc::clone(&renderer_for_load),
                Rc::clone(&tiles_img), Rc::clone(&monsters_img), Rc::clone(&rogues_img), Rc::clone(&items_img),
            ),
        );
        *items_img.borrow_mut() = Some(img);

        // Load animals.png optionally — if it fails, animal sprites use glyph fallback
        {
            let rend = Rc::clone(&renderer_for_load);
            let img = load_image_optional("assets/animals.png", move |animals_el| {
                rend.borrow_mut().set_animals_sheet(animals_el);
            });
            std::mem::drop(img);
        }
    }

    // Initial sizing + camera snap
    {
        let dpr = window.device_pixel_ratio();
        let (w, h) = fit_canvas(&canvas);
        let mut r = renderer.borrow_mut();
        r.resize(w, h, dpr);
        let gm = game.borrow();
        let map = gm.current_map();
        r.camera.snap(gm.player_x as f64, gm.player_y as f64, map.width, map.height);
    }

    // Resize handler
    {
        let canvas = Rc::clone(&canvas);
        let game = Rc::clone(&game);
        let renderer = Rc::clone(&renderer);
        let preview_path = Rc::clone(&preview_path);
        let menu_state = Rc::clone(&menu_state);
        let cb = Closure::<dyn FnMut()>::new(move || {
            let dpr = web_sys::window().expect("no global window").device_pixel_ratio();
            let (w, h) = fit_canvas(&canvas);
            let mut r = renderer.borrow_mut();
            r.resize(w, h, dpr);
            let ms = menu_state.borrow();
            match &ms.app_state {
                AppState::Playing => {
                    let gm = game.borrow();
                    let map = gm.current_map();
                    r.camera.snap(gm.player_x as f64, gm.player_y as f64, map.width, map.height);
                    r.draw(&gm, &preview_path.borrow(), None);
                }
                AppState::MainMenu => r.draw_main_menu(ms.has_save),
                AppState::NewGame { selected_difficulty, seed } => {
                    r.draw_new_game_menu(*selected_difficulty, *seed);
                }
                AppState::Settings => r.draw_settings_menu(),
            }
        });
        window
            .add_event_listener_with_callback("resize", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // Game loop
    let f: AnimFrameClosure = Rc::new(RefCell::new(None));
    let g = Rc::clone(&f);

    let window2 = web_sys::window().expect("no global window");
    let menu_state_loop = Rc::clone(&menu_state);
    *g.borrow_mut() = Some(Closure::new(move || {
        let dpr = web_sys::window().expect("no global window").device_pixel_ratio();

        // === Menu state routing ===
        {
            let app_state = menu_state_loop.borrow().app_state.clone();
            match app_state {
                AppState::MainMenu | AppState::NewGame { .. } | AppState::Settings => {
                    // Handle menu input
                    let actions = input.drain();
                    let (css_w, css_h) = window_css_size();
                    for action in &actions {
                        if let InputAction::Tap(cx, cy) = action {
                            let mut ms = menu_state_loop.borrow_mut();
                            handle_menu_tap(&mut ms, &game, &renderer, &auto_path, &CssHitArea { x: *cx, y: *cy, w: css_w, h: css_h }, dpr);
                        }
                    }
                    // Render menu
                    {
                        let (w, h) = fit_canvas(&canvas);
                        let ms = menu_state_loop.borrow();
                        let mut r = renderer.borrow_mut();
                        r.resize(w, h, dpr);
                        match &ms.app_state {
                            AppState::MainMenu => r.draw_main_menu(ms.has_save),
                            AppState::NewGame { selected_difficulty, seed } => {
                                r.draw_new_game_menu(*selected_difficulty, *seed);
                            }
                            AppState::Settings => r.draw_settings_menu(),
                            AppState::Playing => {} // handled below
                        }
                    }
                    request_animation_frame(&window2, f.borrow().as_ref().expect("game loop closure missing"));
                    return;
                }
                AppState::Playing => {} // fall through to game loop
            }
        }

        // Process input actions
        let actions = input.drain();
        if !actions.is_empty() {
            let mut gm = game.borrow_mut();
            if !gm.alive || gm.won {
                // Return to main menu on death/win
                let mut ms = menu_state_loop.borrow_mut();
                ms.app_state = AppState::MainMenu;
                ms.has_save = false;
                drop(ms);
                drop(gm);
                auto_path.borrow_mut().clear();
            } else {
                let mut map_changed = false;
                let mut go_to_menu = false;
                for action in actions {
                    match action {
                        InputAction::Step(dir) => {
                            if gm.ui.drawer != Drawer::None {
                                // Swipe/movement disabled while drawer is open
                                continue;
                            }
                            gm.ui.inspected = None;
                            auto_path.borrow_mut().clear();
                            let (dx, dy) = match dir {
                                input::Direction::Up => (0, -1),
                                input::Direction::Down => (0, 1),
                                input::Direction::Left => (-1, 0),
                                input::Direction::Right => (1, 0),
                                input::Direction::UpLeft => (-1, -1),
                                input::Direction::UpRight => (1, -1),
                                input::Direction::DownLeft => (-1, 1),
                                input::Direction::DownRight => (1, 1),
                            };
                            let result = gm.move_player(dx, dy);
                            if matches!(result, TurnResult::MapChanged) {
                                map_changed = true;
                            } else if matches!(result, TurnResult::Blocked) {
                                // Step blocked by enemy → attack it
                                let tx = gm.player_x + dx;
                                let ty = gm.player_y + dy;
                                if gm.has_enemy_at(tx, ty) {
                                    let atk_result = gm.attack_adjacent(tx, ty);
                                    if matches!(atk_result, TurnResult::MapChanged) {
                                        map_changed = true;
                                    }
                                }
                            }
                        }
                        InputAction::ExecutePath => {
                            if gm.ui.drawer != Drawer::None {
                                continue;
                            }
                            let pp = preview_path.borrow();
                            if pp.len() > 1 {
                                let target = pp[pp.len() - 1];
                                let enemy_at_target = gm.has_ranged_weapon()
                                    && gm.has_enemy_at(target.0, target.1);
                                if enemy_at_target {
                                    // Fire ranged weapon at the targeted enemy
                                    drop(pp);
                                    let result = gm.ranged_attack(target.0, target.1);
                                    if matches!(result, TurnResult::MapChanged) {
                                        map_changed = true;
                                    }
                                } else {
                                    // Normal pathfinding movement
                                    *auto_path.borrow_mut() = pp[1..].to_vec();
                                }
                            }
                        }
                        InputAction::Tap(css_x, css_y) => {
                            // Skip tap only if actively dragging (drop is handled separately).
                            if matches!(*drag_state.borrow(), DragState::Dragging { .. }) {
                                continue;
                            }
                            // Cancel any pending drag since the finger lifted (tap = touchend)
                            if !matches!(*drag_state.borrow(), DragState::Idle) {
                                *drag_state.borrow_mut() = DragState::Idle;
                            }
                            handle_game_tap(&mut gm, &renderer, css_x, css_y, dpr, &mut go_to_menu);
                        }
                        InputAction::ToggleInventory => {
                            gm.toggle_drawer(Drawer::Inventory);
                        }
                        InputAction::ToggleStats => {
                            gm.toggle_drawer(Drawer::Stats);
                        }
                        InputAction::ToggleSprint => {
                            gm.toggle_sprint();
                        }
                        InputAction::ToggleGlyphMode => {
                            let mut r = renderer.borrow_mut();
                            r.glyph_mode = !r.glyph_mode;
                            save_glyph_mode(r.glyph_mode);
                        }
                        InputAction::QuickUse(slot) => {
                            if gm.ui.drawer == Drawer::None {
                                use_quick_bar_slot(&mut gm, &renderer, slot);
                            }
                        }
                        InputAction::Interact => {
                            if gm.ui.drawer != Drawer::None {
                                continue;
                            }
                            // Try to attack adjacent enemy in facing direction
                            let (fdx, fdy) = if gm.player_facing_left { (-1, 0) } else { (1, 0) };
                            let tx = gm.player_x + fdx;
                            let ty = gm.player_y + fdy;
                            if gm.has_enemy_at(tx, ty) {
                                gm.attack_adjacent(tx, ty);
                            } else {
                                // Try all 8 directions for adjacent enemy (including diagonals)
                                let dirs = [(0, -1), (0, 1), (-1, 0), (1, 0),
                                            (-1, -1), (1, -1), (-1, 1), (1, 1)];
                                let mut attacked = false;
                                for (dx, dy) in dirs {
                                    let ax = gm.player_x + dx;
                                    let ay = gm.player_y + dy;
                                    if gm.has_enemy_at(ax, ay) {
                                        gm.attack_adjacent(ax, ay);
                                        attacked = true;
                                        break;
                                    }
                                }
                                // No adjacent enemy → pick up items at feet
                                if !attacked {
                                    gm.pickup_items_explicit();
                                }
                            }
                        }
                    }
                }
                // Camera snap on map transition
                if map_changed {
                    let mut r = renderer.borrow_mut();
                    let map = gm.current_map();
                    r.camera.snap(gm.player_x as f64, gm.player_y as f64, map.width, map.height);
                    auto_path.borrow_mut().clear();
                }
                // Return to main menu (from in-game settings)
                if go_to_menu {
                    drop(gm);
                    let mut ms = menu_state_loop.borrow_mut();
                    ms.app_state = AppState::MainMenu;
                    ms.has_save = true;
                }
            }
        }

        // Compute live preview path from swipe state + inspect hovered tile
        // OR handle drawer swipe-scrolling if drawer is open
        {
            let mut pp = preview_path.borrow_mut();
            pp.clear();
            if let Some(swipe) = input.swipe_state() {
                let mut gm = game.borrow_mut();
                if gm.alive && !gm.won && gm.ui.drawer == Drawer::None {
                    let dx = (swipe.current_x - swipe.start_x) * 0.7;
                    let dy = (swipe.current_y - swipe.start_y) * 0.7;
                    let (gdx, gdy) = renderer.borrow().camera.css_delta_to_grid(dx, dy, dpr);
                    let dest = (gm.player_x + gdx, gm.player_y + gdy);
                    let map = gm.current_map();
                    // Aim mode: show Bresenham line only when hovering an enemy
                    let enemy_at_dest = gm.has_ranged_weapon()
                        && gm.has_enemy_at(dest.0, dest.1);
                    if enemy_at_dest {
                        let line = bresenham_line(gm.player_x, gm.player_y, dest.0, dest.1);
                        *pp = line;
                    } else if map.is_walkable(dest.0, dest.1) {
                        let path = map.find_path((gm.player_x, gm.player_y), dest);
                        *pp = path;
                    }
                    // Inspect the destination tile during swipe
                    gm.ui.inspected = gm.inspect_tile(dest.0, dest.1);
                } else if gm.ui.drawer == Drawer::Inventory {
                    // Swipe-scroll inventory when drawer is open (skip if dragging)
                    if matches!(*drag_state.borrow(), DragState::Idle) {
                        let slot_h_css = 34.0;
                        let mut base = drawer_swipe_base.borrow_mut();
                        if base.is_none() {
                            *base = Some((gm.ui.inventory_scroll, swipe.start_y));
                        }
                        if let Some((base_scroll, start_y)) = *base {
                            let dy = start_y - swipe.current_y; // positive = scroll down
                            let delta = (dy / slot_h_css).round() as i32;
                            let new_scroll = (base_scroll as i32 + delta).max(0) as usize;
                            gm.set_inventory_scroll(new_scroll);
                        }
                    }
                } else if gm.ui.drawer == Drawer::Stats {
                    // Swipe-scroll stats panel
                    let mut base = drawer_swipe_base.borrow_mut();
                    if base.is_none() {
                        *base = Some((gm.ui.stats_scroll as usize, swipe.start_y));
                    }
                    if let Some((base_scroll, start_y)) = *base {
                        let dy = start_y - swipe.current_y; // positive = scroll down
                        let new_scroll = (base_scroll as f64 + dy).max(0.0);
                        gm.ui.stats_scroll = new_scroll;
                    }
                }
            } else {
                // Swipe ended — clear drawer swipe base
                *drawer_swipe_base.borrow_mut() = None;
            }
        }

        // === Drag-and-drop state machine for quick-bar ===
        {
            let fc = *frame_counter.borrow();
            let is_landscape = renderer.borrow().landscape;
            update_drag_state(
                &mut drag_state.borrow_mut(),
                &game,
                &renderer,
                input.touch_down(),
                fc,
                is_landscape,
                dpr,
            );
            *frame_counter.borrow_mut() = fc.wrapping_add(1);
        }

        // Process auto-move queue (one step every 8 frames ≈ 7.5 tiles/sec)
        process_auto_move(&game, &mut auto_path.borrow_mut(), &mut auto_move_tick.borrow_mut());

        // Update camera — follow the player
        {
            let gm = game.borrow();
            let map = gm.current_map();
            renderer.borrow_mut().camera.follow(
                gm.player_x as f64,
                gm.player_y as f64,
                map.width,
                map.height,
            );
        }

        // Tick animations (floating texts, bump anims, visual effects)
        {
            game.borrow_mut().tick_animations();
        }

        // Advance drawer animation + render
        {
            let gm = game.borrow();
            let mut r = renderer.borrow_mut();
            r.tick_drawer_anim(gm.ui.drawer);
            r.tick_quickbar_flash();
            let ds = drag_state.borrow();
            let drag_info = if let DragState::Dragging { inv_index, touch_x, touch_y, .. } = &*ds {
                gm.inventory.get(*inv_index).map(|item| renderer::DragInfo {
                    item,
                    css_x: *touch_x,
                    css_y: *touch_y,
                })
            } else {
                None
            };
            r.draw(&gm, &preview_path.borrow(), drag_info.as_ref());
        }

        // Schedule next frame
        request_animation_frame(&window2, f.borrow().as_ref().expect("game loop closure missing"));
    }));

    let window3 = web_sys::window().expect("no global window");
    request_animation_frame(&window3, g.borrow().as_ref().expect("game loop closure missing"));

    Ok(())
}
