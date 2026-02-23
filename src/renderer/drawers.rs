use crate::game::{Drawer, Game, Item, ItemKind, QUICKBAR_SLOTS};
use crate::sprites;

use super::{item_kind_color, Renderer};

impl Renderer {
    // ---- Drawer: slide-up panel ----

    pub(super) fn draw_drawer(&self, game: &Game, canvas_w: f64, canvas_h: f64, bottom_h: f64) {
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
                let text_x = icon_x + eq_icon + 4.0 * d;
                ctx.set_font(&self.font(10.0, ""));
                ctx.set_fill_style_str(color);
                let _ = ctx.fill_text(item.name, text_x, sy + eq_h / 2.0 - 5.0 * d);
                // Durability indicator below item name
                if item.durability > 0 {
                    ctx.set_font(&self.font(8.0, ""));
                    ctx.set_fill_style_str("#777");
                    let _ = ctx.fill_text(&format!("Dur: {}", item.durability), text_x, sy + eq_h / 2.0 + 5.0 * d);
                }
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

                ctx.set_font(&self.font(11.0, ""));
                ctx.set_fill_style_str(item_kind_color(&item.kind));
                ctx.set_text_baseline("middle");
                let name_x = pad + icon_size + 8.0 * d;
                let _ = ctx.fill_text(item.name, name_x, iy + slot_h / 2.0);

                // Quick-bar slot badge (if this item is assigned to a slot)
                for s in 0..QUICKBAR_SLOTS {
                    if game.quick_bar.slots[s] == Some(idx) {
                        let name_w = item.name.len() as f64 * 6.5 * d;
                        let badge_x = name_x + name_w + 4.0 * d;
                        let badge_y = iy + slot_h / 2.0;
                        let badge_r = 7.0 * d;
                        ctx.set_fill_style_str("rgba(255,200,80,0.3)");
                        ctx.begin_path();
                        let _ = ctx.arc(badge_x + badge_r, badge_y, badge_r, 0.0, std::f64::consts::TAU);
                        ctx.fill();
                        ctx.set_font(&self.font(8.0, "bold"));
                        ctx.set_fill_style_str("#fc8");
                        ctx.set_text_align("center");
                        let _ = ctx.fill_text(&format!("{}", s + 1), badge_x + badge_r, badge_y);
                        ctx.set_text_align("left");
                        break;
                    }
                }

                // Item effect stats (right-aligned)
                let stat_text = if item.durability > 0 {
                    let base = match &item.effect {
                        crate::game::ItemEffect::BuffAttack(n) => format!("ATK +{}", n),
                        crate::game::ItemEffect::BuffDefense(n) => format!("DEF +{}", n),
                        _ => String::new(),
                    };
                    if base.is_empty() {
                        format!("D:{}", item.durability)
                    } else {
                        format!("{} D:{}", base, item.durability)
                    }
                } else {
                    match &item.effect {
                        crate::game::ItemEffect::Heal(n) => format!("+{} HP", n),
                        crate::game::ItemEffect::DamageAoe(n) => format!("AOE {}", n),
                        crate::game::ItemEffect::BuffAttack(n) => format!("ATK +{}", n),
                        crate::game::ItemEffect::BuffDefense(n) => format!("DEF +{}", n),
                        crate::game::ItemEffect::Feed(n, _) => format!("+{} food", n),
                        crate::game::ItemEffect::RestoreStamina(n) => format!("+{} stam", n),
                    }
                };
                ctx.set_font(&self.font(9.0, ""));
                ctx.set_fill_style_str("#777");
                ctx.set_text_align("right");
                ctx.set_text_baseline("middle");
                let _ = ctx.fill_text(&stat_text, text_right, iy + slot_h / 2.0);
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

                // Description text (truncated to fit available width)
                let desc = game.inventory_item_desc(sel_idx).unwrap_or_default();
                ctx.set_font(&self.font(10.0, ""));
                ctx.set_fill_style_str("#ccc");
                ctx.set_text_align("left");
                ctx.set_text_baseline("middle");
                let char_w = 6.0 * d;
                let max_chars = ((canvas_w - 2.0 * pad) / char_w).floor() as usize;
                let display_desc = if desc.len() > max_chars && max_chars > 3 {
                    format!("{}...", &desc[..max_chars - 3])
                } else {
                    desc
                };
                let _ = ctx.fill_text(&display_desc, pad, bar_y + detail_bar_h * 0.35);

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
                // Durability indicator (right-aligned)
                if item.durability > 0 {
                    ctx.set_font(&self.font(9.0, ""));
                    ctx.set_fill_style_str("#777");
                    ctx.set_text_align("right");
                    let _ = ctx.fill_text(&format!("D:{}", item.durability), canvas_w - pad, y + eq_icon / 2.0);
                    ctx.set_text_align("left");
                }
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
        self.draw_toggle(toggle_x, toggle_y, toggle_w, toggle_h, self.glyph_mode, 12.0);

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

        let r = 6.0 * d;
        ctx.set_fill_style_str("rgba(120,60,60,0.35)");
        self.fill_rounded_rect(menu_btn_x, menu_btn_y, menu_btn_w, menu_btn_h, r);
        ctx.set_stroke_style_str("rgba(180,80,80,0.4)");
        ctx.set_line_width(1.0 * d);
        self.stroke_rounded_rect(menu_btn_x, menu_btn_y, menu_btn_w, menu_btn_h, r);
        ctx.set_font(&self.font(12.0, "bold"));
        ctx.set_fill_style_str("#f88");
        ctx.set_text_align("center");
        ctx.set_text_baseline("middle");
        let _ = ctx.fill_text("Main Menu", menu_btn_x + menu_btn_w / 2.0, menu_btn_y + menu_btn_h / 2.0);

        ctx.restore();
    }
}
