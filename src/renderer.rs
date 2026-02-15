use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::camera::Camera;
use crate::game::Game;
use crate::map::{Tile, Visibility};
use crate::sprites::{self, Sheet, SpriteRef};

/// Loaded sprite sheet images, indexed by Sheet enum.
pub struct SpriteSheets {
    pub tiles: HtmlImageElement,
    pub monsters: HtmlImageElement,
    pub rogues: HtmlImageElement,
    pub items: HtmlImageElement,
}

impl SpriteSheets {
    fn get(&self, sheet: Sheet) -> &HtmlImageElement {
        match sheet {
            Sheet::Tiles => &self.tiles,
            Sheet::Monsters => &self.monsters,
            Sheet::Rogues => &self.rogues,
            Sheet::Items => &self.items,
        }
    }
}

pub struct Renderer {
    ctx: CanvasRenderingContext2d,
    pub camera: Camera,
    sheets: Option<SpriteSheets>,
}

impl Renderer {
    pub fn new(ctx: CanvasRenderingContext2d) -> Self {
        Self {
            ctx,
            camera: Camera::new(),
            sheets: None,
        }
    }

    pub fn set_sheets(&mut self, sheets: SpriteSheets) {
        self.sheets = Some(sheets);
    }

    pub fn resize(&mut self, width: f64, height: f64) {
        self.camera.set_viewport(width, height);
    }

    /// Draw a sprite at screen position. Returns true if drawn.
    fn draw_sprite(&self, sprite: SpriteRef, dx: f64, dy: f64, dw: f64, dh: f64) -> bool {
        self.draw_sprite_ex(sprite, dx, dy, dw, dh, false)
    }

    /// Draw a sprite, optionally mirrored horizontally. Returns true if drawn.
    fn draw_sprite_ex(&self, sprite: SpriteRef, dx: f64, dy: f64, dw: f64, dh: f64, flip: bool) -> bool {
        if let Some(sheets) = &self.sheets {
            let img = sheets.get(sprite.sheet);
            if flip {
                let ctx = &self.ctx;
                ctx.save();
                // Translate to the sprite center, flip x, draw offset back
                ctx.translate(dx + dw, dy).unwrap();
                ctx.scale(-1.0, 1.0).unwrap();
                let _ = ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    img,
                    sprite.src_x(), sprite.src_y(), 32.0, 32.0,
                    0.0, 0.0, dw, dh,
                );
                ctx.restore();
            } else {
                let _ = self.ctx.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    img,
                    sprite.src_x(), sprite.src_y(), 32.0, 32.0,
                    dx, dy, dw, dh,
                );
            }
            true
        } else {
            false
        }
    }

    pub fn draw(&self, game: &Game, preview_path: &[(i32, i32)]) {
        let ctx = &self.ctx;
        let cam = &self.camera;
        let cell = cam.cell_size();

        // Crisp pixel art — must set every frame (canvas resize resets context state)
        ctx.set_image_smoothing_enabled(false);

        // Clear
        ctx.set_fill_style_str("#000");
        ctx.fill_rect(0.0, 0.0, cam.viewport_w() * cell, cam.viewport_h() * cell + cell);

        // Draw only visible tiles
        let (min_x, min_y, max_x, max_y) = cam.visible_range(game.current_map().width, game.current_map().height);

        let map = game.current_map();
        for y in min_y..max_y {
            for x in min_x..max_x {
                let vis = map.get_visibility(x, y);
                if vis == Visibility::Hidden {
                    continue; // black — already cleared
                }

                let (px, py) = cam.world_to_screen(x, y);
                let tile = map.get(x, y);

                // Determine wall orientation: face if tile below is not a wall
                let wall_face = if tile == Tile::Wall {
                    map.get(x, y + 1) != Tile::Wall
                } else {
                    false
                };

                let sprite = sprites::tile_sprite(tile, x, y, wall_face);
                if !self.draw_sprite(sprite, px, py, cell, cell) {
                    self.draw_tile_fallback(tile, px, py, cell);
                }

                // Dim seen tiles (not currently visible)
                if vis == Visibility::Seen {
                    ctx.set_fill_style_str("rgba(0,0,0,0.5)");
                    ctx.fill_rect(px, py, cell, cell);
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

        // Draw enemies (only on Visible tiles)
        for e in &game.enemies {
            if e.hp <= 0 {
                continue;
            }
            if e.x < min_x || e.x >= max_x || e.y < min_y || e.y >= max_y {
                continue;
            }
            if map.get_visibility(e.x, e.y) != Visibility::Visible {
                continue;
            }
            let (ex, ey) = cam.world_to_screen(e.x, e.y);
            let sprite = sprites::enemy_sprite(e.glyph);
            if !self.draw_sprite_ex(sprite, ex, ey, cell, cell, !e.facing_left) {
                // Fallback: text glyph
                let font_size = (cell * 0.8).round();
                ctx.set_font(&format!("{font_size}px monospace"));
                ctx.set_text_align("center");
                ctx.set_text_baseline("middle");
                let color = if e.glyph == 'D' { "#f44" } else { "#4f4" };
                ctx.set_fill_style_str(color);
                let _ = ctx.fill_text(
                    &e.glyph.to_string(),
                    ex + cell / 2.0,
                    ey + cell / 2.0,
                );
            }
        }

        // Draw player
        let (px, py) = cam.world_to_screen(game.player_x, game.player_y);
        let player_sprite = sprites::player_sprite();
        if !self.draw_sprite_ex(player_sprite, px, py, cell, cell, !game.player_facing_left) {
            let font_size = (cell * 0.8).round();
            ctx.set_font(&format!("{font_size}px monospace"));
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");
            ctx.set_fill_style_str("#fff");
            let _ = ctx.fill_text("@", px + cell / 2.0, py + cell / 2.0);
        }

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

    /// Fallback rendering when sprite sheets aren't loaded yet.
    fn draw_tile_fallback(&self, tile: Tile, px: f64, py: f64, cell: f64) {
        let ctx = &self.ctx;
        match tile {
            Tile::Wall => {
                ctx.set_fill_style_str(tile.color());
                ctx.fill_rect(px, py, cell, cell);
            }
            Tile::Floor | Tile::Grass => {
                ctx.set_fill_style_str(tile.color());
                ctx.fill_rect(px + 1.0, py + 1.0, cell - 2.0, cell - 2.0);
            }
            _ => {
                ctx.set_fill_style_str(tile.color());
                ctx.fill_rect(px + 1.0, py + 1.0, cell - 2.0, cell - 2.0);
                let tile_font_size = (cell * 0.7).round();
                ctx.set_font(&format!("{tile_font_size}px monospace"));
                ctx.set_text_align("center");
                ctx.set_text_baseline("middle");
                ctx.set_fill_style_str("#ddd");
                let _ = ctx.fill_text(
                    &tile.glyph().to_string(),
                    px + cell / 2.0,
                    py + cell / 2.0,
                );
            }
        }
    }
}
