use crate::game::{Drawer, SkillKind, QUICKBAR_SLOTS};

// ---- UI layout constants (CSS pixels, pre-DPR) ----

/// Number of bottom-bar buttons (Inventory, Stats, Sprint, Settings).
const BAR_BUTTON_COUNT: f64 = 4.0;
/// Maximum width of a single bottom-bar button.
const BAR_BUTTON_MAX_W: f64 = 110.0;

/// Portrait quick-bar slot size (CSS px).
pub(crate) const QBAR_SLOT_SIZE: f64 = 36.0;
/// Portrait quick-bar slot gap (CSS px).
pub(crate) const QBAR_SLOT_PAD: f64 = 6.0;

/// Landscape side-panel padding.
const PANEL_PAD: f64 = 10.0;
/// Landscape side-panel button height.
const PANEL_BTN_H: f64 = 30.0;
/// Landscape side-panel button gap.
const PANEL_BTN_GAP: f64 = 4.0;
/// Landscape quick-bar slot size.
const PANEL_QBAR_SLOT_SIZE: f64 = 30.0;
/// Landscape quick-bar slot gap.
const PANEL_QBAR_SLOT_PAD: f64 = 4.0;
/// Landscape quick-bar total row height (slot + gap).
const PANEL_QBAR_ROW_H: f64 = 36.0;
/// Landscape tile-detail strip height (44 + 6 gap).
const PANEL_DETAIL_H: f64 = 50.0;

/// Portrait drawer fraction of canvas height (inventory/stats).
const DRAWER_FRAC: f64 = 0.55;
/// Portrait settings drawer fraction.
const SETTINGS_DRAWER_FRAC: f64 = 0.45;
/// Portrait drawer padding.
const DRAWER_PAD: f64 = 12.0;

/// Portrait inventory: equipment row height.
const INV_EQ_H: f64 = 30.0;
/// Portrait inventory: equipment row gap.
const INV_EQ_GAP: f64 = 4.0;
/// Portrait inventory: item slot height.
const INV_SLOT_H: f64 = 34.0;

/// Landscape inventory: equipment row height.
const INV_EQ_H_LANDSCAPE: f64 = 22.0;
/// Landscape inventory: equipment row gap.
const INV_EQ_GAP_LANDSCAPE: f64 = 2.0;
/// Landscape inventory: item slot height.
const INV_SLOT_H_LANDSCAPE: f64 = 26.0;

/// CSS coordinates for hit-testing (pre-DPR).
pub(crate) struct CssHitArea {
    pub x: f64,
    pub y: f64,
    pub w: f64,
    pub h: f64,
}

/// State needed for drawer hit-testing.
pub(crate) struct DrawerHitState {
    pub drawer: Drawer,
    pub item_count: usize,
    pub inventory_scroll: usize,
    pub skill_points: u32,
    pub selected_eq_slot: Option<usize>,
}

/// Result of tapping the bottom bar.
pub(crate) enum BarTap {
    OpenDrawer(Drawer),
    Sprint,
}

/// Result of tapping inside a drawer.
pub(crate) enum DrawerTap {
    /// Tapped an inventory item at the given index (absolute, not visual).
    InventoryItem(usize),
    /// Tapped an equipment slot (0–5).
    EquipmentSlot(usize),
    /// Unequip the item in the given equipment slot.
    Unequip(usize),
    /// Scroll the inventory list up.
    ScrollUp,
    /// Scroll the inventory list down.
    ScrollDown,
    /// Use/Equip button tapped for the selected item (detail bar).
    UseEquip(usize),
    /// Drop button tapped for the selected item (detail bar).
    Drop(usize),
    /// Allocate a skill point to the given attribute.
    StatsAllocate(SkillKind),
    /// Toggle glyph rendering mode.
    ToggleGlyphMode,
    /// Return to main menu from in-game settings.
    MainMenu,
    /// Tapped inside the drawer but not on an actionable element.
    Consumed,
}

