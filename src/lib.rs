mod camera;
mod errors;
mod game;
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

use game::{Drawer, Game, ItemKind, SkillKind, TurnResult};
use input::{Input, InputAction};
use map::{bresenham_line, Map};
use renderer::{Renderer, SpriteSheets};
use world::World;

/// Result of tapping the bottom bar.
enum BarTap {
    OpenDrawer(Drawer),
    Sprint,
}

/// Bottom bar button layout: returns which button was tapped (if any).
/// `css_y` and `canvas_h_css` are in CSS pixels (pre-DPR).
fn hit_test_bottom_bar(css_x: f64, css_y: f64, css_w: f64, css_h: f64, bar_h_css: f64) -> Option<BarTap> {
    let bar_top = css_h - bar_h_css;
    if css_y < bar_top {
        return None;
    }
    let btn_count = 4.0;
    let btn_w = (css_w / btn_count).min(110.0);
    let total_w = btn_w * btn_count;
    let start_x = (css_w - total_w) / 2.0;
    let idx = ((css_x - start_x) / btn_w).floor() as i32;
    match idx {
        0 => Some(BarTap::OpenDrawer(Drawer::Inventory)),
        1 => Some(BarTap::OpenDrawer(Drawer::Stats)),
        2 => Some(BarTap::Sprint),
        3 => Some(BarTap::OpenDrawer(Drawer::Settings)),
        _ => None,
    }
}

/// Result of tapping inside a drawer.
enum DrawerTap {
    /// Tapped an inventory item at the given index (absolute, not visual).
    InventoryItem(usize),
    /// Scroll the inventory list up.
    ScrollUp,
    /// Scroll the inventory list down.
    ScrollDown,
    /// Use/Equip button tapped for the selected item (detail bar).
    UseEquip(usize),
    /// Drop button tapped for the selected item (detail bar).
    Drop(usize),
    /// Inline Use/Equip button on a list item row.
    InlineUseEquip(usize),
    /// Inline Drop button on a list item row.
    InlineDrop(usize),
    /// Allocate a skill point to the given attribute.
    StatsAllocate(SkillKind),
    /// Toggle glyph rendering mode.
    ToggleGlyphMode,
    /// Tapped inside the drawer but not on an actionable element.
    Consumed,
}

