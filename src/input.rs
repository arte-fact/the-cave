use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{KeyboardEvent, TouchEvent};

#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

/// Represents current swipe state for path preview.
#[derive(Clone, Copy)]
pub struct SwipeState {
    /// Touch start position in CSS pixels.
    pub start_x: f64,
    pub start_y: f64,
    /// Current touch position in CSS pixels.
    pub current_x: f64,
    pub current_y: f64,
}

/// What happened on input this frame.
pub enum InputAction {
    /// Single-step move (keyboard or short swipe).
    Step(Direction),
    /// Swipe released — execute the path to the previewed destination.
    ExecutePath,
    /// Tap at a CSS pixel position (for UI buttons and tile inspection).
    Tap(f64, f64),
    /// Toggle inventory drawer (keyboard shortcut).
    ToggleInventory,
    /// Toggle stats drawer (keyboard shortcut).
    ToggleStats,
    /// Toggle sprint mode.
    ToggleSprint,
    /// Toggle glyph rendering mode.
    ToggleGlyphMode,
    /// Interact: attack adjacent enemy (facing direction) or pick up items at feet.
    Interact,
    /// Use item in quick-bar slot (0-indexed).
    QuickUse(usize),
}

/// Tracks whether a finger is currently touching the screen.
#[derive(Clone, Copy, Debug)]
pub struct TouchDown {
    pub start_x: f64,
    pub start_y: f64,
    pub current_x: f64,
    pub current_y: f64,
}

pub struct Input {
    queue: Rc<RefCell<Vec<InputAction>>>,
    /// Live swipe state, set during touchmove, cleared on touchend.
    swipe: Rc<RefCell<Option<SwipeState>>>,
    /// Whether finger is currently down (set on touchstart, cleared on touchend).
    touch_down: Rc<RefCell<Option<TouchDown>>>,
}

impl Input {
    pub fn new(canvas: &web_sys::HtmlCanvasElement) -> Self {
        let queue: Rc<RefCell<Vec<InputAction>>> = Rc::new(RefCell::new(Vec::new()));
        let swipe: Rc<RefCell<Option<SwipeState>>> = Rc::new(RefCell::new(None));
        let touch_down: Rc<RefCell<Option<TouchDown>>> = Rc::new(RefCell::new(None));

        Self::bind_touch(canvas, Rc::clone(&queue), Rc::clone(&swipe), Rc::clone(&touch_down));
        Self::bind_keyboard(Rc::clone(&queue));

        Self { queue, swipe, touch_down }
    }

    pub fn drain(&self) -> Vec<InputAction> {
        self.queue.borrow_mut().drain(..).collect()
    }

    /// Returns the current live swipe state (if finger is down and dragging).
    pub fn swipe_state(&self) -> Option<SwipeState> {
        *self.swipe.borrow()
    }

    /// Returns the current touch-down state (finger on screen).
    pub fn touch_down(&self) -> Option<TouchDown> {
        *self.touch_down.borrow()
    }

