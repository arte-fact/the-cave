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

pub struct Input {
    queue: Rc<RefCell<Vec<Direction>>>,
}

impl Input {
    pub fn new(canvas: &web_sys::HtmlCanvasElement) -> Self {
        let queue: Rc<RefCell<Vec<Direction>>> = Rc::new(RefCell::new(Vec::new()));

        Self::bind_touch(canvas, Rc::clone(&queue));
        Self::bind_keyboard(Rc::clone(&queue));

        Self { queue }
    }

    pub fn drain(&self) -> Vec<Direction> {
        self.queue.borrow_mut().drain(..).collect()
    }

    fn bind_touch(canvas: &web_sys::HtmlCanvasElement, queue: Rc<RefCell<Vec<Direction>>>) {
        let start: Rc<RefCell<Option<(f64, f64)>>> = Rc::new(RefCell::new(None));

        // touchstart — record origin
        {
            let start = Rc::clone(&start);
            let cb = Closure::<dyn FnMut(TouchEvent)>::new(move |e: TouchEvent| {
                e.prevent_default();
                if let Some(t) = e.touches().get(0) {
                    *start.borrow_mut() = Some((t.client_x() as f64, t.client_y() as f64));
                }
            });
            canvas
                .add_event_listener_with_callback("touchstart", cb.as_ref().unchecked_ref())
                .unwrap();
            cb.forget();
        }

        // touchend — compute swipe direction
        {
            let start = Rc::clone(&start);
            let queue = queue;
            let cb = Closure::<dyn FnMut(TouchEvent)>::new(move |e: TouchEvent| {
                e.prevent_default();
                if let Some((sx, sy)) = start.borrow_mut().take() {
                    if let Some(t) = e.changed_touches().get(0) {
                        let dx = t.client_x() as f64 - sx;
                        let dy = t.client_y() as f64 - sy;

                        // Minimum 10px swipe threshold; if under, treat as a tap
                        // For taps, ignore (no direction)
                        let threshold = 10.0;
                        if dx.abs() < threshold && dy.abs() < threshold {
                            return;
                        }

                        let dir = if dx.abs() > dy.abs() {
                            if dx > 0.0 { Direction::Right } else { Direction::Left }
                        } else {
                            if dy > 0.0 { Direction::Down } else { Direction::Up }
                        };
                        queue.borrow_mut().push(dir);
                    }
                }
            });
            canvas
                .add_event_listener_with_callback("touchend", cb.as_ref().unchecked_ref())
                .unwrap();
            cb.forget();
        }
    }

    fn bind_keyboard(queue: Rc<RefCell<Vec<Direction>>>) {
        let window = web_sys::window().unwrap();
        let cb = Closure::<dyn FnMut(KeyboardEvent)>::new(move |e: KeyboardEvent| {
            let dir = match e.key().as_str() {
                "ArrowUp" | "w" | "k" => Some(Direction::Up),
                "ArrowDown" | "s" | "j" => Some(Direction::Down),
                "ArrowLeft" | "a" | "h" => Some(Direction::Left),
                "ArrowRight" | "d" | "l" => Some(Direction::Right),
                _ => None,
            };
            if let Some(d) = dir {
                e.prevent_default();
                queue.borrow_mut().push(d);
            }
        });
        window
            .add_event_listener_with_callback("keydown", cb.as_ref().unchecked_ref())
            .unwrap();
        cb.forget();
    }
}