/// Drawer hit test: returns `Some` if the tap landed inside the drawer area.
/// All coordinates are in CSS pixels (pre-DPR).
/// Layout constants here mirror `renderer::draw_inventory_drawer` / `draw_stats_drawer`
/// (which use `base * dpr` in canvas pixels — dividing by dpr gives CSS points).
fn hit_test_drawer(
    css_x: f64,
    css_y: f64,
    css_w: f64,
    css_h: f64,
    bar_h_css: f64,
    drawer: Drawer,
    item_count: usize,
    inventory_scroll: usize,
    selected_item: Option<usize>,
    skill_points: u32,
    stats_scroll: f64,
    has_ranged_weapon: bool,
) -> Option<DrawerTap> {
    let drawer_frac = match drawer {
        Drawer::None => return None,
        Drawer::Inventory => 0.55,
        Drawer::Stats => 0.55,
        Drawer::Settings => 0.35,
    };
    let drawer_h = css_h * drawer_frac;
    let drawer_y = css_h - bar_h_css - drawer_h;

    // Not inside the drawer area
    if css_y < drawer_y || css_y >= css_h - bar_h_css {
        return None;
    }

    // Settings drawer: glyph mode toggle
    if drawer == Drawer::Settings {
        // Toggle button layout mirrors renderer::draw_settings_drawer (CSS space)
        let pad = 12.0;
        let row_y = drawer_y + 40.0;
        let row_h = 40.0;
        let toggle_w = 70.0;
        let toggle_h = 30.0;
        let toggle_x = css_w - pad - toggle_w;
        let toggle_y = row_y + (row_h - toggle_h) / 2.0;
        if css_x >= toggle_x && css_x <= toggle_x + toggle_w
            && css_y >= toggle_y && css_y <= toggle_y + toggle_h
        {
            return Some(DrawerTap::ToggleGlyphMode);
        }
        return Some(DrawerTap::Consumed);
    }

    // Stats drawer: check for skill point allocation buttons
    if drawer == Drawer::Stats {
        if skill_points > 0 {
            // Compute where the skill section starts in CSS space, matching renderer layout.
            // All renderer values are in `val * dpr` canvas pixels; dividing by dpr gives CSS px.
            // content_top = drawer_y + 32 (header)
            // sprite area: icon(36) + gap(6) = 42
            // xp bar area: bar(10) + gap(12) = 22
            // stat rows: (4 base + 1 if ranged + 1 location) * 24
            let stat_row_count = if has_ranged_weapon { 6.0 } else { 5.0 };
            // skill section gap(8) + "Skill Points" header(20) = 28
            let skill_section_offset = 32.0 + 42.0 + 22.0 + stat_row_count * 24.0 + 8.0 + 20.0;
            // Apply scroll offset — renderer shifts content up by stats_scroll
            let skill_section_y = drawer_y + skill_section_offset - stats_scroll;

            let skill_row_h = 30.0;
            let btn_sz = 24.0;
            let pad = 12.0;
            let btn_x = css_w - pad - btn_sz;
            let skills = [SkillKind::Strength, SkillKind::Vitality, SkillKind::Dexterity, SkillKind::Stamina];
            for (i, skill) in skills.iter().enumerate() {
                let row_y = skill_section_y + i as f64 * skill_row_h;
                let btn_y = row_y + (skill_row_h - btn_sz) / 2.0;
                if css_x >= btn_x && css_x <= btn_x + btn_sz
                    && css_y >= btn_y && css_y <= btn_y + btn_sz
                {
                    return Some(DrawerTap::StatsAllocate(*skill));
                }
            }
        }
        return Some(DrawerTap::Consumed);
    }

    // Inventory drawer: match layout from renderer::draw_inventory_drawer
    // Equipment: 3 rows × 2 cols, eq_h=30, eq_gap=4
    let eq_y = drawer_y + 32.0;
    let eq_h = 30.0;
    let eq_gap = 4.0;
    let list_y = eq_y + (eq_h + eq_gap) * 3.0 + 4.0;
    let slot_h = 34.0;
    let detail_bar_h = if selected_item.is_some() { 46.0 } else { 20.0 };
    let avail_h = (drawer_y + drawer_h - detail_bar_h) - list_y;
    let max_visible = (avail_h / slot_h).floor().max(1.0) as usize;
    let end = (inventory_scroll + max_visible).min(item_count);
    let pad = 12.0;
    let scrollbar_w = 12.0;

    // Detail bar buttons hit test (when an item is selected)
    if let Some(sel_idx) = selected_item {
        let bar_y = drawer_y + drawer_h - detail_bar_h;
        if css_y >= bar_y {
            let btn_h = 26.0;
            let btn_gap = 8.0;
            let btn_y = bar_y + detail_bar_h - btn_h - 4.0;

            if css_y >= btn_y && css_y <= btn_y + btn_h {
                // Use/Equip button
                let action_w = 60.0;
                let action_x = css_w - pad - action_w - btn_gap - 60.0;
                if css_x >= action_x && css_x <= action_x + action_w {
                    return Some(DrawerTap::UseEquip(sel_idx));
                }
                // Drop button
                let drop_w = 60.0;
                let drop_x = css_w - pad - drop_w;
                if css_x >= drop_x && css_x <= drop_x + drop_w {
                    return Some(DrawerTap::Drop(sel_idx));
                }
            }
            return Some(DrawerTap::Consumed);
        }
    }

    if css_y >= list_y && item_count > 0 {
        // Inline button hit-test (Use/Equip and Drop buttons on each row)
        let inline_btn_w = 36.0;
        let inline_btn_h = 22.0;
        let inline_btn_gap = 3.0;
        let text_right = css_w - pad - scrollbar_w - 4.0;
        let drop_btn_x = text_right - inline_btn_w;
        let use_btn_x = drop_btn_x - inline_btn_gap - inline_btn_w;

        let vis_idx = ((css_y - list_y) / slot_h).floor() as usize;
        let abs_idx = inventory_scroll + vis_idx;
        if abs_idx < end {
            let row_y = list_y + vis_idx as f64 * slot_h;
            let btn_y = row_y + (slot_h - inline_btn_h) / 2.0;
            if css_y >= btn_y && css_y <= btn_y + inline_btn_h {
                if css_x >= use_btn_x && css_x <= use_btn_x + inline_btn_w {
                    return Some(DrawerTap::InlineUseEquip(abs_idx));
                }
                if css_x >= drop_btn_x && css_x <= drop_btn_x + inline_btn_w {
                    return Some(DrawerTap::InlineDrop(abs_idx));
                }
            }
        }

        // Check if tap is in scrollbar track (right edge of list area)
        let scrollbar_x = css_w - pad - scrollbar_w;
        if css_x >= scrollbar_x && item_count > max_visible {
            let track_h = max_visible as f64 * slot_h;
            let scroll_range = item_count - max_visible;
            let thumb_frac = max_visible as f64 / item_count as f64;
            let min_thumb_h = 20.0;
            let thumb_h = (track_h * thumb_frac).max(min_thumb_h);
            let scroll_frac = if scroll_range > 0 {
                inventory_scroll as f64 / scroll_range as f64
            } else {
                0.0
            };
            let thumb_y = list_y + scroll_frac * (track_h - thumb_h);

            if css_y < thumb_y {
                return Some(DrawerTap::ScrollUp);
            } else if css_y > thumb_y + thumb_h {
                return Some(DrawerTap::ScrollDown);
            }
            return Some(DrawerTap::Consumed); // tap on thumb itself
        }

        // Regular item tap (select, not use)
        let vis_idx = ((css_y - list_y) / slot_h).floor() as usize;
        let abs_idx = inventory_scroll + vis_idx;
        if abs_idx < end {
            return Some(DrawerTap::InventoryItem(abs_idx));
        }
    }

    Some(DrawerTap::Consumed)
}


