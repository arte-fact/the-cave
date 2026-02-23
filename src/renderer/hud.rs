use crate::game::{Drawer, Game, Item, QUICKBAR_SLOTS};
use crate::sprites;

use super::{
    item_kind_color, Renderer, StatBar,
    COLOR_HP_HIGH, COLOR_HP_MED, COLOR_HP_LOW, COLOR_HP_BG,
    COLOR_STAM, COLOR_STAM_SPRINT, COLOR_STAM_BG,
    COLOR_HUNGER_HIGH, COLOR_HUNGER_MED, COLOR_HUNGER_LOW, COLOR_HUNGER_BG,
    COLOR_PANEL_BG, COLOR_PANEL_ACCENT, COLOR_BAR_BG,
    COLOR_TEXT, COLOR_TEXT_DIM, COLOR_TEXT_MUTED, COLOR_TEXT_HEADING,
    COLOR_STAT_LABEL, COLOR_XP_TEXT,
    COLOR_BTN_USE_TEXT, COLOR_BTN_DROP_TEXT,
};

impl Renderer {
    // ---- Landscape compact top bar (stats + combat info) ----

    pub(super) fn draw_landscape_top_bar(&self, game: &Game, bar_w: f64, top_h: f64) {
        let ctx = &self.ctx;
        let d = self.dpr;
        let pad = 8.0 * d;

        // Background
        ctx.set_fill_style_str(COLOR_BAR_BG);
        ctx.fill_rect(0.0, 0.0, bar_w, top_h);
        // Bottom accent
        ctx.set_fill_style_str(COLOR_PANEL_ACCENT);
        ctx.fill_rect(0.0, top_h - 1.0 * d, bar_w, 1.0 * d);

        let bar_h = 8.0 * d;
        let bar_gap = 2.0 * d;
        let bar_r = 2.0 * d;
        let text_inset = 3.0 * d;
        let bars_w = bar_w * 0.44;

        // --- Row 1: HP bar ---
        let row1_y = 3.0 * d;
        let hp_frac = game.player_hp as f64 / game.player_max_hp as f64;
        let hp_color = if hp_frac > 0.5 { COLOR_HP_HIGH } else if hp_frac > 0.25 { COLOR_HP_MED } else { COLOR_HP_LOW };
        self.draw_stat_bar(&StatBar {
            x: pad, y: row1_y, w: bars_w, h: bar_h, r: bar_r, frac: hp_frac,
            bg_color: COLOR_HP_BG, fill_color: hp_color,
            label: &format!("HP {}/{}", game.player_hp, game.player_max_hp),
            font_size: 7.0, text_inset,
        });

        // --- Row 2: Stamina bar ---
        let row2_y = row1_y + bar_h + bar_gap;
        let stam_frac = game.stamina as f64 / game.max_stamina as f64;
        let stam_color = if game.sprinting { COLOR_STAM_SPRINT } else { COLOR_STAM };
        let sprint_label = if game.sprinting { "STA (SPRINT)" } else { "STA" };
        self.draw_stat_bar(&StatBar {
            x: pad, y: row2_y, w: bars_w, h: bar_h, r: bar_r, frac: stam_frac,
            bg_color: COLOR_STAM_BG, fill_color: stam_color,
            label: &format!("{} {}/{}", sprint_label, game.stamina, game.max_stamina),
            font_size: 7.0, text_inset,
        });

        // --- Row 3: Hunger bar ---
        let row3_y = row2_y + bar_h + bar_gap;
        let hunger_frac = game.hunger as f64 / game.max_hunger as f64;
        let hunger_color = if hunger_frac > 0.3 { COLOR_HUNGER_HIGH } else if hunger_frac > 0.1 { COLOR_HUNGER_MED } else { COLOR_HUNGER_LOW };
        self.draw_stat_bar(&StatBar {
            x: pad, y: row3_y, w: bars_w, h: bar_h, r: bar_r, frac: hunger_frac,
            bg_color: COLOR_HUNGER_BG, fill_color: hunger_color,
            label: &format!("FOOD {}/{}", game.hunger, game.max_hunger),
            font_size: 7.0, text_inset,
        });

        // --- Right side: location + combat stats + XP ---
        ctx.set_text_align("right");
        ctx.set_text_baseline("middle");

        // Right-side max width: prevent overlap with left stat bars
        let right_max_w = bar_w - pad - bars_w - pad * 2.0;

        // Location (top right)
        ctx.set_font(&self.font(9.0, "bold"));
        ctx.set_fill_style_str(COLOR_TEXT_HEADING);
        self.fill_text_truncated(&game.location_name(), bar_w - pad - right_max_w, row1_y + bar_h / 2.0, right_max_w);

        // ATK / DEF / LVL
        let atk = game.effective_attack();
        let def = game.effective_defense();
        ctx.set_font(&self.font(8.0, ""));
        ctx.set_fill_style_str(COLOR_STAT_LABEL);
        ctx.set_text_align("right");
        let _ = ctx.fill_text(
            &format!("ATK {} DEF {} LVL {}", atk, def, game.player_level),
            bar_w - pad, row2_y + bar_h / 2.0,
        );

        // XP
        let xp_needed = game.xp_to_next_level();
        ctx.set_fill_style_str(COLOR_XP_TEXT);
        let _ = ctx.fill_text(
            &format!("XP {}/{}", game.player_xp, xp_needed),
            bar_w - pad, row3_y + bar_h / 2.0,
        );
    }

