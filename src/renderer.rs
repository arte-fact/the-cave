use web_sys::CanvasRenderingContext2d;

use crate::game::Game;
use crate::map::Tile;

pub struct Renderer {
    ctx: CanvasRenderingContext2d,
    cell_size: f64,
    offset_x: f64,
    offset_y: f64,
    canvas_w: f64,
    canvas_h: f64,
}

impl Renderer {
    pub fn new(ctx: CanvasRenderingContext2d) -> Self {
        Self {
            ctx,
            cell_size: 0.0,
            offset_x: 0.0,
            offset_y: 0.0,
            canvas_w: 0.0,
            canvas_h: 0.0,
        }
    }

    pub fn canvas_w(&self) -> f64 {
        self.canvas_w
    }

    pub fn canvas_h(&self) -> f64 {
        self.canvas_h
    }

    /// Convert CSS pixel coordinates to grid tile coordinates.
    pub fn css_to_grid(&self, css_x: f64, css_y: f64, dpr: f64) -> (i32, i32) {
        let px = css_x * dpr;
        let py = css_y * dpr;
        let gx = ((px - self.offset_x) / self.cell_size).floor() as i32;
        let gy = ((py - self.offset_y) / self.cell_size).floor() as i32;
        (gx, gy)
    }

    /// Convert a CSS pixel delta (swipe displacement) to grid tile offset.
    pub fn css_delta_to_grid(&self, dx: f64, dy: f64, dpr: f64) -> (i32, i32) {
        let gx = (dx * dpr / self.cell_size).round() as i32;
        let gy = (dy * dpr / self.cell_size).round() as i32;
        (gx, gy)
    }

    pub fn resize(&mut self, width: f64, height: f64, game: &Game) {
        self.canvas_w = width;
        self.canvas_h = height;

        let cell_w = width / game.map.width as f64;
        let cell_h = height / game.map.height as f64;
        self.cell_size = cell_w.min(cell_h).floor();

        let grid_w = self.cell_size * game.map.width as f64;
        let grid_h = self.cell_size * game.map.height as f64;
        self.offset_x = ((width - grid_w) / 2.0).floor();
        self.offset_y = ((height - grid_h) / 2.0).floor();
    }

    pub fn draw(&self, game: &Game, preview_path: &[(i32, i32)]) {
        let ctx = &self.ctx;

        // Clear
        ctx.set_fill_style_str("#000");
        ctx.fill_rect(0.0, 0.0, self.canvas_w, self.canvas_h);

        // Draw tiles
        for y in 0..game.map.height {
            for x in 0..game.map.width {
                let px = self.offset_x + x as f64 * self.cell_size;
                let py = self.offset_y + y as f64 * self.cell_size;
                match game.map.get(x, y) {
                    Tile::Wall => {
                        ctx.set_fill_style_str("#333");
                        ctx.fill_rect(px, py, self.cell_size, self.cell_size);
                    }
                    Tile::Floor => {
                        ctx.set_fill_style_str("#111");
                        ctx.fill_rect(px + 1.0, py + 1.0, self.cell_size - 2.0, self.cell_size - 2.0);
                    }
                }
            }
        }

        // Draw path preview (highlighted tiles)
        if preview_path.len() > 1 {
            for (i, &(tx, ty)) in preview_path.iter().enumerate() {
                let px = self.offset_x + tx as f64 * self.cell_size;
                let py = self.offset_y + ty as f64 * self.cell_size;
                if i == preview_path.len() - 1 {
                    // Destination tile
                    ctx.set_fill_style_str("rgba(0,180,255,0.4)");
                } else {
                    // Path tile
                    ctx.set_fill_style_str("rgba(0,180,255,0.2)");
                }
                ctx.fill_rect(px + 1.0, py + 1.0, self.cell_size - 2.0, self.cell_size - 2.0);
            }
        }

        let font_size = (self.cell_size * 0.8).round();
        ctx.set_font(&format!("{font_size}px monospace"));
        ctx.set_text_align("center");
        ctx.set_text_baseline("middle");

        // Draw enemies
        for e in &game.enemies {
            if e.hp <= 0 {
                continue;
            }
            let ex = self.offset_x + e.x as f64 * self.cell_size;
            let ey = self.offset_y + e.y as f64 * self.cell_size;
            let color = if e.glyph == 'D' { "#f44" } else { "#4f4" };
            ctx.set_fill_style_str(color);
            let _ = ctx.fill_text(
                &e.glyph.to_string(),
                ex + self.cell_size / 2.0,
                ey + self.cell_size / 2.0,
            );
        }

        // Draw player @
        let px = self.offset_x + game.player_x as f64 * self.cell_size;
        let py = self.offset_y + game.player_y as f64 * self.cell_size;
        ctx.set_fill_style_str("#fff");
        let _ = ctx.fill_text(
            "@",
            px + self.cell_size / 2.0,
            py + self.cell_size / 2.0,
        );

        // HUD — HP bar at top
        let hud_y = 4.0;
        let bar_w = self.canvas_w * 0.3;
        let bar_h = 14.0;
        let bar_x = 8.0;
        let hp_frac = game.player_hp as f64 / game.player_max_hp as f64;

        ctx.set_fill_style_str("#400");
        ctx.fill_rect(bar_x, hud_y, bar_w, bar_h);
        let hp_color = if hp_frac > 0.5 { "#0c0" } else if hp_frac > 0.25 { "#cc0" } else { "#c00" };
        ctx.set_fill_style_str(hp_color);
        ctx.fill_rect(bar_x, hud_y, bar_w * hp_frac.max(0.0), bar_h);

        ctx.set_font("12px monospace");
        ctx.set_fill_style_str("#fff");
        ctx.set_text_align("left");
        ctx.set_text_baseline("top");
        let _ = ctx.fill_text(
            &format!("HP {}/{}", game.player_hp, game.player_max_hp),
            bar_x + 4.0,
            hud_y + 1.0,
        );

        // Messages — last 3 at bottom
        let msg_count = game.messages.len();
        let show = if msg_count > 3 { 3 } else { msg_count };
        ctx.set_font("11px monospace");
        ctx.set_fill_style_str("#aaa");
        ctx.set_text_align("left");
        ctx.set_text_baseline("bottom");
        for i in 0..show {
            let msg = &game.messages[msg_count - show + i];
            let y = self.canvas_h - (show - i) as f64 * 14.0;
            let _ = ctx.fill_text(msg, 8.0, y);
        }

        // Death / Victory overlay
        if !game.alive || game.won {
            ctx.set_fill_style_str("rgba(0,0,0,0.7)");
            ctx.fill_rect(0.0, 0.0, self.canvas_w, self.canvas_h);

            let big = (self.canvas_w * 0.06).min(48.0).round();
            ctx.set_font(&format!("{big}px monospace"));
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");

            if game.won {
                ctx.set_fill_style_str("#ff0");
                let _ = ctx.fill_text("YOU WON!", self.canvas_w / 2.0, self.canvas_h / 2.0 - big);
            } else {
                ctx.set_fill_style_str("#f44");
                let _ = ctx.fill_text("YOU DIED", self.canvas_w / 2.0, self.canvas_h / 2.0 - big);
            }

            let small = (big * 0.4).round();
            ctx.set_font(&format!("{small}px monospace"));
            ctx.set_fill_style_str("#888");
            let _ = ctx.fill_text(
                "Tap or press any key to restart",
                self.canvas_w / 2.0,
                self.canvas_h / 2.0 + big * 0.5,
            );
        }
    }
}
