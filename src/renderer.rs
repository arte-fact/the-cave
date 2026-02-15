use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::camera::Camera;
use crate::game::{Drawer, Game, ItemKind};
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

/// Heights of the fixed HUD regions (in canvas pixels).
const TOP_BAR_H: f64 = 36.0;
const DETAIL_STRIP_H: f64 = 52.0;
const BOTTOM_BAR_H: f64 = 44.0;
const MSG_AREA_H: f64 = 42.0;

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

    /// Height of the bottom bar in canvas pixels.
    pub fn bottom_bar_height(&self) -> f64 {
        BOTTOM_BAR_H
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

    /// Helper: draw a rounded-rect filled region.
    fn fill_rounded_rect(&self, x: f64, y: f64, w: f64, h: f64, r: f64) {
        let ctx = &self.ctx;
        ctx.begin_path();
        ctx.move_to(x + r, y);
        ctx.line_to(x + w - r, y);
        ctx.arc_to(x + w, y, x + w, y + r, r).unwrap();
        ctx.line_to(x + w, y + h - r);
        ctx.arc_to(x + w, y + h, x + w - r, y + h, r).unwrap();
        ctx.line_to(x + r, y + h);
        ctx.arc_to(x, y + h, x, y + h - r, r).unwrap();
        ctx.line_to(x, y + r);
        ctx.arc_to(x, y, x + r, y, r).unwrap();
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

        // ---- UI overlay ----
        self.draw_top_bar(game, canvas_w);
        self.draw_tile_detail(game, canvas_w);
        self.draw_messages(game, canvas_w, canvas_h);
        self.draw_bottom_bar(game, canvas_w, canvas_h);
        self.draw_drawer(game, canvas_w, canvas_h);

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
                let wall_face = tile == Tile::Wall && map.get(x, y + 1) != Tile::Wall;
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

    fn draw_top_bar(&self, game: &Game, canvas_w: f64) {
        let ctx = &self.ctx;
        let pad = 6.0;

        // Semi-transparent background strip
        ctx.set_fill_style_str("rgba(0,0,0,0.75)");
        ctx.fill_rect(0.0, 0.0, canvas_w, TOP_BAR_H);

        // HP bar — left side
        let bar_x = pad;
        let bar_y = 6.0;
        let bar_w = canvas_w * 0.32;
        let bar_h = 12.0;
        let hp_frac = game.player_hp as f64 / game.player_max_hp as f64;

        ctx.set_fill_style_str("#2a0a0a");
        self.fill_rounded_rect(bar_x, bar_y, bar_w, bar_h, 3.0);
        let hp_color = if hp_frac > 0.5 { "#2a2" }
            else if hp_frac > 0.25 { "#aa2" }
            else { "#a22" };
        ctx.set_fill_style_str(hp_color);
        self.fill_rounded_rect(bar_x, bar_y, bar_w * hp_frac.max(0.0), bar_h, 3.0);

        ctx.set_font("10px monospace");
        ctx.set_fill_style_str("#fff");
        ctx.set_text_align("left");
        ctx.set_text_baseline("middle");
        let _ = ctx.fill_text(
            &format!("HP {}/{}", game.player_hp, game.player_max_hp),
            bar_x + 4.0, bar_y + bar_h / 2.0,
        );

        // Stats row below HP bar
        let stat_y = bar_y + bar_h + 4.0;
        ctx.set_font("9px monospace");
        ctx.set_fill_style_str("#8cf");
        let atk = game.effective_attack();
        let def = game.effective_defense();
        let _ = ctx.fill_text(
            &format!("ATK {} DEF {} LVL {}", atk, def, game.player_level),
            bar_x, stat_y,
        );

        // Location — right side
        ctx.set_text_align("right");
        ctx.set_fill_style_str("#ccc");
        ctx.set_font("10px monospace");
        let _ = ctx.fill_text(&game.location_name(), canvas_w - pad, bar_y + bar_h / 2.0);

        // XP progress — right side, below location
        let xp_needed = game.xp_to_next_level();
        ctx.set_font("9px monospace");
        ctx.set_fill_style_str("#a8f");
        let _ = ctx.fill_text(
            &format!("XP {}/{}", game.player_xp, xp_needed),
            canvas_w - pad, stat_y,
        );
    }

    // ---- Tile detail strip (below top bar, shown when inspecting) ----

    fn draw_tile_detail(&self, game: &Game, canvas_w: f64) {
        let info = match &game.inspected {
            Some(info) => info,
            None => return,
        };
        let ctx = &self.ctx;
        let y0 = TOP_BAR_H;
        let pad = 8.0;

        // Background
        ctx.set_fill_style_str("rgba(0,0,0,0.8)");
        ctx.fill_rect(0.0, y0, canvas_w, DETAIL_STRIP_H);
        // Thin accent line at top
        ctx.set_fill_style_str("rgba(0,180,255,0.6)");
        ctx.fill_rect(0.0, y0, canvas_w, 1.0);

        let icon_size = 28.0;
        let text_x = pad + icon_size + 8.0;
        let line1_y = y0 + 10.0;
        let line2_y = y0 + 26.0;
        let line3_y = y0 + 40.0;

        // Enemy info takes priority display
        if let Some(ref ei) = info.enemy {
            // Draw enemy sprite
            let e_glyph = game.enemies.iter()
                .find(|e| e.name == ei.name && e.hp > 0)
                .map(|e| e.glyph)
                .unwrap_or('?');
            let sprite = sprites::enemy_sprite(e_glyph);
            self.draw_sprite(sprite, pad, y0 + (DETAIL_STRIP_H - icon_size) / 2.0, icon_size, icon_size);

            // Name + HP
            ctx.set_font("12px monospace");
            ctx.set_fill_style_str("#f88");
            ctx.set_text_align("left");
            ctx.set_text_baseline("top");
            let _ = ctx.fill_text(ei.name, text_x, line1_y);

            // Stats
            ctx.set_font("10px monospace");
            ctx.set_fill_style_str("#ccc");
            let _ = ctx.fill_text(
                &format!("HP {} ATK {}", ei.hp, ei.attack),
                text_x, line2_y,
            );

            // Description
            ctx.set_fill_style_str("#999");
            let _ = ctx.fill_text(ei.desc, text_x, line3_y);
        } else if let Some(ref ii) = info.item {
            // Draw item sprite
            if let Some(gi) = game.ground_items.iter().find(|gi| gi.item.name == ii.name) {
                let sprite = sprites::item_sprite(gi.item.name);
                self.draw_sprite(sprite, pad, y0 + (DETAIL_STRIP_H - icon_size) / 2.0, icon_size, icon_size);
            }

            ctx.set_font("12px monospace");
            ctx.set_fill_style_str("#ff0");
            ctx.set_text_align("left");
            ctx.set_text_baseline("top");
            let _ = ctx.fill_text(ii.name, text_x, line1_y);

            ctx.set_font("10px monospace");
            ctx.set_fill_style_str("#ccc");
            let _ = ctx.fill_text(&ii.desc, text_x, line2_y);
        } else if info.is_player {
            // Player sprite
            let sprite = sprites::player_sprite();
            self.draw_sprite(sprite, pad, y0 + (DETAIL_STRIP_H - icon_size) / 2.0, icon_size, icon_size);

            ctx.set_font("12px monospace");
            ctx.set_fill_style_str("#fff");
            ctx.set_text_align("left");
            ctx.set_text_baseline("top");
            let _ = ctx.fill_text("You", text_x, line1_y);

            ctx.set_font("10px monospace");
            ctx.set_fill_style_str("#ccc");
            let _ = ctx.fill_text(
                &format!("HP {}/{} ATK {} DEF {}",
                    game.player_hp, game.player_max_hp,
                    game.effective_attack(), game.effective_defense()),
                text_x, line2_y,
            );
        } else {
            // Tile info
            ctx.set_font("12px monospace");
            ctx.set_fill_style_str("#ccc");
            ctx.set_text_align("left");
            ctx.set_text_baseline("top");
            let _ = ctx.fill_text(info.tile_name, text_x - icon_size - 4.0, line1_y);

            ctx.set_font("10px monospace");
            ctx.set_fill_style_str("#888");
            let _ = ctx.fill_text(info.tile_desc, text_x - icon_size - 4.0, line2_y);
        }
    }

    // ---- Messages (above bottom bar) ----

    fn draw_messages(&self, game: &Game, canvas_w: f64, canvas_h: f64) {
        let ctx = &self.ctx;
        let msg_bottom = canvas_h - BOTTOM_BAR_H;
        let msg_top = msg_bottom - MSG_AREA_H;

        // Semi-transparent message background
        ctx.set_fill_style_str("rgba(0,0,0,0.5)");
        ctx.fill_rect(0.0, msg_top, canvas_w, MSG_AREA_H);

        let msg_count = game.messages.len();
        let show = msg_count.min(3);
        ctx.set_font("10px monospace");
        ctx.set_text_align("left");
        ctx.set_text_baseline("bottom");
        for i in 0..show {
            let msg = &game.messages[msg_count - show + i];
            let alpha = if i == show - 1 { "ddd" }
                else if i == show - 2 { "999" }
                else { "666" };
            ctx.set_fill_style_str(&format!("#{alpha}"));
            let y = msg_bottom - (show - 1 - i) as f64 * 13.0 - 4.0;
            let _ = ctx.fill_text(msg, 8.0, y);
        }
    }

    // ---- Bottom action bar ----

    fn draw_bottom_bar(&self, game: &Game, canvas_w: f64, canvas_h: f64) {
        let ctx = &self.ctx;
        let bar_y = canvas_h - BOTTOM_BAR_H;

        // Solid dark background
        ctx.set_fill_style_str("rgba(12,12,18,0.92)");
        ctx.fill_rect(0.0, bar_y, canvas_w, BOTTOM_BAR_H);
        // Top edge line
        ctx.set_fill_style_str("rgba(255,255,255,0.08)");
        ctx.fill_rect(0.0, bar_y, canvas_w, 1.0);

        // Button layout: evenly spaced across width
        let btn_count = 3.0;
        let btn_w = (canvas_w / btn_count).min(140.0);
        let btn_h = 30.0;
        let btn_y = bar_y + (BOTTOM_BAR_H - btn_h) / 2.0;
        let total_w = btn_w * btn_count;
        let start_x = (canvas_w - total_w) / 2.0;

        let buttons: [(&str, Drawer, &str); 3] = [
            ("Inventory", Drawer::Inventory, "#58f"),
            ("Stats", Drawer::Stats, "#a8f"),
            ("Menu", Drawer::None, "#888"),
        ];

        for (i, (label, drawer, color)) in buttons.iter().enumerate() {
            let bx = start_x + i as f64 * btn_w + 4.0;
            let bw = btn_w - 8.0;

            // Highlight if this drawer is open
            if game.drawer == *drawer && *drawer != Drawer::None {
                ctx.set_fill_style_str("rgba(255,255,255,0.1)");
                self.fill_rounded_rect(bx, btn_y, bw, btn_h, 6.0);
            }

            ctx.set_font("11px monospace");
            ctx.set_fill_style_str(color);
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");
            let _ = ctx.fill_text(label, bx + bw / 2.0, btn_y + btn_h / 2.0);
        }
    }

    // ---- Drawer: slide-up panel ----

    fn draw_drawer(&self, game: &Game, canvas_w: f64, canvas_h: f64) {
        match game.drawer {
            Drawer::None => return,
            Drawer::Inventory => self.draw_inventory_drawer(game, canvas_w, canvas_h),
            Drawer::Stats => self.draw_stats_drawer(game, canvas_w, canvas_h),
        }
    }

    fn draw_inventory_drawer(&self, game: &Game, canvas_w: f64, canvas_h: f64) {
        let ctx = &self.ctx;
        let drawer_h = canvas_h * 0.55;
        let drawer_y = canvas_h - BOTTOM_BAR_H - drawer_h;
        let pad = 12.0;

        // Background
        ctx.set_fill_style_str("rgba(8,8,16,0.94)");
        self.fill_rounded_rect(0.0, drawer_y, canvas_w, drawer_h, 12.0);
        // Top accent
        ctx.set_fill_style_str("rgba(80,130,255,0.3)");
        self.fill_rounded_rect(canvas_w * 0.3, drawer_y, canvas_w * 0.4, 3.0, 1.5);

        // Title
        ctx.set_font("13px monospace");
        ctx.set_fill_style_str("#fff");
        ctx.set_text_align("center");
        ctx.set_text_baseline("top");
        let _ = ctx.fill_text("INVENTORY", canvas_w / 2.0, drawer_y + 10.0);

        // Equipment slots
        let eq_y = drawer_y + 30.0;
        ctx.set_font("10px monospace");
        ctx.set_text_align("left");
        ctx.set_text_baseline("top");

        // Weapon slot
        ctx.set_fill_style_str("rgba(255,255,255,0.06)");
        self.fill_rounded_rect(pad, eq_y, canvas_w / 2.0 - pad * 1.5, 32.0, 4.0);
        let wep_icon_x = pad + 4.0;
        if let Some(ref w) = game.equipped_weapon {
            let sprite = sprites::item_sprite(w.name);
            self.draw_sprite(sprite, wep_icon_x, eq_y + 4.0, 24.0, 24.0);
            ctx.set_fill_style_str("#8af");
            let _ = ctx.fill_text(w.name, wep_icon_x + 28.0, eq_y + 10.0);
        } else {
            ctx.set_fill_style_str("#555");
            let _ = ctx.fill_text("No weapon", wep_icon_x + 4.0, eq_y + 10.0);
        }

        // Armor slot
        let armor_x = canvas_w / 2.0 + pad * 0.5;
        ctx.set_fill_style_str("rgba(255,255,255,0.06)");
        self.fill_rounded_rect(armor_x, eq_y, canvas_w / 2.0 - pad * 1.5, 32.0, 4.0);
        if let Some(ref a) = game.equipped_armor {
            let sprite = sprites::item_sprite(a.name);
            self.draw_sprite(sprite, armor_x + 4.0, eq_y + 4.0, 24.0, 24.0);
            ctx.set_fill_style_str("#afa");
            let _ = ctx.fill_text(a.name, armor_x + 32.0, eq_y + 10.0);
        } else {
            ctx.set_fill_style_str("#555");
            let _ = ctx.fill_text("No armor", armor_x + 4.0, eq_y + 10.0);
        }

        // Item list
        let list_y = eq_y + 40.0;
        let slot_h = 30.0;
        let icon_size = 24.0;

        if game.inventory.is_empty() {
            ctx.set_fill_style_str("#555");
            ctx.set_font("10px monospace");
            ctx.set_text_baseline("top");
            let _ = ctx.fill_text("No items", pad + 4.0, list_y + 4.0);
        } else {
            for (i, item) in game.inventory.iter().enumerate() {
                let iy = list_y + i as f64 * slot_h;
                if iy + slot_h > drawer_y + drawer_h - 4.0 { break; } // clip

                // Subtle row background
                if i % 2 == 0 {
                    ctx.set_fill_style_str("rgba(255,255,255,0.03)");
                    ctx.fill_rect(pad, iy, canvas_w - pad * 2.0, slot_h);
                }

                // Icon
                let sprite = sprites::item_sprite(item.name);
                self.draw_sprite(sprite, pad + 4.0, iy + (slot_h - icon_size) / 2.0, icon_size, icon_size);

                // Name
                let color = match item.kind {
                    ItemKind::Potion => "#f88",
                    ItemKind::Scroll => "#88f",
                    ItemKind::Weapon => "#aaf",
                    ItemKind::Armor => "#afa",
                };
                ctx.set_font("11px monospace");
                ctx.set_fill_style_str(color);
                ctx.set_text_baseline("middle");
                let _ = ctx.fill_text(item.name, pad + 32.0, iy + slot_h / 2.0);

                // Effect hint on right
                let hint = match &item.effect {
                    crate::game::ItemEffect::Heal(n) => format!("+{} HP", n),
                    crate::game::ItemEffect::DamageAoe(n) => format!("{} DMG", n),
                    crate::game::ItemEffect::BuffAttack(n) => format!("+{} ATK", n),
                    crate::game::ItemEffect::BuffDefense(n) => format!("+{} DEF", n),
                };
                ctx.set_text_align("right");
                ctx.set_fill_style_str("#888");
                ctx.set_font("9px monospace");
                let _ = ctx.fill_text(&hint, canvas_w - pad - 4.0, iy + slot_h / 2.0);
                ctx.set_text_align("left");
            }
        }

        // Slot count
        ctx.set_font("9px monospace");
        ctx.set_fill_style_str("#555");
        ctx.set_text_align("right");
        ctx.set_text_baseline("bottom");
        let _ = ctx.fill_text(
            &format!("{}/10", game.inventory.len()),
            canvas_w - pad, drawer_y + drawer_h - 6.0,
        );
    }

    fn draw_stats_drawer(&self, game: &Game, canvas_w: f64, canvas_h: f64) {
        let ctx = &self.ctx;
        let drawer_h = canvas_h * 0.45;
        let drawer_y = canvas_h - BOTTOM_BAR_H - drawer_h;
        let pad = 12.0;

        // Background
        ctx.set_fill_style_str("rgba(8,8,16,0.94)");
        self.fill_rounded_rect(0.0, drawer_y, canvas_w, drawer_h, 12.0);
        ctx.set_fill_style_str("rgba(160,80,255,0.3)");
        self.fill_rounded_rect(canvas_w * 0.3, drawer_y, canvas_w * 0.4, 3.0, 1.5);

        // Title
        ctx.set_font("13px monospace");
        ctx.set_fill_style_str("#fff");
        ctx.set_text_align("center");
        ctx.set_text_baseline("top");
        let _ = ctx.fill_text("CHARACTER", canvas_w / 2.0, drawer_y + 10.0);

        let mut y = drawer_y + 34.0;
        let line_h = 22.0;

        // Player sprite + level
        let sprite = sprites::player_sprite();
        self.draw_sprite(sprite, pad, y, 32.0, 32.0);
        ctx.set_font("12px monospace");
        ctx.set_fill_style_str("#fff");
        ctx.set_text_align("left");
        ctx.set_text_baseline("top");
        let _ = ctx.fill_text(&format!("Level {}", game.player_level), pad + 40.0, y + 2.0);
        ctx.set_font("10px monospace");
        ctx.set_fill_style_str("#a8f");
        let _ = ctx.fill_text(
            &format!("XP {}/{}", game.player_xp, game.xp_to_next_level()),
            pad + 40.0, y + 18.0,
        );

        y += 42.0;

        // XP progress bar
        let xp_bar_w = canvas_w - pad * 2.0;
        let xp_frac = if game.xp_to_next_level() > 0 {
            game.player_xp as f64 / game.xp_to_next_level() as f64
        } else { 1.0 };
        ctx.set_fill_style_str("#1a0a2a");
        self.fill_rounded_rect(pad, y, xp_bar_w, 8.0, 3.0);
        ctx.set_fill_style_str("#a6f");
        self.fill_rounded_rect(pad, y, xp_bar_w * xp_frac, 8.0, 3.0);

        y += 20.0;

        // Stat rows
        let stats: [(&str, String, &str); 4] = [
            ("HP", format!("{} / {}", game.player_hp, game.player_max_hp), "#4c4"),
            ("Attack", format!("{}", game.effective_attack()), "#8cf"),
            ("Defense", format!("{}", game.effective_defense()), "#fc8"),
            ("Location", game.location_name(), "#ccc"),
        ];

        for (label, value, color) in &stats {
            ctx.set_font("10px monospace");
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
        y += 6.0;
        ctx.set_font("10px monospace");
        ctx.set_fill_style_str("#666");
        ctx.set_text_align("left");
        let _ = ctx.fill_text("Equipment", pad, y);
        y += 16.0;

        if let Some(ref w) = game.equipped_weapon {
            let sprite = sprites::item_sprite(w.name);
            self.draw_sprite(sprite, pad, y, 20.0, 20.0);
            ctx.set_fill_style_str("#8af");
            ctx.set_text_baseline("middle");
            let _ = ctx.fill_text(w.name, pad + 24.0, y + 10.0);
        } else {
            ctx.set_fill_style_str("#444");
            ctx.set_text_baseline("top");
            let _ = ctx.fill_text("- No weapon", pad, y);
        }
        y += 24.0;

        if let Some(ref a) = game.equipped_armor {
            let sprite = sprites::item_sprite(a.name);
            self.draw_sprite(sprite, pad, y, 20.0, 20.0);
            ctx.set_fill_style_str("#afa");
            ctx.set_text_baseline("middle");
            let _ = ctx.fill_text(a.name, pad + 24.0, y + 10.0);
        } else {
            ctx.set_fill_style_str("#444");
            ctx.set_text_baseline("top");
            let _ = ctx.fill_text("- No armor", pad, y);
        }
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