    // ---- Landscape bottom message strip ----

    pub(super) fn draw_landscape_messages(&self, game: &Game, bar_w: f64, canvas_h: f64, msg_h: f64) {
        let ctx = &self.ctx;
        let d = self.dpr;
        let msg_y = canvas_h - msg_h;
        let pad = 8.0 * d;

        // Background
        ctx.set_fill_style_str("rgba(0,0,0,0.6)");
        ctx.fill_rect(0.0, msg_y, bar_w, msg_h);
        // Top accent
        ctx.set_fill_style_str("rgba(255,255,255,0.06)");
        ctx.fill_rect(0.0, msg_y, bar_w, 1.0 * d);

        let msg_count = game.messages.len();
        let show = msg_count.min(2);
        if show == 0 { return; }

        let msg_max_w = bar_w - pad * 2.0;
        ctx.set_font(&self.font(8.0, ""));
        ctx.set_text_align("left");
        ctx.set_text_baseline("middle");
        let line_h = 11.0 * d;
        let start_y = msg_y + (msg_h - show as f64 * line_h) / 2.0 + line_h / 2.0;

        for i in 0..show {
            let msg = &game.messages[msg_count - show + i];
            let color = if i == show - 1 { "#bbb" } else { "#666" };
            ctx.set_fill_style_str(color);
            self.fill_text_truncated(msg, pad, start_y + i as f64 * line_h, msg_max_w);
        }
    }

    // ---- Landscape side panel (permanent right panel, interactive elements only) ----

