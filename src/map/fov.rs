use super::{Map, Visibility};

impl Map {
    // --- Visibility / FOV ---

    pub fn get_visibility(&self, x: i32, y: i32) -> Visibility {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return Visibility::Hidden;
        }
        self.visibility[(y * self.width + x) as usize]
    }

    fn set_visible(&mut self, x: i32, y: i32) {
        if x >= 0 && y >= 0 && x < self.width && y < self.height {
            self.visibility[(y * self.width + x) as usize] = Visibility::Visible;
        }
    }

    /// Demote all Visible tiles to Seen (called before recomputing FOV).
    pub fn age_visibility(&mut self) {
        for v in &mut self.visibility {
            if *v == Visibility::Visible {
                *v = Visibility::Seen;
            }
        }
    }

    /// Compute field of view from (px, py) with the given radius using
    /// recursive shadowcasting (8 octants).
    pub fn compute_fov(&mut self, px: i32, py: i32, radius: i32) {
        self.set_visible(px, py);

        // Octant multipliers: [col_to_x, depth_to_x, col_to_y, depth_to_y]
        // Maps (col, depth) in octant-local space to (dx, dy) in map space.
        const OCTANTS: [[i32; 4]; 8] = [
            [ 1,  0,  0,  1],  // E-SE
            [ 0,  1,  1,  0],  // SE-S
            [ 0,  1, -1,  0],  // NE-N
            [ 1,  0,  0, -1],  // E-NE
            [-1,  0,  0, -1],  // W-NW
            [ 0, -1, -1,  0],  // NW-N
            [ 0, -1,  1,  0],  // SW-S
            [-1,  0,  0,  1],  // W-SW
        ];

        for oct in &OCTANTS {
            self.cast_light(px, py, radius, 1, 1.0, 0.0, oct);
        }
    }

    /// Recursive shadowcasting for one octant.
    /// `depth` = distance from player along the octant's primary axis.
    /// `start_slope`/`end_slope` = the visible arc (1.0 = 45° diagonal, 0.0 = straight ahead).
    /// Scans columns from high (diagonal) to low (axis), recurses when blocked.
    fn cast_light(
        &mut self,
        px: i32, py: i32,
        radius: i32,
        depth: i32,
        mut start_slope: f64,
        end_slope: f64,
        oct: &[i32; 4],
    ) {
        if start_slope < end_slope || depth > radius {
            return;
        }

        let radius_sq = radius * radius;

        for d in depth..=radius {
            let mut blocked = false;
            let mut new_start = start_slope;

            // Scan columns from high (near diagonal) to low (near axis)
            let mut col = d;
            while col >= 0 {
                let map_x = px + col * oct[0] + d * oct[1];
                let map_y = py + col * oct[2] + d * oct[3];

                // Slopes for this cell's edges
                let l_slope = (col as f64 + 0.5) / (d as f64 - 0.5);
                let r_slope = (col as f64 - 0.5) / (d as f64 + 0.5);

                if start_slope < r_slope {
                    col -= 1;
                    continue;
                }
                if end_slope > l_slope {
                    break;
                }

                // Within radius circle?
                if col * col + d * d <= radius_sq {
                    self.set_visible(map_x, map_y);
                }

                let is_opaque = map_x < 0 || map_y < 0
                    || map_x >= self.width || map_y >= self.height
                    || self.get(map_x, map_y).is_opaque();

                if blocked {
                    if is_opaque {
                        new_start = r_slope;
                    } else {
                        blocked = false;
                        start_slope = new_start;
                    }
                } else if is_opaque {
                    blocked = true;
                    self.cast_light(px, py, radius, d + 1, start_slope, l_slope, oct);
                    new_start = r_slope;
                }

                col -= 1;
            }

            if blocked {
                break;
            }
        }
    }

    /// A* pathfinding. Returns the path from `start` to `goal` (inclusive of both),
    /// or empty vec if unreachable. Moves in 4 cardinal directions only.
    pub fn find_path(&self, start: (i32, i32), goal: (i32, i32)) -> Vec<(i32, i32)> {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;

        if !self.is_walkable(goal.0, goal.1) {
            return Vec::new();
        }
        if start == goal {
            return vec![start];
        }

        let idx = |x: i32, y: i32| (y * self.width + x) as usize;
        let len = (self.width * self.height) as usize;
        let mut g_score = vec![i32::MAX; len];
        let mut came_from: Vec<(i32, i32)> = vec![(-1, -1); len];
        // Cost: cardinal=10, diagonal=14 (≈10×√2). Octile distance heuristic
        // to match, so straight lines are preferred over zigzags.
        const CARDINAL: i32 = 10;
        const DIAGONAL: i32 = 14;
        let heuristic = |x: i32, y: i32| {
            let dx = (x - goal.0).abs();
            let dy = (y - goal.1).abs();
            CARDINAL * (dx + dy) + (DIAGONAL - 2 * CARDINAL) * dx.min(dy)
        };

        g_score[idx(start.0, start.1)] = 0;
        // (f_score, g, x, y) — Reverse for min-heap
        let mut open = BinaryHeap::new();
        open.push(Reverse((heuristic(start.0, start.1), 0, start.0, start.1)));

        while let Some(Reverse((_f, g, x, y))) = open.pop() {
            if (x, y) == goal {
                // Reconstruct path
                let mut path = vec![goal];
                let mut cur = goal;
                while cur != start {
                    cur = came_from[idx(cur.0, cur.1)];
                    path.push(cur);
                }
                path.reverse();
                return path;
            }

            if g > g_score[idx(x, y)] {
                continue; // stale entry
            }

            for (dx, dy) in [(1,0),(-1,0),(0,1),(0,-1),(1,1),(-1,-1),(1,-1),(-1,1)] {
                let (nx, ny) = (x + dx, y + dy);
                if !self.is_walkable(nx, ny) {
                    continue;
                }
                // Prevent diagonal movement through walls (corner-cutting)
                if dx != 0 && dy != 0
                    && (!self.is_walkable(x + dx, y) || !self.is_walkable(x, y + dy))
                {
                    continue;
                }
                let step_cost = if dx != 0 && dy != 0 { DIAGONAL } else { CARDINAL };
                let ng = g + step_cost;
                let ni = idx(nx, ny);
                if ng < g_score[ni] {
                    g_score[ni] = ng;
                    came_from[ni] = (x, y);
                    open.push(Reverse((ng + heuristic(nx, ny), ng, nx, ny)));
                }
            }
        }

        Vec::new() // unreachable
    }

    /// Check whether there is a clear line of sight from (x0,y0) to (x1,y1).
    /// Uses Bresenham's line to walk intermediate tiles. Opaque tiles block LOS,
    /// but the endpoint itself does not block (you can "see" a wall).
    pub fn has_line_of_sight(&self, x0: i32, y0: i32, x1: i32, y1: i32) -> bool {
        let line = bresenham_line(x0, y0, x1, y1);
        // Skip the start and end — only intermediate tiles block
        for &(x, y) in line.iter().skip(1) {
            if x == x1 && y == y1 {
                break; // endpoint reached, don't check it
            }
            if x < 0 || y < 0 || x >= self.width || y >= self.height {
                return false;
            }
            if self.tiles[(y * self.width + x) as usize].is_opaque() {
                return false;
            }
        }
        true
    }
}

/// Compute a Bresenham line from (x0,y0) to (x1,y1).
/// Returns all tiles along the line, including start and end.
pub fn bresenham_line(x0: i32, y0: i32, x1: i32, y1: i32) -> Vec<(i32, i32)> {
    let mut points = Vec::new();
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut x = x0;
    let mut y = y0;

    loop {
        points.push((x, y));
        if x == x1 && y == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
    points
}
