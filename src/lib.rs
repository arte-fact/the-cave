mod game;
mod input;
mod renderer;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

use game::Game;
use input::Input;
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

    let game = Rc::new(RefCell::new(Game::new(20, 20)));
    let renderer = Rc::new(RefCell::new(Renderer::new(ctx)));
    let input = Rc::new(Input::new(&canvas));
    let canvas = Rc::new(canvas);

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
        let cb = Closure::<dyn FnMut()>::new(move || {
            let (w, h) = fit_canvas(&canvas);
            renderer.borrow_mut().resize(w, h, &game.borrow());
            renderer.borrow().draw(&game.borrow());
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
        // Process input
        for dir in input.drain() {
            let (dx, dy) = match dir {
                input::Direction::Up => (0, -1),
                input::Direction::Down => (0, 1),
                input::Direction::Left => (-1, 0),
                input::Direction::Right => (1, 0),
            };
            game.borrow_mut().move_player(dx, dy);
        }

        // Render
        renderer.borrow().draw(&game.borrow());

        // Schedule next frame
        request_animation_frame(&window2, f.borrow().as_ref().unwrap());
    }));

    let window3 = web_sys::window().unwrap();
    request_animation_frame(&window3, g.borrow().as_ref().unwrap());

    Ok(())
}
