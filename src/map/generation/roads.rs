use crate::config::MapGenConfig;
use super::super::{Map, Tile};

impl Map {
    /// Build roads connecting all dungeon entrances using MST + weighted A*.
    pub fn build_roads(&mut self, entrances: &[(i32, i32)], cfg: &MapGenConfig) {
        if entrances.len() < 2 {
            return;
        }

        let mst_edges = prim_mst(entrances);

        // For each MST edge, run weighted A* and carve the road
        for (a, b) in mst_edges {
            let path = self.weighted_path(entrances[a], entrances[b], cfg);
            for (x, y) in path {
                let tile = self.get(x, y);
                if tile == Tile::Grass || tile == Tile::Tree {
                    self.set(x, y, Tile::Road);
                }
            }
        }
    }

    /// Weighted A* for road carving. Costs configurable via MapGenConfig.
    /// Allows pathing through trees (to carve roads) and grass.
    fn weighted_path(&self, start: (i32, i32), goal: (i32, i32), cfg: &MapGenConfig) -> Vec<(i32, i32)> {
        use std::collections::BinaryHeap;
        use std::cmp::Reverse;

        let idx = |x: i32, y: i32| (y * self.width + x) as usize;
        let len = (self.width * self.height) as usize;
        let mut g_score = vec![i32::MAX; len];
        let mut came_from: Vec<(i32, i32)> = vec![(-1, -1); len];
        let heuristic = |x: i32, y: i32| (x - goal.0).abs() + (y - goal.1).abs();

        g_score[idx(start.0, start.1)] = 0;
        let mut open = BinaryHeap::new();
        open.push(Reverse((heuristic(start.0, start.1), 0, start.0, start.1)));

        while let Some(Reverse((_f, g, x, y))) = open.pop() {
            if (x, y) == goal {
                return reconstruct_path(&came_from, self.width, start, goal);
            }

            if g > g_score[idx(x, y)] {
                continue;
            }

            for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let (nx, ny) = (x + dx, y + dy);
                if nx <= 0 || ny <= 0 || nx >= self.width - 1 || ny >= self.height - 1 {
                    continue; // stay off the border
                }
                let cost = match tile_cost(self.get(nx, ny), cfg) {
                    Some(c) => c,
                    None => continue,
                };
                let ng = g + cost;
                let ni = idx(nx, ny);
                if ng < g_score[ni] {
                    g_score[ni] = ng;
                    came_from[ni] = (x, y);
                    open.push(Reverse((ng + heuristic(nx, ny), ng, nx, ny)));
                }
            }
        }

        Vec::new()
    }

    /// Find a walkable tile near the center for spawning.
    /// Prefers Road tiles, then any walkable tile.
    pub fn find_road_spawn(&self) -> (i32, i32) {
        let cx = self.width / 2;
        let cy = self.height / 2;
        let max_r = self.width.max(self.height);
        // First pass: look for Road near center
        for r in 0..max_r {
            for dy in -r..=r {
                for dx in -r..=r {
                    let x = cx + dx;
                    let y = cy + dy;
                    if x >= 0 && y >= 0 && x < self.width && y < self.height
                        && self.get(x, y) == Tile::Road
                    {
                        return (x, y);
                    }
                }
            }
        }
        // Fallback: any walkable tile
        self.find_spawn()
    }
}

/// Build MST using Prim's algorithm on Euclidean distances between entrances.
fn prim_mst(entrances: &[(i32, i32)]) -> Vec<(usize, usize)> {
    let n = entrances.len();
    let mut in_tree = vec![false; n];
    let mut min_cost = vec![f64::MAX; n];
    let mut min_edge = vec![0usize; n]; // which tree node is closest
    in_tree[0] = true;

    for i in 1..n {
        let dx = (entrances[i].0 - entrances[0].0) as f64;
        let dy = (entrances[i].1 - entrances[0].1) as f64;
        min_cost[i] = (dx * dx + dy * dy).sqrt();
        min_edge[i] = 0;
    }

    let mut edges: Vec<(usize, usize)> = Vec::new();
    for _ in 1..n {
        // Find cheapest edge to non-tree node
        let mut best = usize::MAX;
        let mut best_cost = f64::MAX;
        for i in 0..n {
            if !in_tree[i] && min_cost[i] < best_cost {
                best_cost = min_cost[i];
                best = i;
            }
        }
        if best == usize::MAX {
            break;
        }
        in_tree[best] = true;
        edges.push((min_edge[best], best));

        // Update costs
        for i in 0..n {
            if !in_tree[i] {
                let dx = (entrances[i].0 - entrances[best].0) as f64;
                let dy = (entrances[i].1 - entrances[best].1) as f64;
                let dist = (dx * dx + dy * dy).sqrt();
                if dist < min_cost[i] {
                    min_cost[i] = dist;
                    min_edge[i] = best;
                }
            }
        }
    }

    edges
}

/// Road pathing tile cost. Returns None for impassable tiles.
fn tile_cost(tile: Tile, cfg: &MapGenConfig) -> Option<i32> {
    match tile {
        Tile::Grass => Some(cfg.road_cost_grass),
        Tile::Tree => Some(cfg.road_cost_tree),
        Tile::Road => Some(cfg.road_cost_road),
        Tile::Floor => Some(cfg.road_cost_floor),
        Tile::DungeonEntrance => Some(cfg.road_cost_entrance),
        _ => None, // Wall, etc = impassable
    }
}

/// Reconstruct path from came_from array.
fn reconstruct_path(came_from: &[(i32, i32)], width: i32, start: (i32, i32), goal: (i32, i32)) -> Vec<(i32, i32)> {
    let idx = |x: i32, y: i32| (y * width + x) as usize;
    let mut path = vec![goal];
    let mut cur = goal;
    while cur != start {
        cur = came_from[idx(cur.0, cur.1)];
        path.push(cur);
    }
    path.reverse();
    path
}
