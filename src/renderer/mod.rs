mod world;
mod hud;
mod drawers;
mod menus;

use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::camera::Camera;
use crate::game::{Drawer, Game, Item, ItemKind, QUICKBAR_SLOTS};
use crate::sprites::{self, Sheet, SpriteRef};

/// Info passed from lib.rs to the renderer when a drag is active.
/// Parameters for drawing a stat bar (HP, stamina, hunger).
pub(super) struct StatBar<'a> {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
    pub r: f64,
    pub frac: f64,
    pub bg_color: &'a str,
    pub fill_color: &'a str,
    pub label: &'a str,
    pub font_size: f64,
    pub text_inset: f64,
}

pub struct DragInfo<'a> {
    pub item: &'a Item,
    /// Touch position in CSS pixels.
    pub css_x: f64,
    pub css_y: f64,
}

/// Map an ItemKind to its display color (used across HUD panels).
pub(super) fn item_kind_color(kind: &ItemKind) -> &'static str {
    match kind {
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
    }
}

/// Loaded sprite sheet images, indexed by Sheet enum.
pub struct SpriteSheets {
    pub tiles: HtmlImageElement,
    pub monsters: HtmlImageElement,
    pub rogues: HtmlImageElement,
    pub items: HtmlImageElement,
    pub animals: Option<HtmlImageElement>,
}

impl SpriteSheets {
    pub(super) fn get(&self, sheet: Sheet) -> Option<&HtmlImageElement> {
        match sheet {
            Sheet::Tiles => Some(&self.tiles),
            Sheet::Monsters => Some(&self.monsters),
            Sheet::Rogues => Some(&self.rogues),
            Sheet::Items => Some(&self.items),
            Sheet::Animals => self.animals.as_ref(),
        }
    }
}

/// Base heights of the fixed HUD regions (in CSS points, scaled by DPR).
const TOP_BAR_BASE: f64 = 52.0;
const DETAIL_STRIP_BASE: f64 = 52.0;
const BOTTOM_BAR_BASE: f64 = 48.0;
const QUICKBAR_BASE: f64 = 44.0;
const MSG_AREA_BASE: f64 = 42.0;

/// Speed of drawer slide animation (fraction per frame at 60fps).
const DRAWER_ANIM_SPEED: f64 = 0.15;

/// Width of the right-side panel in landscape mode (CSS pixels).
const SIDE_PANEL_CSS_W: f64 = 220.0;

/// Height of the compact top bar in landscape mode (CSS pixels).
const LANDSCAPE_TOP_BAR_BASE: f64 = 34.0;

/// Height of the bottom message strip in landscape mode (CSS pixels).
const LANDSCAPE_MSG_BAR_BASE: f64 = 26.0;