/// Load glyph_mode setting from localStorage.
fn load_glyph_mode() -> bool {
    web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|s| s.get_item("glyph_mode").ok().flatten())
        .map(|v| v == "true")
        .unwrap_or(false)
}

/// Save glyph_mode setting to localStorage.
fn save_glyph_mode(enabled: bool) {
    if let Some(storage) = web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
    {
        let _ = storage.set_item("glyph_mode", if enabled { "true" } else { "false" });
    }
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

fn new_game() -> Game {
    let seed = js_sys::Date::now() as u64 ^ 0xDEAD_BEEF;
    let mut map = Map::generate_forest(200, 200, seed);
    let entrances = map.place_dungeons(seed.wrapping_add(1));
    map.build_roads(&entrances);
    let world = World::new(map, entrances, seed.wrapping_add(2));
    let mut game = Game::new_overworld(world);
    game.spawn_enemies(seed.wrapping_mul(6364136223846793005));
    game.spawn_overworld_items(seed.wrapping_add(3));
    game.spawn_overworld_food(seed.wrapping_add(4));
    game.update_fov();
    game
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

    // Auto-move queue: steps remaining from a pathfind swipe
    let auto_path: Rc<RefCell<Vec<(i32, i32)>>> = Rc::new(RefCell::new(Vec::new()));
    // Preview path: computed during live swipe for rendering
    let preview_path: Rc<RefCell<Vec<(i32, i32)>>> = Rc::new(RefCell::new(Vec::new()));
    // Frame counter for throttling auto-move speed
    let auto_move_tick: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    // Drawer swipe-scroll base: (base_scroll, start_y) when swipe began
    let drawer_swipe_base: Rc<RefCell<Option<(usize, f64)>>> = Rc::new(RefCell::new(None));

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
        let cb = Closure::<dyn FnMut()>::new(move || {
            let dpr = web_sys::window().expect("no global window").device_pixel_ratio();
            let (w, h) = fit_canvas(&canvas);
            let gm = game.borrow();
            let mut r = renderer.borrow_mut();
            r.resize(w, h, dpr);
            let map = gm.current_map();
            r.camera.snap(gm.player_x as f64, gm.player_y as f64, map.width, map.height);
            r.draw(&gm, &preview_path.borrow());
        });
        window
            .add_event_listener_with_callback("resize", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // Game loop
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = Rc::clone(&f);

    let window2 = web_sys::window().expect("no global window");
    *g.borrow_mut() = Some(Closure::new(move || {
        let dpr = web_sys::window().expect("no global window").device_pixel_ratio();

        // Process input actions
        let actions = input.drain();
        if !actions.is_empty() {
            let mut gm = game.borrow_mut();
            if !gm.alive || gm.won {
                *gm = new_game();
                let mut r = renderer.borrow_mut();
                let map = gm.current_map();
                r.camera.snap(gm.player_x as f64, gm.player_y as f64, map.width, map.height);
                auto_path.borrow_mut().clear();
            } else {
                let mut map_changed = false;
                for action in actions {
                    match action {
                        InputAction::Step(dir) => {
                            if gm.drawer != Drawer::None {
                                // Swipe/movement disabled while drawer is open
                                continue;
                            }
                            gm.inspected = None;
                            auto_path.borrow_mut().clear();
                            let (dx, dy) = match dir {
                                input::Direction::Up => (0, -1),
                                input::Direction::Down => (0, 1),
                                input::Direction::Left => (-1, 0),
                                input::Direction::Right => (1, 0),
                            };
                            let result = gm.move_player(dx, dy);
                            if matches!(result, TurnResult::MapChanged) {
                                map_changed = true;
                            } else if matches!(result, TurnResult::Blocked) {
                                // Step blocked by enemy → attack it
                                let tx = gm.player_x + dx;
                                let ty = gm.player_y + dy;
                                if gm.enemies.iter().any(|e| e.x == tx && e.y == ty && e.hp > 0) {
                                    let atk_result = gm.attack_adjacent(tx, ty);
                                    if matches!(atk_result, TurnResult::MapChanged) {
                                        map_changed = true;
                                    }
                                }
                            }
                        }
                        InputAction::ExecutePath => {
                            if gm.drawer != Drawer::None {
                                continue;
                            }
                            let pp = preview_path.borrow();
                            if pp.len() > 1 {
                                let target = pp[pp.len() - 1];
                                let enemy_at_target = gm.has_ranged_weapon()
                                    && gm.enemies.iter().any(|e| e.x == target.0 && e.y == target.1 && e.hp > 0);
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
                            let (css_w, css_h) = window_css_size();
                            let bar_h_css = renderer.borrow().bottom_bar_height() / dpr;

                            // Bottom bar hit test first
                            if let Some(tap) = hit_test_bottom_bar(css_x, css_y, css_w, css_h, bar_h_css) {
                                match tap {
                                    BarTap::OpenDrawer(drawer) => {
                                        gm.toggle_drawer(drawer);
                                    }
                                    BarTap::Sprint => {
                                        gm.toggle_sprint();
                                    }
                                }
                            } else if let Some(dtap) = hit_test_drawer(
                                css_x, css_y, css_w, css_h, bar_h_css,
                                gm.drawer, gm.inventory.len(), gm.inventory_scroll,
                                gm.selected_inventory_item, gm.skill_points,
                                gm.stats_scroll, gm.has_ranged_weapon(),
                            ) {
                                // Tap landed inside an open drawer
                                match dtap {
                                    DrawerTap::InventoryItem(idx) => {
                                        // Tap selects (or deselects if same)
                                        if gm.selected_inventory_item == Some(idx) {
                                            gm.selected_inventory_item = None;
                                        } else {
                                            gm.selected_inventory_item = Some(idx);
                                        }
                                    }
                                    DrawerTap::UseEquip(idx) | DrawerTap::InlineUseEquip(idx) => {
                                        if idx < gm.inventory.len() {
                                            let is_consumable = matches!(
                                                gm.inventory[idx].kind,
                                                ItemKind::Potion | ItemKind::Scroll | ItemKind::Food
                                            );
                                            if is_consumable {
                                                gm.use_item(idx);
                                                // Using a consumable costs a turn — close drawer to see it
                                                gm.drawer = Drawer::None;
                                                gm.advance_turn();
                                            } else {
                                                gm.equip_item(idx);
                                            }
                                            gm.selected_inventory_item = None;
                                        }
                                    }
                                    DrawerTap::Drop(idx) | DrawerTap::InlineDrop(idx) => {
                                        gm.drop_item(idx);
                                        gm.selected_inventory_item = None;
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
                                    DrawerTap::Consumed => {}
                                }
                            } else if css_y < css_h - bar_h_css {
                                // Tap in game area
                                let r = renderer.borrow();
                                let (wx, wy) = r.camera.screen_to_world(css_x * dpr, css_y * dpr);
                                drop(r);
                                let dist = (wx - gm.player_x).abs() + (wy - gm.player_y).abs();
                                // Tap adjacent enemy → explicit attack
                                if dist == 1 && gm.enemies.iter().any(|e| e.x == wx && e.y == wy && e.hp > 0) {
                                    gm.attack_adjacent(wx, wy);
                                } else if dist == 0 {
                                    // Tap on player tile → pick up items
                                    gm.pickup_items_explicit();
                                }
                                // Always inspect the tile
                                gm.inspected = gm.inspect_tile(wx, wy);
                            }
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
                        InputAction::Interact => {
                            if gm.drawer != Drawer::None {
                                continue;
                            }
                            // Try to attack adjacent enemy in facing direction
                            let (fdx, fdy) = if gm.player_facing_left { (-1, 0) } else { (1, 0) };
                            let tx = gm.player_x + fdx;
                            let ty = gm.player_y + fdy;
                            if gm.enemies.iter().any(|e| e.x == tx && e.y == ty && e.hp > 0) {
                                gm.attack_adjacent(tx, ty);
                            } else {
                                // Try all 4 directions for adjacent enemy
                                let dirs = [(0, -1), (0, 1), (-1, 0), (1, 0)];
                                let mut attacked = false;
                                for (dx, dy) in dirs {
                                    let ax = gm.player_x + dx;
                                    let ay = gm.player_y + dy;
                                    if gm.enemies.iter().any(|e| e.x == ax && e.y == ay && e.hp > 0) {
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
            }
        }

        // Compute live preview path from swipe state + inspect hovered tile
        // OR handle drawer swipe-scrolling if drawer is open
        {
            let mut pp = preview_path.borrow_mut();
            pp.clear();
            if let Some(swipe) = input.swipe_state() {
                let mut gm = game.borrow_mut();
                if gm.alive && !gm.won && gm.drawer == Drawer::None {
                    let dx = (swipe.current_x - swipe.start_x) * 0.7;
                    let dy = (swipe.current_y - swipe.start_y) * 0.7;
                    let (gdx, gdy) = renderer.borrow().camera.css_delta_to_grid(dx, dy, dpr);
                    let dest = (gm.player_x + gdx, gm.player_y + gdy);
                    let map = gm.current_map();
                    // Aim mode: show Bresenham line only when hovering an enemy
                    let enemy_at_dest = gm.has_ranged_weapon()
                        && gm.enemies.iter().any(|e| e.x == dest.0 && e.y == dest.1 && e.hp > 0);
                    if enemy_at_dest {
                        let line = bresenham_line(gm.player_x, gm.player_y, dest.0, dest.1);
                        *pp = line;
                    } else if map.is_walkable(dest.0, dest.1) {
                        let path = map.find_path((gm.player_x, gm.player_y), dest);
                        *pp = path;
                    }
                    // Inspect the destination tile during swipe
                    gm.inspected = gm.inspect_tile(dest.0, dest.1);
                } else if gm.drawer == Drawer::Inventory {
                    // Swipe-scroll inventory when drawer is open
                    let slot_h_css = 34.0;
                    let mut base = drawer_swipe_base.borrow_mut();
                    if base.is_none() {
                        *base = Some((gm.inventory_scroll, swipe.start_y));
                    }
                    if let Some((base_scroll, start_y)) = *base {
                        let dy = start_y - swipe.current_y; // positive = scroll down
                        let delta = (dy / slot_h_css).round() as i32;
                        let new_scroll = (base_scroll as i32 + delta).max(0) as usize;
                        gm.set_inventory_scroll(new_scroll);
                    }
                } else if gm.drawer == Drawer::Stats {
                    // Swipe-scroll stats panel
                    let mut base = drawer_swipe_base.borrow_mut();
                    if base.is_none() {
                        *base = Some((gm.stats_scroll as usize, swipe.start_y));
                    }
                    if let Some((base_scroll, start_y)) = *base {
                        let dy = start_y - swipe.current_y; // positive = scroll down
                        let new_scroll = (base_scroll as f64 + dy).max(0.0);
                        gm.stats_scroll = new_scroll;
                    }
                }
            } else {
                // Swipe ended — clear drawer swipe base
                *drawer_swipe_base.borrow_mut() = None;
            }
        }

        // Process auto-move queue (one step every 8 frames ≈ 7.5 tiles/sec)
        {
            let mut ap = auto_path.borrow_mut();
            if !ap.is_empty() {
                let mut tick = auto_move_tick.borrow_mut();
                *tick += 1;
                if *tick >= 8 {
                    *tick = 0;
                    let mut gm = game.borrow_mut();
                    if gm.alive && !gm.won {
                        let (nx, ny) = ap[0];
                        let dx = nx - gm.player_x;
                        let dy = ny - gm.player_y;
                        let result = gm.move_player(dx, dy);
                        ap.remove(0);
                        if matches!(result, TurnResult::Blocked) {
                            // Blocked by enemy → attack and stop auto-move
                            if gm.enemies.iter().any(|e| e.x == nx && e.y == ny && e.hp > 0) {
                                gm.attack_adjacent(nx, ny);
                            }
                            ap.clear();
                        } else if gm.player_x != nx || gm.player_y != ny || matches!(result, TurnResult::MapChanged) {
                            ap.clear();
                        }
                    } else {
                        ap.clear();
                    }
                }
            }
        }

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
            r.tick_drawer_anim(gm.drawer);
            r.draw(&gm, &preview_path.borrow());
        }

        // Schedule next frame
        request_animation_frame(&window2, f.borrow().as_ref().expect("game loop closure missing"));
    }));

    let window3 = web_sys::window().expect("no global window");
    request_animation_frame(&window3, g.borrow().as_ref().expect("game loop closure missing"));

    Ok(())
}
