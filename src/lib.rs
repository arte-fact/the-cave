mod camera;
mod game;
mod input;
mod map;
mod renderer;
mod sprites;
mod world;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, HtmlImageElement};

use game::{Game, TurnResult};
use input::{Input, InputAction};
use map::Map;
use renderer::{Renderer, SpriteSheets};
use world::World;

fn request_animation_frame(window: &web_sys::Window, f: &Closure<dyn FnMut()>) {
    window
        .request_animation_frame(f.as_ref().unchecked_ref())
        .unwrap();
}

fn fit_canvas(canvas: &HtmlCanvasElement) -> (f64, f64) {
    let window = web_sys::window().unwrap();
    let dpr = window.device_pixel_ratio();
    let css_w = window.inner_width().unwrap().as_f64().unwrap();
    let css_h = window.inner_height().unwrap().as_f64().unwrap();
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
    game.update_fov();
    game
}

/// Load an image from a URL and call `on_load` when ready.
fn load_image(src: &str, on_load: impl FnMut() + 'static) -> HtmlImageElement {
    let img = HtmlImageElement::new().unwrap();
    let cb = Closure::<dyn FnMut()>::new(on_load);
    img.set_onload(Some(cb.as_ref().unchecked_ref()));
    cb.forget();
    img.set_src(src);
    img
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<HtmlCanvasElement>()?;

    let ctx = canvas
        .get_context("2d")?
        .unwrap()
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
                    let sheets = SpriteSheets {
                        tiles: t.borrow_mut().take().unwrap(),
                        monsters: m.borrow_mut().take().unwrap(),
                        rogues: r.borrow_mut().take().unwrap(),
                        items: i.borrow_mut().take().unwrap(),
                    };
                    rend.borrow_mut().set_sheets(sheets);
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
        let (w, h) = fit_canvas(&canvas);
        let mut r = renderer.borrow_mut();
        r.resize(w, h);
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
            let (w, h) = fit_canvas(&canvas);
            let gm = game.borrow();
            let mut r = renderer.borrow_mut();
            r.resize(w, h);
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

    let window2 = web_sys::window().unwrap();
    *g.borrow_mut() = Some(Closure::new(move || {
        let dpr = web_sys::window().unwrap().device_pixel_ratio();

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
                            let pp = preview_path.borrow();
                            if pp.len() > 1 {
                                *auto_path.borrow_mut() = pp[1..].to_vec();
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

        // Compute live preview path from swipe state
        {
            let mut pp = preview_path.borrow_mut();
            pp.clear();
            if let Some(swipe) = input.swipe_state() {
                let gm = game.borrow();
                if gm.alive && !gm.won {
                    let dx = (swipe.current_x - swipe.start_x) * 0.5;
                    let dy = (swipe.current_y - swipe.start_y) * 0.5;
                    let (gdx, gdy) = renderer.borrow().camera.css_delta_to_grid(dx, dy, dpr);
                    let dest = (gm.player_x + gdx, gm.player_y + gdy);
                    let map = gm.current_map();
                    if map.is_walkable(dest.0, dest.1) {
                        let path = map.find_path((gm.player_x, gm.player_y), dest);
                        *pp = path;
                    }
                }
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

        // Render
        renderer.borrow().draw(&game.borrow(), &preview_path.borrow());

        // Schedule next frame
        request_animation_frame(&window2, f.borrow().as_ref().unwrap());
    }));

    let window3 = web_sys::window().unwrap();
    request_animation_frame(&window3, g.borrow().as_ref().unwrap());

    Ok(())
}