pub struct Renderer {
    pub(super) ctx: CanvasRenderingContext2d,
    pub camera: Camera,
    pub(super) sheets: Option<SpriteSheets>,
    /// Device pixel ratio — used to scale HUD elements.
    pub(super) dpr: f64,
    /// Drawer slide animation progress: 0.0 = fully closed, 1.0 = fully open.
    pub(super) drawer_anim: f64,
    /// Which drawer was last opened (kept during close animation).
    pub(super) last_drawer: Drawer,
    /// When true, render ASCII glyphs instead of sprite sheets.
    pub glyph_mode: bool,
    /// True when the canvas is in landscape orientation (width > height).
    pub landscape: bool,
    /// Per-slot flash animation progress (0.0 = off, 1.0 = bright, decays to 0).
    pub quickbar_flash: [f64; QUICKBAR_SLOTS],
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
            quickbar_flash: [0.0; QUICKBAR_SLOTS],
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
        let cell = if self.landscape {
            // Size tiles for the game area (excluding side panel) so that
            // VIEWPORT_TILES_WIDE tiles fit in the playable region, not the
            // full canvas. This prevents oversized tiles in landscape.
            let game_area_w = width - SIDE_PANEL_CSS_W * dpr;
            self.camera.set_viewport_for_area(width, height, game_area_w)
        } else {
            self.camera.set_viewport(width, height)
        };
        if self.landscape {
            // In landscape: compact top bar, bottom message strip, side panel on right.
            self.camera.pad_top = self.landscape_top_bar_h() / cell;
            self.camera.pad_bottom = self.landscape_msg_bar_h() / cell;
            self.camera.pad_right = self.side_panel_w() / cell / 2.0;
        } else {
            // Portrait: original top/bottom bar layout + quick bar.
            self.camera.pad_top = (self.top_bar_h() + self.detail_strip_h()) / cell;
            self.camera.pad_bottom = (self.bottom_bar_h() + self.quickbar_h() + self.msg_area_h()) / cell;
            self.camera.pad_right = 0.0;
        }
    }

    /// Side panel width in canvas pixels (landscape mode).
    pub(super) fn side_panel_w(&self) -> f64 {
        SIDE_PANEL_CSS_W * self.dpr
    }

    /// Scaled HUD dimension helpers.
    pub(super) fn top_bar_h(&self) -> f64 { TOP_BAR_BASE * self.dpr }
    pub(super) fn detail_strip_h(&self) -> f64 { DETAIL_STRIP_BASE * self.dpr }
    pub(super) fn bottom_bar_h(&self) -> f64 { BOTTOM_BAR_BASE * self.dpr }
    pub(super) fn quickbar_h(&self) -> f64 { QUICKBAR_BASE * self.dpr }
    pub(super) fn msg_area_h(&self) -> f64 { MSG_AREA_BASE * self.dpr }
    pub(super) fn landscape_top_bar_h(&self) -> f64 { LANDSCAPE_TOP_BAR_BASE * self.dpr }
    pub(super) fn landscape_msg_bar_h(&self) -> f64 { LANDSCAPE_MSG_BAR_BASE * self.dpr }

    /// Trigger a flash animation on a quick-bar slot.
    pub fn flash_quickbar_slot(&mut self, slot: usize) {
        if slot < self.quickbar_flash.len() {
            self.quickbar_flash[slot] = 1.0;
        }
    }

    /// Tick quick-bar flash animations (call once per frame).
    pub fn tick_quickbar_flash(&mut self) {
        for v in &mut self.quickbar_flash {
            if *v > 0.0 {
                *v = (*v - 0.08).max(0.0);
            }
        }
    }

    /// Height of the bottom bar in canvas pixels.
    pub fn bottom_bar_height(&self) -> f64 {
        self.bottom_bar_h()
    }

    /// Height of the quick-bar in canvas pixels.
    pub fn quickbar_height(&self) -> f64 {
        self.quickbar_h()
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
    pub(super) fn font(&self, base_px: f64, weight: &str) -> String {
        let sz = (base_px * self.dpr).round();
        if weight.is_empty() {
            format!("{sz}px monospace")
        } else {
            format!("{weight} {sz}px monospace")
        }
    }

    /// Draw a sprite at screen position. Returns true if drawn.
    pub(super) fn draw_sprite(&self, sprite: SpriteRef, dx: f64, dy: f64, dw: f64, dh: f64) -> bool {
        self.draw_sprite_ex(sprite, dx, dy, dw, dh, false)
    }

    /// Draw a sprite, optionally mirrored horizontally. Returns true if drawn.
    pub(super) fn draw_sprite_ex(&self, sprite: SpriteRef, dx: f64, dy: f64, dw: f64, dh: f64, flip: bool) -> bool {
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

    /// Trace a rounded-rect path (shared by fill and stroke helpers).
    fn trace_rounded_rect(&self, x: f64, y: f64, w: f64, h: f64, r: f64) {
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
    }

    /// Helper: draw a rounded-rect filled region.
    pub(super) fn fill_rounded_rect(&self, x: f64, y: f64, w: f64, h: f64, r: f64) {
        self.trace_rounded_rect(x, y, w, h, r);
        self.ctx.fill();
    }

    /// Helper: stroke a rounded-rect border (set stroke style + line width before calling).
    pub(super) fn stroke_rounded_rect(&self, x: f64, y: f64, w: f64, h: f64, r: f64) {
        self.trace_rounded_rect(x, y, w, h, r);
        self.ctx.stroke();
    }

    /// Draw a toggle button (ON/OFF pill). Used in settings panels.
    pub(super) fn draw_toggle(&self, x: f64, y: f64, w: f64, h: f64, on: bool, font_size: f64) {
        let ctx = &self.ctx;
        let r = h / 2.0;
        if on {
            ctx.set_fill_style_str("rgba(80,200,120,0.35)");
            self.fill_rounded_rect(x, y, w, h, r);
            ctx.set_font(&self.font(font_size, "bold"));
            ctx.set_fill_style_str("#8f8");
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");
            let _ = ctx.fill_text("ON", x + w / 2.0, y + h / 2.0);
        } else {
            ctx.set_fill_style_str("rgba(100,100,100,0.25)");
            self.fill_rounded_rect(x, y, w, h, r);
            ctx.set_font(&self.font(font_size, "bold"));
            ctx.set_fill_style_str("#888");
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");
            let _ = ctx.fill_text("OFF", x + w / 2.0, y + h / 2.0);
        }
    }

    /// Draw a stat bar (HP, stamina, hunger style). Sets fill style and renders.
    pub(super) fn draw_stat_bar(&self, bar: &StatBar) {
        let ctx = &self.ctx;
        ctx.set_fill_style_str(bar.bg_color);
        self.fill_rounded_rect(bar.x, bar.y, bar.w, bar.h, bar.r);
        ctx.set_fill_style_str(bar.fill_color);
        self.fill_rounded_rect(bar.x, bar.y, bar.w * bar.frac.max(0.0), bar.h, bar.r);
        ctx.set_font(&self.font(bar.font_size, "bold"));
        ctx.set_fill_style_str("#fff");
        ctx.set_text_align("left");
        ctx.set_text_baseline("middle");
        let _ = ctx.fill_text(bar.label, bar.x + bar.text_inset, bar.y + bar.h / 2.0);
    }

    pub fn draw(&self, game: &Game, preview_path: &[(i32, i32)], drag: Option<&DragInfo>) {
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
            // Landscape: compact top bar + streamlined side panel + bottom messages
            let panel_w = self.side_panel_w();
            let panel_x = canvas_w - panel_w;
            let top_h = self.landscape_top_bar_h();
            let msg_h = self.landscape_msg_bar_h();

            self.draw_landscape_top_bar(game, panel_x, top_h);
            self.draw_side_panel(game, panel_x, panel_w, canvas_h);
            self.draw_landscape_messages(game, panel_x, canvas_h, msg_h);
        } else {
            // Portrait: original top/bottom bar layout + quick bar
            let top_h = self.top_bar_h();
            let bottom_h = self.bottom_bar_h();
            let qbar_h = self.quickbar_h();
            let msg_h = self.msg_area_h();
            // Combined height of bottom bar + quick bar for message/drawer positioning
            let bottom_region = bottom_h + qbar_h;
            self.draw_top_bar(game, canvas_w, top_h);
            self.draw_tile_detail(game, canvas_w, top_h);
            self.draw_messages(game, canvas_w, canvas_h, bottom_region, msg_h);
            self.draw_bottom_bar(game, canvas_w, canvas_h, bottom_h);
            self.draw_quick_bar(game, canvas_w, canvas_h, bottom_h, qbar_h);
            self.draw_drawer(game, canvas_w, canvas_h, bottom_region);
        }

        // Death / Victory overlay (on top of everything)
        if !game.alive || game.won {
            self.draw_end_overlay(game, canvas_w, canvas_h);
        }

        // Drag ghost (drawn on top of everything)
        if let Some(drag) = drag {
            self.draw_drag_ghost(drag, canvas_w, canvas_h);
        }
    }

    /// Draw the dragged item ghost at the finger position + highlight quick-bar slot under finger.
    fn draw_drag_ghost(&self, drag: &DragInfo, canvas_w: f64, canvas_h: f64) {
        let ctx = &self.ctx;
        let d = self.dpr;
        let cx = drag.css_x * d;
        let cy = drag.css_y * d;
        let ghost_size = 40.0 * d;
        let gx = cx - ghost_size / 2.0;
        let gy = cy - ghost_size - 10.0 * d; // offset above finger

        // Draw ghost sprite with transparency
        ctx.save();
        ctx.set_global_alpha(0.75);
        // Shadow
        ctx.set_shadow_color("rgba(0,0,0,0.6)");
        ctx.set_shadow_blur(8.0 * d);
        ctx.set_shadow_offset_x(2.0 * d);
        ctx.set_shadow_offset_y(2.0 * d);

        let color = item_kind_color(&drag.item.kind);
        ctx.set_fill_style_str("rgba(20,20,30,0.85)");
        self.fill_rounded_rect(gx, gy, ghost_size, ghost_size, 6.0 * d);
        ctx.set_stroke_style_str(color);
        ctx.set_line_width(2.0 * d);
        self.stroke_rounded_rect(gx, gy, ghost_size, ghost_size, 6.0 * d);

        // Reset shadow for sprite
        ctx.set_shadow_color("transparent");
        ctx.set_shadow_blur(0.0);

        let sprite = sprites::item_sprite(drag.item.name);
        let inset = 4.0 * d;
        let spr_size = ghost_size - inset * 2.0;
        if !self.draw_sprite(sprite, gx + inset, gy + inset, spr_size, spr_size) {
            ctx.set_font(&self.font(20.0, "bold"));
            ctx.set_fill_style_str(color);
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");
            let _ = ctx.fill_text(
                &drag.item.glyph.to_string(),
                gx + ghost_size / 2.0,
                gy + ghost_size / 2.0,
            );
        }
        ctx.restore();

        // Highlight quick-bar slot under finger
        if !self.landscape {
            let bottom_h = self.bottom_bar_h();
            let qbar_h = self.quickbar_h();
            let bar_y = canvas_h - bottom_h - qbar_h;
            let slot_size = 36.0 * d;
            let slot_pad = 6.0 * d;
            let total_w = QUICKBAR_SLOTS as f64 * (slot_size + slot_pad) - slot_pad;
            let start_x = (canvas_w - total_w) / 2.0;
            let slot_y = bar_y + (qbar_h - slot_size) / 2.0;

            for i in 0..QUICKBAR_SLOTS {
                let sx = start_x + i as f64 * (slot_size + slot_pad);
                if cx >= sx && cx <= sx + slot_size && cy >= slot_y && cy <= slot_y + slot_size {
                    ctx.save();
                    ctx.set_stroke_style_str("rgba(255,200,80,0.7)");
                    ctx.set_line_width(2.5 * d);
                    self.stroke_rounded_rect(sx, slot_y, slot_size, slot_size, 5.0 * d);
                    ctx.restore();
                    break;
                }
            }
        }
    }

    /// Width of the side panel in CSS pixels (for hit-testing from lib.rs).
    pub fn side_panel_css_w(&self) -> f64 {
        if self.landscape { SIDE_PANEL_CSS_W } else { 0.0 }
    }
}
