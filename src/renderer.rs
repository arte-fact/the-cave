use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::camera::Camera;
use crate::game::{BumpAnim, Drawer, EffectKind, Game, Item, ItemKind};
use crate::map::{Tile, Visibility};
use crate::sprites::{self, Sheet, SpriteRef};

/// Loaded sprite sheet images, indexed by Sheet enum.
pub struct SpriteSheets {
    pub tiles: HtmlImageElement,
    pub monsters: HtmlImageElement,
    pub rogues: HtmlImageElement,
    pub items: HtmlImageElement,
    pub animals: Option<HtmlImageElement>,
}

impl SpriteSheets {
    fn get(&self, sheet: Sheet) -> Option<&HtmlImageElement> {
        match sheet {
            Sheet::Tiles => Some(&self.tiles),
            Sheet::Monsters => Some(&self.monsters),
            Sheet::Rogues => Some(&self.rogues),
            Sheet::Items => Some(&self.items),
            Sheet::Animals => self.animals.as_ref(),
            // AnimatedTiles sheet not yet loaded
            _ => Some(&self.tiles),
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

/// Width of the right-side panel in landscape mode (CSS pixels).
const SIDE_PANEL_CSS_W: f64 = 220.0;

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
    /// When true, render ASCII glyphs instead of sprite sheets.
    pub glyph_mode: bool,
    /// True when the canvas is in landscape orientation (width > height).
    pub landscape: bool,
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
            glyph_mode: false,
            landscape: false,
        }
    }

    pub fn set_sheets(&mut self, sheets: SpriteSheets) {
        self.sheets = Some(sheets);
    }

    /// Set the optional animals sprite sheet after initial load.
    pub fn set_animals_sheet(&mut self, img: HtmlImageElement) {
        if let Some(sheets) = &mut self.sheets {
            sheets.animals = Some(img);
        }
    }

    pub fn resize(&mut self, width: f64, height: f64, dpr: f64) {
        self.dpr = dpr;
        // Detect landscape: canvas is wider than tall (typical mobile landscape).
        self.landscape = width > height;
        let cell = self.camera.set_viewport(width, height);
        if self.landscape {
            // In landscape: no top/bottom bars, pad right for side panel.
            self.camera.pad_top = 0.0;
            self.camera.pad_bottom = 0.0;
            self.camera.pad_right = self.side_panel_w() / cell / 2.0;
        } else {
            // Portrait: original top/bottom bar layout.
            self.camera.pad_top = (self.top_bar_h() + self.detail_strip_h()) / cell;
            self.camera.pad_bottom = (self.bottom_bar_h() + self.msg_area_h()) / cell;
            self.camera.pad_right = 0.0;
        }
    }

    /// Side panel width in canvas pixels (landscape mode).
    fn side_panel_w(&self) -> f64 {
        SIDE_PANEL_CSS_W * self.dpr
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
            if let Some(img) = sheets.get(sprite.sheet) {
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
                return true;
            }
        }
        false
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

        if self.landscape {
            // Landscape: full-height game view + permanent side panel on right
            let panel_w = self.side_panel_w();
            let panel_x = canvas_w - panel_w;
            self.draw_side_panel(game, panel_x, panel_w, canvas_h);
        } else {
            // Portrait: original top/bottom bar layout
            let top_h = self.top_bar_h();
            let bottom_h = self.bottom_bar_h();
            let msg_h = self.msg_area_h();
            self.draw_top_bar(game, canvas_w, top_h);
            self.draw_tile_detail(game, canvas_w, top_h);
            self.draw_messages(game, canvas_w, canvas_h, bottom_h, msg_h);
            self.draw_bottom_bar(game, canvas_w, canvas_h, bottom_h);
            self.draw_drawer(game, canvas_w, canvas_h, bottom_h);
        }