/// Bottom bar button layout: returns which button was tapped (if any).
/// `css_y` and `canvas_h_css` are in CSS pixels (pre-DPR).
pub(crate) fn hit_test_bottom_bar(css_x: f64, css_y: f64, css_w: f64, css_h: f64, bar_h_css: f64) -> Option<BarTap> {
    let bar_top = css_h - bar_h_css;
    if css_y < bar_top {
        return None;
    }
    let btn_w = (css_w / BAR_BUTTON_COUNT).min(BAR_BUTTON_MAX_W);
    let total_w = btn_w * BAR_BUTTON_COUNT;
    let start_x = (css_w - total_w) / 2.0;
    let idx = ((css_x - start_x) / btn_w).floor() as i32;
    match idx {
        0 => Some(BarTap::OpenDrawer(Drawer::Inventory)),
        1 => Some(BarTap::OpenDrawer(Drawer::Stats)),
        2 => Some(BarTap::Sprint),
        3 => Some(BarTap::OpenDrawer(Drawer::Settings)),
        _ => None,
    }
}

/// Hit-test the quick-use bar (portrait mode). Returns the slot index (0–3) if tapped.
/// The quick bar sits above the bottom bar: top = css_h - bar_h_css - qbar_h_css.
pub(crate) fn hit_test_quick_bar(
    css_x: f64,
    css_y: f64,
    css_w: f64,
    css_h: f64,
    bar_h_css: f64,
    qbar_h_css: f64,
) -> Option<usize> {
    let qbar_top = css_h - bar_h_css - qbar_h_css;
    let qbar_bottom = css_h - bar_h_css;
    if css_y < qbar_top || css_y >= qbar_bottom {
        return None;
    }
    // Slot layout mirrors draw_quick_bar (CSS space, pre-DPR)
    let total_w = QUICKBAR_SLOTS as f64 * (QBAR_SLOT_SIZE + QBAR_SLOT_PAD) - QBAR_SLOT_PAD;
    let start_x = (css_w - total_w) / 2.0;

    for i in 0..QUICKBAR_SLOTS {
        let sx = start_x + i as f64 * (QBAR_SLOT_SIZE + QBAR_SLOT_PAD);
        if css_x >= sx && css_x <= sx + QBAR_SLOT_SIZE {
            return Some(i);
        }
    }
    None
}

/// Hit-test the landscape side panel. Returns Some(BarTap) if a button was hit,
/// or None if the tap didn't land on a button (but may still be in the panel area).
/// Coordinates are in CSS pixels.
pub(crate) fn hit_test_side_panel_buttons(
    css_x: f64,
    css_y: f64,
    css_w: f64,
    panel_css_w: f64,
) -> Option<BarTap> {
    let panel_x = css_w - panel_css_w;
    if css_x < panel_x {
        return None;
    }

    // Button layout mirrors draw_side_panel: 2×2 grid.
    // Stats are now in the top bar, so the panel starts directly with
    // optional tile detail + quick-bar + buttons.
    // Layout: pad(10) + [detail(44+6)] + quickbar(30+6) + buttons
    let x = panel_x + PANEL_PAD;
    let inner_w = panel_css_w - PANEL_PAD * 2.0;
    let btn_w = (inner_w - PANEL_BTN_GAP) / 2.0;

    // Try with/without detail strip
    for detail_offset in &[0.0_f64, PANEL_DETAIL_H] {
        let by = PANEL_PAD + detail_offset + PANEL_QBAR_ROW_H;
        for i in 0..4 {
            let col = i % 2;
            let row = i / 2;
            let bx = x + col as f64 * (btn_w + PANEL_BTN_GAP);
            let button_y = by + row as f64 * (PANEL_BTN_H + PANEL_BTN_GAP);

            if css_x >= bx && css_x <= bx + btn_w
                && css_y >= button_y && css_y <= button_y + PANEL_BTN_H
            {
                return match i {
                    0 => Some(BarTap::OpenDrawer(Drawer::Inventory)),
                    1 => Some(BarTap::OpenDrawer(Drawer::Stats)),
                    2 => Some(BarTap::Sprint),
                    3 => Some(BarTap::OpenDrawer(Drawer::Settings)),
                    _ => None,
                };
            }
        }
    }
    None
}

