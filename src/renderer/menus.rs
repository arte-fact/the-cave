use crate::game::Game;

use super::Renderer;

impl Renderer {
    // ---- Death / Victory ----

    pub(super) fn draw_end_overlay(&self, game: &Game, canvas_w: f64, canvas_h: f64) {
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

    // ---- Menu screens ----

    /// Draw the main menu screen.
    pub fn draw_main_menu(&self, has_save: bool) {
        let ctx = &self.ctx;
        let d = self.dpr;
        let canvas_w = ctx.canvas().unwrap().width() as f64;
        let canvas_h = ctx.canvas().unwrap().height() as f64;
        let compact = canvas_h < canvas_w;

        // Background
        ctx.set_fill_style_str("#0a0a14");
        ctx.fill_rect(0.0, 0.0, canvas_w, canvas_h);

        // Decorative border
        ctx.set_stroke_style_str("rgba(100,140,200,0.2)");
        ctx.set_line_width(2.0 * d);
        ctx.stroke_rect(20.0 * d, 20.0 * d, canvas_w - 40.0 * d, canvas_h - 40.0 * d);

        // Title
        let ref_dim = canvas_w.min(canvas_h);
        let title_size = (ref_dim * 0.08).min(60.0 * d).round();
        ctx.set_font(&format!("bold {title_size}px monospace"));
        ctx.set_fill_style_str("#c8e0ff");
        ctx.set_text_align("center");
        ctx.set_text_baseline("middle");
        let title_y = if compact { canvas_h * 0.15 } else { canvas_h * 0.22 };
        let _ = ctx.fill_text("THE CAVE", canvas_w / 2.0, title_y);

        // Subtitle
        let sub_size = (title_size * 0.3).round();
        ctx.set_font(&format!("{sub_size}px monospace"));
        ctx.set_fill_style_str("#667");
        let _ = ctx.fill_text("A roguelike adventure", canvas_w / 2.0, title_y + title_size * 0.8);

        // Menu buttons
        let btn_w = (ref_dim * 0.5).min(280.0 * d);
        let btn_h = if compact { 36.0 * d } else { 44.0 * d };
        let gap = if compact { 10.0 * d } else { 16.0 * d };
        let start_y = if compact { canvas_h * 0.38 } else { canvas_h * 0.45 };
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

        let (bg, border, fg) = if enabled {
            ("rgba(60,80,120,0.4)", "rgba(100,140,200,0.5)", "#dde8ff")
        } else {
            ("rgba(40,40,50,0.3)", "rgba(60,60,70,0.3)", "#445")
        };

        ctx.set_fill_style_str(bg);
        self.fill_rounded_rect(x, y, w, h, r);
        ctx.set_stroke_style_str(border);
        ctx.set_line_width(1.0 * d);
        self.stroke_rounded_rect(x, y, w, h, r);

        let font_size = (14.0 * d).round();
        ctx.set_font(&format!("bold {font_size}px monospace"));
        ctx.set_fill_style_str(fg);
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
        let compact = canvas_h < canvas_w;
        let ref_dim = canvas_w.min(canvas_h);

        // Background
        ctx.set_fill_style_str("#0a0a14");
        ctx.fill_rect(0.0, 0.0, canvas_w, canvas_h);

        // Title
        let title_size = (ref_dim * 0.06).min(40.0 * d).round();
        ctx.set_font(&format!("bold {title_size}px monospace"));
        ctx.set_fill_style_str("#c8e0ff");
        ctx.set_text_align("center");
        ctx.set_text_baseline("middle");
        let title_y = if compact { canvas_h * 0.08 } else { canvas_h * 0.12 };
        let _ = ctx.fill_text("NEW GAME", canvas_w / 2.0, title_y);

        // Back button (top-left)
        let back_size = (12.0 * d).round();
        ctx.set_font(&format!("{back_size}px monospace"));
        ctx.set_fill_style_str("#8af");
        ctx.set_text_align("left");
        ctx.set_text_baseline("top");
        let _ = ctx.fill_text("< Back", 16.0 * d, 12.0 * d);

        // Difficulty selection
        let section_y = if compact { canvas_h * 0.16 } else { canvas_h * 0.22 };
        let label_size = (13.0 * d).round();
        ctx.set_font(&format!("bold {label_size}px monospace"));
        ctx.set_fill_style_str("#aab");
        ctx.set_text_align("center");
        let _ = ctx.fill_text("DIFFICULTY", canvas_w / 2.0, section_y);

        let difficulties = [Difficulty::Easy, Difficulty::Normal, Difficulty::Hard];
        let btn_w = (ref_dim * 0.7).min(300.0 * d);
        let btn_h = if compact { 36.0 * d } else { 52.0 * d };
        let gap = if compact { 6.0 * d } else { 10.0 * d };
        let btn_x = (canvas_w - btn_w) / 2.0;
        let list_y = section_y + if compact { 18.0 * d } else { 24.0 * d };

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
                self.stroke_rounded_rect(btn_x, y, btn_w, btn_h, 6.0 * d);
            }

            // Difficulty name
            let name_size = (14.0 * d).round();
            ctx.set_font(&format!("bold {name_size}px monospace"));
            if selected { ctx.set_fill_style_str("#fff"); } else { ctx.set_fill_style_str("#899"); }
            ctx.set_text_align("left");
            ctx.set_text_baseline("top");
            let _ = ctx.fill_text(diff.label(), btn_x + 14.0 * d, y + 8.0 * d);

            // Description (skip in compact when button is too short)
            if !compact {
                let desc_size = (10.0 * d).round();
                ctx.set_font(&format!("{desc_size}px monospace"));
                if selected { ctx.set_fill_style_str("#8af"); } else { ctx.set_fill_style_str("#556"); }
                ctx.set_text_baseline("bottom");
                let _ = ctx.fill_text(diff.description(), btn_x + 14.0 * d, y + btn_h - 8.0 * d);
            }
        }

