use crate::camera::Camera;
use crate::game::{BumpAnim, EffectKind, Game};
use crate::map::{Tile, Visibility};
use crate::sprites;

use super::{item_kind_color, Renderer};

impl Renderer {
    // ---- World layer ----

    pub(super) fn draw_world(&self, game: &Game, preview_path: &[(i32, i32)]) {
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
                    let dungeon_style = game.world.current_dungeon_style();
                    let sprite = sprites::tile_sprite(tile, x, y, wall_face, dungeon_style);
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
                && game.has_enemy_at(aim_target.0, aim_target.1);
            if is_aiming {
                self.draw_aim_preview(game, cam, cell, preview_path);
            } else {
                self.draw_move_preview(cam, cell, preview_path);
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
        let drew_sprite = !self.glyph_mode && {
            let player_sprite = sprites::player_sprite();
            self.draw_sprite_ex(player_sprite, prx, pry, cell, cell, !game.player_facing_left)
        };
        if !drew_sprite {
            self.draw_player_glyph(prx, pry, cell);
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

    /// Draw ranged aim line overlay with hit chance coloring.
    fn draw_aim_preview(&self, game: &Game, cam: &Camera, cell: f64, path: &[(i32, i32)]) {
        let ctx = &self.ctx;
        let max_range = game.ranged_max_range();
        let last = path.len() - 1;

        for (i, &(tx, ty)) in path.iter().enumerate() {
            if i == 0 { continue; } // skip player tile
            let (px, py) = cam.world_to_screen(tx, ty);
            let dist = ((tx - game.player_x).abs()).max((ty - game.player_y).abs());
            let is_target = i == last;

            let color = if dist > max_range {
                "rgba(255,50,50,0.35)"
            } else if is_target {
                let chance = game.ranged_hit_chance(dist);
                if chance >= 70 { "rgba(50,255,50,0.5)" }
                else if chance >= 40 { "rgba(255,200,50,0.5)" }
                else { "rgba(255,100,50,0.5)" }
            } else {
                "rgba(255,180,50,0.25)"
            };
            ctx.set_fill_style_str(color);
            ctx.fill_rect(px + 1.0, py + 1.0, cell - 2.0, cell - 2.0);

            // Draw hit chance label on target tile
            if is_target && dist <= max_range && dist > 0 {
                let hit_chance = game.ranged_hit_chance(dist);
                let font_size = (cell * 0.35).round().max(8.0);
                ctx.set_font(&format!("{font_size}px monospace"));
                ctx.set_text_align("center");
                ctx.set_text_baseline("middle");
                ctx.set_fill_style_str("#fff");
                let _ = ctx.fill_text(&format!("{hit_chance}%"), px + cell / 2.0, py + cell / 2.0);
            }
        }
    }

    /// Draw blue movement preview path overlay.
    fn draw_move_preview(&self, cam: &Camera, cell: f64, path: &[(i32, i32)]) {
        let ctx = &self.ctx;
        let last = path.len() - 1;
        for (i, &(tx, ty)) in path.iter().enumerate() {
            let (px, py) = cam.world_to_screen(tx, ty);
            let alpha = if i == last { "0.4" } else { "0.2" };
            ctx.set_fill_style_str(&format!("rgba(0,180,255,{alpha})"));
            ctx.fill_rect(px + 1.0, py + 1.0, cell - 2.0, cell - 2.0);
        }
    }

    /// Draw the player "@" glyph (used in glyph mode and as sprite fallback).
    fn draw_player_glyph(&self, x: f64, y: f64, cell: f64) {
        let ctx = &self.ctx;
        let font_size = (cell * 0.8).round();
        ctx.set_font(&format!("{font_size}px monospace"));
        ctx.set_text_align("center");
        ctx.set_text_baseline("middle");
        ctx.set_fill_style_str("#fff");
        let _ = ctx.fill_text("@", x + cell / 2.0, y + cell / 2.0);
    }

    /// Glyph-mode tile rendering: colored background + ASCII character.
    fn draw_tile_glyph(&self, tile: Tile, px: f64, py: f64, cell: f64) {
        let ctx = &self.ctx;
        ctx.set_fill_style_str(tile.color());
        ctx.fill_rect(px, py, cell, cell);
        // Foreground glyph (wall is solid block — no glyph needed)
        let (fg_color, fg_char) = match tile {
            Tile::Wall => return,
            Tile::Floor => ("#333", "."),
            Tile::Tree => ("#0a0", "T"),
            Tile::Grass => ("#2a2", "."),
            Tile::Road => ("#a86", "="),
            Tile::DungeonEntrance => ("#fa0", ">"),
            Tile::StairsDown => ("#aaf", ">"),
            Tile::StairsUp => ("#aaf", "<"),
        };
        ctx.set_fill_style_str(fg_color);
        let font_size = (cell * 0.7).round();
        ctx.set_font(&format!("{font_size}px monospace"));
        ctx.set_text_align("center");
        ctx.set_text_baseline("middle");
        let _ = ctx.fill_text(fg_char, px + cell / 2.0, py + cell / 2.0);
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
                'w' | 'b' | 'B' | 'y' | 'x' | 'F' | 'Z' => "#a84",
                '1' | '2' | '0' => "#8c8",
                'K' | 'l' | '6' | '7' => "#c4f",
                'T' => "#4a4",
                _ => "#4f4",
            }
        };
        ctx.set_fill_style_str(color);
        let _ = ctx.fill_text(&glyph.to_string(), ex + cell / 2.0, ey + cell / 2.0);
    }

    /// Glyph-mode item rendering.
    fn draw_item_glyph(&self, glyph: char, kind: &crate::game::ItemKind, ix: f64, iy: f64, cell: f64) {
        let ctx = &self.ctx;
        let font_size = (cell * 0.6).round();
        ctx.set_font(&format!("{font_size}px monospace"));
        ctx.set_text_align("center");
        ctx.set_text_baseline("middle");
        ctx.set_fill_style_str(item_kind_color(kind));
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
}

/// Compute pixel offset for a bump animation. Returns (dx, dy) in canvas pixels.
pub(super) fn bump_offset(anims: &[BumpAnim], is_player: bool, enemy_idx: usize, cell: f64) -> (f64, f64) {
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
pub(super) fn format_color_alpha(hex: &str, alpha: f64) -> String {
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