/// Hit-test the landscape side panel quick-bar. Returns Some(slot) if tapped.
/// The quick bar sits between the optional tile detail and the button grid.
pub(crate) fn hit_test_side_panel_quickbar(
    css_x: f64,
    css_y: f64,
    css_w: f64,
    panel_css_w: f64,
    has_detail: bool,
) -> Option<usize> {
    let panel_x = css_w - panel_css_w;
    if css_x < panel_x {
        return None;
    }
    let inner_w = panel_css_w - PANEL_PAD * 2.0;
    let x = panel_x + PANEL_PAD;

    let detail_offset = if has_detail { PANEL_DETAIL_H } else { 0.0 };
    let qbar_y = PANEL_PAD + detail_offset;

    if css_y < qbar_y || css_y > qbar_y + PANEL_QBAR_SLOT_SIZE {
        return None;
    }

    let total_slot_w = QUICKBAR_SLOTS as f64 * (PANEL_QBAR_SLOT_SIZE + PANEL_QBAR_SLOT_PAD) - PANEL_QBAR_SLOT_PAD;
    let slot_start_x = x + (inner_w - total_slot_w) / 2.0;

    for i in 0..QUICKBAR_SLOTS {
        let sx = slot_start_x + i as f64 * (PANEL_QBAR_SLOT_SIZE + PANEL_QBAR_SLOT_PAD);
        if css_x >= sx && css_x <= sx + PANEL_QBAR_SLOT_SIZE {
            return Some(i);
        }
    }
    None
}

/// Hit-test the landscape side panel drawer area. Returns Some(DrawerTap) if
/// the tap landed on an actionable element, None if outside the panel.
pub(crate) fn hit_test_side_panel_drawer(
    area: &CssHitArea,
    panel_css_w: f64,
    state: &DrawerHitState,
) -> Option<DrawerTap> {
    let panel_x = area.w - panel_css_w;
    if area.x < panel_x {
        return None;
    }

    if state.drawer == Drawer::None {
        return None;
    }

    let x = panel_x + PANEL_PAD;
    let inner_w = panel_css_w - PANEL_PAD * 2.0;

    // Drawer content starts below buttons.
    // Layout: pad + detail(50) + quickbar(36) + buttons(2 rows) + separator(13).
    let drawer_start = PANEL_PAD + PANEL_DETAIL_H + PANEL_QBAR_ROW_H
        + (PANEL_BTN_H + PANEL_BTN_GAP) * 2.0 + 6.0 + 1.0 + 6.0;

    // Settings drawer
    if state.drawer == Drawer::Settings {
        let row_y = drawer_start + 18.0;
        let row_h = 30.0;
        let toggle_w = 50.0;
        let toggle_h = 22.0;
        let toggle_x = x + inner_w - toggle_w;
        let toggle_y = row_y + (row_h - toggle_h) / 2.0;
        if area.x >= toggle_x && area.x <= toggle_x + toggle_w
            && area.y >= toggle_y && area.y <= toggle_y + toggle_h
        {
            return Some(DrawerTap::ToggleGlyphMode);
        }
        let menu_y = row_y + row_h + 8.0 + 20.0;
        let menu_w = inner_w * 0.8;
        let menu_x = x + (inner_w - menu_w) / 2.0;
        let menu_h = 28.0;
        if area.x >= menu_x && area.x <= menu_x + menu_w
            && area.y >= menu_y && area.y <= menu_y + menu_h
        {
            return Some(DrawerTap::MainMenu);
        }
        return Some(DrawerTap::Consumed);
    }

    // Stats drawer: skill allocation buttons
    if state.drawer == Drawer::Stats && state.skill_points > 0 {
        let skill_start = drawer_start + 16.0 + 36.0 + 5.0 * 18.0 + 6.0 + 14.0;
        let skill_row_h = 24.0;
        let btn_sz = 18.0;
        let skills = [SkillKind::Strength, SkillKind::Vitality, SkillKind::Dexterity, SkillKind::Stamina];
        for (i, skill) in skills.iter().enumerate() {
            let row_y = skill_start + i as f64 * skill_row_h;
            let btn_x = x + inner_w - btn_sz;
            let btn_y = row_y + (skill_row_h - btn_sz) / 2.0;
            if area.x >= btn_x && area.x <= btn_x + btn_sz
                && area.y >= btn_y && area.y <= btn_y + btn_sz
            {
                return Some(DrawerTap::StatsAllocate(*skill));
            }
        }
        return Some(DrawerTap::Consumed);
    }

    // Inventory drawer
    if state.drawer == Drawer::Inventory {
        let eq_start = drawer_start + 16.0;
        let list_start = eq_start + 6.0 * (INV_EQ_H_LANDSCAPE + INV_EQ_GAP_LANDSCAPE) + 4.0;
        let avail_h = area.h - list_start - 16.0;
        let max_visible = (avail_h / INV_SLOT_H_LANDSCAPE).floor().max(1.0) as usize;
        let end = (state.inventory_scroll + max_visible).min(state.item_count);

        // Equipment slot hit-test (6 vertical rows)
        if area.y >= eq_start && area.y < list_start {
            let slot_idx = ((area.y - eq_start) / (INV_EQ_H_LANDSCAPE + INV_EQ_GAP_LANDSCAPE)).floor() as usize;
            if slot_idx < 6 {
                // Check if tapping the inline Unequip button on a selected slot
                if state.selected_eq_slot == Some(slot_idx) {
                    let ubtn_w = 36.0;
                    let ubtn_x = panel_x + PANEL_PAD + inner_w - ubtn_w - PANEL_PAD;
                    if area.x >= ubtn_x {
                        return Some(DrawerTap::Unequip(slot_idx));
                    }
                }
                return Some(DrawerTap::EquipmentSlot(slot_idx));
            }
        }

        if area.y >= list_start && state.item_count > 0 {
            let vis_idx = ((area.y - list_start) / INV_SLOT_H_LANDSCAPE).floor() as usize;
            let abs_idx = state.inventory_scroll + vis_idx;
            if abs_idx < end {
                return Some(DrawerTap::InventoryItem(abs_idx));
            }
        }
        return Some(DrawerTap::Consumed);
    }

    Some(DrawerTap::Consumed)
}

