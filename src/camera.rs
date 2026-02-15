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
            self.x = self.x.clamp(half_w, mw - half_w);
        }
        if self.viewport_h >= mh {
            self.y = mh / 2.0;
        } else {
            self.y = self.y.clamp(half_h, mh - half_h);
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
mod tests {
    use super::*;

    // --- Viewport calculation ---

    #[test]
    fn viewport_size_from_canvas() {
        let mut cam = Camera::new();
        let cell = cam.set_viewport(660.0, 440.0);
        assert_eq!(cell, 44.0); // 660 / 15 = 44
        assert!((cam.viewport_w() - 660.0 / 44.0).abs() < 0.01);
        assert!((cam.viewport_h() - 440.0 / 44.0).abs() < 0.01);
    }

    #[test]
    fn viewport_cell_size_at_least_one() {
        let mut cam = Camera::new();
        let cell = cam.set_viewport(10.0, 5.0);
        assert!(cell >= 1.0);
    }

    // --- Clamping ---

    #[test]
    fn clamp_at_left_edge() {
        let mut cam = Camera::new();
        cam.set_viewport(200.0, 200.0);
        cam.snap(-100.0, 10.0, 80, 50);
        let half_w = cam.viewport_w() / 2.0;
        assert!(cam.x >= half_w, "camera should not go past left edge");
    }

    #[test]
    fn clamp_at_right_edge() {
        let mut cam = Camera::new();
        cam.set_viewport(200.0, 200.0);
        cam.snap(999.0, 10.0, 80, 50);
        let half_w = cam.viewport_w() / 2.0;
        assert!(cam.x <= 80.0 - half_w, "camera should not go past right edge");
    }

    #[test]
    fn clamp_at_top_edge() {
        let mut cam = Camera::new();
        cam.set_viewport(200.0, 200.0);
        cam.snap(40.0, -100.0, 80, 50);
        let half_h = cam.viewport_h() / 2.0;
        assert!(cam.y >= half_h, "camera should not go past top edge");
    }

    #[test]
    fn clamp_at_bottom_edge() {
        let mut cam = Camera::new();
        cam.set_viewport(200.0, 200.0);
        cam.snap(40.0, 999.0, 80, 50);
        let half_h = cam.viewport_h() / 2.0;
        assert!(cam.y <= 50.0 - half_h, "camera should not go past bottom edge");
    }

    #[test]
    fn clamp_centers_when_viewport_larger_than_map() {
        let mut cam = Camera::new();
        cam.set_viewport(660.0, 440.0); // ~22 wide, ~14.7 tall
        // Small map that fits inside the viewport
        cam.snap(5.0, 5.0, 10, 10);
        assert!((cam.x - 5.0).abs() < 0.01, "should center on small map x");
        assert!((cam.y - 5.0).abs() < 0.01, "should center on small map y");
    }

    // --- Follow (lerp) ---

    #[test]
    fn follow_moves_toward_target() {
        let mut cam = Camera::new();
        cam.set_viewport(660.0, 440.0);
        cam.snap(40.0, 25.0, 80, 50);
        let old_x = cam.x;
        cam.follow(45.0, 25.0, 80, 50);
        assert!(cam.x > old_x, "camera should move toward target");
        assert!(cam.x < 45.0, "camera should not overshoot target in one step");
    }

    #[test]
    fn follow_snaps_when_close() {
        let mut cam = Camera::new();
        cam.set_viewport(660.0, 440.0);
        cam.snap(40.0, 25.0, 80, 50);
        cam.follow(40.05, 25.05, 80, 50);
        assert!((cam.x - 40.05).abs() < 0.01, "should snap when < 0.1");
        assert!((cam.y - 25.05).abs() < 0.01, "should snap when < 0.1");
    }

    #[test]
    fn follow_eventually_converges() {
        let mut cam = Camera::new();
        cam.set_viewport(660.0, 440.0);
        cam.snap(40.0, 25.0, 80, 50);
        for _ in 0..100 {
            cam.follow(50.0, 30.0, 80, 50);
        }
        assert!((cam.x - 50.0).abs() < 0.01, "should converge to target x");
        assert!((cam.y - 30.0).abs() < 0.01, "should converge to target y");
    }

    #[test]
    fn follow_ease_out_faster_when_far() {
        // When far from target, the camera should cover more distance per frame
        // than when close. This tests the ease-out property.
        let mut cam_far = Camera::new();
        cam_far.set_viewport(660.0, 440.0);
        cam_far.snap(30.0, 25.0, 80, 50);
        cam_far.follow(40.0, 25.0, 80, 50); // 10 tiles away
        let step_far = cam_far.x - 30.0;

        let mut cam_close = Camera::new();
        cam_close.set_viewport(660.0, 440.0);
        cam_close.snap(39.0, 25.0, 80, 50);
        cam_close.follow(40.0, 25.0, 80, 50); // 1 tile away
        let step_close = cam_close.x - 39.0;

        // Far step (absolute) should be bigger than close step (absolute)
        assert!(step_far > step_close,
            "far step {step_far} should exceed close step {step_close}");
    }

    #[test]
    fn follow_converges_within_20_frames() {
        // Ease-out should converge faster than the old constant lerp
        let mut cam = Camera::new();
        cam.set_viewport(660.0, 440.0);
        cam.snap(40.0, 25.0, 80, 50);
        for _ in 0..20 {
            cam.follow(45.0, 25.0, 80, 50);
        }
        assert!((cam.x - 45.0).abs() < 0.01, "should converge in 20 frames");
    }

    // --- Visible range ---

    #[test]
    fn visible_range_within_map_bounds() {
        let mut cam = Camera::new();
        cam.set_viewport(660.0, 440.0);
        cam.snap(40.0, 25.0, 80, 50);
        let (min_x, min_y, max_x, max_y) = cam.visible_range(80, 50);
        assert!(min_x >= 0);
        assert!(min_y >= 0);
        assert!(max_x <= 80);
        assert!(max_y <= 50);
    }

    #[test]
    fn visible_range_covers_player() {
        let mut cam = Camera::new();
        cam.set_viewport(660.0, 440.0);
        let player = (40, 25);
        cam.snap(player.0 as f64, player.1 as f64, 80, 50);
        let (min_x, min_y, max_x, max_y) = cam.visible_range(80, 50);
        assert!(player.0 >= min_x && player.0 < max_x, "player x should be visible");
        assert!(player.1 >= min_y && player.1 < max_y, "player y should be visible");
    }

    // --- Coordinate round-trips ---

    #[test]
    fn world_screen_round_trip() {
        let mut cam = Camera::new();
        cam.set_viewport(660.0, 440.0);
        cam.snap(40.0, 25.0, 80, 50);

        // Test several tiles around the camera center
        for &(wx, wy) in &[(40, 25), (42, 27), (35, 20), (50, 30)] {
            let (sx, sy) = cam.world_to_screen(wx, wy);
            let (rx, ry) = cam.screen_to_world(sx, sy);
            assert_eq!(rx, wx, "round-trip x for ({wx},{wy})");
            assert_eq!(ry, wy, "round-trip y for ({wx},{wy})");
        }
    }

    #[test]
    fn world_screen_round_trip_at_edges() {
        let mut cam = Camera::new();
        cam.set_viewport(660.0, 440.0);
        // Camera at a corner — integer position, no clamping issues
        cam.snap(11.0, 8.0, 80, 50);

        for &(wx, wy) in &[(1, 1), (5, 5), (11, 8)] {
            let (sx, sy) = cam.world_to_screen(wx, wy);
            let (rx, ry) = cam.screen_to_world(sx, sy);
            assert_eq!(rx, wx, "edge round-trip x for ({wx},{wy})");
            assert_eq!(ry, wy, "edge round-trip y for ({wx},{wy})");
        }
    }

    #[test]
    fn screen_center_maps_to_camera_position() {
        let mut cam = Camera::new();
        cam.set_viewport(660.0, 440.0);
        cam.snap(40.0, 25.0, 80, 50);
        let cx = cam.canvas_w / 2.0;
        let cy = cam.canvas_h / 2.0;
        let (wx, wy) = cam.screen_to_world(cx, cy);
        assert_eq!(wx, 40);
        assert_eq!(wy, 25);
    }

    #[test]
    fn css_delta_to_grid_basic() {
        let mut cam = Camera::new();
        cam.set_viewport(660.0, 440.0); // cell = 44
        // DPR=1, move 88 CSS pixels right = 2 tiles
        let (gx, gy) = cam.css_delta_to_grid(88.0, 0.0, 1.0);
        assert_eq!(gx, 2);
        assert_eq!(gy, 0);
    }

    #[test]
    fn css_delta_to_grid_with_dpr() {
        let mut cam = Camera::new();
        cam.set_viewport(660.0, 440.0); // cell = 44
        // DPR=2, 44 CSS pixels = 88 physical pixels = 2 tiles
        let (gx, gy) = cam.css_delta_to_grid(44.0, 0.0, 2.0);
        assert_eq!(gx, 2);
        assert_eq!(gy, 0);
    }
}