        // Seed display
        let seed_gap = if compact { 6.0 * d } else { 10.0 * d };
        let seed_y = list_y + (btn_h + gap) * 3.0 + seed_gap;
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
        let start_gap = if compact { 28.0 * d } else { 44.0 * d };
        let start_y = seed_y + start_gap;
        let start_w = (ref_dim * 0.5).min(220.0 * d);
        let start_h = if compact { 38.0 * d } else { 48.0 * d };
        let start_x = (canvas_w - start_w) / 2.0;
        ctx.set_fill_style_str("rgba(40,160,80,0.4)");
        self.fill_rounded_rect(start_x, start_y, start_w, start_h, 8.0 * d);
        ctx.set_stroke_style_str("rgba(80,200,120,0.6)");
        ctx.set_line_width(2.0 * d);
        self.stroke_rounded_rect(start_x, start_y, start_w, start_h, 8.0 * d);

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
        let compact = canvas_h < canvas_w;
        let ref_dim = canvas_w.min(canvas_h);

        // Background
        ctx.set_fill_style_str("#0a0a14");
        ctx.fill_rect(0.0, 0.0, canvas_w, canvas_h);

        // Title
        let title_size = (ref_dim * 0.06).min(40.0 * d).round();
        ctx.set_font(&format!("bold {title_size}px monospace"));
        ctx.set_fill_style_str("#c8e0ff");
        ctx.set_text_align("center");
        ctx.set_text_baseline("middle");
        let title_y = if compact { canvas_h * 0.08 } else { canvas_h * 0.12 };
        let _ = ctx.fill_text("SETTINGS", canvas_w / 2.0, title_y);

        // Back button
        let back_size = (12.0 * d).round();
        ctx.set_font(&format!("{back_size}px monospace"));
        ctx.set_fill_style_str("#8af");
        ctx.set_text_align("left");
        ctx.set_text_baseline("top");
        let _ = ctx.fill_text("< Back", 16.0 * d, 12.0 * d);

        // Settings rows
        let row_w = (ref_dim * 0.8).min(340.0 * d);
        let row_h = if compact { 38.0 * d } else { 44.0 * d };
        let gap = 8.0 * d;
        let row_x = (canvas_w - row_w) / 2.0;
        let pad = 14.0 * d;

        // Glyph Mode row
        let row_y = if compact { canvas_h * 0.18 } else { canvas_h * 0.25 };
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
        self.draw_toggle(toggle_x, toggle_y, toggle_w, toggle_h, self.glyph_mode, 11.0);

        // Description
        ctx.set_font(&self.font(9.0, ""));
        ctx.set_fill_style_str("#556");
        ctx.set_text_align("left");
        ctx.set_text_baseline("top");
        let _ = ctx.fill_text("Classic ASCII rendering", row_x + pad, row_y + row_h + 4.0 * d);

        // Controls row
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
