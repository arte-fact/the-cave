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

use game::{Drawer, Game, ItemKind, TurnResult};
use input::{Input, InputAction};
use map::Map;
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
        3 => Some(BarTap::OpenDrawer(Drawer::None)), // Menu (closes drawers)
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
) -> Option<DrawerTap> {
    let drawer_frac = match drawer {
        Drawer::None => return None,
        Drawer::Inventory => 0.55,
        Drawer::Stats => 0.45,
    };
    let drawer_h = css_h * drawer_frac;
    let drawer_y = css_h - bar_h_css - drawer_h;

    // Not inside the drawer area
    if css_y < drawer_y || css_y >= css_h - bar_h_css {
        return None;
    }

    // Stats drawer has no interactive elements
    if drawer == Drawer::Stats {
        return Some(DrawerTap::Consumed);
    }

    // Inventory drawer: match layout from renderer::draw_inventory_drawer
    // Equipment: 3 rows × 2 cols, eq_h=30, eq_gap=4
    let eq_y = drawer_y + 32.0;
    let eq_h = 30.0;
    let eq_gap = 4.0;
    let list_y = eq_y + (eq_h + eq_gap) * 3.0 + 4.0;
    let slot_h = 34.0;
    let footer_h = 20.0;
    let avail_h = (drawer_y + drawer_h - footer_h) - list_y;
    let max_visible = (avail_h / slot_h).floor().max(1.0) as usize;
    let end = (inventory_scroll + max_visible).min(item_count);
    let pad = 12.0;
    let scrollbar_w = 12.0;

    if css_y >= list_y && item_count > 0 {
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

        // Regular item tap
        let vis_idx = ((css_y - list_y) / slot_h).floor() as usize;
        let abs_idx = inventory_scroll + vis_idx;
        if abs_idx < end {
            return Some(DrawerTap::InventoryItem(abs_idx));
        }
    }

    Some(DrawerTap::Consumed)
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
    let renderer = Rc::new(RefCell::new(Renderer::new(ctx)));
    let input = Rc::new(Input::new(&canvas));
    let canvas = Rc::new(canvas);

    // Auto-move queue: steps remaining from a pathfind swipe
    let auto_path: Rc<RefCell<Vec<(i32, i32)>>> = Rc::new(RefCell::new(Vec::new()));
    // Preview path: computed during live swipe for rendering
    let preview_path: Rc<RefCell<Vec<(i32, i32)>>> = Rc::new(RefCell::new(Vec::new()));
    // Frame counter for throttling auto-move speed
    let auto_move_tick: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));

    // Load sprite sheets asynchronously
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
                    rend.borrow_mut().set_sheets(SpriteSheets { tiles, monsters, rogues, items });
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
                            if matches!(gm.move_player(dx, dy), TurnResult::MapChanged) {
                                map_changed = true;
                            }
                        }
                        InputAction::ExecutePath => {
                            if gm.drawer != Drawer::None {
                                continue;
                            }
                            let pp = preview_path.borrow();
                            if pp.len() > 1 {
                                *auto_path.borrow_mut() = pp[1..].to_vec();
                            }
                        }
                        InputAction::Tap(css_x, css_y) => {
                            let (css_w, css_h) = window_css_size();
                            let bar_h_css = renderer.borrow().bottom_bar_height() / dpr;

                            // Bottom bar hit test first
                            if let Some(tap) = hit_test_bottom_bar(css_x, css_y, css_w, css_h, bar_h_css) {
                                match tap {
                                    BarTap::OpenDrawer(Drawer::None) => {
                                        gm.drawer = Drawer::None;
                                    }
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
                            ) {
                                // Tap landed inside an open drawer
                                match dtap {
                                    DrawerTap::InventoryItem(idx) => {
                                        match &gm.inventory[idx].kind {
                                            ItemKind::Potion | ItemKind::Scroll | ItemKind::Food => {
                                                gm.use_item(idx);
                                            }
                                            ItemKind::Weapon | ItemKind::Armor | ItemKind::Helmet
                                            | ItemKind::Shield | ItemKind::Boots | ItemKind::Ring => {
                                                gm.equip_item(idx);
                                            }
                                        }
                                    }
                                    DrawerTap::ScrollUp => {
                                        gm.scroll_inventory(-1);
                                    }
                                    DrawerTap::ScrollDown => {
                                        gm.scroll_inventory(1);
                                    }
                                    DrawerTap::Consumed => {}
                                }
                            } else if css_y < css_h - bar_h_css {
                                // Tap in game area — inspect the tapped tile
                                let r = renderer.borrow();
                                let (wx, wy) = r.camera.screen_to_world(css_x * dpr, css_y * dpr);
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
                    if map.is_walkable(dest.0, dest.1) {
                        let path = map.find_path((gm.player_x, gm.player_y), dest);
                        *pp = path;
                    }
                    // Inspect the destination tile during swipe
                    gm.inspected = gm.inspect_tile(dest.0, dest.1);
                }
            } else {
                // Clear inspection when not swiping (unless tapped)
                // Only clear if no active swipe — taps persist until next action
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
                        // Clear auto-path if player didn't reach expected tile or map changed
                        if gm.player_x != nx || gm.player_y != ny || matches!(result, TurnResult::MapChanged) {
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
