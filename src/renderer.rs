use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::camera::Camera;
use crate::game::{Drawer, Game, Item, ItemKind};
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
            // Animals and AnimatedTiles sheets not yet loaded
            _ => &self.tiles,
        }
    }
}

/// Base heights of the fixed HUD regions (in CSS points, scaled by DPR).
const TOP_BAR_BASE: f64 = 52.0;
const DETAIL_STRIP_BASE: f64 = 52.0;
const BOTTOM_BAR_BASE: f64 = 48.0;
const MSG_AREA_BASE: f64 = 42.0;

/// Speed of drawer slide animation (fraction per frame at 60fps).
const DRAWER_ANIM_SPEED: f64 = 0.15;

pub struct Renderer {
    ctx: CanvasRenderingContext2d,
    pub camera: Camera,
    sheets: Option<SpriteSheets>,
    /// Device pixel ratio — used to scale HUD elements.
    dpr: f64,
    /// Drawer slide animation progress: 0.0 = fully closed, 1.0 = fully open.
    drawer_anim: f64,
    /// Which drawer was last opened (kept during close animation).
    last_drawer: Drawer,
}

impl Renderer {
    pub fn new(ctx: CanvasRenderingContext2d) -> Self {
        Self {
            ctx,
            camera: Camera::new(),
            sheets: None,
            dpr: 1.0,
            drawer_anim: 0.0,
            last_drawer: Drawer::None,
        }
    }

    pub fn set_sheets(&mut self, sheets: SpriteSheets) {
        self.sheets = Some(sheets);
    }

    pub fn resize(&mut self, width: f64, height: f64, dpr: f64) {
        self.dpr = dpr;
        let cell = self.camera.set_viewport(width, height);
        // Pad camera so map content clears HUD overlays at edges.
        self.camera.pad_top = (self.top_bar_h() + self.detail_strip_h()) / cell;
        self.camera.pad_bottom = (self.bottom_bar_h() + self.msg_area_h()) / cell;
    }

    /// Scaled HUD dimension helpers.
    fn top_bar_h(&self) -> f64 { TOP_BAR_BASE * self.dpr }
    fn detail_strip_h(&self) -> f64 { DETAIL_STRIP_BASE * self.dpr }
    fn bottom_bar_h(&self) -> f64 { BOTTOM_BAR_BASE * self.dpr }
    fn msg_area_h(&self) -> f64 { MSG_AREA_BASE * self.dpr }

    /// Height of the bottom bar in canvas pixels.
    pub fn bottom_bar_height(&self) -> f64 {
        self.bottom_bar_h()
    }

    /// Advance drawer animation toward the current drawer state.
    /// Call once per frame before `draw()`.
    pub fn tick_drawer_anim(&mut self, current_drawer: Drawer) {
        if current_drawer != Drawer::None {
            self.last_drawer = current_drawer;
            self.drawer_anim = (self.drawer_anim + DRAWER_ANIM_SPEED).min(1.0);
        } else {
            self.drawer_anim = (self.drawer_anim - DRAWER_ANIM_SPEED).max(0.0);
        }
    }


