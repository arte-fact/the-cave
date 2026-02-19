/// Camera tracks a viewport into the world. Pure logic, no rendering dependencies.
pub struct Camera {
    /// Camera center position in world-tile units.
    pub x: f64,
    pub y: f64,
    /// Visible tile counts (derived from canvas ÷ cell_size).
    viewport_w: f64,
    viewport_h: f64,
    /// Pixels per tile.
    cell_size: f64,
    /// Raw canvas dimensions in pixels (stored to avoid precision loss).
    canvas_w: f64,
    canvas_h: f64,
    /// Extra padding in tile units so map content clears HUD overlays at edges.
    pub pad_top: f64,
    pub pad_bottom: f64,
    pub pad_right: f64,
}

/// How many tiles should be visible across the screen width.
/// Lower = closer/bigger tiles. 15 is tuned for mobile-first play.
const VIEWPORT_TILES_WIDE: f64 = 15.0;

impl Camera {
    pub fn new() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            viewport_w: 0.0,
            viewport_h: 0.0,
            cell_size: 1.0,
            canvas_w: 0.0,
            canvas_h: 0.0,
            pad_top: 0.0,
            pad_bottom: 0.0,
            pad_right: 0.0,
        }
    }

    pub fn viewport_w(&self) -> f64 {
        self.viewport_w
    }

    pub fn viewport_h(&self) -> f64 {
        self.viewport_h
    }

    pub fn cell_size(&self) -> f64 {
        self.cell_size
    }

    /// Recalculate viewport dimensions from canvas pixel size.
    /// Returns the computed cell_size (pixels per tile).
    pub fn set_viewport(&mut self, canvas_w: f64, canvas_h: f64) -> f64 {
        self.canvas_w = canvas_w;
        self.canvas_h = canvas_h;
        self.cell_size = (canvas_w / VIEWPORT_TILES_WIDE).floor().max(1.0);
        self.viewport_w = canvas_w / self.cell_size;
        self.viewport_h = canvas_h / self.cell_size;
        self.cell_size
    }

    /// Smoothly follow a target position with ease-out curve.
    /// Moves fast when far from target, decelerates as it approaches.
    pub fn follow(&mut self, target_x: f64, target_y: f64, map_w: i32, map_h: i32) {
        let dx = target_x - self.x;
        let dy = target_y - self.y;
        let dist = (dx * dx + dy * dy).sqrt();

        // Ease-out: speed ramps up with distance, giving snappy start + smooth stop.
        // Clamp between 0.15 (gentle coast) and 0.4 (fast catch-up).
        let lerp_speed = (0.15 + dist * 0.06).min(0.4);

        self.x += dx * lerp_speed;
        self.y += dy * lerp_speed;

        // Snap when close enough
        if (self.x - target_x).abs() < 0.1 {
            self.x = target_x;
        }
        if (self.y - target_y).abs() < 0.1 {
            self.y = target_y;
        }

        self.clamp(map_w, map_h);
    }

    /// Immediately center on target, clamped to map bounds.
    pub fn snap(&mut self, target_x: f64, target_y: f64, map_w: i32, map_h: i32) {
        self.x = target_x;
        self.y = target_y;
        self.clamp(map_w, map_h);
    }

    fn clamp(&mut self, map_w: i32, map_h: i32) {
        let half_w = self.viewport_w / 2.0;
        let half_h = self.viewport_h / 2.0;
        let mw = map_w as f64;
        let mh = map_h as f64;

        if self.viewport_w >= mw {
            self.x = mw / 2.0; // viewport bigger than map → center
        } else {
            // pad_right shifts the visible center leftward so map content
            // clears the side panel in landscape mode.
            let max_x = mw - half_w + self.pad_right;
            let min_x = half_w - self.pad_right;
            if min_x < max_x {
                self.x = self.x.clamp(min_x, max_x);
            } else {
                self.x = mw / 2.0;
            }
        }
        if self.viewport_h >= mh {
            self.y = mh / 2.0;
        } else {
            // Allow camera to shift so map content clears HUD overlays at edges.
            // pad_top: extra tiles visible above the map (pushes map below top HUD).
            // pad_bottom: extra tiles visible below the map (pushes map above bottom HUD).
            let min_y = half_h - self.pad_top;
            let max_y = mh - half_h + self.pad_bottom;
            if min_y < max_y {
                self.y = self.y.clamp(min_y, max_y);
            } else {
                // Padding exceeds map size — just center vertically
                self.y = mh / 2.0;
            }
        }
    }

    /// Get the visible tile range: (min_x, min_y, max_x, max_y).
    /// min is inclusive, max is exclusive. Clamped to [0, map_size).
    pub fn visible_range(&self, map_w: i32, map_h: i32) -> (i32, i32, i32, i32) {
        let half_w = self.viewport_w / 2.0;
        let half_h = self.viewport_h / 2.0;
        let min_x = (self.x - half_w).floor() as i32;
        let min_y = (self.y - half_h).floor() as i32;
        let max_x = (self.x + half_w).ceil() as i32;
        let max_y = (self.y + half_h).ceil() as i32;
        (min_x.max(0), min_y.max(0), max_x.min(map_w), max_y.min(map_h))
    }

    /// Convert world tile (x, y) to screen pixel position.
    /// Uses canvas dimensions directly to avoid float precision loss.
    pub fn world_to_screen(&self, wx: i32, wy: i32) -> (f64, f64) {
        let sx = (wx as f64 - self.x) * self.cell_size + self.canvas_w / 2.0;
        let sy = (wy as f64 - self.y) * self.cell_size + self.canvas_h / 2.0;
        (sx, sy)
    }

    /// Convert screen pixel position to world tile coordinates (floored to int).
    pub fn screen_to_world(&self, sx: f64, sy: f64) -> (i32, i32) {
        let wx = ((sx - self.canvas_w / 2.0) / self.cell_size + self.x).floor() as i32;
        let wy = ((sy - self.canvas_h / 2.0) / self.cell_size + self.y).floor() as i32;
        (wx, wy)
    }

    /// Convert a CSS pixel delta (swipe displacement) to grid tile offset.
    pub fn css_delta_to_grid(&self, dx: f64, dy: f64, dpr: f64) -> (i32, i32) {
        let gx = (dx * dpr / self.cell_size).round() as i32;
        let gy = (dy * dpr / self.cell_size).round() as i32;
        (gx, gy)
    }
}

#[cfg(test)]
mod tests;
