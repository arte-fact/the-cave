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

#[test]
fn set_viewport_for_area_uses_game_area_for_cell_size() {
    let mut cam = Camera::new();
    // Full canvas 1600×720, game area 1160 (side panel takes 440)
    let cell = cam.set_viewport_for_area(1600.0, 720.0, 1160.0);
    // cell_size = floor(1160 / 15) = 77
    assert_eq!(cell, 77.0);
    // Viewport spans the full canvas at the smaller cell size
    assert!((cam.viewport_w() - 1600.0 / 77.0).abs() < 0.01);
    assert!((cam.viewport_h() - 720.0 / 77.0).abs() < 0.01);
}

#[test]
fn set_viewport_for_area_matches_set_viewport_when_equal() {
    let mut cam1 = Camera::new();
    let mut cam2 = Camera::new();
    let cell1 = cam1.set_viewport(660.0, 440.0);
    let cell2 = cam2.set_viewport_for_area(660.0, 440.0, 660.0);
    assert_eq!(cell1, cell2);
    assert_eq!(cam1.viewport_w(), cam2.viewport_w());
    assert_eq!(cam1.viewport_h(), cam2.viewport_h());
}

#[test]
fn landscape_tile_density_matches_portrait() {
    // Portrait: 720×1600, cell_size from 720
    let mut portrait = Camera::new();
    let portrait_cell = portrait.set_viewport(720.0, 1600.0);

    // Landscape: 1600×720, cell_size from height (720) to match portrait density
    let mut landscape = Camera::new();
    let landscape_cell = landscape.set_viewport_for_area(1600.0, 720.0, 720.0);

    assert_eq!(portrait_cell, landscape_cell,
        "landscape tile size should match portrait for same device");
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

// --- Padding ---

#[test]
fn pad_top_allows_camera_above_map_edge() {
    let mut cam = Camera::new();
    cam.set_viewport(200.0, 200.0);
    cam.pad_top = 3.0;
    cam.snap(40.0, -100.0, 80, 50);
    let half_h = cam.viewport_h() / 2.0;
    // Camera allowed 3 tiles above normal limit
    assert!((cam.y - (half_h - 3.0)).abs() < 0.01,
        "should clamp to half_h - pad_top, got {}", cam.y);
}

#[test]
fn pad_bottom_allows_camera_below_map_edge() {
    let mut cam = Camera::new();
    cam.set_viewport(200.0, 200.0);
    cam.pad_bottom = 2.0;
    cam.snap(40.0, 999.0, 80, 50);
    let half_h = cam.viewport_h() / 2.0;
    // Camera allowed 2 tiles below normal limit
    assert!((cam.y - (50.0 - half_h + 2.0)).abs() < 0.01,
        "should clamp to mh - half_h + pad_bottom, got {}", cam.y);
}

#[test]
fn pad_both_directions() {
    let mut cam = Camera::new();
    cam.set_viewport(200.0, 200.0);
    cam.pad_top = 2.0;
    cam.pad_bottom = 3.0;
    let half_h = cam.viewport_h() / 2.0;

    // At top edge
    cam.snap(40.0, -100.0, 80, 50);
    assert!((cam.y - (half_h - 2.0)).abs() < 0.01);

    // At bottom edge
    cam.snap(40.0, 999.0, 80, 50);
    assert!((cam.y - (50.0 - half_h + 3.0)).abs() < 0.01);
}

#[test]
fn padding_centers_when_exceeding_map() {
    let mut cam = Camera::new();
    cam.set_viewport(660.0, 440.0); // ~14.7 tiles high
    cam.pad_top = 10.0;
    cam.pad_bottom = 10.0;
    // Small map (10 tiles) — padding exceeds map, should center
    cam.snap(5.0, 5.0, 20, 10);
    assert!((cam.y - 5.0).abs() < 0.01, "should center on small map, got {}", cam.y);
}

#[test]
fn no_padding_same_as_before() {
    let mut cam = Camera::new();
    cam.set_viewport(200.0, 200.0);
    // pad_top=0, pad_bottom=0 (defaults)
    let half_h = cam.viewport_h() / 2.0;

    cam.snap(40.0, -100.0, 80, 50);
    assert!((cam.y - half_h).abs() < 0.01, "top clamp unchanged without padding");

    cam.snap(40.0, 999.0, 80, 50);
    assert!((cam.y - (50.0 - half_h)).abs() < 0.01, "bottom clamp unchanged without padding");
}
