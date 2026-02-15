mod game;
mod input;
mod map;
mod renderer;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use game::Game;
use input::{Input, InputAction};
use map::Map;
use renderer::Renderer;

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
    let map = Map::generate(30, 20, seed);
    let mut game = Game::new(map);
    game.spawn_enemies(seed.wrapping_mul(6364136223846793005));
    game
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

    // Initial sizing
    {
        let (w, h) = fit_canvas(&canvas);
        renderer.borrow_mut().resize(w, h, &game.borrow());
    }

    // Resize handler
    {
        let canvas = Rc::clone(&canvas);
        let game = Rc::clone(&game);
        let renderer = Rc::clone(&renderer);
        let preview_path = Rc::clone(&preview_path);
        let cb = Closure::<dyn FnMut()>::new(move || {
            let (w, h) = fit_canvas(&canvas);
            renderer.borrow_mut().resize(w, h, &game.borrow());
            renderer.borrow().draw(&game.borrow(), &preview_path.borrow());
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

        // Process input actions (before preview clear, so ExecutePath can read it)
        let actions = input.drain();
        if !actions.is_empty() {
            let mut gm = game.borrow_mut();
            if !gm.alive || gm.won {
                *gm = new_game();
                let (w, h) = (renderer.borrow().canvas_w(), renderer.borrow().canvas_h());
                renderer.borrow_mut().resize(w, h, &gm);
                auto_path.borrow_mut().clear();
            } else {
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
                            gm.move_player(dx, dy);
                        }
                        InputAction::ExecutePath => {
                            // Grab the preview path and queue it for auto-move
                            let pp = preview_path.borrow();
                            if pp.len() > 1 {
                                // Skip first element (current position)
                                *auto_path.borrow_mut() = pp[1..].to_vec();
                            }
                        }
                    }
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
                    let dx = swipe.current_x - swipe.start_x;
                    let dy = swipe.current_y - swipe.start_y;
                    let (gdx, gdy) = renderer.borrow().css_delta_to_grid(dx, dy, dpr);
                    let dest = (gm.player_x + gdx, gm.player_y + gdy);
                    if gm.map.is_walkable(dest.0, dest.1) {
                        let path = gm.map.find_path((gm.player_x, gm.player_y), dest);
                        *pp = path;
                    }
                }
            }
        }

        // Process auto-move queue (one step per frame)
        {
            let mut ap = auto_path.borrow_mut();
            if !ap.is_empty() {
                let mut gm = game.borrow_mut();
                if gm.alive && !gm.won {
                    let (nx, ny) = ap[0];
                    let dx = nx - gm.player_x;
                    let dy = ny - gm.player_y;
                    gm.move_player(dx, dy);
                    ap.remove(0);
                    // Stop if we got blocked (wall, enemy, etc.)
                    if gm.player_x != nx || gm.player_y != ny {
                        ap.clear();
                    }
                } else {
                    ap.clear();
                }
            }
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