    fn bind_touch(
        canvas: &web_sys::HtmlCanvasElement,
        queue: Rc<RefCell<Vec<InputAction>>>,
        swipe: Rc<RefCell<Option<SwipeState>>>,
        touch_down: Rc<RefCell<Option<TouchDown>>>,
    ) {
        let start: Rc<RefCell<Option<(f64, f64)>>> = Rc::new(RefCell::new(None));

        // touchstart — record origin
        {
            let start = Rc::clone(&start);
            let swipe = Rc::clone(&swipe);
            let touch_down = Rc::clone(&touch_down);
            let cb = Closure::<dyn FnMut(TouchEvent)>::new(move |e: TouchEvent| {
                e.prevent_default();
                if let Some(t) = e.touches().get(0) {
                    let sx = t.client_x() as f64;
                    let sy = t.client_y() as f64;
                    *start.borrow_mut() = Some((sx, sy));
                    *swipe.borrow_mut() = Some(SwipeState {
                        start_x: sx,
                        start_y: sy,
                        current_x: sx,
                        current_y: sy,
                    });
                    *touch_down.borrow_mut() = Some(TouchDown {
                        start_x: sx, start_y: sy,
                        current_x: sx, current_y: sy,
                    });
                }
            });
            if let Err(e) = canvas.add_event_listener_with_callback("touchstart", cb.as_ref().unchecked_ref()) {
                crate::errors::report_error(&format!("touchstart listener failed: {:?}", e));
            }
            cb.forget();
        }

        // touchmove — update live swipe position
        {
            let start = Rc::clone(&start);
            let swipe = Rc::clone(&swipe);
            let touch_down = Rc::clone(&touch_down);
            let cb = Closure::<dyn FnMut(TouchEvent)>::new(move |e: TouchEvent| {
                e.prevent_default();
                if let Some((sx, sy)) = *start.borrow() {
                    if let Some(t) = e.touches().get(0) {
                        let cx = t.client_x() as f64;
                        let cy = t.client_y() as f64;
                        *swipe.borrow_mut() = Some(SwipeState {
                            start_x: sx,
                            start_y: sy,
                            current_x: cx,
                            current_y: cy,
                        });
                        if let Some(td) = touch_down.borrow_mut().as_mut() {
                            td.current_x = cx;
                            td.current_y = cy;
                        }
                    }
                }
            });
            if let Err(e) = canvas.add_event_listener_with_callback("touchmove", cb.as_ref().unchecked_ref()) {
                crate::errors::report_error(&format!("touchmove listener failed: {:?}", e));
            }
            cb.forget();
        }

        // touchend — execute path, single-step, or tap
        {
            let start = Rc::clone(&start);
            let swipe = Rc::clone(&swipe);
            let touch_down = Rc::clone(&touch_down);
            let cb = Closure::<dyn FnMut(TouchEvent)>::new(move |e: TouchEvent| {
                e.prevent_default();
                let has_swipe = swipe.borrow().is_some();
                *swipe.borrow_mut() = None;
                *touch_down.borrow_mut() = None;

                if let Some((sx, sy)) = start.borrow_mut().take() {
                    if let Some(t) = e.changed_touches().get(0) {
                        let dx = t.client_x() as f64 - sx;
                        let dy = t.client_y() as f64 - sy;

                        let threshold = 10.0;
                        if dx.abs() < threshold && dy.abs() < threshold {
                            // Tap — emit CSS-pixel position
                            queue.borrow_mut().push(InputAction::Tap(sx, sy));
                            return;
                        }

                        // Long swipe (>40px) = pathfinding mode
                        let dist = (dx * dx + dy * dy).sqrt();
                        if dist > 40.0 && has_swipe {
                            queue.borrow_mut().push(InputAction::ExecutePath);
                        } else {
                            // Short swipe = single step (supports diagonals)
                            let adx = dx.abs();
                            let ady = dy.abs();
                            // Diagonal when both axes exceed threshold and
                            // the ratio between them is within 3:1
                            let diagonal = adx > threshold && ady > threshold
                                && adx < ady * 3.0 && ady < adx * 3.0;
                            let dir = if diagonal {
                                match (dx > 0.0, dy > 0.0) {
                                    (true, false) => Direction::UpRight,
                                    (false, false) => Direction::UpLeft,
                                    (true, true) => Direction::DownRight,
                                    (false, true) => Direction::DownLeft,
                                }
                            } else if adx > ady {
                                if dx > 0.0 { Direction::Right } else { Direction::Left }
                            } else if dy > 0.0 { Direction::Down } else { Direction::Up };
                            queue.borrow_mut().push(InputAction::Step(dir));
                        }
                    }
                }
            });
            if let Err(e) = canvas.add_event_listener_with_callback("touchend", cb.as_ref().unchecked_ref()) {
                crate::errors::report_error(&format!("touchend listener failed: {:?}", e));
            }
            cb.forget();
        }
    }

    fn bind_keyboard(queue: Rc<RefCell<Vec<InputAction>>>) {
        let window = web_sys::window().expect("no global window");
        let cb = Closure::<dyn FnMut(KeyboardEvent)>::new(move |e: KeyboardEvent| {
            match e.key().as_str() {
                "ArrowUp" | "k" => {
                    e.prevent_default();
                    queue.borrow_mut().push(InputAction::Step(Direction::Up));
                }
                "ArrowDown" | "j" => {
                    e.prevent_default();
                    queue.borrow_mut().push(InputAction::Step(Direction::Down));
                }
                "ArrowLeft" | "h" => {
                    e.prevent_default();
                    queue.borrow_mut().push(InputAction::Step(Direction::Left));
                }
                "ArrowRight" | "l" => {
                    e.prevent_default();
                    queue.borrow_mut().push(InputAction::Step(Direction::Right));
                }
                "y" => {
                    e.prevent_default();
                    queue.borrow_mut().push(InputAction::Step(Direction::UpLeft));
                }
                "u" => {
                    e.prevent_default();
                    queue.borrow_mut().push(InputAction::Step(Direction::UpRight));
                }
                "b" => {
                    e.prevent_default();
                    queue.borrow_mut().push(InputAction::Step(Direction::DownLeft));
                }
                "n" => {
                    e.prevent_default();
                    queue.borrow_mut().push(InputAction::Step(Direction::DownRight));
                }
                "i" => {
                    e.prevent_default();
                    queue.borrow_mut().push(InputAction::ToggleInventory);
                }
                "c" => {
                    e.prevent_default();
                    queue.borrow_mut().push(InputAction::ToggleStats);
                }
                "s" => {
                    e.prevent_default();
                    queue.borrow_mut().push(InputAction::ToggleSprint);
                }
                "g" => {
                    e.prevent_default();
                    queue.borrow_mut().push(InputAction::ToggleGlyphMode);
                }
                "f" | " " => {
                    e.prevent_default();
                    queue.borrow_mut().push(InputAction::Interact);
                }
                "1" => { queue.borrow_mut().push(InputAction::QuickUse(0)); }
                "2" => { queue.borrow_mut().push(InputAction::QuickUse(1)); }
                "3" => { queue.borrow_mut().push(InputAction::QuickUse(2)); }
                "4" => { queue.borrow_mut().push(InputAction::QuickUse(3)); }
                _ => {}
            }
        });
        if let Err(e) = window.add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref()) {
            crate::errors::report_error(&format!("keydown listener failed: {:?}", e));
        }
        cb.forget();
    }
}
