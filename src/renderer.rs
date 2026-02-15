use web_sys::CanvasRenderingContext2d;

use crate::camera::Camera;
use crate::game::Game;
use crate::map::Tile;

pub struct Renderer {
    ctx: CanvasRenderingContext2d,
    pub camera: Camera,
}

impl Renderer {
    pub fn new(ctx: CanvasRenderingContext2d) -> Self {
        Self {
            ctx,
            camera: Camera::new(),
        }
    }

    pub fn resize(&mut self, width: f64, height: f64) {
        self.camera.set_viewport(width, height);
    }

    pub fn draw(&self, game: &Game, preview_path: &[(i32, i32)]) {
        let ctx = &self.ctx;
        let cam = &self.camera;
        let cell = cam.cell_size();

        // Clear
        ctx.set_fill_style_str("#000");
        ctx.fill_rect(0.0, 0.0, cam.viewport_w() * cell, cam.viewport_h() * cell + cell);

        // Draw only visible tiles
        let (min_x, min_y, max_x, max_y) = cam.visible_range(game.map.width, game.map.height);
        for y in min_y..max_y {
            for x in min_x..max_x {
                let (px, py) = cam.world_to_screen(x, y);
                match game.map.get(x, y) {
                    Tile::Wall => {
                        ctx.set_fill_style_str("#333");
                        ctx.fill_rect(px, py, cell, cell);
                    }
                    Tile::Floor => {
                        ctx.set_fill_style_str("#111");
                        ctx.fill_rect(px + 1.0, py + 1.0, cell - 2.0, cell - 2.0);
                    }
                }
            }
        }

        // Draw path preview (highlighted tiles)
        if preview_path.len() > 1 {
            for (i, &(tx, ty)) in preview_path.iter().enumerate() {
                let (px, py) = cam.world_to_screen(tx, ty);
                if i == preview_path.len() - 1 {
                    ctx.set_fill_style_str("rgba(0,180,255,0.4)");
                } else {
                    ctx.set_fill_style_str("rgba(0,180,255,0.2)");
                }
                ctx.fill_rect(px + 1.0, py + 1.0, cell - 2.0, cell - 2.0);
            }
        }

        let font_size = (cell * 0.8).round();
        ctx.set_font(&format!("{font_size}px monospace"));
        ctx.set_text_align("center");
        ctx.set_text_baseline("middle");

        // Draw enemies (only if visible)
        for e in &game.enemies {
            if e.hp <= 0 {
                continue;
            }
            if e.x < min_x || e.x >= max_x || e.y < min_y || e.y >= max_y {
                continue;
            }
            let (ex, ey) = cam.world_to_screen(e.x, e.y);
            let color = if e.glyph == 'D' { "#f44" } else { "#4f4" };
            ctx.set_fill_style_str(color);
            let _ = ctx.fill_text(
                &e.glyph.to_string(),
                ex + cell / 2.0,
                ey + cell / 2.0,
            );
        }

        // Draw player @
        let (px, py) = cam.world_to_screen(game.player_x, game.player_y);
        ctx.set_fill_style_str("#fff");
        let _ = ctx.fill_text("@", px + cell / 2.0, py + cell / 2.0);

        // HUD — HP bar at top
        let canvas_w = cam.viewport_w() * cell;
        let canvas_h = cam.viewport_h() * cell;
        let hud_y = 4.0;
        let bar_w = canvas_w * 0.3;
        let bar_h = 14.0;
        let bar_x = 8.0;
        let hp_frac = game.player_hp as f64 / game.player_max_hp as f64;

        ctx.set_fill_style_str("#400");
        ctx.fill_rect(bar_x, hud_y, bar_w, bar_h);
        let hp_color = if hp_frac > 0.5 {
            "#0c0"
        } else if hp_frac > 0.25 {
            "#cc0"
        } else {
            "#c00"
        };
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
            let y = canvas_h - (show - i) as f64 * 14.0;
            let _ = ctx.fill_text(msg, 8.0, y);
        }

        // Death / Victory overlay
        if !game.alive || game.won {
            ctx.set_fill_style_str("rgba(0,0,0,0.7)");
            ctx.fill_rect(0.0, 0.0, canvas_w, canvas_h);

            let big = (canvas_w * 0.06).min(48.0).round();
            ctx.set_font(&format!("{big}px monospace"));
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");

            if game.won {
                ctx.set_fill_style_str("#ff0");
                let _ = ctx.fill_text("YOU WON!", canvas_w / 2.0, canvas_h / 2.0 - big);
            } else {
                ctx.set_fill_style_str("#f44");
                let _ = ctx.fill_text("YOU DIED", canvas_w / 2.0, canvas_h / 2.0 - big);
            }

            let small = (big * 0.4).round();
            ctx.set_font(&format!("{small}px monospace"));
            ctx.set_fill_style_str("#888");
            let _ = ctx.fill_text(
                "Tap or press any key to restart",
                canvas_w / 2.0,
                canvas_h / 2.0 + big * 0.5,
            );
        }
    }
}