        // Death / Victory overlay (on top of everything)
        if !game.alive || game.won {
            self.draw_end_overlay(game, canvas_w, canvas_h);
        }
    }

    // ---- Landscape side panel (permanent right drawer) ----

    fn draw_side_panel(&self, game: &Game, panel_x: f64, panel_w: f64, canvas_h: f64) {
        let ctx = &self.ctx;
        let d = self.dpr;
        let pad = 10.0 * d;
        let inner_w = panel_w - pad * 2.0;

        // Panel background
        ctx.set_fill_style_str("rgba(8,8,16,0.94)");
        ctx.fill_rect(panel_x, 0.0, panel_w, canvas_h);
        // Left border accent
        ctx.set_fill_style_str("rgba(80,130,255,0.2)");
        ctx.fill_rect(panel_x, 0.0, 1.0 * d, canvas_h);

        let mut y = pad;
        let x = panel_x + pad;
        let bar_h = 10.0 * d;
        let bar_gap = 4.0 * d;
        let bar_r = 3.0 * d;
        let text_inset = 4.0 * d;

        // --- Location name ---
        ctx.set_font(&self.font(11.0, "bold"));
        ctx.set_fill_style_str("#c8e0ff");
        ctx.set_text_align("center");
        ctx.set_text_baseline("top");
        let _ = ctx.fill_text(&game.location_name(), panel_x + panel_w / 2.0, y);
        y += 16.0 * d;

        // --- HP bar ---
        let hp_frac = game.player_hp as f64 / game.player_max_hp as f64;
        ctx.set_fill_style_str("#2a0a0a");
        self.fill_rounded_rect(x, y, inner_w, bar_h, bar_r);
        let hp_color = if hp_frac > 0.5 { "#2a2" }
            else if hp_frac > 0.25 { "#aa2" }
            else { "#a22" };
        ctx.set_fill_style_str(hp_color);
        self.fill_rounded_rect(x, y, inner_w * hp_frac.max(0.0), bar_h, bar_r);
        ctx.set_font(&self.font(8.0, "bold"));
        ctx.set_fill_style_str("#fff");
        ctx.set_text_align("left");
        ctx.set_text_baseline("middle");
        let _ = ctx.fill_text(
            &format!("HP {}/{}", game.player_hp, game.player_max_hp),
            x + text_inset, y + bar_h / 2.0,
        );
        y += bar_h + bar_gap;

        // --- Stamina bar ---
        let stam_frac = game.stamina as f64 / game.max_stamina as f64;
        ctx.set_fill_style_str("#0a0a2a");
        self.fill_rounded_rect(x, y, inner_w, bar_h, bar_r);
        let stam_color = if game.sprinting { "#4af" } else { "#28a" };
        ctx.set_fill_style_str(stam_color);
        self.fill_rounded_rect(x, y, inner_w * stam_frac.max(0.0), bar_h, bar_r);
        ctx.set_font(&self.font(8.0, ""));
        ctx.set_fill_style_str("#fff");
        let sprint_label = if game.sprinting { "STA (SPRINT)" } else { "STA" };
        let _ = ctx.fill_text(
            &format!("{} {}/{}", sprint_label, game.stamina, game.max_stamina),
            x + text_inset, y + bar_h / 2.0,
        );
        y += bar_h + bar_gap;

        // --- Hunger bar ---
        let hunger_frac = game.hunger as f64 / game.max_hunger as f64;
        ctx.set_fill_style_str("#2a1a0a");
        self.fill_rounded_rect(x, y, inner_w, bar_h, bar_r);
        let hunger_color = if hunger_frac > 0.3 { "#a82" }
            else if hunger_frac > 0.1 { "#a52" }
            else { "#a22" };
        ctx.set_fill_style_str(hunger_color);
        self.fill_rounded_rect(x, y, inner_w * hunger_frac.max(0.0), bar_h, bar_r);
        ctx.set_font(&self.font(8.0, ""));
        ctx.set_fill_style_str("#fff");
        let _ = ctx.fill_text(
            &format!("FOOD {}/{}", game.hunger, game.max_hunger),
            x + text_inset, y + bar_h / 2.0,
        );
        y += bar_h + bar_gap * 2.0;

        // --- Combat stats ---
        let atk = game.effective_attack();
        let def = game.effective_defense();
        ctx.set_font(&self.font(9.0, ""));
        ctx.set_fill_style_str("#8cf");
        ctx.set_text_align("left");
        ctx.set_text_baseline("top");
        let _ = ctx.fill_text(
            &format!("ATK {} DEF {} LVL {}", atk, def, game.player_level),
            x, y,
        );
        y += 14.0 * d;

        let xp_needed = game.xp_to_next_level();
        ctx.set_fill_style_str("#a8f");
        let _ = ctx.fill_text(&format!("XP {}/{}", game.player_xp, xp_needed), x, y);
        y += 18.0 * d;

        // --- Separator ---
        ctx.set_fill_style_str("rgba(255,255,255,0.08)");
        ctx.fill_rect(x, y, inner_w, 1.0 * d);
        y += 6.0 * d;

        // --- Tile detail (if inspecting) ---
        if let Some(ref info) = game.inspected {
            let detail_h = 44.0 * d;
            ctx.set_fill_style_str("rgba(0,180,255,0.08)");
            self.fill_rounded_rect(x, y, inner_w, detail_h, 4.0 * d);

            let icon_size = 24.0 * d;
            let text_x = x + icon_size + 6.0 * d;

            if let Some(ref ei) = info.enemy {
                let e_glyph = game.enemies.iter()
                    .find(|e| e.name == ei.name && e.hp > 0)
                    .map(|e| e.glyph)
                    .unwrap_or('?');
                let sprite = sprites::enemy_sprite(e_glyph);
                self.draw_sprite(sprite, x + 2.0 * d, y + (detail_h - icon_size) / 2.0, icon_size, icon_size);

                ctx.set_font(&self.font(10.0, "bold"));
                ctx.set_fill_style_str("#f88");
                ctx.set_text_align("left");
                ctx.set_text_baseline("top");
                let _ = ctx.fill_text(ei.name, text_x, y + 4.0 * d);
                ctx.set_font(&self.font(8.0, ""));
                ctx.set_fill_style_str("#ccc");
                let _ = ctx.fill_text(&format!("HP {} ATK {}", ei.hp, ei.attack), text_x, y + 18.0 * d);
            } else if let Some(ref ii) = info.item {
                if let Some(gi) = game.ground_items.iter().find(|gi| gi.item.name == ii.name) {
                    let sprite = sprites::item_sprite(gi.item.name);
                    self.draw_sprite(sprite, x + 2.0 * d, y + (detail_h - icon_size) / 2.0, icon_size, icon_size);
                }
                ctx.set_font(&self.font(10.0, "bold"));
                ctx.set_fill_style_str("#ff0");
                ctx.set_text_align("left");
                ctx.set_text_baseline("top");
                let _ = ctx.fill_text(ii.name, text_x, y + 4.0 * d);
                ctx.set_font(&self.font(8.0, ""));
                ctx.set_fill_style_str("#ccc");
                let _ = ctx.fill_text(&ii.desc, text_x, y + 18.0 * d);
            } else if info.is_player {
                ctx.set_font(&self.font(10.0, "bold"));
                ctx.set_fill_style_str("#fff");
                ctx.set_text_align("left");
                ctx.set_text_baseline("top");
                let _ = ctx.fill_text("You", text_x, y + 4.0 * d);
            } else {
                ctx.set_font(&self.font(10.0, "bold"));
                ctx.set_fill_style_str("#ccc");
                ctx.set_text_align("left");
                ctx.set_text_baseline("top");
                let _ = ctx.fill_text(info.tile_name, x + 4.0 * d, y + 4.0 * d);
                ctx.set_font(&self.font(8.0, ""));
                ctx.set_fill_style_str("#888");
                let _ = ctx.fill_text(info.tile_desc, x + 4.0 * d, y + 18.0 * d);
            }
            y += detail_h + 6.0 * d;
        }

        // --- Action buttons (2×2 grid) ---
        let btn_h = 30.0 * d;
        let btn_gap = 4.0 * d;
        let btn_w = (inner_w - btn_gap) / 2.0;

        let sprint_color = if game.sprinting { "#4ef" } else { "#48a" };
        let sprint_label = if game.sprinting { "SPRINT" } else { "Sprint" };

        let buttons: [(&str, &str, bool); 4] = [
            ("Inventory", if game.drawer == Drawer::Inventory { "#8af" } else { "#58f" }, game.drawer == Drawer::Inventory),
            ("Stats", if game.drawer == Drawer::Stats { "#c8f" } else { "#a8f" }, game.drawer == Drawer::Stats),
            (sprint_label, sprint_color, game.sprinting),
            ("Settings", if game.drawer == Drawer::Settings { "#ccc" } else { "#888" }, game.drawer == Drawer::Settings),
        ];

        for (i, (label, color, active)) in buttons.iter().enumerate() {
            let col = i % 2;
            let row = i / 2;
            let bx = x + col as f64 * (btn_w + btn_gap);
            let by = y + row as f64 * (btn_h + btn_gap);

            if *active {
                ctx.set_fill_style_str("rgba(255,255,255,0.12)");
            } else {
                ctx.set_fill_style_str("rgba(255,255,255,0.05)");
            }
            self.fill_rounded_rect(bx, by, btn_w, btn_h, 4.0 * d);

            ctx.set_font(&self.font(10.0, "bold"));
            ctx.set_fill_style_str(color);
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");
            let _ = ctx.fill_text(label, bx + btn_w / 2.0, by + btn_h / 2.0);
        }
        y += (btn_h + btn_gap) * 2.0 + 6.0 * d;

        // --- Separator ---
        ctx.set_fill_style_str("rgba(255,255,255,0.08)");
        ctx.fill_rect(x, y, inner_w, 1.0 * d);
        y += 6.0 * d;

        // --- Drawer content (if open) ---
        let which = if game.drawer != Drawer::None {
            game.drawer
        } else if self.drawer_anim > 0.0 {
            self.last_drawer
        } else {
            Drawer::None
        };

        if which != Drawer::None && self.drawer_anim > 0.0 {
            let remaining_h = canvas_h - y;
            let alpha = self.drawer_anim;
            ctx.save();
            ctx.set_global_alpha(alpha);
            match which {
                Drawer::Inventory => self.draw_side_inventory(game, x, y, inner_w, remaining_h),
                Drawer::Stats => self.draw_side_stats(game, x, y, inner_w, remaining_h),
                Drawer::Settings => self.draw_side_settings(game, x, y, inner_w, remaining_h),
                Drawer::None => {}
            }
            ctx.restore();
            return; // drawer takes remaining space
        }

        // --- Messages (when no drawer open) ---
        let msg_count = game.messages.len();
        let show = msg_count.min(5);
        ctx.set_font(&self.font(8.0, ""));
        ctx.set_text_align("left");
        ctx.set_text_baseline("top");
        let line_h = 12.0 * d;
        for i in 0..show {
            let msg = &game.messages[msg_count - show + i];
            let alpha_val = if i == show - 1 { "#ccc" }
                else if i == show - 2 { "#888" }
                else { "#555" };
            ctx.set_fill_style_str(alpha_val);
            let _ = ctx.fill_text(msg, x, y + i as f64 * line_h);
        }
    }

    /// Draw inventory content inside the landscape side panel.
    fn draw_side_inventory(&self, game: &Game, x: f64, y: f64, w: f64, h: f64) {
        let ctx = &self.ctx;
        let d = self.dpr;
        let pad = 4.0 * d;
        let mut cy = y;

        // Title
        ctx.set_font(&self.font(10.0, "bold"));
        ctx.set_fill_style_str("#fff");
        ctx.set_text_align("center");
        ctx.set_text_baseline("top");
        let _ = ctx.fill_text("INVENTORY", x + w / 2.0, cy);
        cy += 16.0 * d;

        // Equipment slots (compact vertical list)
        let eq_icon = 18.0 * d;
        let eq_h = 22.0 * d;
        let eq_gap = 2.0 * d;
        ctx.set_text_align("left");
        ctx.set_text_baseline("middle");

        let eq_slots: [(&Option<Item>, &str, &str); 6] = [
            (&game.equipped_weapon,  "#8af", "No weapon"),
            (&game.equipped_armor,   "#afa", "No armor"),
            (&game.equipped_helmet,  "#fc8", "No helmet"),
            (&game.equipped_shield,  "#adf", "No shield"),
            (&game.equipped_boots,   "#da8", "No boots"),
            (&game.equipped_ring,    "#ff8", "No ring"),
        ];
        for &(slot, color, empty_label) in &eq_slots {
            ctx.set_fill_style_str("rgba(255,255,255,0.04)");
            self.fill_rounded_rect(x, cy, w, eq_h, 3.0 * d);
            if let Some(ref item) = slot {
                let sprite = sprites::item_sprite(item.name);
                self.draw_sprite(sprite, x + pad, cy + (eq_h - eq_icon) / 2.0, eq_icon, eq_icon);
                ctx.set_font(&self.font(8.0, ""));
                ctx.set_fill_style_str(color);
                let _ = ctx.fill_text(item.name, x + eq_icon + pad * 2.0, cy + eq_h / 2.0);
            } else {
                ctx.set_font(&self.font(8.0, ""));
                ctx.set_fill_style_str("#444");
                let _ = ctx.fill_text(empty_label, x + pad, cy + eq_h / 2.0);
            }
            cy += eq_h + eq_gap;
        }

        cy += 4.0 * d;

        // Item list
        let slot_h = 26.0 * d;
        let icon_size = 20.0 * d;
        let avail_h = y + h - cy - 16.0 * d;
        let max_visible = (avail_h / slot_h).floor().max(1.0) as usize;

        if game.inventory.is_empty() {
            ctx.set_fill_style_str("#555");
            ctx.set_font(&self.font(9.0, ""));
            ctx.set_text_baseline("top");
            let _ = ctx.fill_text("No items", x + pad, cy);
        } else {
            let scroll = game.inventory_scroll;
            let total = game.inventory.len();
            let end = (scroll + max_visible).min(total);

            for (vi, idx) in (scroll..end).enumerate() {
                let item = &game.inventory[idx];
                let iy = cy + vi as f64 * slot_h;

                if game.selected_inventory_item == Some(idx) {
                    ctx.set_fill_style_str("rgba(80,130,255,0.18)");
                    ctx.fill_rect(x, iy, w, slot_h);
                } else if vi % 2 == 0 {
                    ctx.set_fill_style_str("rgba(255,255,255,0.03)");
                    ctx.fill_rect(x, iy, w, slot_h);
                }

                let sprite = sprites::item_sprite(item.name);
                self.draw_sprite(sprite, x + pad, iy + (slot_h - icon_size) / 2.0, icon_size, icon_size);

                let color = match item.kind {
                    ItemKind::Potion => "#f88",
                    ItemKind::Scroll => "#88f",
                    ItemKind::Weapon => "#aaf",
                    ItemKind::RangedWeapon => "#8df",
                    ItemKind::Armor => "#afa",
                    ItemKind::Helmet => "#fc8",
                    ItemKind::Shield => "#adf",
                    ItemKind::Boots => "#da8",
                    ItemKind::Food => "#fa8",
                    ItemKind::Ring => "#ff8",
                };
                ctx.set_font(&self.font(8.0, ""));
                ctx.set_fill_style_str(color);
                ctx.set_text_baseline("middle");
                let _ = ctx.fill_text(item.name, x + icon_size + pad * 2.0, iy + slot_h / 2.0);

                // Inline action buttons
                let btn_w = 24.0 * d;
                let btn_h = 16.0 * d;
                let btn_y = iy + (slot_h - btn_h) / 2.0;
                let drop_x = x + w - btn_w - 2.0 * d;
                let use_x = drop_x - btn_w - 2.0 * d;

                let action_label = match item.kind {
                    ItemKind::Potion | ItemKind::Scroll | ItemKind::Food => "U",
                    _ => "E",
                };
                ctx.set_fill_style_str("rgba(80,200,120,0.2)");
                self.fill_rounded_rect(use_x, btn_y, btn_w, btn_h, 2.0 * d);
                ctx.set_font(&self.font(7.0, "bold"));
                ctx.set_fill_style_str("#8f8");
                ctx.set_text_align("center");
                let _ = ctx.fill_text(action_label, use_x + btn_w / 2.0, iy + slot_h / 2.0);

                ctx.set_fill_style_str("rgba(200,80,80,0.2)");
                self.fill_rounded_rect(drop_x, btn_y, btn_w, btn_h, 2.0 * d);
                ctx.set_fill_style_str("#f88");
                let _ = ctx.fill_text("X", drop_x + btn_w / 2.0, iy + slot_h / 2.0);

                ctx.set_text_align("left");
            }

            // Slot count
            ctx.set_font(&self.font(8.0, ""));
            ctx.set_fill_style_str("#555");
            ctx.set_text_align("right");
            ctx.set_text_baseline("bottom");
            let _ = ctx.fill_text(&format!("{}/10", total), x + w, y + h - 2.0 * d);
        }
    }

    /// Draw stats content inside the landscape side panel.
    fn draw_side_stats(&self, game: &Game, x: f64, y: f64, w: f64, h: f64) {
        let ctx = &self.ctx;
        let d = self.dpr;
        let mut cy = y;

        // Title
        ctx.set_font(&self.font(10.0, "bold"));
        ctx.set_fill_style_str("#fff");
        ctx.set_text_align("center");
        ctx.set_text_baseline("top");
        let _ = ctx.fill_text("CHARACTER", x + w / 2.0, cy);
        cy += 16.0 * d;

        // Player sprite + level
        let icon_sz = 28.0 * d;
        let sprite = sprites::player_sprite();
        self.draw_sprite(sprite, x, cy, icon_sz, icon_sz);
        ctx.set_font(&self.font(10.0, "bold"));
        ctx.set_fill_style_str("#fff");
        ctx.set_text_align("left");
        ctx.set_text_baseline("top");
        let _ = ctx.fill_text(&format!("Level {}", game.player_level), x + icon_sz + 6.0 * d, cy + 2.0 * d);
        ctx.set_font(&self.font(8.0, ""));
        ctx.set_fill_style_str("#a8f");
        let _ = ctx.fill_text(
            &format!("XP {}/{}", game.player_xp, game.xp_to_next_level()),
            x + icon_sz + 6.0 * d, cy + 14.0 * d,
        );
        cy += icon_sz + 8.0 * d;

        // Stat rows
        let line_h = 18.0 * d;
        let mut stats: Vec<(&str, String, &str)> = vec![
            ("HP", format!("{}/{}", game.player_hp, game.player_max_hp), "#4c4"),
            ("Attack", format!("{}", game.effective_attack()), "#8cf"),
            ("Defense", format!("{}", game.effective_defense()), "#fc8"),
            ("Dexterity", format!("{}", game.player_dexterity), "#adf"),
        ];
        if game.has_ranged_weapon() {
            stats.push(("Range", format!("{}", game.ranged_max_range()), "#fa8"));
        }

        for (label, value, color) in &stats {
            ctx.set_font(&self.font(9.0, ""));
            ctx.set_fill_style_str("#888");
            ctx.set_text_align("left");
            ctx.set_text_baseline("top");
            let _ = ctx.fill_text(label, x, cy);
            ctx.set_fill_style_str(color);
            ctx.set_text_align("right");
            let _ = ctx.fill_text(value, x + w, cy);
            cy += line_h;
        }

        cy += 6.0 * d;

        // Skill points
        if game.skill_points > 0 {
            ctx.set_font(&self.font(9.0, "bold"));
            ctx.set_fill_style_str("#ff0");
            ctx.set_text_align("left");
            ctx.set_text_baseline("top");
            let _ = ctx.fill_text(&format!("{} skill pts", game.skill_points), x, cy);
            cy += 14.0 * d;

            let skill_row_h = 24.0 * d;
            let btn_sz = 18.0 * d;
            let skills: [(&str, &str, String, &str); 4] = [
                ("STR", "Strength", format!("{}", game.strength), "#f84"),
                ("VIT", "Vitality", format!("{}", game.vitality), "#4f4"),
                ("DEX", "Dexterity", format!("{}", game.player_dexterity), "#adf"),
                ("STA", "Stamina", format!("{}", game.max_stamina), "#8df"),
            ];
            for (abbr, _label, value, color) in &skills {
                ctx.set_font(&self.font(8.0, "bold"));
                ctx.set_fill_style_str(color);
                ctx.set_text_align("left");
                ctx.set_text_baseline("middle");
                let row_mid = cy + skill_row_h / 2.0;
                let _ = ctx.fill_text(abbr, x, row_mid);

                ctx.set_font(&self.font(9.0, "bold"));
                ctx.set_text_align("right");
                let _ = ctx.fill_text(value, x + w - btn_sz - 6.0 * d, row_mid);

                // + button
                let btn_x = x + w - btn_sz;
                let btn_y = cy + (skill_row_h - btn_sz) / 2.0;
                ctx.set_fill_style_str("rgba(100,200,100,0.25)");
                self.fill_rounded_rect(btn_x, btn_y, btn_sz, btn_sz, 3.0 * d);
                ctx.set_font(&self.font(12.0, "bold"));
                ctx.set_fill_style_str("#4f4");
                ctx.set_text_align("center");
                let _ = ctx.fill_text("+", btn_x + btn_sz / 2.0, cy + skill_row_h / 2.0);

                cy += skill_row_h;
            }
        }

        cy += 6.0 * d;

        // Equipment
        ctx.set_font(&self.font(9.0, "bold"));
        ctx.set_fill_style_str("#666");
        ctx.set_text_align("left");
        ctx.set_text_baseline("top");
        let _ = ctx.fill_text("Equipment", x, cy);
        cy += 14.0 * d;

        let eq_icon = 18.0 * d;
        let eq_slots: [(&Option<Item>, &str, &str); 6] = [
            (&game.equipped_weapon,  "#8af", "- No weapon"),
            (&game.equipped_armor,   "#afa", "- No armor"),
            (&game.equipped_helmet,  "#fc8", "- No helmet"),
            (&game.equipped_shield,  "#adf", "- No shield"),
            (&game.equipped_boots,   "#da8", "- No boots"),
            (&game.equipped_ring,    "#ff8", "- No ring"),
        ];
        for &(slot, color, empty_label) in &eq_slots {
            if cy + eq_icon > y + h { break; }
            if let Some(ref item) = slot {
                let sprite = sprites::item_sprite(item.name);
                self.draw_sprite(sprite, x, cy, eq_icon, eq_icon);
                ctx.set_font(&self.font(8.0, ""));
                ctx.set_fill_style_str(color);
                ctx.set_text_baseline("middle");
                let _ = ctx.fill_text(item.name, x + eq_icon + 4.0 * d, cy + eq_icon / 2.0);
            } else {
                ctx.set_font(&self.font(8.0, ""));
                ctx.set_fill_style_str("#444");
                ctx.set_text_baseline("top");
                let _ = ctx.fill_text(empty_label, x, cy);
            }
            cy += eq_icon + 4.0 * d;
        }
    }

    /// Draw settings content inside the landscape side panel.
    fn draw_side_settings(&self, game: &Game, x: f64, y: f64, w: f64, _h: f64) {
        let ctx = &self.ctx;
        let d = self.dpr;
        let mut cy = y;

        // Title
        ctx.set_font(&self.font(10.0, "bold"));
        ctx.set_fill_style_str("#fff");
        ctx.set_text_align("center");
        ctx.set_text_baseline("top");
        let _ = ctx.fill_text("SETTINGS", x + w / 2.0, cy);
        cy += 18.0 * d;

        // Glyph mode toggle
        ctx.set_font(&self.font(10.0, ""));
        ctx.set_fill_style_str("#ccc");
        ctx.set_text_align("left");
        ctx.set_text_baseline("middle");
        let row_h = 30.0 * d;
        let _ = ctx.fill_text("Glyph Mode", x, cy + row_h / 2.0);

        let toggle_w = 50.0 * d;
        let toggle_h = 22.0 * d;
        let toggle_x = x + w - toggle_w;
        let toggle_y = cy + (row_h - toggle_h) / 2.0;
        let toggle_r = toggle_h / 2.0;

        if self.glyph_mode {
            ctx.set_fill_style_str("rgba(80,200,120,0.35)");
            self.fill_rounded_rect(toggle_x, toggle_y, toggle_w, toggle_h, toggle_r);
            ctx.set_font(&self.font(9.0, "bold"));
            ctx.set_fill_style_str("#8f8");
            ctx.set_text_align("center");
            let _ = ctx.fill_text("ON", toggle_x + toggle_w / 2.0, toggle_y + toggle_h / 2.0);
        } else {
            ctx.set_fill_style_str("rgba(100,100,100,0.25)");
            self.fill_rounded_rect(toggle_x, toggle_y, toggle_w, toggle_h, toggle_r);
            ctx.set_font(&self.font(9.0, "bold"));
            ctx.set_fill_style_str("#888");
            ctx.set_text_align("center");
            let _ = ctx.fill_text("OFF", toggle_x + toggle_w / 2.0, toggle_y + toggle_h / 2.0);
        }
        cy += row_h + 8.0 * d;

        // Difficulty
        ctx.set_font(&self.font(9.0, ""));
        ctx.set_fill_style_str("#667");
        ctx.set_text_align("left");
        let diff_label = crate::config::Difficulty::from_config(&game.config).label();
        let _ = ctx.fill_text(&format!("Difficulty: {diff_label}"), x, cy);
        cy += 20.0 * d;

        // Main menu button
        let btn_w = w * 0.8;
        let btn_h = 28.0 * d;
        let btn_x = x + (w - btn_w) / 2.0;
        ctx.set_fill_style_str("rgba(120,60,60,0.35)");
        self.fill_rounded_rect(btn_x, cy, btn_w, btn_h, 4.0 * d);
        ctx.set_font(&self.font(10.0, "bold"));
        ctx.set_fill_style_str("#f88");
        ctx.set_text_align("center");
        ctx.set_text_baseline("middle");
        let _ = ctx.fill_text("Main Menu", btn_x + btn_w / 2.0, cy + btn_h / 2.0);
    }

    /// Width of the side panel in CSS pixels (for hit-testing from lib.rs).
    pub fn side_panel_css_w(&self) -> f64 {
        if self.landscape { SIDE_PANEL_CSS_W } else { 0.0 }
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
                if self.glyph_mode {
                    self.draw_tile_glyph(tile, px, py, cell);
                } else {
                    let wall_face = tile == Tile::Wall
                        && y + 1 < map.height
                        && map.get(x, y + 1) != Tile::Wall;
                    let sprite = sprites::tile_sprite(tile, x, y, wall_face);
                    if !self.draw_sprite(sprite, px, py, cell, cell) {
                        self.draw_tile_fallback(tile, px, py, cell);
                    }
                }
                if vis == Visibility::Seen {
                    ctx.set_fill_style_str("rgba(0,0,0,0.5)");
                    ctx.fill_rect(px, py, cell, cell);
                }
            }
        }

        // Path preview / Aim line
        if preview_path.len() > 1 {
            let aim_target = preview_path[preview_path.len() - 1];
            let is_aiming = game.has_ranged_weapon()
                && game.enemies.iter().any(|e| e.x == aim_target.0 && e.y == aim_target.1 && e.hp > 0);
            if is_aiming {
                // Aim mode: draw line with color based on range/hit chance
                let max_range = game.ranged_max_range();
                for (i, &(tx, ty)) in preview_path.iter().enumerate() {
                    if i == 0 { continue; } // skip player tile
                    let (px, py) = cam.world_to_screen(tx, ty);
                    let dist = ((tx - game.player_x).abs()).max((ty - game.player_y).abs());
                    let is_last = i == preview_path.len() - 1;

                    if dist > max_range {
                        // Out of range: red
                        ctx.set_fill_style_str("rgba(255,50,50,0.35)");
                    } else {
                        let hit_chance = game.ranged_hit_chance(dist);
                        if is_last {
                            // Target tile: brighter based on hit chance
                            if hit_chance >= 70 {
                                ctx.set_fill_style_str("rgba(50,255,50,0.5)");
                            } else if hit_chance >= 40 {
                                ctx.set_fill_style_str("rgba(255,200,50,0.5)");
                            } else {
                                ctx.set_fill_style_str("rgba(255,100,50,0.5)");
                            }
                        } else {
                            // Line body: amber
                            ctx.set_fill_style_str("rgba(255,180,50,0.25)");
                        }
                    }
                    ctx.fill_rect(px + 1.0, py + 1.0, cell - 2.0, cell - 2.0);

                    // Draw hit chance on target tile
                    if is_last && dist <= max_range && dist > 0 {
                        let hit_chance = game.ranged_hit_chance(dist);
                        let font_size = (cell * 0.35).round().max(8.0);
                        ctx.set_font(&format!("{font_size}px monospace"));
                        ctx.set_text_align("center");
                        ctx.set_text_baseline("middle");
                        ctx.set_fill_style_str("#fff");
                        let _ = ctx.fill_text(
                            &format!("{hit_chance}%"),
                            px + cell / 2.0,
                            py + cell / 2.0,
                        );
                    }
                }
            } else {
                // Normal movement preview: blue
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
        }

        // Enemies (with bump animation offsets)
        let map = game.current_map();
        for (ei, e) in game.enemies.iter().enumerate() {
            if e.hp <= 0 { continue; }
            if e.x < min_x || e.x >= max_x || e.y < min_y || e.y >= max_y { continue; }
            if map.get_visibility(e.x, e.y) != Visibility::Visible { continue; }
            let (ex, ey) = cam.world_to_screen(e.x, e.y);
            let (bx, by) = bump_offset(&game.bump_anims, false, ei, cell);
            let rx = ex + bx;
            let ry = ey + by;
            let has_bump = game.bump_anims.iter().any(|b| !b.is_player && b.enemy_idx == ei);
            if self.glyph_mode {
                self.draw_enemy_glyph(e.glyph, e.is_ranged, rx, ry, cell);
            } else {
                let sprite = sprites::enemy_sprite(e.glyph);
                if !self.draw_sprite_ex(sprite, rx, ry, cell, cell, !e.facing_left) {
                    self.draw_enemy_glyph(e.glyph, e.is_ranged, rx, ry, cell);
                }
            }
            // Red flash overlay when hit
            if has_bump {
                ctx.set_fill_style_str("rgba(255,60,60,0.35)");
                ctx.fill_rect(rx, ry, cell, cell);
            }
        }

        // Ground items
        for gi in &game.ground_items {
            if gi.x < min_x || gi.x >= max_x || gi.y < min_y || gi.y >= max_y { continue; }
            if map.get_visibility(gi.x, gi.y) != Visibility::Visible { continue; }
            let (ix, iy) = cam.world_to_screen(gi.x, gi.y);
            if self.glyph_mode {
                self.draw_item_glyph(gi.item.glyph, &gi.item.kind, ix, iy, cell);
            } else {
                let sprite = sprites::item_sprite(gi.item.name);
                if !self.draw_sprite(sprite, ix, iy, cell, cell) {
                    self.draw_item_glyph(gi.item.glyph, &gi.item.kind, ix, iy, cell);
                }
            }
        }

        // Player (with bump animation offset)
        let (px, py) = cam.world_to_screen(game.player_x, game.player_y);
        let (pbx, pby) = bump_offset(&game.bump_anims, true, 0, cell);
        let prx = px + pbx;
        let pry = py + pby;
        if self.glyph_mode {
            let font_size = (cell * 0.8).round();
            ctx.set_font(&format!("{font_size}px monospace"));
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");
            ctx.set_fill_style_str("#fff");
            let _ = ctx.fill_text("@", prx + cell / 2.0, pry + cell / 2.0);
        } else {
            let player_sprite = sprites::player_sprite();
            if !self.draw_sprite_ex(player_sprite, prx, pry, cell, cell, !game.player_facing_left) {
                let font_size = (cell * 0.8).round();
                ctx.set_font(&format!("{font_size}px monospace"));
                ctx.set_text_align("center");
                ctx.set_text_baseline("middle");
                ctx.set_fill_style_str("#fff");
                let _ = ctx.fill_text("@", prx + cell / 2.0, pry + cell / 2.0);
            }
        }
        // Red flash on player when taking damage
        let player_taking_damage = game.bump_anims.iter().any(|b| b.is_player && (b.dx * b.dx + b.dy * b.dy) > 0.001);
        if player_taking_damage {
            ctx.set_fill_style_str("rgba(255,60,60,0.3)");
            ctx.fill_rect(prx, pry, cell, cell);
        }

        // Visual effects
        self.draw_visual_effects(game, cam, cell);

        // Floating texts
        self.draw_floating_texts(game, cam, cell);
    }

    /// Render active floating text indicators.
    fn draw_floating_texts(&self, game: &Game, cam: &Camera, cell: f64) {
        let ctx = &self.ctx;
        for ft in &game.floating_texts {
            let (sx, sy) = cam.world_to_screen(ft.world_x, ft.world_y);
            let rise = ft.age * cell * 0.8;
            let alpha = (1.0 - ft.age).max(0.0);
            let font_size = (cell * 0.35).round().max(10.0);
            ctx.set_font(&format!("bold {font_size}px monospace"));
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");
            // Shadow
            ctx.set_fill_style_str(&format!("rgba(0,0,0,{:.2})", alpha * 0.7));
            let _ = ctx.fill_text(&ft.text, sx + cell / 2.0 + 1.0, sy + cell * 0.3 - rise + 1.0);
            // Text
            ctx.set_fill_style_str(&format_color_alpha(ft.color, alpha));
            let _ = ctx.fill_text(&ft.text, sx + cell / 2.0, sy + cell * 0.3 - rise);
        }
    }

    /// Render active visual effects (AOE blasts, heal glow, etc.).
    fn draw_visual_effects(&self, game: &Game, cam: &Camera, cell: f64) {
        let ctx = &self.ctx;
        for ve in &game.visual_effects {
            let (sx, sy) = cam.world_to_screen(ve.x, ve.y);
            let cx = sx + cell / 2.0;
            let cy = sy + cell / 2.0;
            match ve.kind {
                EffectKind::AoeBlast => {
                    // Expanding fire ring
                    let max_r = cell * 3.5;
                    let r = ve.age * max_r;
                    let alpha = (1.0 - ve.age) * 0.6;
                    ctx.begin_path();
                    let _ = ctx.arc(cx, cy, r, 0.0, std::f64::consts::TAU);
                    ctx.set_stroke_style_str(&format!("rgba(255,120,30,{:.2})", alpha));
                    ctx.set_line_width(cell * 0.15);
                    ctx.stroke();
                    // Inner glow
                    let inner_alpha = (1.0 - ve.age) * 0.25;
                    ctx.set_fill_style_str(&format!("rgba(255,80,20,{:.2})", inner_alpha));
                    ctx.begin_path();
                    let _ = ctx.arc(cx, cy, r * 0.7, 0.0, std::f64::consts::TAU);
                    ctx.fill();
                }
                EffectKind::HealGlow => {
                    // Green glow that expands and fades
                    let r = cell * 0.5 + ve.age * cell * 0.3;
                    let alpha = (1.0 - ve.age) * 0.5;
                    ctx.set_fill_style_str(&format!("rgba(80,255,80,{:.2})", alpha));
                    ctx.begin_path();
                    let _ = ctx.arc(cx, cy, r, 0.0, std::f64::consts::TAU);
                    ctx.fill();
                    // Rising plus sign
                    let rise = ve.age * cell * 0.4;
                    let t_alpha = (1.0 - ve.age).max(0.0);
                    ctx.set_fill_style_str(&format!("rgba(120,255,120,{:.2})", t_alpha));
                    let cross_sz = cell * 0.08;
                    ctx.fill_rect(cx - cross_sz, cy - cross_sz * 3.0 - rise, cross_sz * 2.0, cross_sz * 6.0);
                    ctx.fill_rect(cx - cross_sz * 3.0, cy - cross_sz - rise, cross_sz * 6.0, cross_sz * 2.0);
                }
                EffectKind::PoisonCloud => {
                    // Purple poison cloud
                    let r = cell * 0.4 + ve.age * cell * 0.4;
                    let alpha = (1.0 - ve.age) * 0.45;
                    ctx.set_fill_style_str(&format!("rgba(160,60,220,{:.2})", alpha));
                    ctx.begin_path();
                    let _ = ctx.arc(cx, cy, r, 0.0, std::f64::consts::TAU);
                    ctx.fill();
                    // Bubbles
                    let bubble_alpha = (1.0 - ve.age) * 0.6;
                    ctx.set_fill_style_str(&format!("rgba(200,100,255,{:.2})", bubble_alpha));
                    let br = cell * 0.06;
                    let phase = ve.age * 8.0;
                    for j in 0..3 {
                        let angle = phase + j as f64 * 2.1;
                        let bx = cx + angle.cos() * r * 0.5;
                        let by = cy + angle.sin() * r * 0.5 - ve.age * cell * 0.3;
                        ctx.begin_path();
                        let _ = ctx.arc(bx, by, br, 0.0, std::f64::consts::TAU);
                        ctx.fill();
                    }
                }
                EffectKind::EnergizeEffect => {
                    // Blue-cyan sparkles
                    let alpha = (1.0 - ve.age) * 0.6;
                    ctx.set_fill_style_str(&format!("rgba(80,200,255,{:.2})", alpha));
                    let phase = ve.age * 12.0;
                    for j in 0..5 {
                        let angle = phase + j as f64 * 1.26;
                        let dist = cell * 0.3 * (1.0 + ve.age * 0.5);
                        let bx = cx + angle.cos() * dist;
                        let by = cy + angle.sin() * dist - ve.age * cell * 0.2;
                        let sr = cell * 0.04 * (1.0 - ve.age);
                        ctx.begin_path();
                        let _ = ctx.arc(bx, by, sr, 0.0, std::f64::consts::TAU);
                        ctx.fill();
                    }
                }
            }
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
            let _ = ctx.fill_text(&format!("HP {} ATK {} DEF {}", ei.hp, ei.attack, ei.defense), text_x, line2_y);

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
            ("Settings", if game.drawer == Drawer::Settings { "#ccc" } else { "#888" }, game.drawer == Drawer::Settings),
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
            Drawer::Settings => self.draw_settings_drawer(game, canvas_w, canvas_h, bottom_h, t),
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

        // Detail bar at bottom when an item is selected, otherwise just slot count
        let detail_bar_h = if game.selected_inventory_item.is_some() { 46.0 * d } else { 20.0 * d };
        let avail_h = (drawer_y + drawer_h - detail_bar_h) - list_y;
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
            let selected = game.selected_inventory_item;

            let inline_btn_w = 36.0 * d;
            let inline_btn_h = 22.0 * d;
            let inline_btn_gap = 3.0 * d;

            for (vi, idx) in (scroll..end).enumerate() {
                let item = &game.inventory[idx];
                let iy = list_y + vi as f64 * slot_h;

                // Row background — highlight selected item
                if selected == Some(idx) {
                    ctx.set_fill_style_str("rgba(80,130,255,0.18)");
                    ctx.fill_rect(pad, iy, canvas_w - pad * 2.0 - scrollbar_w, slot_h);
                } else if vi % 2 == 0 {
                    ctx.set_fill_style_str("rgba(255,255,255,0.03)");
                    ctx.fill_rect(pad, iy, canvas_w - pad * 2.0 - scrollbar_w, slot_h);
                }

                let sprite = sprites::item_sprite(item.name);
                self.draw_sprite(sprite, pad + 4.0 * d, iy + (slot_h - icon_size) / 2.0, icon_size, icon_size);

                let color = match item.kind {
                    ItemKind::Potion => "#f88",
                    ItemKind::Scroll => "#88f",
                    ItemKind::Weapon => "#aaf",
                    ItemKind::RangedWeapon => "#8df",
                    ItemKind::Armor => "#afa",
                    ItemKind::Helmet => "#fc8",
                    ItemKind::Shield => "#adf",
                    ItemKind::Boots => "#da8",
                    ItemKind::Food => "#fa8",
                    ItemKind::Ring => "#ff8",
                };
                ctx.set_font(&self.font(11.0, ""));
                ctx.set_fill_style_str(color);
                ctx.set_text_baseline("middle");
                let _ = ctx.fill_text(item.name, pad + icon_size + 8.0 * d, iy + slot_h / 2.0);

                // Inline Use/Equip button
                let btn_y = iy + (slot_h - inline_btn_h) / 2.0;
                let drop_x = text_right - inline_btn_w;
                let use_x = drop_x - inline_btn_gap - inline_btn_w;
                let action_label = match item.kind {
                    ItemKind::Potion | ItemKind::Scroll | ItemKind::Food => "Use",
                    _ => "Eq",
                };
                ctx.set_fill_style_str("rgba(80,200,120,0.2)");
                self.fill_rounded_rect(use_x, btn_y, inline_btn_w, inline_btn_h, 3.0 * d);
                ctx.set_font(&self.font(9.0, "bold"));
                ctx.set_fill_style_str("#8f8");
                ctx.set_text_align("center");
                let _ = ctx.fill_text(action_label, use_x + inline_btn_w / 2.0, btn_y + inline_btn_h / 2.0);

                // Inline Drop button
                ctx.set_fill_style_str("rgba(200,80,80,0.2)");
                self.fill_rounded_rect(drop_x, btn_y, inline_btn_w, inline_btn_h, 3.0 * d);
                ctx.set_fill_style_str("#f88");
                let _ = ctx.fill_text("Del", drop_x + inline_btn_w / 2.0, btn_y + inline_btn_h / 2.0);
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

        // Bottom bar: detail bar if selected, slot count otherwise
        let bar_y = drawer_y + drawer_h - detail_bar_h;
        if let Some(sel_idx) = game.selected_inventory_item {
            if sel_idx < game.inventory.len() {
                let item = &game.inventory[sel_idx];

                // Detail bar background
                ctx.set_fill_style_str("rgba(40,40,60,0.95)");
                ctx.fill_rect(0.0, bar_y, canvas_w, detail_bar_h);
                ctx.set_fill_style_str("rgba(80,130,255,0.15)");
                ctx.fill_rect(0.0, bar_y, canvas_w, 1.0 * d);

                // Description text
                let desc = game.inventory_item_desc(sel_idx).unwrap_or_default();
                ctx.set_font(&self.font(10.0, ""));
                ctx.set_fill_style_str("#ccc");
                ctx.set_text_align("left");
                ctx.set_text_baseline("middle");
                let _ = ctx.fill_text(&desc, pad, bar_y + detail_bar_h * 0.35);

                // Action buttons
                let btn_h = 26.0 * d;
                let btn_y = bar_y + detail_bar_h - btn_h - 4.0 * d;
                let btn_gap = 8.0 * d;

                // Use/Equip button
                let action_label = match item.kind {
                    ItemKind::Potion | ItemKind::Scroll | ItemKind::Food => "Use",
                    _ => "Equip",
                };
                let action_w = 60.0 * d;
                let action_x = canvas_w - pad - action_w - btn_gap - 60.0 * d;
                ctx.set_fill_style_str("rgba(80,200,120,0.25)");
                self.fill_rounded_rect(action_x, btn_y, action_w, btn_h, 4.0 * d);
                ctx.set_font(&self.font(11.0, "bold"));
                ctx.set_fill_style_str("#8f8");
                ctx.set_text_align("center");
                ctx.set_text_baseline("middle");
                let _ = ctx.fill_text(action_label, action_x + action_w / 2.0, btn_y + btn_h / 2.0);

                // Drop button
                let drop_w = 60.0 * d;
                let drop_x = canvas_w - pad - drop_w;
                ctx.set_fill_style_str("rgba(200,80,80,0.25)");
                self.fill_rounded_rect(drop_x, btn_y, drop_w, btn_h, 4.0 * d);
                ctx.set_font(&self.font(11.0, "bold"));
                ctx.set_fill_style_str("#f88");
                ctx.set_text_align("center");
                let _ = ctx.fill_text("Drop", drop_x + drop_w / 2.0, btn_y + btn_h / 2.0);

                // Slot count (small, left side)
                ctx.set_font(&self.font(9.0, ""));
                ctx.set_fill_style_str("#555");
                ctx.set_text_align("left");
                ctx.set_text_baseline("middle");
                let _ = ctx.fill_text(
                    &format!("{}/10", game.inventory.len()),
                    pad, btn_y + btn_h / 2.0,
                );
            }
        } else {
            // Slot count only
            ctx.set_font(&self.font(10.0, ""));
            ctx.set_fill_style_str("#555");
            ctx.set_text_align("right");
            ctx.set_text_baseline("bottom");
            let _ = ctx.fill_text(
                &format!("{}/10", game.inventory.len()),
                canvas_w - pad, drawer_y + drawer_h - 6.0 * d,
            );
        }

        ctx.restore(); // pop clip
    }

    /// Height of a single skill row in CSS-space (pre-DPR), for hit-testing.
    pub fn stats_skill_row_h(&self) -> f64 {
        30.0
    }

    fn draw_stats_drawer(&self, game: &Game, canvas_w: f64, canvas_h: f64, bottom_h: f64, anim_t: f64) {
        let ctx = &self.ctx;
        let d = self.dpr;
        let drawer_h = canvas_h * 0.55;
        let base_y = canvas_h - bottom_h - drawer_h;
        let slide_offset = drawer_h * (1.0 - anim_t);
        let drawer_y = base_y + slide_offset;
        let pad = 12.0 * d;
        let scroll_off = game.stats_scroll * d;

        ctx.save();
        ctx.begin_path();
        ctx.rect(0.0, base_y, canvas_w, drawer_h);
        ctx.clip();

        // Background
        ctx.set_fill_style_str("rgba(8,8,16,0.94)");
        self.fill_rounded_rect(0.0, drawer_y, canvas_w, drawer_h, 12.0 * d);
        ctx.set_fill_style_str("rgba(160,80,255,0.3)");
        self.fill_rounded_rect(canvas_w * 0.3, drawer_y, canvas_w * 0.4, 3.0 * d, 1.5 * d);

        // Header (fixed, not scrolled)
        ctx.set_font(&self.font(14.0, "bold"));
        ctx.set_fill_style_str("#fff");
        ctx.set_text_align("center");
        ctx.set_text_baseline("top");
        let _ = ctx.fill_text("CHARACTER", canvas_w / 2.0, drawer_y + 10.0 * d);

        // Scrollable content starts here
        let content_top = drawer_y + 32.0 * d;
        let mut y = content_top - scroll_off;

        let icon_sz = 36.0 * d;
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

        // Stat summary rows
        let mut stats: Vec<(&str, String, &str)> = vec![
            ("HP", format!("{} / {}", game.player_hp, game.player_max_hp), "#4c4"),
            ("Attack", format!("{}", game.effective_attack()), "#8cf"),
            ("Defense", format!("{}", game.effective_defense()), "#fc8"),
            ("Dexterity", format!("{}", game.player_dexterity), "#adf"),
        ];
        if game.has_ranged_weapon() {
            stats.push(("Range", format!("{}", game.ranged_max_range()), "#fa8"));
        }
        stats.push(("Location", game.location_name(), "#ccc"));

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

        // ---- Skill Points Section ----
        y += 8.0 * d;
        ctx.set_font(&self.font(11.0, "bold"));
        ctx.set_fill_style_str("#666");
        ctx.set_text_align("left");
        ctx.set_text_baseline("top");
        let _ = ctx.fill_text("Skill Points", pad, y);
        // Unspent count
        if game.skill_points > 0 {
            ctx.set_fill_style_str("#ff0");
            ctx.set_text_align("right");
            let _ = ctx.fill_text(&format!("{} available", game.skill_points), canvas_w - pad, y);
        }
        y += 20.0 * d;

        let skill_row_h = self.stats_skill_row_h() * d;
        let btn_sz = 24.0 * d;
        let has_points = game.skill_points > 0;

        let skills: [(&str, &str, String, &str); 4] = [
            ("STR", "Strength", format!("{}", game.strength), "#f84"),
            ("VIT", "Vitality", format!("{}", game.vitality), "#4f4"),
            ("DEX", "Dexterity", format!("{}", game.player_dexterity), "#adf"),
            ("STA", "Stamina", format!("{}", game.max_stamina), "#8df"),
        ];
        for (abbr, label, value, color) in &skills {
            // Abbreviation badge
            ctx.set_font(&self.font(10.0, "bold"));
            ctx.set_fill_style_str(color);
            ctx.set_text_align("left");
            ctx.set_text_baseline("middle");
            let row_mid = y + skill_row_h / 2.0;
            let _ = ctx.fill_text(abbr, pad, row_mid);

            // Label
            ctx.set_font(&self.font(11.0, ""));
            ctx.set_fill_style_str("#aaa");
            let _ = ctx.fill_text(label, pad + 36.0 * d, row_mid);

            // Value
            ctx.set_font(&self.font(12.0, "bold"));
            ctx.set_fill_style_str(color);
            ctx.set_text_align("right");
            let btn_area = if has_points { btn_sz + 8.0 * d } else { 0.0 };
            let _ = ctx.fill_text(value, canvas_w - pad - btn_area, row_mid);

            // "+" button when skill points available
            if has_points {
                let btn_x = canvas_w - pad - btn_sz;
                let btn_y = y + (skill_row_h - btn_sz) / 2.0;
                ctx.set_fill_style_str("rgba(100,200,100,0.25)");
                self.fill_rounded_rect(btn_x, btn_y, btn_sz, btn_sz, 4.0 * d);
                ctx.set_font(&self.font(14.0, "bold"));
                ctx.set_fill_style_str("#4f4");
                ctx.set_text_align("center");
                ctx.set_text_baseline("middle");
                let _ = ctx.fill_text("+", btn_x + btn_sz / 2.0, btn_y + btn_sz / 2.0);
            }

            y += skill_row_h;
        }

        // ---- Equipment Section ----
        y += 8.0 * d;
        ctx.set_font(&self.font(11.0, "bold"));
        ctx.set_fill_style_str("#666");
        ctx.set_text_align("left");
        ctx.set_text_baseline("top");
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

    // ---- Settings drawer ----

    fn draw_settings_drawer(&self, game: &Game, canvas_w: f64, canvas_h: f64, bottom_h: f64, anim_t: f64) {
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

        // Background
        ctx.set_fill_style_str("rgba(8,8,16,0.94)");
        self.fill_rounded_rect(0.0, drawer_y, canvas_w, drawer_h, 12.0 * d);
        ctx.set_fill_style_str("rgba(120,120,120,0.3)");
        self.fill_rounded_rect(canvas_w * 0.3, drawer_y, canvas_w * 0.4, 3.0 * d, 1.5 * d);

        // Title
        ctx.set_font(&self.font(14.0, "bold"));
        ctx.set_fill_style_str("#fff");
        ctx.set_text_align("center");
        ctx.set_text_baseline("top");
        let _ = ctx.fill_text("SETTINGS", canvas_w / 2.0, drawer_y + 10.0 * d);

        // Glyph Mode toggle row
        let row_y = drawer_y + 40.0 * d;
        let row_h = 40.0 * d;

        // Label
        ctx.set_font(&self.font(13.0, ""));
        ctx.set_fill_style_str("#ccc");
        ctx.set_text_align("left");
        ctx.set_text_baseline("middle");
        let _ = ctx.fill_text("Glyph Mode", pad, row_y + row_h / 2.0);

        // Toggle button
        let toggle_w = 70.0 * d;
        let toggle_h = 30.0 * d;
        let toggle_x = canvas_w - pad - toggle_w;
        let toggle_y = row_y + (row_h - toggle_h) / 2.0;
        let toggle_r = toggle_h / 2.0;

        if self.glyph_mode {
            ctx.set_fill_style_str("rgba(80,200,120,0.35)");
            self.fill_rounded_rect(toggle_x, toggle_y, toggle_w, toggle_h, toggle_r);
            ctx.set_font(&self.font(12.0, "bold"));
            ctx.set_fill_style_str("#8f8");
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");
            let _ = ctx.fill_text("ON", toggle_x + toggle_w / 2.0, toggle_y + toggle_h / 2.0);
        } else {
            ctx.set_fill_style_str("rgba(100,100,100,0.25)");
            self.fill_rounded_rect(toggle_x, toggle_y, toggle_w, toggle_h, toggle_r);
            ctx.set_font(&self.font(12.0, "bold"));
            ctx.set_fill_style_str("#888");
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");
            let _ = ctx.fill_text("OFF", toggle_x + toggle_w / 2.0, toggle_y + toggle_h / 2.0);
        }

        // Description text
        ctx.set_font(&self.font(10.0, ""));
        ctx.set_fill_style_str("#666");
        ctx.set_text_align("left");
        ctx.set_text_baseline("top");
        let _ = ctx.fill_text("Classic ASCII rendering. Keyboard: G to toggle.", pad, row_y + row_h + 4.0 * d);

        // Difficulty info row
        let diff_y = row_y + row_h + 24.0 * d;
        ctx.set_font(&self.font(11.0, ""));
        ctx.set_fill_style_str("#667");
        ctx.set_text_align("left");
        ctx.set_text_baseline("middle");
        let diff_label = crate::config::Difficulty::from_config(&game.config).label();
        let _ = ctx.fill_text(&format!("Difficulty: {diff_label}"), pad, diff_y + 10.0 * d);

        // Main Menu button
        let menu_btn_w = (canvas_w * 0.5).min(180.0 * d);
        let menu_btn_h = 34.0 * d;
        let menu_btn_x = (canvas_w - menu_btn_w) / 2.0;
        let menu_btn_y = diff_y + 32.0 * d;

        ctx.set_fill_style_str("rgba(120,60,60,0.35)");
        self.fill_rounded_rect(menu_btn_x, menu_btn_y, menu_btn_w, menu_btn_h, 6.0 * d);
        ctx.set_stroke_style_str("rgba(180,80,80,0.4)");
        ctx.set_line_width(1.0 * d);
        ctx.begin_path();
        let r = 6.0 * d;
        ctx.move_to(menu_btn_x + r, menu_btn_y);
        ctx.line_to(menu_btn_x + menu_btn_w - r, menu_btn_y);
        let _ = ctx.arc_to(menu_btn_x + menu_btn_w, menu_btn_y, menu_btn_x + menu_btn_w, menu_btn_y + r, r);
        ctx.line_to(menu_btn_x + menu_btn_w, menu_btn_y + menu_btn_h - r);
        let _ = ctx.arc_to(menu_btn_x + menu_btn_w, menu_btn_y + menu_btn_h, menu_btn_x + menu_btn_w - r, menu_btn_y + menu_btn_h, r);
        ctx.line_to(menu_btn_x + r, menu_btn_y + menu_btn_h);
        let _ = ctx.arc_to(menu_btn_x, menu_btn_y + menu_btn_h, menu_btn_x, menu_btn_y + menu_btn_h - r, r);
        ctx.line_to(menu_btn_x, menu_btn_y + r);
        let _ = ctx.arc_to(menu_btn_x, menu_btn_y, menu_btn_x + r, menu_btn_y, r);
        ctx.close_path();
        ctx.stroke();
        ctx.set_font(&self.font(12.0, "bold"));
        ctx.set_fill_style_str("#f88");
        ctx.set_text_align("center");
        ctx.set_text_baseline("middle");
        let _ = ctx.fill_text("Main Menu", menu_btn_x + menu_btn_w / 2.0, menu_btn_y + menu_btn_h / 2.0);

        ctx.restore();
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

    /// Glyph-mode tile rendering: colored background + ASCII character.
    fn draw_tile_glyph(&self, tile: Tile, px: f64, py: f64, cell: f64) {
        let ctx = &self.ctx;
        // Background fill
        ctx.set_fill_style_str(tile.color());
        ctx.fill_rect(px, py, cell, cell);
        // Foreground glyph (skip for wall/floor/grass — background is enough)
        let ch = tile.glyph();
        let fg = match tile {
            Tile::Wall => return, // solid block, no glyph needed
            Tile::Floor => { ctx.set_fill_style_str("#333"); "." }
            Tile::Tree => { ctx.set_fill_style_str("#0a0"); "T" }
            Tile::Grass => { ctx.set_fill_style_str("#2a2"); "." }
            Tile::Road => { ctx.set_fill_style_str("#a86"); "=" }
            Tile::DungeonEntrance => { ctx.set_fill_style_str("#fa0"); ">" }
            Tile::StairsDown => { ctx.set_fill_style_str("#aaf"); ">" }
            Tile::StairsUp => { ctx.set_fill_style_str("#aaf"); "<" }
        };
        let _ = ch; // glyph already matched above
        let font_size = (cell * 0.7).round();
        ctx.set_font(&format!("{font_size}px monospace"));
        ctx.set_text_align("center");
        ctx.set_text_baseline("middle");
        let _ = ctx.fill_text(fg, px + cell / 2.0, py + cell / 2.0);
    }

    /// Glyph-mode enemy rendering.
    fn draw_enemy_glyph(&self, glyph: char, is_ranged: bool, ex: f64, ey: f64, cell: f64) {
        let ctx = &self.ctx;
        let font_size = (cell * 0.8).round();
        ctx.set_font(&format!("{font_size}px monospace"));
        ctx.set_text_align("center");
        ctx.set_text_baseline("middle");
        let color = if is_ranged {
            "#ff0"
        } else {
            match glyph {
                'D' => "#f44",
                'w' | 'b' | 'B' => "#a84",
                'K' | 'l' => "#c4f",
                'T' => "#4a4",
                _ => "#4f4",
            }
        };
        ctx.set_fill_style_str(color);
        let _ = ctx.fill_text(&glyph.to_string(), ex + cell / 2.0, ey + cell / 2.0);
    }

    /// Glyph-mode item rendering.
    fn draw_item_glyph(&self, glyph: char, kind: &ItemKind, ix: f64, iy: f64, cell: f64) {
        let ctx = &self.ctx;
        let font_size = (cell * 0.6).round();
        ctx.set_font(&format!("{font_size}px monospace"));
        ctx.set_text_align("center");
        ctx.set_text_baseline("middle");
        let color = match kind {
            ItemKind::Potion => "#f88",
            ItemKind::Scroll => "#88f",
            ItemKind::Weapon | ItemKind::RangedWeapon => "#aaf",
            ItemKind::Armor | ItemKind::Helmet | ItemKind::Shield | ItemKind::Boots => "#afa",
            ItemKind::Food => "#fa8",
            ItemKind::Ring => "#ff8",
        };
        ctx.set_fill_style_str(color);
        let _ = ctx.fill_text(&glyph.to_string(), ix + cell / 2.0, iy + cell / 2.0);
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

    // ---- Menu screens ----

    /// Draw the main menu screen.
    pub fn draw_main_menu(&self, has_save: bool) {
        let ctx = &self.ctx;
        let d = self.dpr;
        let canvas_w = ctx.canvas().unwrap().width() as f64;
        let canvas_h = ctx.canvas().unwrap().height() as f64;

        // Background
        ctx.set_fill_style_str("#0a0a14");
        ctx.fill_rect(0.0, 0.0, canvas_w, canvas_h);

        // Decorative border
        ctx.set_stroke_style_str("rgba(100,140,200,0.2)");
        ctx.set_line_width(2.0 * d);
        ctx.stroke_rect(20.0 * d, 20.0 * d, canvas_w - 40.0 * d, canvas_h - 40.0 * d);

        // Title
        let title_size = (canvas_w * 0.08).min(60.0 * d).round();
        ctx.set_font(&format!("bold {title_size}px monospace"));
        ctx.set_fill_style_str("#c8e0ff");
        ctx.set_text_align("center");
        ctx.set_text_baseline("middle");
        let _ = ctx.fill_text("THE CAVE", canvas_w / 2.0, canvas_h * 0.22);

        // Subtitle
        let sub_size = (title_size * 0.3).round();
        ctx.set_font(&format!("{sub_size}px monospace"));
        ctx.set_fill_style_str("#667");
        let _ = ctx.fill_text("A roguelike adventure", canvas_w / 2.0, canvas_h * 0.22 + title_size * 0.8);

        // Menu buttons
        let btn_w = (canvas_w * 0.5).min(280.0 * d);
        let btn_h = 44.0 * d;
        let gap = 16.0 * d;
        let start_y = canvas_h * 0.45;
        let btn_x = (canvas_w - btn_w) / 2.0;

        // New Game
        self.draw_menu_button(btn_x, start_y, btn_w, btn_h, "New Game", true);

        // Continue (only if save exists)
        let continue_y = start_y + btn_h + gap;
        self.draw_menu_button(btn_x, continue_y, btn_w, btn_h, "Continue", has_save);

        // Settings
        let settings_y = continue_y + btn_h + gap;
        self.draw_menu_button(btn_x, settings_y, btn_w, btn_h, "Settings", true);

        // Version
        let ver_size = (10.0 * d).round();
        ctx.set_font(&format!("{ver_size}px monospace"));
        ctx.set_fill_style_str("#334");
        ctx.set_text_align("center");
        ctx.set_text_baseline("bottom");
        let _ = ctx.fill_text("v0.1", canvas_w / 2.0, canvas_h - 12.0 * d);
    }

    /// Draw a menu button (rounded rect with label).
    fn draw_menu_button(&self, x: f64, y: f64, w: f64, h: f64, label: &str, enabled: bool) {
        let ctx = &self.ctx;
        let d = self.dpr;
        let r = 6.0 * d;

        if enabled {
            ctx.set_fill_style_str("rgba(60,80,120,0.4)");
        } else {
            ctx.set_fill_style_str("rgba(40,40,50,0.3)");
        }
        self.fill_rounded_rect(x, y, w, h, r);

        // Border
        if enabled {
            ctx.set_stroke_style_str("rgba(100,140,200,0.5)");
        } else {
            ctx.set_stroke_style_str("rgba(60,60,70,0.3)");
        }
        ctx.set_line_width(1.0 * d);
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
        ctx.stroke();

        let font_size = (14.0 * d).round();
        ctx.set_font(&format!("bold {font_size}px monospace"));
        if enabled {
            ctx.set_fill_style_str("#dde8ff");
        } else {
            ctx.set_fill_style_str("#445");
        }
        ctx.set_text_align("center");
        ctx.set_text_baseline("middle");
        let _ = ctx.fill_text(label, x + w / 2.0, y + h / 2.0);
    }

    /// Draw the new game menu (difficulty selection + seed display).
    pub fn draw_new_game_menu(&self, selected_difficulty: usize, seed: u64) {
        use crate::config::Difficulty;

        let ctx = &self.ctx;
        let d = self.dpr;
        let canvas_w = ctx.canvas().unwrap().width() as f64;
        let canvas_h = ctx.canvas().unwrap().height() as f64;

        // Background
        ctx.set_fill_style_str("#0a0a14");
        ctx.fill_rect(0.0, 0.0, canvas_w, canvas_h);

        // Title
        let title_size = (canvas_w * 0.06).min(40.0 * d).round();
        ctx.set_font(&format!("bold {title_size}px monospace"));
        ctx.set_fill_style_str("#c8e0ff");
        ctx.set_text_align("center");
        ctx.set_text_baseline("middle");
        let _ = ctx.fill_text("NEW GAME", canvas_w / 2.0, canvas_h * 0.12);

        // Back button (top-left)
        let back_size = (12.0 * d).round();
        ctx.set_font(&format!("{back_size}px monospace"));
        ctx.set_fill_style_str("#8af");
        ctx.set_text_align("left");
        ctx.set_text_baseline("top");
        let _ = ctx.fill_text("< Back", 16.0 * d, 12.0 * d);

        // Difficulty selection
        let section_y = canvas_h * 0.22;
        let label_size = (13.0 * d).round();
        ctx.set_font(&format!("bold {label_size}px monospace"));
        ctx.set_fill_style_str("#aab");
        ctx.set_text_align("center");
        let _ = ctx.fill_text("DIFFICULTY", canvas_w / 2.0, section_y);

        let difficulties = [Difficulty::Easy, Difficulty::Normal, Difficulty::Hard];
        let btn_w = (canvas_w * 0.7).min(300.0 * d);
        let btn_h = 52.0 * d;
        let gap = 10.0 * d;
        let btn_x = (canvas_w - btn_w) / 2.0;
        let list_y = section_y + 24.0 * d;

        for (i, diff) in difficulties.iter().enumerate() {
            let y = list_y + (btn_h + gap) * i as f64;
            let selected = i == selected_difficulty;

            // Button background
            if selected {
                ctx.set_fill_style_str("rgba(60,120,200,0.35)");
            } else {
                ctx.set_fill_style_str("rgba(40,40,60,0.3)");
            }
            self.fill_rounded_rect(btn_x, y, btn_w, btn_h, 6.0 * d);

            // Selection indicator
            if selected {
                ctx.set_stroke_style_str("#8af");
                ctx.set_line_width(2.0 * d);
                ctx.begin_path();
                let r = 6.0 * d;
                ctx.move_to(btn_x + r, y);
                ctx.line_to(btn_x + btn_w - r, y);
                let _ = ctx.arc_to(btn_x + btn_w, y, btn_x + btn_w, y + r, r);
                ctx.line_to(btn_x + btn_w, y + btn_h - r);
                let _ = ctx.arc_to(btn_x + btn_w, y + btn_h, btn_x + btn_w - r, y + btn_h, r);
                ctx.line_to(btn_x + r, y + btn_h);
                let _ = ctx.arc_to(btn_x, y + btn_h, btn_x, y + btn_h - r, r);
                ctx.line_to(btn_x, y + r);
                let _ = ctx.arc_to(btn_x, y, btn_x + r, y, r);
                ctx.close_path();
                ctx.stroke();
            }

            // Difficulty name
            let name_size = (14.0 * d).round();
            ctx.set_font(&format!("bold {name_size}px monospace"));
            if selected { ctx.set_fill_style_str("#fff"); } else { ctx.set_fill_style_str("#899"); }
            ctx.set_text_align("left");
            ctx.set_text_baseline("top");
            let _ = ctx.fill_text(diff.label(), btn_x + 14.0 * d, y + 8.0 * d);

            // Description
            let desc_size = (10.0 * d).round();
            ctx.set_font(&format!("{desc_size}px monospace"));
            if selected { ctx.set_fill_style_str("#8af"); } else { ctx.set_fill_style_str("#556"); }
            ctx.set_text_baseline("bottom");
            let _ = ctx.fill_text(diff.description(), btn_x + 14.0 * d, y + btn_h - 8.0 * d);
        }

        // Seed display
        let seed_y = list_y + (btn_h + gap) * 3.0 + 10.0 * d;
        let seed_size = (11.0 * d).round();
        ctx.set_font(&format!("{seed_size}px monospace"));
        ctx.set_fill_style_str("#556");
        ctx.set_text_align("center");
        ctx.set_text_baseline("middle");
        let seed_str = format!("Seed: {:08X}", seed & 0xFFFF_FFFF);
        let _ = ctx.fill_text(&seed_str, canvas_w / 2.0, seed_y);

        // Tap seed to randomize hint
        let hint_size = (9.0 * d).round();
        ctx.set_font(&format!("{hint_size}px monospace"));
        ctx.set_fill_style_str("#334");
        let _ = ctx.fill_text("Tap seed to randomize", canvas_w / 2.0, seed_y + 16.0 * d);

        // Start button
        let start_y = seed_y + 44.0 * d;
        let start_w = (canvas_w * 0.5).min(220.0 * d);
        let start_h = 48.0 * d;
        let start_x = (canvas_w - start_w) / 2.0;
        ctx.set_fill_style_str("rgba(40,160,80,0.4)");
        self.fill_rounded_rect(start_x, start_y, start_w, start_h, 8.0 * d);
        ctx.set_stroke_style_str("rgba(80,200,120,0.6)");
        ctx.set_line_width(2.0 * d);
        ctx.begin_path();
        let r = 8.0 * d;
        ctx.move_to(start_x + r, start_y);
        ctx.line_to(start_x + start_w - r, start_y);
        let _ = ctx.arc_to(start_x + start_w, start_y, start_x + start_w, start_y + r, r);
        ctx.line_to(start_x + start_w, start_y + start_h - r);
        let _ = ctx.arc_to(start_x + start_w, start_y + start_h, start_x + start_w - r, start_y + start_h, r);
        ctx.line_to(start_x + r, start_y + start_h);
        let _ = ctx.arc_to(start_x, start_y + start_h, start_x, start_y + start_h - r, r);
        ctx.line_to(start_x, start_y + r);
        let _ = ctx.arc_to(start_x, start_y, start_x + r, start_y, r);
        ctx.close_path();
        ctx.stroke();

        let start_font = (16.0 * d).round();
        ctx.set_font(&format!("bold {start_font}px monospace"));
        ctx.set_fill_style_str("#8f8");
        ctx.set_text_align("center");
        ctx.set_text_baseline("middle");
        let _ = ctx.fill_text("START", start_x + start_w / 2.0, start_y + start_h / 2.0);
    }

    /// Draw the settings menu (standalone, not the in-game drawer).
    pub fn draw_settings_menu(&self) {
        let ctx = &self.ctx;
        let d = self.dpr;
        let canvas_w = ctx.canvas().unwrap().width() as f64;
        let canvas_h = ctx.canvas().unwrap().height() as f64;

        // Background
        ctx.set_fill_style_str("#0a0a14");
        ctx.fill_rect(0.0, 0.0, canvas_w, canvas_h);

        // Title
        let title_size = (canvas_w * 0.06).min(40.0 * d).round();
        ctx.set_font(&format!("bold {title_size}px monospace"));
        ctx.set_fill_style_str("#c8e0ff");
        ctx.set_text_align("center");
        ctx.set_text_baseline("middle");
        let _ = ctx.fill_text("SETTINGS", canvas_w / 2.0, canvas_h * 0.12);

        // Back button
        let back_size = (12.0 * d).round();
        ctx.set_font(&format!("{back_size}px monospace"));
        ctx.set_fill_style_str("#8af");
        ctx.set_text_align("left");
        ctx.set_text_baseline("top");
        let _ = ctx.fill_text("< Back", 16.0 * d, 12.0 * d);

        // Settings rows
        let row_w = (canvas_w * 0.8).min(340.0 * d);
        let row_h = 44.0 * d;
        let gap = 8.0 * d;
        let row_x = (canvas_w - row_w) / 2.0;
        let pad = 14.0 * d;

        // Glyph Mode row
        let row_y = canvas_h * 0.25;
        ctx.set_fill_style_str("rgba(30,30,50,0.5)");
        self.fill_rounded_rect(row_x, row_y, row_w, row_h, 6.0 * d);

        ctx.set_font(&self.font(13.0, ""));
        ctx.set_fill_style_str("#ccc");
        ctx.set_text_align("left");
        ctx.set_text_baseline("middle");
        let _ = ctx.fill_text("Glyph Mode", row_x + pad, row_y + row_h / 2.0);

        // Toggle
        let toggle_w = 60.0 * d;
        let toggle_h = 28.0 * d;
        let toggle_x = row_x + row_w - pad - toggle_w;
        let toggle_y = row_y + (row_h - toggle_h) / 2.0;
        if self.glyph_mode {
            ctx.set_fill_style_str("rgba(80,200,120,0.35)");
            self.fill_rounded_rect(toggle_x, toggle_y, toggle_w, toggle_h, toggle_h / 2.0);
            ctx.set_font(&self.font(11.0, "bold"));
            ctx.set_fill_style_str("#8f8");
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");
            let _ = ctx.fill_text("ON", toggle_x + toggle_w / 2.0, toggle_y + toggle_h / 2.0);
        } else {
            ctx.set_fill_style_str("rgba(100,100,100,0.25)");
            self.fill_rounded_rect(toggle_x, toggle_y, toggle_w, toggle_h, toggle_h / 2.0);
            ctx.set_font(&self.font(11.0, "bold"));
            ctx.set_fill_style_str("#888");
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");
            let _ = ctx.fill_text("OFF", toggle_x + toggle_w / 2.0, toggle_y + toggle_h / 2.0);
        }

        // Description
        ctx.set_font(&self.font(9.0, ""));
        ctx.set_fill_style_str("#556");
        ctx.set_text_align("left");
        ctx.set_text_baseline("top");
        let _ = ctx.fill_text("Classic ASCII rendering", row_x + pad, row_y + row_h + 4.0 * d);

        // FOV Radius row
        let row2_y = row_y + row_h + gap + 20.0 * d;
        ctx.set_fill_style_str("rgba(30,30,50,0.5)");
        self.fill_rounded_rect(row_x, row2_y, row_w, row_h, 6.0 * d);

        ctx.set_font(&self.font(13.0, ""));
        ctx.set_fill_style_str("#ccc");
        ctx.set_text_align("left");
        ctx.set_text_baseline("middle");
        let _ = ctx.fill_text("Controls", row_x + pad, row2_y + row_h / 2.0);

        // Info text
        ctx.set_font(&self.font(10.0, ""));
        ctx.set_fill_style_str("#667");
        ctx.set_text_align("left");
        let _ = ctx.fill_text("Arrows/Swipe: move", row_x + pad, row2_y + row_h / 2.0 + 16.0 * d);

        // Controls info section
        let info_y = row2_y + row_h + gap + 4.0 * d;
        let info_size = (10.0 * d).round();
        ctx.set_font(&format!("{info_size}px monospace"));
        ctx.set_fill_style_str("#556");
        ctx.set_text_align("left");
        ctx.set_text_baseline("top");
        let controls = [
            "G: Toggle glyph mode",
            "I: Inventory    C: Stats",
            "S: Sprint    F/Space: Interact",
            "Swipe: Move / Aim ranged",
        ];
        for (i, line) in controls.iter().enumerate() {
            let _ = ctx.fill_text(line, row_x + pad, info_y + i as f64 * (info_size + 4.0 * d));
        }
    }
}

/// Compute pixel offset for a bump animation. Returns (dx, dy) in canvas pixels.
fn bump_offset(anims: &[BumpAnim], is_player: bool, enemy_idx: usize, cell: f64) -> (f64, f64) {
    for ba in anims {
        if ba.is_player == is_player && (is_player || ba.enemy_idx == enemy_idx) {
            // Triangle wave: go out then back
            let t = if ba.progress < 0.5 {
                ba.progress * 2.0
            } else {
                (1.0 - ba.progress) * 2.0
            };
            return (ba.dx * t * cell, ba.dy * t * cell);
        }
    }
    (0.0, 0.0)
}

/// Format a hex color string with alpha (e.g., "#f44" → "rgba(255,68,68,0.5)").
fn format_color_alpha(hex: &str, alpha: f64) -> String {
    let hex = hex.trim_start_matches('#');
    let (r, g, b) = if hex.len() == 3 {
        let r = u8::from_str_radix(&hex[0..1], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[1..2], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[2..3], 16).unwrap_or(0);
        (r * 17, g * 17, b * 17)
    } else if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
        (r, g, b)
    } else {
        (255, 255, 255)
    };
    format!("rgba({},{},{},{:.2})", r, g, b, alpha)
}
