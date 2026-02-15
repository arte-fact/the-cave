use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    let window = web_sys::window().expect("no global window");
    let document = window.document().expect("no document on window");

    let canvas = document
        .get_element_by_id("canvas")
        .expect("no #canvas element")
        .dyn_into::<HtmlCanvasElement>()?;

    let ctx = canvas
        .get_context("2d")?
        .expect("no 2d context")
        .dyn_into::<CanvasRenderingContext2d>()?;

    let width = canvas.width() as f64;
    let height = canvas.height() as f64;

    // Black background
    ctx.set_fill_style_str("#000");
    ctx.fill_rect(0.0, 0.0, width, height);

    // Draw @ in the center
    ctx.set_font("48px monospace");
    ctx.set_fill_style_str("#fff");
    ctx.set_text_align("center");
    ctx.set_text_baseline("middle");
    ctx.fill_text("@", width / 2.0, height / 2.0)?;

    Ok(())
}
