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
}

pub struct Input {
    queue: Rc<RefCell<Vec<InputAction>>>,
    /// Live swipe state, set during touchmove, cleared on touchend.
    swipe: Rc<RefCell<Option<SwipeState>>>,
}

impl Input {
    pub fn new(canvas: &web_sys::HtmlCanvasElement) -> Self {
        let queue: Rc<RefCell<Vec<InputAction>>> = Rc::new(RefCell::new(Vec::new()));
        let swipe: Rc<RefCell<Option<SwipeState>>> = Rc::new(RefCell::new(None));

        Self::bind_touch(canvas, Rc::clone(&queue), Rc::clone(&swipe));
        Self::bind_keyboard(Rc::clone(&queue));

        Self { queue, swipe }
    }

    pub fn drain(&self) -> Vec<InputAction> {
        self.queue.borrow_mut().drain(..).collect()
    }

    /// Returns the current live swipe state (if finger is down and dragging).
    pub fn swipe_state(&self) -> Option<SwipeState> {
        *self.swipe.borrow()
    }

    fn bind_touch(
        canvas: &web_sys::HtmlCanvasElement,
        queue: Rc<RefCell<Vec<InputAction>>>,
        swipe: Rc<RefCell<Option<SwipeState>>>,
    ) {
        let start: Rc<RefCell<Option<(f64, f64)>>> = Rc::new(RefCell::new(None));

        // touchstart — record origin
        {
            let start = Rc::clone(&start);
            let swipe = Rc::clone(&swipe);
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
                }
            });
            canvas
                .add_event_listener_with_callback("touchstart", cb.as_ref().unchecked_ref())
                .unwrap();
            cb.forget();
        }

        // touchmove — update live swipe position
        {
            let start = Rc::clone(&start);
            let swipe = Rc::clone(&swipe);
            let cb = Closure::<dyn FnMut(TouchEvent)>::new(move |e: TouchEvent| {
                e.prevent_default();
                if let Some((sx, sy)) = *start.borrow() {
                    if let Some(t) = e.touches().get(0) {
                        *swipe.borrow_mut() = Some(SwipeState {
                            start_x: sx,
                            start_y: sy,
                            current_x: t.client_x() as f64,
                            current_y: t.client_y() as f64,
                        });
                    }
                }
            });
            canvas
                .add_event_listener_with_callback("touchmove", cb.as_ref().unchecked_ref())
                .unwrap();
            cb.forget();
        }

        // touchend — execute path, single-step, or tap
        {
            let start = Rc::clone(&start);
            let swipe = Rc::clone(&swipe);
            let queue = queue;
            let cb = Closure::<dyn FnMut(TouchEvent)>::new(move |e: TouchEvent| {
                e.prevent_default();
                let has_swipe = swipe.borrow().is_some();
                *swipe.borrow_mut() = None;

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
                            // Short swipe = single step
                            let dir = if dx.abs() > dy.abs() {
                                if dx > 0.0 { Direction::Right } else { Direction::Left }
                            } else {
                                if dy > 0.0 { Direction::Down } else { Direction::Up }
                            };
                            queue.borrow_mut().push(InputAction::Step(dir));
                        }
                    }
                }
            });
            canvas
                .add_event_listener_with_callback("touchend", cb.as_ref().unchecked_ref())
                .unwrap();
            cb.forget();
        }
    }

    fn bind_keyboard(queue: Rc<RefCell<Vec<InputAction>>>) {
        let window = web_sys::window().unwrap();
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
                "i" => {
                    e.prevent_default();
                    queue.borrow_mut().push(InputAction::ToggleInventory);
                }
                "c" => {
                    e.prevent_default();
                    queue.borrow_mut().push(InputAction::ToggleStats);
                }
                _ => {}
            }
        });
        window
            .add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget();
    }
}