    pub(super) fn draw_side_panel(&self, game: &Game, panel_x: f64, panel_w: f64, canvas_h: f64) {
        let ctx = &self.ctx;
        let d = self.dpr;
        let pad = 10.0 * d;
        let inner_w = panel_w - pad * 2.0;

        // Panel background
        ctx.set_fill_style_str(COLOR_PANEL_BG);
        ctx.fill_rect(panel_x, 0.0, panel_w, canvas_h);
        // Left border accent
        ctx.set_fill_style_str("rgba(80,130,255,0.2)");
        ctx.fill_rect(panel_x, 0.0, 1.0 * d, canvas_h);

        let mut y = pad;
        let x = panel_x + pad;

        // --- Tile detail (if inspecting) ---
        if let Some(ref info) = game.ui.inspected {
            let detail_h = 44.0 * d;
            ctx.set_fill_style_str("rgba(0,180,255,0.08)");
            self.fill_rounded_rect(x, y, inner_w, detail_h, 4.0 * d);

            let icon_size = 24.0 * d;
            let text_x = x + icon_size + 6.0 * d;

            let detail_text_max = x + inner_w - text_x;

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
                self.fill_text_truncated(ei.name, text_x, y + 4.0 * d, detail_text_max);
                ctx.set_font(&self.font(8.0, ""));
                ctx.set_fill_style_str(COLOR_TEXT);
                self.fill_text_truncated(&format!("HP {} ATK {}", ei.hp, ei.attack), text_x, y + 18.0 * d, detail_text_max);
            } else if let Some(ref ii) = info.item {
                if let Some(gi) = game.ground_items.iter().find(|gi| gi.item.name == ii.name) {
                    let sprite = sprites::item_sprite(gi.item.name);
                    self.draw_sprite(sprite, x + 2.0 * d, y + (detail_h - icon_size) / 2.0, icon_size, icon_size);
                }
                ctx.set_font(&self.font(10.0, "bold"));
                ctx.set_fill_style_str("#ff0");
                ctx.set_text_align("left");
                ctx.set_text_baseline("top");
                self.fill_text_truncated(ii.name, text_x, y + 4.0 * d, detail_text_max);
                ctx.set_font(&self.font(8.0, ""));
                ctx.set_fill_style_str(COLOR_TEXT);
                self.fill_text_truncated(&ii.desc, text_x, y + 18.0 * d, detail_text_max);
            } else if info.is_player {
                ctx.set_font(&self.font(10.0, "bold"));
                ctx.set_fill_style_str("#fff");
                ctx.set_text_align("left");
                ctx.set_text_baseline("top");
                let _ = ctx.fill_text("You", text_x, y + 4.0 * d);
            } else {
                ctx.set_font(&self.font(10.0, "bold"));
                ctx.set_fill_style_str(COLOR_TEXT);
                ctx.set_text_align("left");
                ctx.set_text_baseline("top");
                self.fill_text_truncated(info.tile_name, x + 4.0 * d, y + 4.0 * d, inner_w - 8.0 * d);
                ctx.set_font(&self.font(8.0, ""));
                ctx.set_fill_style_str(COLOR_TEXT_DIM);
                self.fill_text_truncated(info.tile_desc, x + 4.0 * d, y + 18.0 * d, inner_w - 8.0 * d);
            }
            y += detail_h + 6.0 * d;
        }

        // --- Quick-use bar (horizontal row of 4 slots) ---
        {
            let slot_size = 30.0 * d;
            let slot_pad = 4.0 * d;
            let total_slot_w = QUICKBAR_SLOTS as f64 * (slot_size + slot_pad) - slot_pad;
            let slot_start_x = x + (inner_w - total_slot_w) / 2.0;
            let radius = 4.0 * d;

            for i in 0..QUICKBAR_SLOTS {
                let sx = slot_start_x + i as f64 * (slot_size + slot_pad);
                if let Some(inv_idx) = game.quick_bar.slots[i] {
                    if let Some(item) = game.inventory.get(inv_idx) {
                        let color = item_kind_color(&item.kind);
                        ctx.set_fill_style_str("rgba(30,30,40,0.9)");
                        self.fill_rounded_rect(sx, y, slot_size, slot_size, radius);
                        ctx.set_stroke_style_str(color);
                        ctx.set_line_width(1.0 * d);
                        self.stroke_rounded_rect(sx, y, slot_size, slot_size, radius);
                        let sprite = sprites::item_sprite(item.name);
                        let inset = 2.0 * d;
                        let spr_size = slot_size - inset * 2.0;
                        if !self.draw_sprite(sprite, sx + inset, y + inset, spr_size, spr_size) {
                            ctx.set_font(&self.font(14.0, "bold"));
                            ctx.set_fill_style_str(color);
                            ctx.set_text_align("center");
                            ctx.set_text_baseline("middle");
                            let _ = ctx.fill_text(
                                &item.glyph.to_string(),
                                sx + slot_size / 2.0,
                                y + slot_size / 2.0,
                            );
                        }
                        ctx.set_font(&self.font(7.0, "bold"));
                        ctx.set_fill_style_str("rgba(255,255,255,0.5)");
                        ctx.set_text_align("left");
                        ctx.set_text_baseline("top");
                        let _ = ctx.fill_text(&format!("{}", i + 1), sx + 2.0 * d, y + 1.0 * d);
                    } else {
                        self.draw_empty_quickbar_slot(sx, y, slot_size, radius, i, d);
                    }
                } else {
                    self.draw_empty_quickbar_slot(sx, y, slot_size, radius, i, d);
                }
            }
            y += slot_size + 6.0 * d;
        }