/// Drawer hit test: returns `Some` if the tap landed inside the drawer area.
/// All coordinates are in CSS pixels (pre-DPR).
/// Layout constants here mirror `renderer::draw_inventory_drawer` / `draw_stats_drawer`
/// (which use `base * dpr` in canvas pixels — dividing by dpr gives CSS points).
pub(crate) fn hit_test_drawer(
    area: &CssHitArea,
    bar_h_css: f64,
    state: &DrawerHitState,
    selected_item: Option<usize>,
    stats_scroll: f64,
    has_ranged_weapon: bool,
) -> Option<DrawerTap> {
    let drawer_frac = match state.drawer {
        Drawer::None => return None,
        Drawer::Inventory | Drawer::Stats => DRAWER_FRAC,
        Drawer::Settings => SETTINGS_DRAWER_FRAC,
    };
    let drawer_h = area.h * drawer_frac;
    let drawer_y = area.h - bar_h_css - drawer_h;

    if area.y < drawer_y || area.y >= area.h - bar_h_css {
        return None;
    }

    // Settings drawer: glyph mode toggle + main menu button
    if state.drawer == Drawer::Settings {
        let pad = 12.0;
        let row_y = drawer_y + 40.0;
        let row_h = 40.0;
        let toggle_w = 70.0;
        let toggle_h = 30.0;
        let toggle_x = area.w - pad - toggle_w;
        let toggle_y = row_y + (row_h - toggle_h) / 2.0;
        if area.x >= toggle_x && area.x <= toggle_x + toggle_w
            && area.y >= toggle_y && area.y <= toggle_y + toggle_h
        {
            return Some(DrawerTap::ToggleGlyphMode);
        }
        let diff_y = row_y + row_h + 24.0;
        let menu_btn_w = (area.w * 0.5).min(180.0);
        let menu_btn_h = 34.0;
        let menu_btn_x = (area.w - menu_btn_w) / 2.0;
        let menu_btn_y = diff_y + 32.0;
        if area.x >= menu_btn_x && area.x <= menu_btn_x + menu_btn_w
            && area.y >= menu_btn_y && area.y <= menu_btn_y + menu_btn_h
        {
            return Some(DrawerTap::MainMenu);
        }
        return Some(DrawerTap::Consumed);
    }

    // Stats drawer: check for skill point allocation buttons
    if state.drawer == Drawer::Stats && state.skill_points > 0 {
        let stat_row_count = if has_ranged_weapon { 6.0 } else { 5.0 };
        let skill_section_offset = 32.0 + 42.0 + 22.0 + stat_row_count * 24.0 + 8.0 + 20.0;
        let skill_section_y = drawer_y + skill_section_offset - stats_scroll;

        let skill_row_h = 30.0;
        let btn_sz = 24.0;
        let pad = 12.0;
        let btn_x = area.w - pad - btn_sz;
        let skills = [SkillKind::Strength, SkillKind::Vitality, SkillKind::Dexterity, SkillKind::Stamina];
        for (i, skill) in skills.iter().enumerate() {
            let row_y = skill_section_y + i as f64 * skill_row_h;
            let btn_y = row_y + (skill_row_h - btn_sz) / 2.0;
            if area.x >= btn_x && area.x <= btn_x + btn_sz
                && area.y >= btn_y && area.y <= btn_y + btn_sz
            {
                return Some(DrawerTap::StatsAllocate(*skill));
            }
        }
        return Some(DrawerTap::Consumed);
    }

    // Inventory drawer: match layout from renderer::draw_inventory_drawer
    let eq_y = drawer_y + 32.0;
    let list_y = eq_y + (INV_EQ_H + INV_EQ_GAP) * 3.0 + 4.0;
    let has_selection = selected_item.is_some() || state.selected_eq_slot.is_some();
    let detail_bar_h = if has_selection { 46.0 } else { 20.0 };
    let avail_h = (drawer_y + drawer_h - detail_bar_h) - list_y;
    let max_visible = (avail_h / INV_SLOT_H).floor().max(1.0) as usize;
    let end = (state.inventory_scroll + max_visible).min(state.item_count);
    let scrollbar_w = 12.0;

    // Equipment slot hit-test (6 slots in 3×2 grid)
    if area.y >= eq_y && area.y < list_y {
        let right_x = area.w / 2.0 + DRAWER_PAD * 0.5;
        let is_right = area.x >= right_x;
        let row = ((area.y - eq_y) / (INV_EQ_H + INV_EQ_GAP)).floor() as usize;
        if row < 3 {
            let col = if is_right { 1 } else { 0 };
            let slot_idx = row * 2 + col;
            return Some(DrawerTap::EquipmentSlot(slot_idx));
        }
    }

    // Detail bar buttons hit test (when an inventory item is selected)
    if let Some(sel_idx) = selected_item {
        let bar_y = drawer_y + drawer_h - detail_bar_h;
        if area.y >= bar_y {
            let btn_h = 26.0;
            let btn_gap = 8.0;
            let btn_y = bar_y + detail_bar_h - btn_h - 4.0;

            if area.y >= btn_y && area.y <= btn_y + btn_h {
                let action_w = 60.0;
                let action_x = area.w - DRAWER_PAD - action_w - btn_gap - 60.0;
                if area.x >= action_x && area.x <= action_x + action_w {
                    return Some(DrawerTap::UseEquip(sel_idx));
                }
                let drop_w = 60.0;
                let drop_x = area.w - DRAWER_PAD - drop_w;
                if area.x >= drop_x && area.x <= drop_x + drop_w {
                    return Some(DrawerTap::Drop(sel_idx));
                }
            }
            return Some(DrawerTap::Consumed);
        }
    }
    // Detail bar buttons hit test (when an equipment slot is selected)
    if let Some(eq_slot) = state.selected_eq_slot {
        let bar_y = drawer_y + drawer_h - detail_bar_h;
        if area.y >= bar_y {
            let btn_h = 26.0;
            let btn_y = bar_y + detail_bar_h - btn_h - 4.0;
            if area.y >= btn_y && area.y <= btn_y + btn_h {
                let unequip_w = 80.0;
                let unequip_x = area.w - DRAWER_PAD - unequip_w;
                if area.x >= unequip_x && area.x <= unequip_x + unequip_w {
                    return Some(DrawerTap::Unequip(eq_slot));
                }
            }
            return Some(DrawerTap::Consumed);
        }
    }

    if area.y >= list_y && state.item_count > 0 {
        let scrollbar_x = area.w - DRAWER_PAD - scrollbar_w;
        if area.x >= scrollbar_x && state.item_count > max_visible {
            let track_h = max_visible as f64 * INV_SLOT_H;
            let scroll_range = state.item_count - max_visible;
            let thumb_frac = max_visible as f64 / state.item_count as f64;
            let min_thumb_h = 20.0;
            let thumb_h = (track_h * thumb_frac).max(min_thumb_h);
            let scroll_frac = if scroll_range > 0 {
                state.inventory_scroll as f64 / scroll_range as f64
            } else {
                0.0
            };
            let thumb_y = list_y + scroll_frac * (track_h - thumb_h);

            if area.y < thumb_y {
                return Some(DrawerTap::ScrollUp);
            } else if area.y > thumb_y + thumb_h {
                return Some(DrawerTap::ScrollDown);
            }
            return Some(DrawerTap::Consumed);
        }

        let vis_idx = ((area.y - list_y) / INV_SLOT_H).floor() as usize;
        let abs_idx = state.inventory_scroll + vis_idx;
        if abs_idx < end {
            return Some(DrawerTap::InventoryItem(abs_idx));
        }
    }

    Some(DrawerTap::Consumed)
}

