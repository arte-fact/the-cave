//! Error reporting for mobile debugging.
//!
//! Provides a DOM-visible error overlay so panics and runtime errors are
//! readable on mobile devices without developer tools attached.

use wasm_bindgen::prelude::*;

/// Call the JS-side `window.__reportError(msg, source)` function.
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = window, js_name = __reportError)]
    fn js_report_error(msg: &str, source: &str);
}

/// Push an error message into the DOM error overlay.
/// Safe to call at any point after the page has loaded.
pub fn report_error(msg: &str) {
    // DOM overlay
    js_report_error(msg, "rust");
    // Also emit to browser console via web-sys
    web_sys::console::error_1(&msg.into());
}

/// Install a custom panic hook that routes panic info to the DOM overlay
/// instead of silently crashing the WASM module.
pub fn install_panic_hook() {
    use std::panic;
    panic::set_hook(Box::new(|info| {
        let mut msg = String::from("PANIC: ");
        if let Some(s) = info.payload().downcast_ref::<&str>() {
            msg.push_str(s);
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            msg.push_str(s);
        } else {
            msg.push_str("(unknown payload)");
        }
        if let Some(loc) = info.location() {
            msg.push_str(&format!("\n  at {}:{}:{}", loc.file(), loc.line(), loc.column()));
        }
        js_report_error(&msg, "panic");
        web_sys::console::error_1(&msg.into());
    }));
}