    /// Format a font string scaled by DPR.
    fn font(&self, base_px: f64, weight: &str) -> String {
        let sz = (base_px * self.dpr).round();
        if weight.is_empty() {
            format!("{sz}px monospace")
        } else {
            format!("{weight} {sz}px monospace")
        }
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
                let _ = ctx.translate(dx + dw, dy);
                let _ = ctx.scale(-1.0, 1.0);
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

    /// Helper: draw a rounded-rect filled region.
    fn fill_rounded_rect(&self, x: f64, y: f64, w: f64, h: f64, r: f64) {
        let ctx = &self.ctx;
        ctx.begin_path();
        ctx.move_to(x + r, y);
        ctx.line_to(x + w - r, y);
        let _ = ctx.arc_to(x + w, y, x + w, y + r, r);
        ctx.line_to(x + w, y + h - r);
        let _ = ctx.arc_to(x + w, y + h, x + w - r, y + h, r);
        ctx.line_to(x + r, y + h);
        let _ = ctx.arc_to(x, y + h, x, y + h - r, r);
        ctx.line_to(x, y + r);
        let _ = ctx.arc_to(x, y, x + r, y, r);
        ctx.close_path();
        ctx.fill();
    }

    pub fn draw(&self, game: &Game, preview_path: &[(i32, i32)]) {
        let ctx = &self.ctx;
        let cam = &self.camera;
        let cell = cam.cell_size();
        let canvas_w = cam.viewport_w() * cell;
        let canvas_h = cam.viewport_h() * cell;

        // Crisp pixel art
        ctx.set_image_smoothing_enabled(false);

        // Clear
        ctx.set_fill_style_str("#000");
        ctx.fill_rect(0.0, 0.0, canvas_w, canvas_h + cell);

        // ---- World rendering (tiles, entities) ----
        self.draw_world(game, preview_path);

        // ---- UI overlay (all use self.dpr-scaled sizes) ----
        let top_h = self.top_bar_h();
        let bottom_h = self.bottom_bar_h();
        let msg_h = self.msg_area_h();
        self.draw_top_bar(game, canvas_w, top_h);
        self.draw_tile_detail(game, canvas_w, top_h);
        self.draw_messages(game, canvas_w, canvas_h, bottom_h, msg_h);
        self.draw_bottom_bar(game, canvas_w, canvas_h, bottom_h);
        self.draw_drawer(game, canvas_w, canvas_h, bottom_h);

        // Death / Victory overlay (on top of everything)
        if !game.alive || game.won {
            self.draw_end_overlay(game, canvas_w, canvas_h);
        }
    }

    // ---- World layer ----

    fn draw_world(&self, game: &Game, preview_path: &[(i32, i32)]) {
        let ctx = &self.ctx;
        let cam = &self.camera;
        let cell = cam.cell_size();
        let map = game.current_map();
        let (min_x, min_y, max_x, max_y) = cam.visible_range(map.width, map.height);

        // Tiles
        for y in min_y..max_y {
            for x in min_x..max_x {
                let vis = map.get_visibility(x, y);
                if vis == Visibility::Hidden {
                    continue;
                }
                let (px, py) = cam.world_to_screen(x, y);
                let tile = map.get(x, y);
                let wall_face = tile == Tile::Wall
                    && y + 1 < map.height
                    && map.get(x, y + 1) != Tile::Wall;
                let sprite = sprites::tile_sprite(tile, x, y, wall_face);
                if !self.draw_sprite(sprite, px, py, cell, cell) {
                    self.draw_tile_fallback(tile, px, py, cell);
                }
                if vis == Visibility::Seen {
                    ctx.set_fill_style_str("rgba(0,0,0,0.5)");
                    ctx.fill_rect(px, py, cell, cell);
                }
            }
        }

        // Path preview
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

        // Enemies
        let map = game.current_map();
        for e in &game.enemies {
            if e.hp <= 0 { continue; }
            if e.x < min_x || e.x >= max_x || e.y < min_y || e.y >= max_y { continue; }
            if map.get_visibility(e.x, e.y) != Visibility::Visible { continue; }
            let (ex, ey) = cam.world_to_screen(e.x, e.y);
            let sprite = sprites::enemy_sprite(e.glyph);
            if !self.draw_sprite_ex(sprite, ex, ey, cell, cell, !e.facing_left) {
                let font_size = (cell * 0.8).round();
                ctx.set_font(&format!("{font_size}px monospace"));
                ctx.set_text_align("center");
                ctx.set_text_baseline("middle");
                let color = match e.glyph {
                    'D' => "#f44",
                    'w' | 'b' | 'B' => "#a84",
                    _ => "#4f4",
                };
                ctx.set_fill_style_str(color);
                let _ = ctx.fill_text(&e.glyph.to_string(), ex + cell / 2.0, ey + cell / 2.0);
            }
        }

        // Ground items
        for gi in &game.ground_items {
            if gi.x < min_x || gi.x >= max_x || gi.y < min_y || gi.y >= max_y { continue; }
            if map.get_visibility(gi.x, gi.y) != Visibility::Visible { continue; }
            let (ix, iy) = cam.world_to_screen(gi.x, gi.y);
            let sprite = sprites::item_sprite(gi.item.name);
            if !self.draw_sprite(sprite, ix, iy, cell, cell) {
                let font_size = (cell * 0.6).round();
                ctx.set_font(&format!("{font_size}px monospace"));
                ctx.set_text_align("center");
                ctx.set_text_baseline("middle");
                ctx.set_fill_style_str("#ff0");
                let _ = ctx.fill_text(&gi.item.glyph.to_string(), ix + cell / 2.0, iy + cell / 2.0);
            }
        }

        // Player
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
    }

    // ---- Top bar: player stats ----

    fn draw_top_bar(&self, game: &Game, canvas_w: f64, top_h: f64) {
        let ctx = &self.ctx;
        let d = self.dpr;
        let pad = 8.0 * d;

        // Semi-transparent background strip
        ctx.set_fill_style_str("rgba(0,0,0,0.75)");
        ctx.fill_rect(0.0, 0.0, canvas_w, top_h);

        let bar_h = 12.0 * d;
        let bar_gap = 3.0 * d;
        let bar_w = canvas_w * 0.44;
        let bar_x = pad;
        let bar_r = 3.0 * d;
        let text_inset = 4.0 * d;

        // Row 1: HP bar — left
        let row1_y = 5.0 * d;
        let hp_frac = game.player_hp as f64 / game.player_max_hp as f64;
        ctx.set_fill_style_str("#2a0a0a");
        self.fill_rounded_rect(bar_x, row1_y, bar_w, bar_h, bar_r);
        let hp_color = if hp_frac > 0.5 { "#2a2" }
            else if hp_frac > 0.25 { "#aa2" }
            else { "#a22" };
        ctx.set_fill_style_str(hp_color);
        self.fill_rounded_rect(bar_x, row1_y, bar_w * hp_frac.max(0.0), bar_h, bar_r);
        ctx.set_font(&self.font(10.0, "bold"));
        ctx.set_fill_style_str("#fff");
        ctx.set_text_align("left");
        ctx.set_text_baseline("middle");
        let _ = ctx.fill_text(
            &format!("HP {}/{}", game.player_hp, game.player_max_hp),
            bar_x + text_inset, row1_y + bar_h / 2.0,
        );

        // Row 2: Stamina bar
        let row2_y = row1_y + bar_h + bar_gap;
        let stam_frac = game.stamina as f64 / game.max_stamina as f64;
        ctx.set_fill_style_str("#0a0a2a");
        self.fill_rounded_rect(bar_x, row2_y, bar_w, bar_h, bar_r);
        let stam_color = if game.sprinting { "#4af" } else { "#28a" };
        ctx.set_fill_style_str(stam_color);
        self.fill_rounded_rect(bar_x, row2_y, bar_w * stam_frac.max(0.0), bar_h, bar_r);
        ctx.set_font(&self.font(10.0, ""));
        ctx.set_fill_style_str("#fff");
        let sprint_label = if game.sprinting { "STA (SPRINT)" } else { "STA" };
        let _ = ctx.fill_text(
            &format!("{} {}/{}", sprint_label, game.stamina, game.max_stamina),
            bar_x + text_inset, row2_y + bar_h / 2.0,
        );

        // Row 3: Hunger bar
        let row3_y = row2_y + bar_h + bar_gap;
        let hunger_frac = game.hunger as f64 / game.max_hunger as f64;
        ctx.set_fill_style_str("#2a1a0a");
        self.fill_rounded_rect(bar_x, row3_y, bar_w, bar_h, bar_r);
        let hunger_color = if hunger_frac > 0.3 { "#a82" }
            else if hunger_frac > 0.1 { "#a52" }
            else { "#a22" };
        ctx.set_fill_style_str(hunger_color);
        self.fill_rounded_rect(bar_x, row3_y, bar_w * hunger_frac.max(0.0), bar_h, bar_r);
        ctx.set_font(&self.font(10.0, ""));
        ctx.set_fill_style_str("#fff");
        let _ = ctx.fill_text(
            &format!("FOOD {}/{}", game.hunger, game.max_hunger),
            bar_x + text_inset, row3_y + bar_h / 2.0,
        );

        // Right column: location + stats + XP
        ctx.set_text_align("right");
        ctx.set_fill_style_str("#ccc");
        ctx.set_font(&self.font(11.0, "bold"));
        let _ = ctx.fill_text(&game.location_name(), canvas_w - pad, row1_y + bar_h / 2.0);

        let atk = game.effective_attack();
        let def = game.effective_defense();
        ctx.set_font(&self.font(10.0, ""));
        ctx.set_fill_style_str("#8cf");
        let _ = ctx.fill_text(
            &format!("ATK {} DEF {} LVL {}", atk, def, game.player_level),
            canvas_w - pad, row2_y + bar_h / 2.0,
        );

        let xp_needed = game.xp_to_next_level();
        ctx.set_fill_style_str("#a8f");
        let _ = ctx.fill_text(
            &format!("XP {}/{}", game.player_xp, xp_needed),
            canvas_w - pad, row3_y + bar_h / 2.0,
        );
    }

    // ---- Tile detail strip (below top bar, shown when inspecting) ----

    fn draw_tile_detail(&self, game: &Game, canvas_w: f64, top_h: f64) {
        let info = match &game.inspected {
            Some(info) => info,
            None => return,
        };
        let ctx = &self.ctx;
        let d = self.dpr;
        let strip_h = self.detail_strip_h();
        let y0 = top_h;
        let pad = 8.0 * d;

        // Background
        ctx.set_fill_style_str("rgba(0,0,0,0.8)");
        ctx.fill_rect(0.0, y0, canvas_w, strip_h);
        ctx.set_fill_style_str("rgba(0,180,255,0.6)");
        ctx.fill_rect(0.0, y0, canvas_w, 1.0);

        let icon_size = 32.0 * d;
        let text_x = pad + icon_size + 8.0 * d;
        let line1_y = y0 + 8.0 * d;
        let line2_y = y0 + 24.0 * d;
        let line3_y = y0 + 38.0 * d;

        if let Some(ref ei) = info.enemy {
            let e_glyph = game.enemies.iter()
                .find(|e| e.name == ei.name && e.hp > 0)
                .map(|e| e.glyph)
                .unwrap_or('?');
            let sprite = sprites::enemy_sprite(e_glyph);
            self.draw_sprite(sprite, pad, y0 + (strip_h - icon_size) / 2.0, icon_size, icon_size);

            ctx.set_font(&self.font(13.0, "bold"));
            ctx.set_fill_style_str("#f88");
            ctx.set_text_align("left");
            ctx.set_text_baseline("top");
            let _ = ctx.fill_text(ei.name, text_x, line1_y);

            ctx.set_font(&self.font(11.0, ""));
            ctx.set_fill_style_str("#ccc");
            let _ = ctx.fill_text(&format!("HP {} ATK {}", ei.hp, ei.attack), text_x, line2_y);

            ctx.set_fill_style_str("#999");
            let _ = ctx.fill_text(ei.desc, text_x, line3_y);
        } else if let Some(ref ii) = info.item {
            if let Some(gi) = game.ground_items.iter().find(|gi| gi.item.name == ii.name) {
                let sprite = sprites::item_sprite(gi.item.name);
                self.draw_sprite(sprite, pad, y0 + (strip_h - icon_size) / 2.0, icon_size, icon_size);
            }

            ctx.set_font(&self.font(13.0, "bold"));
            ctx.set_fill_style_str("#ff0");
            ctx.set_text_align("left");
            ctx.set_text_baseline("top");
            let _ = ctx.fill_text(ii.name, text_x, line1_y);

            ctx.set_font(&self.font(11.0, ""));
            ctx.set_fill_style_str("#ccc");
            let _ = ctx.fill_text(&ii.desc, text_x, line2_y);
        } else if info.is_player {
            let sprite = sprites::player_sprite();
            self.draw_sprite(sprite, pad, y0 + (strip_h - icon_size) / 2.0, icon_size, icon_size);

            ctx.set_font(&self.font(13.0, "bold"));
            ctx.set_fill_style_str("#fff");
            ctx.set_text_align("left");
            ctx.set_text_baseline("top");
            let _ = ctx.fill_text("You", text_x, line1_y);

            ctx.set_font(&self.font(11.0, ""));
            ctx.set_fill_style_str("#ccc");
            let _ = ctx.fill_text(
                &format!("HP {}/{} ATK {} DEF {}",
                    game.player_hp, game.player_max_hp,
                    game.effective_attack(), game.effective_defense()),
                text_x, line2_y,
            );
        } else {
            ctx.set_font(&self.font(13.0, "bold"));
            ctx.set_fill_style_str("#ccc");
            ctx.set_text_align("left");
            ctx.set_text_baseline("top");
            let _ = ctx.fill_text(info.tile_name, pad, line1_y);

            ctx.set_font(&self.font(11.0, ""));
            ctx.set_fill_style_str("#888");
            let _ = ctx.fill_text(info.tile_desc, pad, line2_y);
        }
    }

    // ---- Messages (above bottom bar) ----

    fn draw_messages(&self, game: &Game, canvas_w: f64, canvas_h: f64, bottom_h: f64, msg_h: f64) {
        let ctx = &self.ctx;
        let d = self.dpr;
        let msg_bottom = canvas_h - bottom_h;
        let msg_top = msg_bottom - msg_h;

        ctx.set_fill_style_str("rgba(0,0,0,0.5)");
        ctx.fill_rect(0.0, msg_top, canvas_w, msg_h);

        let msg_count = game.messages.len();
        let show = msg_count.min(3);
        ctx.set_font(&self.font(11.0, ""));
        ctx.set_text_align("left");
        ctx.set_text_baseline("bottom");
        let line_h = 14.0 * d;
        for i in 0..show {
            let msg = &game.messages[msg_count - show + i];
            let alpha = if i == show - 1 { "ddd" }
                else if i == show - 2 { "999" }
                else { "666" };
            ctx.set_fill_style_str(&format!("#{alpha}"));
            let y = msg_bottom - (show - 1 - i) as f64 * line_h - 4.0 * d;
            let _ = ctx.fill_text(msg, 8.0 * d, y);
        }
    }

    // ---- Bottom action bar ----

    fn draw_bottom_bar(&self, game: &Game, canvas_w: f64, canvas_h: f64, bottom_h: f64) {
        let ctx = &self.ctx;
        let d = self.dpr;
        let bar_y = canvas_h - bottom_h;

        ctx.set_fill_style_str("rgba(12,12,18,0.92)");
        ctx.fill_rect(0.0, bar_y, canvas_w, bottom_h);
        ctx.set_fill_style_str("rgba(255,255,255,0.08)");
        ctx.fill_rect(0.0, bar_y, canvas_w, 1.0);

        let btn_count = 4.0;
        let btn_w = (canvas_w / btn_count).min(120.0 * d);
        let btn_h = 34.0 * d;
        let btn_y = bar_y + (bottom_h - btn_h) / 2.0;
        let total_w = btn_w * btn_count;
        let start_x = (canvas_w - total_w) / 2.0;

        let sprint_color = if game.sprinting { "#4ef" } else { "#48a" };
        let sprint_label = if game.sprinting { "SPRINT" } else { "Sprint" };

        let buttons: [(&str, &str, bool); 4] = [
            ("Inventory", if game.drawer == Drawer::Inventory { "#8af" } else { "#58f" }, game.drawer == Drawer::Inventory),
            ("Stats", if game.drawer == Drawer::Stats { "#c8f" } else { "#a8f" }, game.drawer == Drawer::Stats),
            (sprint_label, sprint_color, game.sprinting),
            ("Menu", "#888", false),
        ];

        for (i, (label, color, active)) in buttons.iter().enumerate() {
            let bx = start_x + i as f64 * btn_w + 3.0 * d;
            let bw = btn_w - 6.0 * d;

            if *active {
                ctx.set_fill_style_str("rgba(255,255,255,0.12)");
                self.fill_rounded_rect(bx, btn_y, bw, btn_h, 6.0 * d);
            }

            ctx.set_font(&self.font(12.0, "bold"));
            ctx.set_fill_style_str(color);
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");
            let _ = ctx.fill_text(label, bx + bw / 2.0, btn_y + btn_h / 2.0);
        }
    }

    // ---- Drawer: slide-up panel ----

    fn draw_drawer(&self, game: &Game, canvas_w: f64, canvas_h: f64, bottom_h: f64) {
        let t = self.drawer_anim;
        if t <= 0.0 {
            return; // fully closed, nothing to draw
        }

        // Use current drawer if open, or last_drawer during close animation
        let which = if game.drawer != Drawer::None {
            game.drawer
        } else {
            self.last_drawer
        };
        if which == Drawer::None {
            return;
        }

        // Backdrop — dims the game world behind the drawer
        let backdrop_alpha = 0.5 * t;
        self.ctx.set_fill_style_str(&format!("rgba(0,0,0,{:.2})", backdrop_alpha));
        self.ctx.fill_rect(0.0, 0.0, canvas_w, canvas_h - bottom_h);

        match which {
            Drawer::Inventory => self.draw_inventory_drawer(game, canvas_w, canvas_h, bottom_h, t),
            Drawer::Stats => self.draw_stats_drawer(game, canvas_w, canvas_h, bottom_h, t),
            Drawer::None => {}
        }
    }

    fn draw_inventory_drawer(&self, game: &Game, canvas_w: f64, canvas_h: f64, bottom_h: f64, anim_t: f64) {
        let ctx = &self.ctx;
        let d = self.dpr;
        let drawer_h = canvas_h * 0.55;
        let base_y = canvas_h - bottom_h - drawer_h;
        // Slide up from bottom: at t=0 the drawer is fully below the bar, at t=1 it's in place.
        let slide_offset = drawer_h * (1.0 - anim_t);
        let drawer_y = base_y + slide_offset;
        let pad = 12.0 * d;

        // Clip to avoid drawing above the destination or below the bar
        ctx.save();
        ctx.begin_path();
        ctx.rect(0.0, base_y, canvas_w, drawer_h);
        ctx.clip();

        ctx.set_fill_style_str("rgba(8,8,16,0.94)");
        self.fill_rounded_rect(0.0, drawer_y, canvas_w, drawer_h, 12.0 * d);
        ctx.set_fill_style_str("rgba(80,130,255,0.3)");
        self.fill_rounded_rect(canvas_w * 0.3, drawer_y, canvas_w * 0.4, 3.0 * d, 1.5 * d);

        // Title
        ctx.set_font(&self.font(14.0, "bold"));
        ctx.set_fill_style_str("#fff");
        ctx.set_text_align("center");
        ctx.set_text_baseline("top");
        let _ = ctx.fill_text("INVENTORY", canvas_w / 2.0, drawer_y + 10.0 * d);

        // Equipment slots (3 rows x 2 columns)
        let eq_y = drawer_y + 32.0 * d;
        let eq_h = 30.0 * d;
        let eq_gap = 4.0 * d;
        let eq_icon = 24.0 * d;
        let half_w = canvas_w / 2.0 - pad * 1.5;
        let right_x = canvas_w / 2.0 + pad * 0.5;
        ctx.set_text_align("left");
        ctx.set_text_baseline("top");

        let slot_pairs: [(&Option<Item>, f64, f64, &str, &str); 6] = [
            (&game.equipped_weapon,  pad,     eq_y,                        "#8af", "No weapon"),
            (&game.equipped_armor,   right_x, eq_y,                        "#afa", "No armor"),
            (&game.equipped_helmet,  pad,     eq_y + eq_h + eq_gap,        "#fc8", "No helmet"),
            (&game.equipped_shield,  right_x, eq_y + eq_h + eq_gap,        "#adf", "No shield"),
            (&game.equipped_boots,   pad,     eq_y + (eq_h + eq_gap) * 2.0, "#da8", "No boots"),
            (&game.equipped_ring,    right_x, eq_y + (eq_h + eq_gap) * 2.0, "#ff8", "No ring"),
        ];
        for &(slot, sx, sy, color, empty_label) in &slot_pairs {
            ctx.set_fill_style_str("rgba(255,255,255,0.06)");
            self.fill_rounded_rect(sx, sy, half_w, eq_h, 4.0 * d);
            let icon_x = sx + 4.0 * d;
            if let Some(ref item) = slot {
                let sprite = sprites::item_sprite(item.name);
                self.draw_sprite(sprite, icon_x, sy + (eq_h - eq_icon) / 2.0, eq_icon, eq_icon);
                ctx.set_font(&self.font(10.0, ""));
                ctx.set_fill_style_str(color);
                let _ = ctx.fill_text(item.name, icon_x + eq_icon + 4.0 * d, sy + eq_h / 2.0 - 5.0 * d);
            } else {
                ctx.set_font(&self.font(10.0, ""));
                ctx.set_fill_style_str("#555");
                let _ = ctx.fill_text(empty_label, icon_x + 4.0 * d, sy + eq_h / 2.0 - 5.0 * d);
            }
        }

        // Item list — reserve right margin for scrollbar
        let scrollbar_w = 12.0 * d;
        let list_y = eq_y + (eq_h + eq_gap) * 3.0 + 4.0 * d;
        let slot_h = 34.0 * d;
        let icon_size = 28.0 * d;

        let footer_h = 20.0 * d;
        let avail_h = (drawer_y + drawer_h - footer_h) - list_y;
        let max_visible = (avail_h / slot_h).floor().max(1.0) as usize;

        // Right edge for item text (before scrollbar)
        let text_right = canvas_w - pad - scrollbar_w - 4.0 * d;

        if game.inventory.is_empty() {
            ctx.set_fill_style_str("#555");
            ctx.set_font(&self.font(11.0, ""));
            ctx.set_text_baseline("top");
            let _ = ctx.fill_text("No items", pad + 4.0 * d, list_y + 4.0 * d);
        } else {
            let scroll = game.inventory_scroll;
            let total = game.inventory.len();
            let end = (scroll + max_visible).min(total);

            for (vi, idx) in (scroll..end).enumerate() {
                let item = &game.inventory[idx];
                let iy = list_y + vi as f64 * slot_h;

                if vi % 2 == 0 {
                    ctx.set_fill_style_str("rgba(255,255,255,0.03)");
                    ctx.fill_rect(pad, iy, canvas_w - pad * 2.0 - scrollbar_w, slot_h);
                }

                let sprite = sprites::item_sprite(item.name);
                self.draw_sprite(sprite, pad + 4.0 * d, iy + (slot_h - icon_size) / 2.0, icon_size, icon_size);

                let color = match item.kind {
                    ItemKind::Potion => "#f88",
                    ItemKind::Scroll => "#88f",
                    ItemKind::Weapon => "#aaf",
                    ItemKind::Armor => "#afa",
                    ItemKind::Helmet => "#fc8",
                    ItemKind::Shield => "#adf",
                    ItemKind::Boots => "#da8",
                    ItemKind::Food => "#fa8",
                    ItemKind::Ring => "#ff8",
                };
                ctx.set_font(&self.font(12.0, ""));
                ctx.set_fill_style_str(color);
                ctx.set_text_baseline("middle");
                let _ = ctx.fill_text(item.name, pad + icon_size + 8.0 * d, iy + slot_h / 2.0);

                let hint = match &item.effect {
                    crate::game::ItemEffect::Heal(n) => format!("+{} HP", n),
                    crate::game::ItemEffect::DamageAoe(n) => format!("{} DMG", n),
                    crate::game::ItemEffect::BuffAttack(n) => format!("+{} ATK", n),
                    crate::game::ItemEffect::BuffDefense(n) => format!("+{} DEF", n),
                    crate::game::ItemEffect::Feed(n) => format!("+{} FOOD", n),
                };
                ctx.set_text_align("right");
                ctx.set_fill_style_str("#999");
                ctx.set_font(&self.font(10.0, ""));
                let _ = ctx.fill_text(&hint, text_right, iy + slot_h / 2.0);
                ctx.set_text_align("left");
            }

            // Scrollbar (only when items exceed visible area)
            if total > max_visible {
                let track_x = canvas_w - pad - scrollbar_w + 2.0 * d;
                let track_w = scrollbar_w - 4.0 * d;
                let track_h = max_visible as f64 * slot_h;

                // Track background
                ctx.set_fill_style_str("rgba(255,255,255,0.06)");
                self.fill_rounded_rect(track_x, list_y, track_w, track_h, track_w / 2.0);

                // Thumb
                let scroll_range = total - max_visible;
                let thumb_frac = max_visible as f64 / total as f64;
                let min_thumb_h = 20.0 * d;
                let thumb_h = (track_h * thumb_frac).max(min_thumb_h);
                let scroll_frac = scroll as f64 / scroll_range as f64;
                let thumb_y = list_y + scroll_frac * (track_h - thumb_h);

                ctx.set_fill_style_str("rgba(255,255,255,0.25)");
                self.fill_rounded_rect(track_x, thumb_y, track_w, thumb_h, track_w / 2.0);
            }
        }

        // Slot count
        ctx.set_font(&self.font(10.0, ""));
        ctx.set_fill_style_str("#555");
        ctx.set_text_align("right");
        ctx.set_text_baseline("bottom");
        let _ = ctx.fill_text(
            &format!("{}/10", game.inventory.len()),
            canvas_w - pad, drawer_y + drawer_h - 6.0 * d,
        );

        ctx.restore(); // pop clip
    }

    fn draw_stats_drawer(&self, game: &Game, canvas_w: f64, canvas_h: f64, bottom_h: f64, anim_t: f64) {
        let ctx = &self.ctx;
        let d = self.dpr;
        let drawer_h = canvas_h * 0.45;
        let base_y = canvas_h - bottom_h - drawer_h;
        let slide_offset = drawer_h * (1.0 - anim_t);
        let drawer_y = base_y + slide_offset;
        let pad = 12.0 * d;

        ctx.save();
        ctx.begin_path();
        ctx.rect(0.0, base_y, canvas_w, drawer_h);
        ctx.clip();

        ctx.set_fill_style_str("rgba(8,8,16,0.94)");
        self.fill_rounded_rect(0.0, drawer_y, canvas_w, drawer_h, 12.0 * d);
        ctx.set_fill_style_str("rgba(160,80,255,0.3)");
        self.fill_rounded_rect(canvas_w * 0.3, drawer_y, canvas_w * 0.4, 3.0 * d, 1.5 * d);

        ctx.set_font(&self.font(14.0, "bold"));
        ctx.set_fill_style_str("#fff");
        ctx.set_text_align("center");
        ctx.set_text_baseline("top");
        let _ = ctx.fill_text("CHARACTER", canvas_w / 2.0, drawer_y + 10.0 * d);

        let icon_sz = 36.0 * d;
        let mut y = drawer_y + 32.0 * d;
        let line_h = 24.0 * d;

        // Player sprite + level
        let sprite = sprites::player_sprite();
        self.draw_sprite(sprite, pad, y, icon_sz, icon_sz);
        ctx.set_font(&self.font(13.0, "bold"));
        ctx.set_fill_style_str("#fff");
        ctx.set_text_align("left");
        ctx.set_text_baseline("top");
        let _ = ctx.fill_text(&format!("Level {}", game.player_level), pad + icon_sz + 8.0 * d, y + 2.0 * d);
        ctx.set_font(&self.font(11.0, ""));
        ctx.set_fill_style_str("#a8f");
        let _ = ctx.fill_text(
            &format!("XP {}/{}", game.player_xp, game.xp_to_next_level()),
            pad + icon_sz + 8.0 * d, y + 18.0 * d,
        );

        y += icon_sz + 6.0 * d;

        // XP progress bar
        let xp_bar_w = canvas_w - pad * 2.0;
        let xp_bar_h = 10.0 * d;
        let xp_frac = if game.xp_to_next_level() > 0 {
            game.player_xp as f64 / game.xp_to_next_level() as f64
        } else { 1.0 };
        ctx.set_fill_style_str("#1a0a2a");
        self.fill_rounded_rect(pad, y, xp_bar_w, xp_bar_h, 3.0 * d);
        ctx.set_fill_style_str("#a6f");
        self.fill_rounded_rect(pad, y, xp_bar_w * xp_frac, xp_bar_h, 3.0 * d);

        y += xp_bar_h + 12.0 * d;

        // Stat rows
        let stats: [(&str, String, &str); 4] = [
            ("HP", format!("{} / {}", game.player_hp, game.player_max_hp), "#4c4"),
            ("Attack", format!("{}", game.effective_attack()), "#8cf"),
            ("Defense", format!("{}", game.effective_defense()), "#fc8"),
            ("Location", game.location_name(), "#ccc"),
        ];

        for (label, value, color) in &stats {
            ctx.set_font(&self.font(12.0, ""));
            ctx.set_fill_style_str("#888");
            ctx.set_text_align("left");
            ctx.set_text_baseline("top");
            let _ = ctx.fill_text(label, pad, y);
            ctx.set_fill_style_str(color);
            ctx.set_text_align("right");
            let _ = ctx.fill_text(value, canvas_w - pad, y);
            y += line_h;
        }

        // Equipment section
        y += 8.0 * d;
        ctx.set_font(&self.font(11.0, "bold"));
        ctx.set_fill_style_str("#666");
        ctx.set_text_align("left");
        let _ = ctx.fill_text("Equipment", pad, y);
        y += 20.0 * d;

        let eq_icon = 24.0 * d;
        let eq_slots: [(&Option<Item>, &str, &str); 6] = [
            (&game.equipped_weapon,  "#8af", "- No weapon"),
            (&game.equipped_armor,   "#afa", "- No armor"),
            (&game.equipped_helmet,  "#fc8", "- No helmet"),
            (&game.equipped_shield,  "#adf", "- No shield"),
            (&game.equipped_boots,   "#da8", "- No boots"),
            (&game.equipped_ring,    "#ff8", "- No ring"),
        ];
        for &(slot, color, empty_label) in &eq_slots {
            if let Some(ref item) = slot {
                let sprite = sprites::item_sprite(item.name);
                self.draw_sprite(sprite, pad, y, eq_icon, eq_icon);
                ctx.set_font(&self.font(11.0, ""));
                ctx.set_fill_style_str(color);
                ctx.set_text_baseline("middle");
                let _ = ctx.fill_text(item.name, pad + eq_icon + 6.0 * d, y + eq_icon / 2.0);
            } else {
                ctx.set_font(&self.font(11.0, ""));
                ctx.set_fill_style_str("#444");
                ctx.set_text_baseline("top");
                let _ = ctx.fill_text(empty_label, pad, y);
            }
            y += eq_icon + 6.0 * d;
        }

        ctx.restore(); // pop clip
    }

    // ---- Death / Victory ----

    fn draw_end_overlay(&self, game: &Game, canvas_w: f64, canvas_h: f64) {
        let ctx = &self.ctx;
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