/// Given a Y coordinate and the list start position, determine which inventory
/// row (absolute index) was hit. Shared between portrait and landscape modes.
fn inventory_row_at_y(
    css_y: f64,
    list_y: f64,
    slot_h: f64,
    item_count: usize,
    inventory_scroll: usize,
) -> Option<usize> {
    if css_y < list_y || item_count == 0 {
        return None;
    }
    let vis_idx = ((css_y - list_y) / slot_h).floor() as usize;
    let abs_idx = inventory_scroll + vis_idx;
    if abs_idx < item_count { Some(abs_idx) } else { None }
}

/// Check if CSS coordinates land on an inventory item row in portrait mode.
/// Returns the absolute inventory index if so.
pub(crate) fn hit_test_inventory_item_row(
    _css_x: f64,
    css_y: f64,
    _css_w: f64,
    css_h: f64,
    bar_h_css: f64,
    item_count: usize,
    inventory_scroll: usize,
) -> Option<usize> {
    let drawer_h = css_h * DRAWER_FRAC;
    let drawer_y = css_h - bar_h_css - drawer_h;
    if css_y < drawer_y || css_y >= css_h - bar_h_css {
        return None;
    }
    // Item list starts after: header(32) + 3 eq rows + gap(4)
    let eq_y = drawer_y + 32.0;
    let list_y = eq_y + (INV_EQ_H + INV_EQ_GAP) * 3.0 + 4.0;
    inventory_row_at_y(css_y, list_y, INV_SLOT_H, item_count, inventory_scroll)
}

/// Check if CSS coordinates land on an inventory item row in landscape mode.
pub(crate) fn hit_test_inventory_item_row_landscape(
    css_x: f64,
    css_y: f64,
    css_w: f64,
    _css_h: f64,
    panel_css_w: f64,
    item_count: usize,
    inventory_scroll: usize,
) -> Option<usize> {
    let panel_x = css_w - panel_css_w;
    if css_x < panel_x {
        return None;
    }
    // Panel layout: pad + detail(50) + qbar(36) + buttons(2 rows) + separator
    let drawer_start = PANEL_PAD + PANEL_DETAIL_H + PANEL_QBAR_ROW_H
        + (PANEL_BTN_H + PANEL_BTN_GAP) * 2.0 + 6.0 + 1.0 + 6.0;
    // Items start after: title(16) + 6 eq slots + gap(4)
    let list_start = drawer_start + 16.0 + 6.0 * (INV_EQ_H_LANDSCAPE + INV_EQ_GAP_LANDSCAPE) + 4.0;
    inventory_row_at_y(css_y, list_start, INV_SLOT_H_LANDSCAPE, item_count, inventory_scroll)
}