        // --- Action buttons (2×2 grid) ---
        let btn_h = 30.0 * d;
        let btn_gap = 4.0 * d;
        let btn_w = (inner_w - btn_gap) / 2.0;

        let sprint_color = if game.sprinting { "#4ef" } else { "#48a" };
        let sprint_label = if game.sprinting { "SPRINT" } else { "Sprint" };

        let buttons: [(&str, &str, bool); 4] = [
            ("Inventory", if game.ui.drawer == Drawer::Inventory { "#8af" } else { "#58f" }, game.ui.drawer == Drawer::Inventory),
            ("Stats", if game.ui.drawer == Drawer::Stats { "#c8f" } else { "#a8f" }, game.ui.drawer == Drawer::Stats),
            (sprint_label, sprint_color, game.sprinting),
            ("Settings", if game.ui.drawer == Drawer::Settings { "#ccc" } else { "#888" }, game.ui.drawer == Drawer::Settings),
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
        let which = if game.ui.drawer != Drawer::None {
            game.ui.drawer
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
        }
        // Messages are shown in the bottom bar (draw_landscape_messages), not here.
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
        for (i, &(slot, color, empty_label)) in eq_slots.iter().enumerate() {
            let is_selected_eq = game.selected_equipment_slot == Some(i);
            if is_selected_eq {
                ctx.set_fill_style_str("rgba(80,130,255,0.25)");
            } else {
                ctx.set_fill_style_str("rgba(255,255,255,0.04)");
            }
            self.fill_rounded_rect(x, cy, w, eq_h, 3.0 * d);
            if let Some(ref item) = slot {
                let sprite = sprites::item_sprite(item.name);
                self.draw_sprite(sprite, x + pad, cy + (eq_h - eq_icon) / 2.0, eq_icon, eq_icon);
                let eq_text_max = if is_selected_eq {
                    // Reserve space for the inline "Unequip" button
                    w - eq_icon - pad * 3.0 - 40.0 * d
                } else {
                    w - eq_icon - pad * 3.0
                };
                ctx.set_font(&self.font(8.0, ""));
                ctx.set_fill_style_str(color);
                self.fill_text_truncated(item.name, x + eq_icon + pad * 2.0, cy + eq_h / 2.0, eq_text_max);

                // Show inline Unequip button when selected
                if is_selected_eq {
                    let ubtn_w = 36.0 * d;
                    let ubtn_h = 14.0 * d;
                    let ubtn_x = x + w - ubtn_w - pad;
                    let ubtn_y = cy + (eq_h - ubtn_h) / 2.0;
                    ctx.set_fill_style_str("rgba(200,160,80,0.3)");
                    self.fill_rounded_rect(ubtn_x, ubtn_y, ubtn_w, ubtn_h, 2.0 * d);
                    ctx.set_font(&self.font(6.0, "bold"));
                    ctx.set_fill_style_str("#fc8");
                    ctx.set_text_align("center");
                    let _ = ctx.fill_text("Unequip", ubtn_x + ubtn_w / 2.0, cy + eq_h / 2.0);
                    ctx.set_text_align("left");
                }
            } else {
                ctx.set_font(&self.font(8.0, ""));
                ctx.set_fill_style_str("#444");
                self.fill_text_truncated(empty_label, x + pad, cy + eq_h / 2.0, w - pad * 2.0);
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
            ctx.set_fill_style_str(COLOR_TEXT_MUTED);
            ctx.set_font(&self.font(9.0, ""));
            ctx.set_text_baseline("top");
            let _ = ctx.fill_text("No items", x + pad, cy);
        } else {
            let scroll = game.ui.inventory_scroll;
            let total = game.inventory.len();
            let end = (scroll + max_visible).min(total);

            // Action buttons width to reserve
            let btn_w_reserved = 24.0 * d * 2.0 + 4.0 * d;

            for (vi, idx) in (scroll..end).enumerate() {
                let item = &game.inventory[idx];
                let iy = cy + vi as f64 * slot_h;

                if game.ui.selected_inventory_item == Some(idx) {
                    ctx.set_fill_style_str("rgba(80,130,255,0.18)");
                    ctx.fill_rect(x, iy, w, slot_h);
                } else if vi % 2 == 0 {
                    ctx.set_fill_style_str("rgba(255,255,255,0.03)");
                    ctx.fill_rect(x, iy, w, slot_h);
                }

                let sprite = sprites::item_sprite(item.name);
                self.draw_sprite(sprite, x + pad, iy + (slot_h - icon_size) / 2.0, icon_size, icon_size);

                ctx.set_font(&self.font(8.0, ""));
                ctx.set_fill_style_str(item_kind_color(&item.kind));
                ctx.set_text_baseline("middle");
                let name_x = x + icon_size + pad * 2.0;
                let name_max_w = w - icon_size - pad * 3.0 - btn_w_reserved;
                self.fill_text_truncated(item.name, name_x, iy + slot_h / 2.0, name_max_w);

                // Quick-bar slot badge
                for s in 0..QUICKBAR_SLOTS {
                    if game.quick_bar.slots[s] == Some(idx) {
                        let measured_name_w = ctx.measure_text(item.name).ok().map(|m| m.width()).unwrap_or(0.0);
                        let name_w = measured_name_w.min(name_max_w);
                        let badge_x = name_x + name_w + 3.0 * d;
                        let badge_y = iy + slot_h / 2.0;
                        let badge_r = 5.0 * d;
                        ctx.set_fill_style_str("rgba(255,200,80,0.3)");
                        ctx.begin_path();
                        let _ = ctx.arc(badge_x + badge_r, badge_y, badge_r, 0.0, std::f64::consts::TAU);
                        ctx.fill();
                        ctx.set_font(&self.font(6.0, "bold"));
                        ctx.set_fill_style_str("#fc8");
                        ctx.set_text_align("center");
                        let _ = ctx.fill_text(&format!("{}", s + 1), badge_x + badge_r, badge_y);
                        ctx.set_text_align("left");
                        break;
                    }
                }

                // Inline action buttons
                let btn_w = 24.0 * d;
                let btn_h = 16.0 * d;
                let btn_y = iy + (slot_h - btn_h) / 2.0;
                let drop_x = x + w - btn_w - 2.0 * d;
                let use_x = drop_x - btn_w - 2.0 * d;

                let action_label = if item.kind.is_consumable() { "U" } else { "E" };
                ctx.set_fill_style_str("rgba(80,200,120,0.2)");
                self.fill_rounded_rect(use_x, btn_y, btn_w, btn_h, 2.0 * d);
                ctx.set_font(&self.font(7.0, "bold"));
                ctx.set_fill_style_str(COLOR_BTN_USE_TEXT);
                ctx.set_text_align("center");
                let _ = ctx.fill_text(action_label, use_x + btn_w / 2.0, iy + slot_h / 2.0);

                ctx.set_fill_style_str("rgba(200,80,80,0.2)");
                self.fill_rounded_rect(drop_x, btn_y, btn_w, btn_h, 2.0 * d);
                ctx.set_fill_style_str(COLOR_BTN_DROP_TEXT);
                let _ = ctx.fill_text("X", drop_x + btn_w / 2.0, iy + slot_h / 2.0);

                ctx.set_text_align("left");
            }

            // Slot count
            ctx.set_font(&self.font(8.0, ""));
            ctx.set_fill_style_str(COLOR_TEXT_MUTED);
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
        ctx.set_fill_style_str(COLOR_XP_TEXT);
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
            ctx.set_fill_style_str(COLOR_TEXT_DIM);
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
                let eq_name_x = x + eq_icon + 4.0 * d;
                self.fill_text_truncated(item.name, eq_name_x, cy + eq_icon / 2.0, x + w - eq_name_x);
            } else {
                ctx.set_font(&self.font(8.0, ""));
                ctx.set_fill_style_str("#444");
                ctx.set_text_baseline("top");
                self.fill_text_truncated(empty_label, x, cy, w);
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
        ctx.set_fill_style_str(COLOR_TEXT);
        ctx.set_text_align("left");
        ctx.set_text_baseline("middle");
        let row_h = 30.0 * d;
        let _ = ctx.fill_text("Glyph Mode", x, cy + row_h / 2.0);

        let toggle_w = 50.0 * d;
        let toggle_h = 22.0 * d;
        let toggle_x = x + w - toggle_w;
        let toggle_y = cy + (row_h - toggle_h) / 2.0;
        self.draw_toggle(toggle_x, toggle_y, toggle_w, toggle_h, self.glyph_mode, 9.0);
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

    // ---- Top bar: player stats ----

    pub(super) fn draw_top_bar(&self, game: &Game, canvas_w: f64, top_h: f64) {
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
        let hp_color = if hp_frac > 0.5 { COLOR_HP_HIGH } else if hp_frac > 0.25 { COLOR_HP_MED } else { COLOR_HP_LOW };
        self.draw_stat_bar(&StatBar {
            x: bar_x, y: row1_y, w: bar_w, h: bar_h, r: bar_r, frac: hp_frac,
            bg_color: COLOR_HP_BG, fill_color: hp_color,
            label: &format!("HP {}/{}", game.player_hp, game.player_max_hp),
            font_size: 10.0, text_inset,
        });

        // Row 2: Stamina bar
        let row2_y = row1_y + bar_h + bar_gap;
        let stam_frac = game.stamina as f64 / game.max_stamina as f64;
        let stam_color = if game.sprinting { COLOR_STAM_SPRINT } else { COLOR_STAM };
        let sprint_label = if game.sprinting { "STA (SPRINT)" } else { "STA" };
        self.draw_stat_bar(&StatBar {
            x: bar_x, y: row2_y, w: bar_w, h: bar_h, r: bar_r, frac: stam_frac,
            bg_color: COLOR_STAM_BG, fill_color: stam_color,
            label: &format!("{} {}/{}", sprint_label, game.stamina, game.max_stamina),
            font_size: 10.0, text_inset,
        });

        // Row 3: Hunger bar
        let row3_y = row2_y + bar_h + bar_gap;
        let hunger_frac = game.hunger as f64 / game.max_hunger as f64;
        let hunger_color = if hunger_frac > 0.3 { COLOR_HUNGER_HIGH } else if hunger_frac > 0.1 { COLOR_HUNGER_MED } else { COLOR_HUNGER_LOW };
        self.draw_stat_bar(&StatBar {
            x: bar_x, y: row3_y, w: bar_w, h: bar_h, r: bar_r, frac: hunger_frac,
            bg_color: COLOR_HUNGER_BG, fill_color: hunger_color,
            label: &format!("FOOD {}/{}", game.hunger, game.max_hunger),
            font_size: 10.0, text_inset,
        });

        // Right column: location + stats + XP
        let right_max_w = canvas_w - pad - bar_w - pad * 2.0;
        ctx.set_text_align("left");
        ctx.set_fill_style_str(COLOR_TEXT);
        ctx.set_font(&self.font(11.0, "bold"));
        self.fill_text_truncated(&game.location_name(), bar_x + bar_w + pad, row1_y + bar_h / 2.0, right_max_w);
        ctx.set_text_align("right");

        let atk = game.effective_attack();
        let def = game.effective_defense();
        ctx.set_font(&self.font(10.0, ""));
        ctx.set_fill_style_str(COLOR_STAT_LABEL);
        let _ = ctx.fill_text(
            &format!("ATK {} DEF {} LVL {}", atk, def, game.player_level),
            canvas_w - pad, row2_y + bar_h / 2.0,
        );

        let xp_needed = game.xp_to_next_level();
        ctx.set_fill_style_str(COLOR_XP_TEXT);
        let _ = ctx.fill_text(
            &format!("XP {}/{}", game.player_xp, xp_needed),
            canvas_w - pad, row3_y + bar_h / 2.0,
        );
    }

    // ---- Tile detail strip (below top bar, shown when inspecting) ----

    pub(super) fn draw_tile_detail(&self, game: &Game, canvas_w: f64, top_h: f64) {
        let info = match &game.ui.inspected {
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

        let detail_max_w = canvas_w - text_x - pad;

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
            self.fill_text_truncated(ei.name, text_x, line1_y, detail_max_w);

            ctx.set_font(&self.font(11.0, ""));
            ctx.set_fill_style_str(COLOR_TEXT);
            self.fill_text_truncated(&format!("HP {} ATK {} DEF {}", ei.hp, ei.attack, ei.defense), text_x, line2_y, detail_max_w);

            ctx.set_fill_style_str("#999");
            self.fill_text_truncated(ei.desc, text_x, line3_y, detail_max_w);
        } else if let Some(ref ii) = info.item {
            if let Some(gi) = game.ground_items.iter().find(|gi| gi.item.name == ii.name) {
                let sprite = sprites::item_sprite(gi.item.name);
                self.draw_sprite(sprite, pad, y0 + (strip_h - icon_size) / 2.0, icon_size, icon_size);
            }

            ctx.set_font(&self.font(13.0, "bold"));
            ctx.set_fill_style_str("#ff0");
            ctx.set_text_align("left");
            ctx.set_text_baseline("top");
            self.fill_text_truncated(ii.name, text_x, line1_y, detail_max_w);

            ctx.set_font(&self.font(11.0, ""));
            ctx.set_fill_style_str(COLOR_TEXT);
            self.fill_text_truncated(&ii.desc, text_x, line2_y, detail_max_w);
        } else if info.is_player {
            let sprite = sprites::player_sprite();
            self.draw_sprite(sprite, pad, y0 + (strip_h - icon_size) / 2.0, icon_size, icon_size);

            ctx.set_font(&self.font(13.0, "bold"));
            ctx.set_fill_style_str("#fff");
            ctx.set_text_align("left");
            ctx.set_text_baseline("top");
            let _ = ctx.fill_text("You", text_x, line1_y);

            ctx.set_font(&self.font(11.0, ""));
            ctx.set_fill_style_str(COLOR_TEXT);
            self.fill_text_truncated(
                &format!("HP {}/{} ATK {} DEF {}",
                    game.player_hp, game.player_max_hp,
                    game.effective_attack(), game.effective_defense()),
                text_x, line2_y, detail_max_w,
            );
        } else {
            ctx.set_font(&self.font(13.0, "bold"));
            ctx.set_fill_style_str(COLOR_TEXT);
            ctx.set_text_align("left");
            ctx.set_text_baseline("top");
            self.fill_text_truncated(info.tile_name, pad, line1_y, canvas_w - pad * 2.0);

            ctx.set_font(&self.font(11.0, ""));
            ctx.set_fill_style_str(COLOR_TEXT_DIM);
            self.fill_text_truncated(info.tile_desc, pad, line2_y, canvas_w - pad * 2.0);
        }
    }

    // ---- Messages (above bottom bar) ----

    pub(super) fn draw_messages(&self, game: &Game, canvas_w: f64, canvas_h: f64, bottom_h: f64, msg_h: f64) {
        let ctx = &self.ctx;
        let d = self.dpr;
        let msg_bottom = canvas_h - bottom_h;
        let msg_top = msg_bottom - msg_h;

        ctx.set_fill_style_str("rgba(0,0,0,0.5)");
        ctx.fill_rect(0.0, msg_top, canvas_w, msg_h);

        let msg_count = game.messages.len();
        let show = msg_count.min(3);
        let msg_max_w = canvas_w - 16.0 * d;
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
            self.fill_text_truncated(msg, 8.0 * d, y, msg_max_w);
        }
    }

    // ---- Quick-use bar (portrait, above bottom bar) ----

    pub(super) fn draw_quick_bar(&self, game: &Game, canvas_w: f64, canvas_h: f64, bottom_h: f64, qbar_h: f64) {
        let ctx = &self.ctx;
        let d = self.dpr;
        let bar_y = canvas_h - bottom_h - qbar_h;

        // Background
        ctx.set_fill_style_str("rgba(12,12,18,0.92)");
        ctx.fill_rect(0.0, bar_y, canvas_w, qbar_h);
        // Top accent line
        ctx.set_fill_style_str("rgba(255,255,255,0.08)");
        ctx.fill_rect(0.0, bar_y, canvas_w, 1.0);

        let slot_size = 36.0 * d;
        let slot_pad = 6.0 * d;
        let total_w = QUICKBAR_SLOTS as f64 * (slot_size + slot_pad) - slot_pad;
        let start_x = (canvas_w - total_w) / 2.0;
        let slot_y = bar_y + (qbar_h - slot_size) / 2.0;
        let radius = 5.0 * d;

        for i in 0..QUICKBAR_SLOTS {
            let sx = start_x + i as f64 * (slot_size + slot_pad);

            if let Some(inv_idx) = game.quick_bar.slots[i] {
                // Occupied slot
                if let Some(item) = game.inventory.get(inv_idx) {
                    // Slot background with item kind tint
                    let color = item_kind_color(&item.kind);
                    ctx.set_fill_style_str("rgba(30,30,40,0.9)");
                    self.fill_rounded_rect(sx, slot_y, slot_size, slot_size, radius);

                    // Colored border
                    ctx.set_stroke_style_str(color);
                    ctx.set_line_width(1.5 * d);
                    self.stroke_rounded_rect(sx, slot_y, slot_size, slot_size, radius);

                    // Item sprite
                    let sprite = sprites::item_sprite(item.name);
                    let inset = 3.0 * d;
                    let spr_size = slot_size - inset * 2.0;
                    if !self.draw_sprite(sprite, sx + inset, slot_y + inset, spr_size, spr_size) {
                        // Fallback: glyph
                        ctx.set_font(&self.font(18.0, "bold"));
                        ctx.set_fill_style_str(color);
                        ctx.set_text_align("center");
                        ctx.set_text_baseline("middle");
                        let _ = ctx.fill_text(
                            &item.glyph.to_string(),
                            sx + slot_size / 2.0,
                            slot_y + slot_size / 2.0,
                        );
                    }

                    // Slot number label (top-left corner)
                    ctx.set_font(&self.font(8.0, "bold"));
                    ctx.set_fill_style_str("rgba(255,255,255,0.5)");
                    ctx.set_text_align("left");
                    ctx.set_text_baseline("top");
                    let _ = ctx.fill_text(&format!("{}", i + 1), sx + 3.0 * d, slot_y + 2.0 * d);

                    // Flash overlay on use
                    let flash = self.quickbar_flash[i];
                    if flash > 0.0 {
                        ctx.save();
                        ctx.set_global_alpha(flash * 0.6);
                        ctx.set_fill_style_str("#fff");
                        self.fill_rounded_rect(sx, slot_y, slot_size, slot_size, radius);
                        ctx.restore();
                    }
                } else {
                    // Stale reference — draw as empty
                    self.draw_empty_quickbar_slot(sx, slot_y, slot_size, radius, i, d);
                }
            } else {
                // Empty slot
                self.draw_empty_quickbar_slot(sx, slot_y, slot_size, radius, i, d);
            }
        }
    }

    fn draw_empty_quickbar_slot(&self, sx: f64, sy: f64, size: f64, radius: f64, index: usize, d: f64) {
        let ctx = &self.ctx;
        // Dim background
        ctx.set_fill_style_str("rgba(30,30,40,0.5)");
        self.fill_rounded_rect(sx, sy, size, size, radius);
        // Dashed border effect (solid dim border)
        ctx.set_stroke_style_str("rgba(255,255,255,0.12)");
        ctx.set_line_width(1.0 * d);
        self.stroke_rounded_rect(sx, sy, size, size, radius);
        // Slot number
        ctx.set_font(&self.font(10.0, ""));
        ctx.set_fill_style_str("rgba(255,255,255,0.2)");
        ctx.set_text_align("center");
        ctx.set_text_baseline("middle");
        let _ = ctx.fill_text(&format!("{}", index + 1), sx + size / 2.0, sy + size / 2.0);
    }

    // ---- Bottom action bar ----

    pub(super) fn draw_bottom_bar(&self, game: &Game, canvas_w: f64, canvas_h: f64, bottom_h: f64) {
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
            ("Inventory", if game.ui.drawer == Drawer::Inventory { "#8af" } else { "#58f" }, game.ui.drawer == Drawer::Inventory),
            ("Stats", if game.ui.drawer == Drawer::Stats { "#c8f" } else { "#a8f" }, game.ui.drawer == Drawer::Stats),
            (sprint_label, sprint_color, game.sprinting),
            ("Settings", if game.ui.drawer == Drawer::Settings { "#ccc" } else { "#888" }, game.ui.drawer == Drawer::Settings),
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
}
