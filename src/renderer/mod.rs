mod world;
mod hud;
mod drawers;
mod menus;

use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::camera::Camera;
use crate::game::{Drawer, Game};
use crate::sprites::{Sheet, SpriteRef};

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
    pub(super) fn side_panel_w(&self) -> f64 {
        SIDE_PANEL_CSS_W * self.dpr
    }

    /// Scaled HUD dimension helpers.
    pub(super) fn top_bar_h(&self) -> f64 { TOP_BAR_BASE * self.dpr }
    pub(super) fn detail_strip_h(&self) -> f64 { DETAIL_STRIP_BASE * self.dpr }
    pub(super) fn bottom_bar_h(&self) -> f64 { BOTTOM_BAR_BASE * self.dpr }
    pub(super) fn msg_area_h(&self) -> f64 { MSG_AREA_BASE * self.dpr }

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

    /// Helper: draw a rounded-rect filled region.
    pub(super) fn fill_rounded_rect(&self, x: f64, y: f64, w: f64, h: f64, r: f64) {
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

    /// Width of the side panel in CSS pixels (for hit-testing from lib.rs).
    pub fn side_panel_css_w(&self) -> f64 {
        if self.landscape { SIDE_PANEL_CSS_W } else { 0.0 }
    }
}
