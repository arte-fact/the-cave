mod camera;
mod config;
mod errors;
mod game;
mod input;
mod map;
mod renderer;
mod sprite_atlas;
mod sprites;
mod world;

use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, HtmlImageElement};

use config::{Difficulty, GameConfig};
use game::{Drawer, Game, ItemKind, SkillKind, TurnResult, QUICKBAR_SLOTS};
use input::{Input, InputAction};
use map::{bresenham_line, Map};
use renderer::{Renderer, SpriteSheets};
use world::World;

/// Drag-and-drop state for quick-bar item assignment.
#[derive(Clone, Debug)]
enum DragState {
    /// No drag in progress.
    Idle,
    /// Finger is down on an inventory item; waiting for long-press timer.
    Pending {
        inv_index: usize,
        start_x: f64,
        start_y: f64,
        /// Frame counter when the press started.
        start_frame: u32,
        /// If dragging from a quick-bar slot (for reorder), which slot.
        from_quickbar_slot: Option<usize>,
    },
    /// Long-press confirmed — item is being dragged.
    Dragging {
        inv_index: usize,
        /// Current finger position (CSS pixels).
        touch_x: f64,
        touch_y: f64,
        /// Source: true if dragging from a quick-bar slot (reorder), false from inventory.
        from_quickbar_slot: Option<usize>,
    },
}

/// Application state machine: menus vs playing.
#[derive(Clone, Debug)]
enum AppState {
    MainMenu,
    NewGame {
        selected_difficulty: usize, // 0=Easy, 1=Normal, 2=Hard
        seed: u64,
    },
    Settings,
    Playing,
}

/// Menu state held in an Rc<RefCell<>> alongside the game.
#[derive(Clone, Debug)]
struct MenuState {
    app_state: AppState,
    has_save: bool,
}

/// Result of tapping the bottom bar.
enum BarTap {
    OpenDrawer(Drawer),
    Sprint,
}

/// Bottom bar button layout: returns which button was tapped (if any).
/// `css_y` and `canvas_h_css` are in CSS pixels (pre-DPR).
fn hit_test_bottom_bar(css_x: f64, css_y: f64, css_w: f64, css_h: f64, bar_h_css: f64) -> Option<BarTap> {
    let bar_top = css_h - bar_h_css;
    if css_y < bar_top {
        return None;
    }
    let btn_count = 4.0;
    let btn_w = (css_w / btn_count).min(110.0);
    let total_w = btn_w * btn_count;
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
fn hit_test_quick_bar(
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
    let slot_size = 36.0;
    let slot_pad = 6.0;
    let total_w = QUICKBAR_SLOTS as f64 * (slot_size + slot_pad) - slot_pad;
    let start_x = (css_w - total_w) / 2.0;

    for i in 0..QUICKBAR_SLOTS {
        let sx = start_x + i as f64 * (slot_size + slot_pad);
        if css_x >= sx && css_x <= sx + slot_size {
            return Some(i);
        }
    }
    None
}

/// Hit-test the landscape side panel. Returns Some(BarTap) if a button was hit,
/// or None if the tap didn't land on a button (but may still be in the panel area).
/// Coordinates are in CSS pixels.
fn hit_test_side_panel_buttons(
    css_x: f64,
    css_y: f64,
    css_w: f64,
    panel_css_w: f64,
) -> Option<BarTap> {
    let panel_x = css_w - panel_css_w;
    if css_x < panel_x {
        return None;
    }

    // Button layout mirrors draw_side_panel: 2×2 grid
    // Location at: panel_x + pad, y offset depends on bars + stats drawn above.
    // The buttons start roughly after: location(16) + 3 bars(3×14) + stats(2×14) + gap + separator
    // Approximate: ~130 CSS px from top of panel (tuned to match renderer).
    let pad = 10.0;
    let x = panel_x + pad;
    let inner_w = panel_css_w - pad * 2.0;
    let btn_h = 30.0;
    let btn_gap = 4.0;
    let btn_w = (inner_w - btn_gap) / 2.0;

    // We need to find the button area. The y-offset of buttons in the side panel
    // is variable (depends on whether tile detail is showing). Scan a reasonable range.
    // The buttons are: pad(10) + location(16) + 3 bars(3*(10+4)) + stats_text(14+18) + sep(6+1+6)
    // = 10 + 16 + 42 + 32 + 13 = ~113 CSS px. Without detail strip.
    // With detail: add ~50 CSS px. We can't know exactly, so use a generous range.
    // Actually, let's compute from known CSS values matching the renderer.
    let base_y = pad + 16.0 + 3.0 * (10.0 + 4.0) + 4.0 + 14.0 + 18.0 + 1.0 + 6.0;
    // base_y is approx 101 CSS px. The detail strip adds ~50px if present.
    // We'll check the tap against button positions at base_y and base_y+50.
    // For simplicity, just check if tap is within any of the 4 button cells.

    // Quick bar adds ~36 CSS px (30 slot_size + 6 gap) between detail and buttons.
    // Try combinations: with/without detail, plus quick bar offset.
    let qbar_offset = 36.0;
    for detail_offset in &[0.0_f64, 50.0] {
        let by = base_y + detail_offset + qbar_offset;
        for i in 0..4 {
            let col = i % 2;
            let row = i / 2;
            let bx = x + col as f64 * (btn_w + btn_gap);
            let button_y = by + row as f64 * (btn_h + btn_gap);

            if css_x >= bx && css_x <= bx + btn_w
                && css_y >= button_y && css_y <= button_y + btn_h
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
/// The quick bar sits between the tile detail area and the button grid.
fn hit_test_side_panel_quickbar(
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
    let pad = 10.0;
    let inner_w = panel_css_w - pad * 2.0;
    let x = panel_x + pad;

    // Quick bar y offset: after location + bars + stats + separator + optional detail
    let base_y = pad + 16.0 + 3.0 * (10.0 + 4.0) + 4.0 + 14.0 + 18.0 + 1.0 + 6.0;
    let detail_offset = if has_detail { 50.0 } else { 0.0 };
    let qbar_y = base_y + detail_offset;
    let slot_size = 30.0;
    let slot_pad = 4.0;

    if css_y < qbar_y || css_y > qbar_y + slot_size {
        return None;
    }

    let total_slot_w = QUICKBAR_SLOTS as f64 * (slot_size + slot_pad) - slot_pad;
    let slot_start_x = x + (inner_w - total_slot_w) / 2.0;

    for i in 0..QUICKBAR_SLOTS {
        let sx = slot_start_x + i as f64 * (slot_size + slot_pad);
        if css_x >= sx && css_x <= sx + slot_size {
            return Some(i);
        }
    }
    None
}

/// Hit-test the landscape side panel drawer area. Returns Some(DrawerTap) if
/// the tap landed on an actionable element, None if outside the panel.
fn hit_test_side_panel_drawer(
    css_x: f64,
    css_y: f64,
    css_w: f64,
    css_h: f64,
    panel_css_w: f64,
    drawer: Drawer,
    item_count: usize,
    inventory_scroll: usize,
    skill_points: u32,
    _has_ranged_weapon: bool,
) -> Option<DrawerTap> {
    let panel_x = css_w - panel_css_w;
    if css_x < panel_x {
        return None;
    }

    if drawer == Drawer::None {
        return None;
    }

    let pad = 10.0;
    let x = panel_x + pad;
    let inner_w = panel_css_w - pad * 2.0;

    // The drawer content starts below the buttons area.
    // Buttons end at approximately: base_y + 2*(btn_h+gap) + gap = ~101 + 68 + 6 = 175 CSS px
    // With detail offset: +50. We use a conservative drawer_start.
    let btn_h = 30.0;
    let btn_gap = 4.0;
    let base_y = pad + 16.0 + 3.0 * (10.0 + 4.0) + 4.0 + 14.0 + 18.0 + 1.0 + 6.0;
    // Quick bar (30 + 6) sits between detail/base and buttons
    let qbar_offset = 36.0;
    let drawer_start = base_y + qbar_offset + (btn_h + btn_gap) * 2.0 + 6.0 + 1.0 + 6.0;

    // Settings drawer
    if drawer == Drawer::Settings {
        // Glyph toggle: at drawer_start + 18 (title) + row_h area
        let row_y = drawer_start + 18.0;
        let row_h = 30.0;
        let toggle_w = 50.0;
        let toggle_h = 22.0;
        let toggle_x = x + inner_w - toggle_w;
        let toggle_y = row_y + (row_h - toggle_h) / 2.0;
        if css_x >= toggle_x && css_x <= toggle_x + toggle_w
            && css_y >= toggle_y && css_y <= toggle_y + toggle_h
        {
            return Some(DrawerTap::ToggleGlyphMode);
        }
        // Main menu button
        let menu_y = row_y + row_h + 8.0 + 20.0;
        let menu_w = inner_w * 0.8;
        let menu_x = x + (inner_w - menu_w) / 2.0;
        let menu_h = 28.0;
        if css_x >= menu_x && css_x <= menu_x + menu_w
            && css_y >= menu_y && css_y <= menu_y + menu_h
        {
            return Some(DrawerTap::MainMenu);
        }
        return Some(DrawerTap::Consumed);
    }

    // Stats drawer: skill allocation buttons
    if drawer == Drawer::Stats && skill_points > 0 {
        // Skills start after: drawer_start + title(16) + icon+level(36) + stat rows(~5*18) + gap(6) + header(14)
        let skill_start = drawer_start + 16.0 + 36.0 + 5.0 * 18.0 + 6.0 + 14.0;
        let skill_row_h = 24.0;
        let btn_sz = 18.0;
        let skills = [SkillKind::Strength, SkillKind::Vitality, SkillKind::Dexterity, SkillKind::Stamina];
        for (i, skill) in skills.iter().enumerate() {
            let row_y = skill_start + i as f64 * skill_row_h;
            let btn_x = x + inner_w - btn_sz;
            let btn_y = row_y + (skill_row_h - btn_sz) / 2.0;
            if css_x >= btn_x && css_x <= btn_x + btn_sz
                && css_y >= btn_y && css_y <= btn_y + btn_sz
            {
                return Some(DrawerTap::StatsAllocate(*skill));
            }
        }
        return Some(DrawerTap::Consumed);
    }

    // Inventory drawer
    if drawer == Drawer::Inventory {
        // Items start after: drawer_start + title(16) + 6 eq slots(6*24) + gap(4)
        let eq_h = 22.0;
        let eq_gap = 2.0;
        let list_start = drawer_start + 16.0 + 6.0 * (eq_h + eq_gap) + 4.0;
        let slot_h = 26.0;
        let avail_h = css_h - list_start - 16.0;
        let max_visible = (avail_h / slot_h).floor().max(1.0) as usize;
        let end = (inventory_scroll + max_visible).min(item_count);

        if css_y >= list_start && item_count > 0 {
            let vis_idx = ((css_y - list_start) / slot_h).floor() as usize;
            let abs_idx = inventory_scroll + vis_idx;
            if abs_idx < end {
                // Check inline buttons (Use/Equip and Drop)
                let btn_w = 24.0;
                let btn_h_i = 16.0;
                let drop_x = x + inner_w - btn_w - 2.0;
                let use_x = drop_x - btn_w - 2.0;
                let row_y = list_start + vis_idx as f64 * slot_h;
                let btn_y = row_y + (slot_h - btn_h_i) / 2.0;
                if css_y >= btn_y && css_y <= btn_y + btn_h_i {
                    if css_x >= use_x && css_x <= use_x + btn_w {
                        return Some(DrawerTap::InlineUseEquip(abs_idx));
                    }
                    if css_x >= drop_x && css_x <= drop_x + btn_w {
                        return Some(DrawerTap::InlineDrop(abs_idx));
                    }
                }
                return Some(DrawerTap::InventoryItem(abs_idx));
            }
        }
        return Some(DrawerTap::Consumed);
    }

    Some(DrawerTap::Consumed)
}

/// Result of tapping inside a drawer.
enum DrawerTap {
    /// Tapped an inventory item at the given index (absolute, not visual).
    InventoryItem(usize),
    /// Scroll the inventory list up.
    ScrollUp,
    /// Scroll the inventory list down.
    ScrollDown,
    /// Use/Equip button tapped for the selected item (detail bar).
    UseEquip(usize),
    /// Drop button tapped for the selected item (detail bar).
    Drop(usize),
    /// Inline Use/Equip button on a list item row.
    InlineUseEquip(usize),
    /// Inline Drop button on a list item row.
    InlineDrop(usize),
    /// Allocate a skill point to the given attribute.
    StatsAllocate(SkillKind),
    /// Toggle glyph rendering mode.
    ToggleGlyphMode,
    /// Return to main menu from in-game settings.
    MainMenu,
    /// Tapped inside the drawer but not on an actionable element.
    Consumed,
}

/// Drawer hit test: returns `Some` if the tap landed inside the drawer area.
/// All coordinates are in CSS pixels (pre-DPR).
/// Layout constants here mirror `renderer::draw_inventory_drawer` / `draw_stats_drawer`
/// (which use `base * dpr` in canvas pixels — dividing by dpr gives CSS points).
fn hit_test_drawer(
    css_x: f64,
    css_y: f64,
    css_w: f64,
    css_h: f64,
    bar_h_css: f64,
    drawer: Drawer,
    item_count: usize,
    inventory_scroll: usize,
    selected_item: Option<usize>,
    skill_points: u32,
    stats_scroll: f64,
    has_ranged_weapon: bool,
) -> Option<DrawerTap> {
    let drawer_frac = match drawer {
        Drawer::None => return None,
        Drawer::Inventory => 0.55,
        Drawer::Stats => 0.55,
        Drawer::Settings => 0.45,
    };
    let drawer_h = css_h * drawer_frac;
    let drawer_y = css_h - bar_h_css - drawer_h;

    // Not inside the drawer area
    if css_y < drawer_y || css_y >= css_h - bar_h_css {
        return None;
    }

    // Settings drawer: glyph mode toggle + main menu button
    if drawer == Drawer::Settings {
        // Toggle button layout mirrors renderer::draw_settings_drawer (CSS space)
        let pad = 12.0;
        let row_y = drawer_y + 40.0;
        let row_h = 40.0;
        let toggle_w = 70.0;
        let toggle_h = 30.0;
        let toggle_x = css_w - pad - toggle_w;
        let toggle_y = row_y + (row_h - toggle_h) / 2.0;
        if css_x >= toggle_x && css_x <= toggle_x + toggle_w
            && css_y >= toggle_y && css_y <= toggle_y + toggle_h
        {
            return Some(DrawerTap::ToggleGlyphMode);
        }
        // Main Menu button (centered, below difficulty info)
        let diff_y = row_y + row_h + 24.0;
        let menu_btn_w = (css_w * 0.5).min(180.0);
        let menu_btn_h = 34.0;
        let menu_btn_x = (css_w - menu_btn_w) / 2.0;
        let menu_btn_y = diff_y + 32.0;
        if css_x >= menu_btn_x && css_x <= menu_btn_x + menu_btn_w
            && css_y >= menu_btn_y && css_y <= menu_btn_y + menu_btn_h
        {
            return Some(DrawerTap::MainMenu);
        }
        return Some(DrawerTap::Consumed);
    }

    // Stats drawer: check for skill point allocation buttons
    if drawer == Drawer::Stats {
        if skill_points > 0 {
            // Compute where the skill section starts in CSS space, matching renderer layout.
            // All renderer values are in `val * dpr` canvas pixels; dividing by dpr gives CSS px.
            // content_top = drawer_y + 32 (header)
            // sprite area: icon(36) + gap(6) = 42
            // xp bar area: bar(10) + gap(12) = 22
            // stat rows: (4 base + 1 if ranged + 1 location) * 24
            let stat_row_count = if has_ranged_weapon { 6.0 } else { 5.0 };
            // skill section gap(8) + "Skill Points" header(20) = 28
            let skill_section_offset = 32.0 + 42.0 + 22.0 + stat_row_count * 24.0 + 8.0 + 20.0;
            // Apply scroll offset — renderer shifts content up by stats_scroll
            let skill_section_y = drawer_y + skill_section_offset - stats_scroll;

            let skill_row_h = 30.0;
            let btn_sz = 24.0;
            let pad = 12.0;
            let btn_x = css_w - pad - btn_sz;
            let skills = [SkillKind::Strength, SkillKind::Vitality, SkillKind::Dexterity, SkillKind::Stamina];
            for (i, skill) in skills.iter().enumerate() {
                let row_y = skill_section_y + i as f64 * skill_row_h;
                let btn_y = row_y + (skill_row_h - btn_sz) / 2.0;
                if css_x >= btn_x && css_x <= btn_x + btn_sz
                    && css_y >= btn_y && css_y <= btn_y + btn_sz
                {
                    return Some(DrawerTap::StatsAllocate(*skill));
                }
            }
        }
        return Some(DrawerTap::Consumed);
    }

    // Inventory drawer: match layout from renderer::draw_inventory_drawer
    // Equipment: 3 rows × 2 cols, eq_h=30, eq_gap=4
    let eq_y = drawer_y + 32.0;
    let eq_h = 30.0;
    let eq_gap = 4.0;
    let list_y = eq_y + (eq_h + eq_gap) * 3.0 + 4.0;
    let slot_h = 34.0;
    let detail_bar_h = if selected_item.is_some() { 46.0 } else { 20.0 };
    let avail_h = (drawer_y + drawer_h - detail_bar_h) - list_y;
    let max_visible = (avail_h / slot_h).floor().max(1.0) as usize;
    let end = (inventory_scroll + max_visible).min(item_count);
    let pad = 12.0;
    let scrollbar_w = 12.0;

    // Detail bar buttons hit test (when an item is selected)
    if let Some(sel_idx) = selected_item {
        let bar_y = drawer_y + drawer_h - detail_bar_h;
        if css_y >= bar_y {
            let btn_h = 26.0;
            let btn_gap = 8.0;
            let btn_y = bar_y + detail_bar_h - btn_h - 4.0;

            if css_y >= btn_y && css_y <= btn_y + btn_h {
                // Use/Equip button
                let action_w = 60.0;
                let action_x = css_w - pad - action_w - btn_gap - 60.0;
                if css_x >= action_x && css_x <= action_x + action_w {
                    return Some(DrawerTap::UseEquip(sel_idx));
                }
                // Drop button
                let drop_w = 60.0;
                let drop_x = css_w - pad - drop_w;
                if css_x >= drop_x && css_x <= drop_x + drop_w {
                    return Some(DrawerTap::Drop(sel_idx));
                }
            }
            return Some(DrawerTap::Consumed);
        }
    }

    if css_y >= list_y && item_count > 0 {
        // Inline button hit-test (Use/Equip and Drop buttons on each row)
        let inline_btn_w = 36.0;
        let inline_btn_h = 22.0;
        let inline_btn_gap = 3.0;
        let text_right = css_w - pad - scrollbar_w - 4.0;
        let drop_btn_x = text_right - inline_btn_w;
        let use_btn_x = drop_btn_x - inline_btn_gap - inline_btn_w;

        let vis_idx = ((css_y - list_y) / slot_h).floor() as usize;
        let abs_idx = inventory_scroll + vis_idx;
        if abs_idx < end {
            let row_y = list_y + vis_idx as f64 * slot_h;
            let btn_y = row_y + (slot_h - inline_btn_h) / 2.0;
            if css_y >= btn_y && css_y <= btn_y + inline_btn_h {
                if css_x >= use_btn_x && css_x <= use_btn_x + inline_btn_w {
                    return Some(DrawerTap::InlineUseEquip(abs_idx));
                }
                if css_x >= drop_btn_x && css_x <= drop_btn_x + inline_btn_w {
                    return Some(DrawerTap::InlineDrop(abs_idx));
                }
            }
        }

        // Check if tap is in scrollbar track (right edge of list area)
        let scrollbar_x = css_w - pad - scrollbar_w;
        if css_x >= scrollbar_x && item_count > max_visible {
            let track_h = max_visible as f64 * slot_h;
            let scroll_range = item_count - max_visible;
            let thumb_frac = max_visible as f64 / item_count as f64;
            let min_thumb_h = 20.0;
            let thumb_h = (track_h * thumb_frac).max(min_thumb_h);
            let scroll_frac = if scroll_range > 0 {
                inventory_scroll as f64 / scroll_range as f64
            } else {
                0.0
            };
            let thumb_y = list_y + scroll_frac * (track_h - thumb_h);

            if css_y < thumb_y {
                return Some(DrawerTap::ScrollUp);
            } else if css_y > thumb_y + thumb_h {
                return Some(DrawerTap::ScrollDown);
            }
            return Some(DrawerTap::Consumed); // tap on thumb itself
        }

        // Regular item tap (select, not use)
        let vis_idx = ((css_y - list_y) / slot_h).floor() as usize;
        let abs_idx = inventory_scroll + vis_idx;
        if abs_idx < end {
            return Some(DrawerTap::InventoryItem(abs_idx));
        }
    }

    Some(DrawerTap::Consumed)
}


/// Check if CSS coordinates land on an inventory item row in portrait mode.
/// Returns the absolute inventory index if so.
fn hit_test_inventory_item_row(
    _css_x: f64,
    css_y: f64,
    _css_w: f64,
    css_h: f64,
    bar_h_css: f64,
    item_count: usize,
    inventory_scroll: usize,
) -> Option<usize> {
    // Inventory drawer = 55% of canvas height, above the bottom region
    let drawer_h = css_h * 0.55;
    let drawer_y = css_h - bar_h_css - drawer_h;
    if css_y < drawer_y || css_y >= css_h - bar_h_css {
        return None;
    }
    // Item list starts after: header(32) + 3 eq rows(3*(30+4)) + gap(4)
    let eq_y = drawer_y + 32.0;
    let eq_h = 30.0;
    let eq_gap = 4.0;
    let list_y = eq_y + (eq_h + eq_gap) * 3.0 + 4.0;
    let slot_h = 34.0;
    if css_y < list_y || item_count == 0 {
        return None;
    }
    let vis_idx = ((css_y - list_y) / slot_h).floor() as usize;
    let abs_idx = inventory_scroll + vis_idx;
    if abs_idx < item_count { Some(abs_idx) } else { None }
}

/// Check if CSS coordinates land on an inventory item row in landscape mode.
fn hit_test_inventory_item_row_landscape(
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
    let pad = 10.0;
    let btn_h = 30.0;
    let btn_gap = 4.0;
    let base_y = pad + 16.0 + 3.0 * (10.0 + 4.0) + 4.0 + 14.0 + 18.0 + 1.0 + 6.0;
    // Quick bar + buttons
    let qbar_size = 30.0 + 6.0;
    let drawer_start = base_y + qbar_size + (btn_h + btn_gap) * 2.0 + 6.0 + 1.0 + 6.0;
    // Items start after: title(16) + 6 eq slots(6*(22+2)) + gap(4)
    let eq_h = 22.0;
    let eq_gap = 2.0;
    let list_start = drawer_start + 16.0 + 6.0 * (eq_h + eq_gap) + 4.0;
    let slot_h = 26.0;
    if css_y < list_start || item_count == 0 {
        return None;
    }
    let vis_idx = ((css_y - list_start) / slot_h).floor() as usize;
    let abs_idx = inventory_scroll + vis_idx;
    if abs_idx < item_count { Some(abs_idx) } else { None }
}

/// Load glyph_mode setting from localStorage.
fn load_glyph_mode() -> bool {
    web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|s| s.get_item("glyph_mode").ok().flatten())
        .map(|v| v == "true")
        .unwrap_or(false)
}

/// Save glyph_mode setting to localStorage.
fn save_glyph_mode(enabled: bool) {
    if let Some(storage) = web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
    {
        let _ = storage.set_item("glyph_mode", if enabled { "true" } else { "false" });
    }
}

/// Load last-used difficulty from localStorage. Defaults to Normal (1).
fn load_difficulty() -> usize {
    web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
        .and_then(|s| s.get_item("difficulty").ok().flatten())
        .and_then(|v| v.parse().ok())
        .unwrap_or(1)
}

/// Save difficulty selection to localStorage.
fn save_difficulty(idx: usize) {
    if let Some(storage) = web_sys::window()
        .and_then(|w| w.local_storage().ok().flatten())
    {
        let _ = storage.set_item("difficulty", &idx.to_string());
    }
}

/// Handle a tap event within menu screens. Modifies menu state and
/// may transition to Playing state (creating a new game).
fn handle_menu_tap(
    ms: &mut MenuState,
    game: &Rc<RefCell<Game>>,
    renderer: &Rc<RefCell<Renderer>>,
    auto_path: &Rc<RefCell<Vec<(i32, i32)>>>,
    css_x: f64,
    css_y: f64,
    css_w: f64,
    css_h: f64,
    dpr: f64,
) {
    // Convert to canvas-pixel coords for hit testing
    let cx = css_x * dpr;
    let cy = css_y * dpr;
    let cw = css_w * dpr;
    let ch = css_h * dpr;

    match &ms.app_state {
        AppState::MainMenu => {
            // Button layout matches draw_main_menu
            let btn_w = (cw * 0.5).min(280.0 * dpr);
            let btn_h = 44.0 * dpr;
            let gap = 16.0 * dpr;
            let start_y = ch * 0.45;
            let btn_x = (cw - btn_w) / 2.0;

            // New Game
            if cx >= btn_x && cx <= btn_x + btn_w
                && cy >= start_y && cy <= start_y + btn_h
            {
                ms.app_state = AppState::NewGame {
                    selected_difficulty: load_difficulty(),
                    seed: js_sys::Date::now() as u64 ^ 0xDEAD_BEEF,
                };
                return;
            }

            // Continue
            let continue_y = start_y + btn_h + gap;
            if ms.has_save
                && cx >= btn_x && cx <= btn_x + btn_w
                && cy >= continue_y && cy <= continue_y + btn_h
            {
                ms.app_state = AppState::Playing;
                return;
            }

            // Settings
            let settings_y = continue_y + btn_h + gap;
            if cx >= btn_x && cx <= btn_x + btn_w
                && cy >= settings_y && cy <= settings_y + btn_h
            {
                ms.app_state = AppState::Settings;
            }
        }
        AppState::NewGame { selected_difficulty, seed } => {
            let selected_difficulty = *selected_difficulty;
            let seed = *seed;

            // Back button (top-left area)
            if cx < 100.0 * dpr && cy < 40.0 * dpr {
                ms.app_state = AppState::MainMenu;
                return;
            }

            // Difficulty buttons
            let btn_w = (cw * 0.7).min(300.0 * dpr);
            let btn_h = 52.0 * dpr;
            let gap = 10.0 * dpr;
            let btn_x = (cw - btn_w) / 2.0;
            let section_y = ch * 0.22;
            let list_y = section_y + 24.0 * dpr;

            for i in 0..3 {
                let y = list_y + (btn_h + gap) * i as f64;
                if cx >= btn_x && cx <= btn_x + btn_w
                    && cy >= y && cy <= y + btn_h
                {
                    ms.app_state = AppState::NewGame {
                        selected_difficulty: i,
                        seed,
                    };
                    return;
                }
            }

            // Seed area tap — randomize seed
            let seed_y = list_y + (btn_h + gap) * 3.0 + 10.0 * dpr;
            if cy >= seed_y - 20.0 * dpr && cy <= seed_y + 30.0 * dpr
                && cx >= cw * 0.2 && cx <= cw * 0.8
            {
                ms.app_state = AppState::NewGame {
                    selected_difficulty,
                    seed: js_sys::Date::now() as u64 ^ 0xBEEF_CAFE,
                };
                return;
            }

            // Start button
            let start_y_btn = seed_y + 44.0 * dpr;
            let start_w = (cw * 0.5).min(220.0 * dpr);
            let start_h = 48.0 * dpr;
            let start_x = (cw - start_w) / 2.0;
            if cx >= start_x && cx <= start_x + start_w
                && cy >= start_y_btn && cy <= start_y_btn + start_h
            {
                // Create game with selected difficulty
                let difficulties = [Difficulty::Easy, Difficulty::Normal, Difficulty::Hard];
                let diff = difficulties[selected_difficulty];
                save_difficulty(selected_difficulty);
                let config = GameConfig::from_difficulty(diff);
                let new = new_game_with_config(seed, config);
                *game.borrow_mut() = new;
                {
                    let gm = game.borrow();
                    let mut r = renderer.borrow_mut();
                    let map = gm.current_map();
                    r.camera.snap(gm.player_x as f64, gm.player_y as f64, map.width, map.height);
                }
                auto_path.borrow_mut().clear();
                ms.app_state = AppState::Playing;
                ms.has_save = true;
            }
        }
        AppState::Settings => {
            // Back button
            if cx < 100.0 * dpr && cy < 40.0 * dpr {
                ms.app_state = AppState::MainMenu;
                return;
            }

            // Glyph Mode toggle — same layout as draw_settings_menu
            let row_w = (cw * 0.8).min(340.0 * dpr);
            let row_h = 44.0 * dpr;
            let row_x = (cw - row_w) / 2.0;
            let pad = 14.0 * dpr;
            let row_y = ch * 0.25;

            let toggle_w = 60.0 * dpr;
            let toggle_h = 28.0 * dpr;
            let toggle_x = row_x + row_w - pad - toggle_w;
            let toggle_y = row_y + (row_h - toggle_h) / 2.0;

            if cx >= toggle_x && cx <= toggle_x + toggle_w
                && cy >= toggle_y && cy <= toggle_y + toggle_h
            {
                let mut r = renderer.borrow_mut();
                r.glyph_mode = !r.glyph_mode;
                save_glyph_mode(r.glyph_mode);
            }
        }
        AppState::Playing => {} // shouldn't be called
    }
}

/// Handle a drawer tap result — shared between portrait and landscape modes.
fn handle_drawer_tap(
    gm: &mut Game,
    renderer: &Rc<RefCell<Renderer>>,
    go_to_menu: &mut bool,
    dtap: DrawerTap,
) {
    match dtap {
        DrawerTap::InventoryItem(idx) => {
            if gm.selected_inventory_item == Some(idx) {
                gm.selected_inventory_item = None;
            } else {
                gm.selected_inventory_item = Some(idx);
            }
        }
        DrawerTap::UseEquip(idx) | DrawerTap::InlineUseEquip(idx) => {
            if idx < gm.inventory.len() {
                let is_consumable = matches!(
                    gm.inventory[idx].kind,
                    ItemKind::Potion | ItemKind::Scroll | ItemKind::Food
                );
                if is_consumable {
                    gm.use_item(idx);
                    gm.drawer = Drawer::None;
                    gm.advance_turn();
                } else {
                    gm.equip_item(idx);
                }
                gm.selected_inventory_item = None;
            }
        }
        DrawerTap::Drop(idx) | DrawerTap::InlineDrop(idx) => {
            gm.drop_item(idx);
            gm.selected_inventory_item = None;
        }
        DrawerTap::ScrollUp => {
            gm.scroll_inventory(-1);
        }
        DrawerTap::ScrollDown => {
            gm.scroll_inventory(1);
        }
        DrawerTap::StatsAllocate(skill) => {
            gm.allocate_skill_point(skill);
        }
        DrawerTap::ToggleGlyphMode => {
            let mut r = renderer.borrow_mut();
            r.glyph_mode = !r.glyph_mode;
            save_glyph_mode(r.glyph_mode);
        }
        DrawerTap::MainMenu => {
            gm.drawer = Drawer::None;
            *go_to_menu = true;
        }
        DrawerTap::Consumed => {}
    }
}

fn request_animation_frame(window: &web_sys::Window, f: &Closure<dyn FnMut()>) {
    if let Err(e) = window.request_animation_frame(f.as_ref().unchecked_ref()) {
        errors::report_error(&format!("requestAnimationFrame failed: {:?}", e));
    }
}

/// Get window CSS dimensions, returning fallback (320, 480) on failure.
fn window_css_size() -> (f64, f64) {
    let w = web_sys::window().expect("no global window");
    let css_w = w.inner_width().ok().and_then(|v| v.as_f64()).unwrap_or(320.0);
    let css_h = w.inner_height().ok().and_then(|v| v.as_f64()).unwrap_or(480.0);
    (css_w, css_h)
}

fn fit_canvas(canvas: &HtmlCanvasElement) -> (f64, f64) {
    let window = web_sys::window().expect("no global window");
    let dpr = window.device_pixel_ratio();
    let (css_w, css_h) = window_css_size();
    let px_w = (css_w * dpr).round();
    let px_h = (css_h * dpr).round();
    canvas.set_width(px_w as u32);
    canvas.set_height(px_h as u32);
    (px_w, px_h)
}

fn new_game_with_config(seed: u64, config: GameConfig) -> Game {
    let mut map = Map::generate_forest(200, 200, seed);
    let entrances = map.place_dungeons(seed.wrapping_add(1));
    map.build_roads(&entrances);
    let world = World::new(map, entrances, seed.wrapping_add(2));
    let mut game = Game::new_overworld_with_config(world, config);
    game.spawn_enemies(seed.wrapping_mul(6364136223846793005));
    game.spawn_overworld_items(seed.wrapping_add(3));
    game.spawn_overworld_food(seed.wrapping_add(4));
    game.update_fov();
    game
}

fn new_game() -> Game {
    let seed = js_sys::Date::now() as u64 ^ 0xDEAD_BEEF;
    new_game_with_config(seed, GameConfig::normal())
}

/// Load an image from a URL and call `on_load` when ready.
/// Reports load failures to the error overlay.
fn load_image(src: &str, on_load: impl FnMut() + 'static) -> HtmlImageElement {
    let img = HtmlImageElement::new().expect("failed to create HtmlImageElement");
    let cb = Closure::<dyn FnMut()>::new(on_load);
    img.set_onload(Some(cb.as_ref().unchecked_ref()));
    cb.forget();
    // Report image load errors to the DOM overlay
    let src_owned = src.to_string();
    let err_cb = Closure::<dyn FnMut()>::new(move || {
        errors::report_error(&format!("failed to load sprite sheet: {}", src_owned));
    });
    img.set_onerror(Some(err_cb.as_ref().unchecked_ref()));
    err_cb.forget();
    img.set_src(src);
    img
}

/// Load an optional image — calls `on_load` with the element if it succeeds,
/// silently ignores failures (no error overlay).
fn load_image_optional(src: &str, mut on_load: impl FnMut(HtmlImageElement) + 'static) -> HtmlImageElement {
    let img = HtmlImageElement::new().expect("failed to create HtmlImageElement");
    let img_clone = img.clone();
    let cb = Closure::<dyn FnMut()>::new(move || {
        on_load(img_clone.clone());
    });
    img.set_onload(Some(cb.as_ref().unchecked_ref()));
    cb.forget();
    // Silently ignore load errors for optional sheets
    img.set_src(src);
    img
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    errors::install_panic_hook();

    let window = web_sys::window().expect("no global window");
    let document = window.document().expect("window has no document");

    let canvas = document
        .get_element_by_id("canvas")
        .ok_or_else(|| JsValue::from_str("canvas element not found"))?
        .dyn_into::<HtmlCanvasElement>()?;

    let ctx = canvas
        .get_context("2d")?
        .ok_or_else(|| JsValue::from_str("failed to get 2d context"))?
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    let game = Rc::new(RefCell::new(new_game()));
    let mut rend = Renderer::new(ctx);
    rend.glyph_mode = load_glyph_mode();
    let renderer = Rc::new(RefCell::new(rend));
    let input = Rc::new(Input::new(&canvas));
    let canvas = Rc::new(canvas);
    let menu_state = Rc::new(RefCell::new(MenuState {
        app_state: AppState::MainMenu,
        has_save: false,
    }));

    // Auto-move queue: steps remaining from a pathfind swipe
    let auto_path: Rc<RefCell<Vec<(i32, i32)>>> = Rc::new(RefCell::new(Vec::new()));
    // Preview path: computed during live swipe for rendering
    let preview_path: Rc<RefCell<Vec<(i32, i32)>>> = Rc::new(RefCell::new(Vec::new()));
    // Frame counter for throttling auto-move speed
    let auto_move_tick: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
    // Drawer swipe-scroll base: (base_scroll, start_y) when swipe began
    let drawer_swipe_base: Rc<RefCell<Option<(usize, f64)>>> = Rc::new(RefCell::new(None));
    // Drag-and-drop state for quick-bar item assignment
    let drag_state: Rc<RefCell<DragState>> = Rc::new(RefCell::new(DragState::Idle));
    // Frame counter (used for drag long-press timing)
    let frame_counter: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));

    // Load sprite sheets asynchronously (4 core + optional animals)
    {
        let loaded_count: Rc<RefCell<u32>> = Rc::new(RefCell::new(0));
        let renderer_for_load = Rc::clone(&renderer);

        let tiles_img: Rc<RefCell<Option<HtmlImageElement>>> = Rc::new(RefCell::new(None));
        let monsters_img: Rc<RefCell<Option<HtmlImageElement>>> = Rc::new(RefCell::new(None));
        let rogues_img: Rc<RefCell<Option<HtmlImageElement>>> = Rc::new(RefCell::new(None));
        let items_img: Rc<RefCell<Option<HtmlImageElement>>> = Rc::new(RefCell::new(None));

        let make_on_load = |slot: Rc<RefCell<Option<HtmlImageElement>>>,
                            loaded: Rc<RefCell<u32>>,
                            rend: Rc<RefCell<Renderer>>,
                            t: Rc<RefCell<Option<HtmlImageElement>>>,
                            m: Rc<RefCell<Option<HtmlImageElement>>>,
                            r: Rc<RefCell<Option<HtmlImageElement>>>,
                            i: Rc<RefCell<Option<HtmlImageElement>>>| {
            move || {
                let _ = slot.borrow();
                let mut count = loaded.borrow_mut();
                *count += 1;
                if *count == 4 {
                    let tiles = t.borrow_mut().take().expect("tiles sprite sheet missing");
                    let monsters = m.borrow_mut().take().expect("monsters sprite sheet missing");
                    let rogues = r.borrow_mut().take().expect("rogues sprite sheet missing");
                    let items = i.borrow_mut().take().expect("items sprite sheet missing");
                    rend.borrow_mut().set_sheets(SpriteSheets { tiles, monsters, rogues, items, animals: None });
                }
            }
        };

        let img = load_image(
            "assets/tiles.png",
            make_on_load(
                Rc::clone(&tiles_img), Rc::clone(&loaded_count), Rc::clone(&renderer_for_load),
                Rc::clone(&tiles_img), Rc::clone(&monsters_img), Rc::clone(&rogues_img), Rc::clone(&items_img),
            ),
        );
        *tiles_img.borrow_mut() = Some(img);

        let img = load_image(
            "assets/monsters.png",
            make_on_load(
                Rc::clone(&monsters_img), Rc::clone(&loaded_count), Rc::clone(&renderer_for_load),
                Rc::clone(&tiles_img), Rc::clone(&monsters_img), Rc::clone(&rogues_img), Rc::clone(&items_img),
            ),
        );
        *monsters_img.borrow_mut() = Some(img);

        let img = load_image(
            "assets/rogues.png",
            make_on_load(
                Rc::clone(&rogues_img), Rc::clone(&loaded_count), Rc::clone(&renderer_for_load),
                Rc::clone(&tiles_img), Rc::clone(&monsters_img), Rc::clone(&rogues_img), Rc::clone(&items_img),
            ),
        );
        *rogues_img.borrow_mut() = Some(img);

        let img = load_image(
            "assets/items.png",
            make_on_load(
                Rc::clone(&items_img), Rc::clone(&loaded_count), Rc::clone(&renderer_for_load),
                Rc::clone(&tiles_img), Rc::clone(&monsters_img), Rc::clone(&rogues_img), Rc::clone(&items_img),
            ),
        );
        *items_img.borrow_mut() = Some(img);

        // Load animals.png optionally — if it fails, animal sprites use glyph fallback
        {
            let rend = Rc::clone(&renderer_for_load);
            let img = load_image_optional("assets/animals.png", move |animals_el| {
                rend.borrow_mut().set_animals_sheet(animals_el);
            });
            std::mem::drop(img);
        }
    }

    // Initial sizing + camera snap
    {
        let dpr = window.device_pixel_ratio();
        let (w, h) = fit_canvas(&canvas);
        let mut r = renderer.borrow_mut();
        r.resize(w, h, dpr);
        let gm = game.borrow();
        let map = gm.current_map();
        r.camera.snap(gm.player_x as f64, gm.player_y as f64, map.width, map.height);
    }

    // Resize handler
    {
        let canvas = Rc::clone(&canvas);
        let game = Rc::clone(&game);
        let renderer = Rc::clone(&renderer);
        let preview_path = Rc::clone(&preview_path);
        let menu_state = Rc::clone(&menu_state);
        let cb = Closure::<dyn FnMut()>::new(move || {
            let dpr = web_sys::window().expect("no global window").device_pixel_ratio();
            let (w, h) = fit_canvas(&canvas);
            let mut r = renderer.borrow_mut();
            r.resize(w, h, dpr);
            let ms = menu_state.borrow();
            match &ms.app_state {
                AppState::Playing => {
                    let gm = game.borrow();
                    let map = gm.current_map();
                    r.camera.snap(gm.player_x as f64, gm.player_y as f64, map.width, map.height);
                    r.draw(&gm, &preview_path.borrow(), None);
                }
                AppState::MainMenu => r.draw_main_menu(ms.has_save),
                AppState::NewGame { selected_difficulty, seed } => {
                    r.draw_new_game_menu(*selected_difficulty, *seed);
                }
                AppState::Settings => r.draw_settings_menu(),
            }
        });
        window
            .add_event_listener_with_callback("resize", cb.as_ref().unchecked_ref())?;
        cb.forget();
    }

    // Game loop
    let f: Rc<RefCell<Option<Closure<dyn FnMut()>>>> = Rc::new(RefCell::new(None));
    let g = Rc::clone(&f);

    let window2 = web_sys::window().expect("no global window");
    let menu_state_loop = Rc::clone(&menu_state);
    *g.borrow_mut() = Some(Closure::new(move || {
        let dpr = web_sys::window().expect("no global window").device_pixel_ratio();

        // === Menu state routing ===
        {
            let app_state = menu_state_loop.borrow().app_state.clone();
            match app_state {
                AppState::MainMenu | AppState::NewGame { .. } | AppState::Settings => {
                    // Handle menu input
                    let actions = input.drain();
                    let (css_w, css_h) = window_css_size();
                    for action in &actions {
                        if let InputAction::Tap(cx, cy) = action {
                            let mut ms = menu_state_loop.borrow_mut();
                            handle_menu_tap(&mut ms, &game, &renderer, &auto_path, *cx, *cy, css_w, css_h, dpr);
                        }
                    }
                    // Render menu
                    {
                        let (w, h) = fit_canvas(&canvas);
                        let ms = menu_state_loop.borrow();
                        let mut r = renderer.borrow_mut();
                        r.resize(w, h, dpr);
                        match &ms.app_state {
                            AppState::MainMenu => r.draw_main_menu(ms.has_save),
                            AppState::NewGame { selected_difficulty, seed } => {
                                r.draw_new_game_menu(*selected_difficulty, *seed);
                            }
                            AppState::Settings => r.draw_settings_menu(),
                            AppState::Playing => {} // handled below
                        }
                    }
                    request_animation_frame(&window2, f.borrow().as_ref().expect("game loop closure missing"));
                    return;
                }
                AppState::Playing => {} // fall through to game loop
            }
        }

        // Process input actions
        let actions = input.drain();
        if !actions.is_empty() {
            let mut gm = game.borrow_mut();
            if !gm.alive || gm.won {
                // Return to main menu on death/win
                let mut ms = menu_state_loop.borrow_mut();
                ms.app_state = AppState::MainMenu;
                ms.has_save = false;
                drop(ms);
                drop(gm);
                auto_path.borrow_mut().clear();
            } else {
                let mut map_changed = false;
                let mut go_to_menu = false;
                for action in actions {
                    match action {
                        InputAction::Step(dir) => {
                            if gm.drawer != Drawer::None {
                                // Swipe/movement disabled while drawer is open
                                continue;
                            }
                            gm.inspected = None;
                            auto_path.borrow_mut().clear();
                            let (dx, dy) = match dir {
                                input::Direction::Up => (0, -1),
                                input::Direction::Down => (0, 1),
                                input::Direction::Left => (-1, 0),
                                input::Direction::Right => (1, 0),
                                input::Direction::UpLeft => (-1, -1),
                                input::Direction::UpRight => (1, -1),
                                input::Direction::DownLeft => (-1, 1),
                                input::Direction::DownRight => (1, 1),
                            };
                            let result = gm.move_player(dx, dy);
                            if matches!(result, TurnResult::MapChanged) {
                                map_changed = true;
                            } else if matches!(result, TurnResult::Blocked) {
                                // Step blocked by enemy → attack it
                                let tx = gm.player_x + dx;
                                let ty = gm.player_y + dy;
                                if gm.enemies.iter().any(|e| e.x == tx && e.y == ty && e.hp > 0) {
                                    let atk_result = gm.attack_adjacent(tx, ty);
                                    if matches!(atk_result, TurnResult::MapChanged) {
                                        map_changed = true;
                                    }
                                }
                            }
                        }
                        InputAction::ExecutePath => {
                            if gm.drawer != Drawer::None {
                                continue;
                            }
                            let pp = preview_path.borrow();
                            if pp.len() > 1 {
                                let target = pp[pp.len() - 1];
                                let enemy_at_target = gm.has_ranged_weapon()
                                    && gm.enemies.iter().any(|e| e.x == target.0 && e.y == target.1 && e.hp > 0);
                                if enemy_at_target {
                                    // Fire ranged weapon at the targeted enemy
                                    drop(pp);
                                    let result = gm.ranged_attack(target.0, target.1);
                                    if matches!(result, TurnResult::MapChanged) {
                                        map_changed = true;
                                    }
                                } else {
                                    // Normal pathfinding movement
                                    *auto_path.borrow_mut() = pp[1..].to_vec();
                                }
                            }
                        }
                        InputAction::Tap(css_x, css_y) => {
                            // Skip tap only if actively dragging (drop is handled separately).
                            // Pending state should NOT block taps — it means the long-press
                            // timer hasn't fired yet, so the touch was a normal tap.
                            if matches!(*drag_state.borrow(), DragState::Dragging { .. }) {
                                continue;
                            }
                            // Cancel any pending drag since the finger lifted (tap = touchend)
                            if !matches!(*drag_state.borrow(), DragState::Idle) {
                                *drag_state.borrow_mut() = DragState::Idle;
                            }
                            let (css_w, css_h) = window_css_size();
                            let is_landscape = renderer.borrow().landscape;
                            let panel_w = renderer.borrow().side_panel_css_w();

                            if is_landscape {
                                // Landscape mode: side panel on right
                                let panel_x = css_w - panel_w;
                                if css_x >= panel_x {
                                    // Tap in side panel — check buttons first, then drawer
                                    if let Some(tap) = hit_test_side_panel_buttons(css_x, css_y, css_w, panel_w) {
                                        match tap {
                                            BarTap::OpenDrawer(drawer) => gm.toggle_drawer(drawer),
                                            BarTap::Sprint => gm.toggle_sprint(),
                                        }
                                    } else if let Some(slot) = hit_test_side_panel_quickbar(css_x, css_y, css_w, panel_w, gm.inspected.is_some()) {
                                        // Quick-bar tap in landscape
                                        if let Some(inv_idx) = gm.quick_bar.slots[slot] {
                                            if inv_idx < gm.inventory.len() {
                                                let kind = gm.inventory[inv_idx].kind.clone();
                                                match kind {
                                                    ItemKind::Potion | ItemKind::Scroll | ItemKind::Food => {
                                                        gm.use_item(inv_idx);
                                                        renderer.borrow_mut().flash_quickbar_slot(slot);
                                                    }
                                                    _ => {}
                                                }
                                            }
                                        }
                                    } else if gm.drawer != Drawer::None {
                                        if let Some(dtap) = hit_test_side_panel_drawer(
                                            css_x, css_y, css_w, css_h, panel_w,
                                            gm.drawer, gm.inventory.len(), gm.inventory_scroll,
                                            gm.skill_points,
                                            gm.has_ranged_weapon(),
                                        ) {
                                            handle_drawer_tap(&mut gm, &renderer, &mut go_to_menu, dtap);
                                        }
                                    }
                                    // Side panel taps always consumed
                                } else {
                                    // Tap in game area (left of panel)
                                    let r = renderer.borrow();
                                    let (wx, wy) = r.camera.screen_to_world(css_x * dpr, css_y * dpr);
                                    drop(r);
                                    let dist = (wx - gm.player_x).abs() + (wy - gm.player_y).abs();
                                    if dist == 1 && gm.enemies.iter().any(|e| e.x == wx && e.y == wy && e.hp > 0) {
                                        gm.attack_adjacent(wx, wy);
                                    } else if dist == 0 {
                                        gm.pickup_items_explicit();
                                    }
                                    gm.inspected = gm.inspect_tile(wx, wy);
                                }
                            } else {
                                // Portrait mode: bottom bar + quick bar + drawer
                                let r_borrow = renderer.borrow();
                                let bar_h_css = r_borrow.bottom_bar_height() / dpr;
                                let qbar_h_css = r_borrow.quickbar_height() / dpr;
                                drop(r_borrow);
                                // Combined region for drawer positioning
                                let bottom_region_css = bar_h_css + qbar_h_css;

                                if let Some(tap) = hit_test_bottom_bar(css_x, css_y, css_w, css_h, bar_h_css) {
                                    match tap {
                                        BarTap::OpenDrawer(drawer) => gm.toggle_drawer(drawer),
                                        BarTap::Sprint => gm.toggle_sprint(),
                                    }
                                } else if let Some(slot) = hit_test_quick_bar(css_x, css_y, css_w, css_h, bar_h_css, qbar_h_css) {
                                    // Quick-bar tap: use item if slot is occupied
                                    if let Some(inv_idx) = gm.quick_bar.slots[slot] {
                                        if inv_idx < gm.inventory.len() {
                                            let kind = gm.inventory[inv_idx].kind.clone();
                                            match kind {
                                                ItemKind::Potion | ItemKind::Scroll | ItemKind::Food => {
                                                    gm.use_item(inv_idx);
                                                    renderer.borrow_mut().flash_quickbar_slot(slot);
                                                }
                                                _ => {}
                                            }
                                        }
                                    }
                                } else if let Some(dtap) = hit_test_drawer(
                                    css_x, css_y, css_w, css_h, bottom_region_css,
                                    gm.drawer, gm.inventory.len(), gm.inventory_scroll,
                                    gm.selected_inventory_item, gm.skill_points,
                                    gm.stats_scroll, gm.has_ranged_weapon(),
                                ) {
                                    handle_drawer_tap(&mut gm, &renderer, &mut go_to_menu, dtap);
                                } else if css_y < css_h - bottom_region_css {
                                    // Tap in game area
                                    let r = renderer.borrow();
                                    let (wx, wy) = r.camera.screen_to_world(css_x * dpr, css_y * dpr);
                                    drop(r);
                                    let dist = (wx - gm.player_x).abs() + (wy - gm.player_y).abs();
                                    if dist == 1 && gm.enemies.iter().any(|e| e.x == wx && e.y == wy && e.hp > 0) {
                                        gm.attack_adjacent(wx, wy);
                                    } else if dist == 0 {
                                        gm.pickup_items_explicit();
                                    }
                                    gm.inspected = gm.inspect_tile(wx, wy);
                                }
                            }
                        }
                        InputAction::ToggleInventory => {
                            gm.toggle_drawer(Drawer::Inventory);
                        }
                        InputAction::ToggleStats => {
                            gm.toggle_drawer(Drawer::Stats);
                        }
                        InputAction::ToggleSprint => {
                            gm.toggle_sprint();
                        }
                        InputAction::ToggleGlyphMode => {
                            let mut r = renderer.borrow_mut();
                            r.glyph_mode = !r.glyph_mode;
                            save_glyph_mode(r.glyph_mode);
                        }
                        InputAction::QuickUse(slot) => {
                            if gm.drawer == Drawer::None {
                                if let Some(inv_idx) = gm.quick_bar.slots[slot] {
                                    if inv_idx < gm.inventory.len() {
                                        let kind = gm.inventory[inv_idx].kind.clone();
                                        match kind {
                                            ItemKind::Potion | ItemKind::Scroll | ItemKind::Food => {
                                                gm.use_item(inv_idx);
                                                renderer.borrow_mut().flash_quickbar_slot(slot);
                                            }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                        InputAction::Interact => {
                            if gm.drawer != Drawer::None {
                                continue;
                            }
                            // Try to attack adjacent enemy in facing direction
                            let (fdx, fdy) = if gm.player_facing_left { (-1, 0) } else { (1, 0) };
                            let tx = gm.player_x + fdx;
                            let ty = gm.player_y + fdy;
                            if gm.enemies.iter().any(|e| e.x == tx && e.y == ty && e.hp > 0) {
                                gm.attack_adjacent(tx, ty);
                            } else {
                                // Try all 8 directions for adjacent enemy (including diagonals)
                                let dirs = [(0, -1), (0, 1), (-1, 0), (1, 0),
                                            (-1, -1), (1, -1), (-1, 1), (1, 1)];
                                let mut attacked = false;
                                for (dx, dy) in dirs {
                                    let ax = gm.player_x + dx;
                                    let ay = gm.player_y + dy;
                                    if gm.enemies.iter().any(|e| e.x == ax && e.y == ay && e.hp > 0) {
                                        gm.attack_adjacent(ax, ay);
                                        attacked = true;
                                        break;
                                    }
                                }
                                // No adjacent enemy → pick up items at feet
                                if !attacked {
                                    gm.pickup_items_explicit();
                                }
                            }
                        }
                    }
                }
                // Camera snap on map transition
                if map_changed {
                    let mut r = renderer.borrow_mut();
                    let map = gm.current_map();
                    r.camera.snap(gm.player_x as f64, gm.player_y as f64, map.width, map.height);
                    auto_path.borrow_mut().clear();
                }
                // Return to main menu (from in-game settings)
                if go_to_menu {
                    drop(gm);
                    let mut ms = menu_state_loop.borrow_mut();
                    ms.app_state = AppState::MainMenu;
                    ms.has_save = true;
                }
            }
        }

        // Compute live preview path from swipe state + inspect hovered tile
        // OR handle drawer swipe-scrolling if drawer is open
        {
            let mut pp = preview_path.borrow_mut();
            pp.clear();
            if let Some(swipe) = input.swipe_state() {
                let mut gm = game.borrow_mut();
                if gm.alive && !gm.won && gm.drawer == Drawer::None {
                    let dx = (swipe.current_x - swipe.start_x) * 0.7;
                    let dy = (swipe.current_y - swipe.start_y) * 0.7;
                    let (gdx, gdy) = renderer.borrow().camera.css_delta_to_grid(dx, dy, dpr);
                    let dest = (gm.player_x + gdx, gm.player_y + gdy);
                    let map = gm.current_map();
                    // Aim mode: show Bresenham line only when hovering an enemy
                    let enemy_at_dest = gm.has_ranged_weapon()
                        && gm.enemies.iter().any(|e| e.x == dest.0 && e.y == dest.1 && e.hp > 0);
                    if enemy_at_dest {
                        let line = bresenham_line(gm.player_x, gm.player_y, dest.0, dest.1);
                        *pp = line;
                    } else if map.is_walkable(dest.0, dest.1) {
                        let path = map.find_path((gm.player_x, gm.player_y), dest);
                        *pp = path;
                    }
                    // Inspect the destination tile during swipe
                    gm.inspected = gm.inspect_tile(dest.0, dest.1);
                } else if gm.drawer == Drawer::Inventory {
                    // Swipe-scroll inventory when drawer is open
                    let slot_h_css = 34.0;
                    let mut base = drawer_swipe_base.borrow_mut();
                    if base.is_none() {
                        *base = Some((gm.inventory_scroll, swipe.start_y));
                    }
                    if let Some((base_scroll, start_y)) = *base {
                        let dy = start_y - swipe.current_y; // positive = scroll down
                        let delta = (dy / slot_h_css).round() as i32;
                        let new_scroll = (base_scroll as i32 + delta).max(0) as usize;
                        gm.set_inventory_scroll(new_scroll);
                    }
                } else if gm.drawer == Drawer::Stats {
                    // Swipe-scroll stats panel
                    let mut base = drawer_swipe_base.borrow_mut();
                    if base.is_none() {
                        *base = Some((gm.stats_scroll as usize, swipe.start_y));
                    }
                    if let Some((base_scroll, start_y)) = *base {
                        let dy = start_y - swipe.current_y; // positive = scroll down
                        let new_scroll = (base_scroll as f64 + dy).max(0.0);
                        gm.stats_scroll = new_scroll;
                    }
                }
            } else {
                // Swipe ended — clear drawer swipe base
                *drawer_swipe_base.borrow_mut() = None;
            }
        }

        // === Drag-and-drop state machine for quick-bar ===
        {
            let mut ds = drag_state.borrow_mut();
            let fc = *frame_counter.borrow();
            let td = input.touch_down();
            let gm_drawer = game.borrow().drawer;
            let is_landscape = renderer.borrow().landscape;

            match *ds {
                DragState::Idle => {
                    if let Some(td) = td {
                        let (css_w, css_h) = window_css_size();
                        if gm_drawer == Drawer::Inventory {
                            // Finger down while inventory is open — check if on an item row
                            let inv_idx = if is_landscape {
                                let panel_css_w = renderer.borrow().side_panel_css_w();
                                hit_test_inventory_item_row_landscape(td.start_x, td.start_y, css_w, css_h, panel_css_w,
                                    game.borrow().inventory.len(), game.borrow().inventory_scroll)
                            } else {
                                let bar_h_css = renderer.borrow().bottom_bar_height() / dpr;
                                let qbar_h_css = renderer.borrow().quickbar_height() / dpr;
                                let bottom_region = bar_h_css + qbar_h_css;
                                hit_test_inventory_item_row(td.start_x, td.start_y, css_w, css_h, bottom_region,
                                    game.borrow().inventory.len(), game.borrow().inventory_scroll)
                            };
                            if let Some(idx) = inv_idx {
                                let gm = game.borrow();
                                // Only allow dragging consumables
                                if idx < gm.inventory.len() {
                                    let kind = &gm.inventory[idx].kind;
                                    if matches!(kind, ItemKind::Potion | ItemKind::Scroll | ItemKind::Food) {
                                        *ds = DragState::Pending {
                                            inv_index: idx,
                                            start_x: td.start_x,
                                            start_y: td.start_y,
                                            start_frame: fc,
                                            from_quickbar_slot: None,
                                        };
                                    }
                                }
                            }
                        }
                        // When drawer is closed, do NOT start drags on quick-bar slots.
                        // Taps on quick-bar slots are handled by the tap input processing.
                    }
                }
                DragState::Pending { inv_index, start_x, start_y, start_frame, from_quickbar_slot } => {
                    if let Some(td) = td {
                        let dx = td.current_x - start_x;
                        let dy = td.current_y - start_y;
                        let dist = (dx * dx + dy * dy).sqrt();
                        if dist > 12.0 {
                            // Moved too far — cancel, let normal scroll/swipe handle it
                            *ds = DragState::Idle;
                        } else if fc.wrapping_sub(start_frame) >= 18 {
                            // ~300ms at 60fps — promote to dragging
                            *ds = DragState::Dragging {
                                inv_index,
                                touch_x: td.current_x,
                                touch_y: td.current_y,
                                from_quickbar_slot,
                            };
                        }
                    } else {
                        // Finger lifted before long-press — cancel
                        *ds = DragState::Idle;
                    }
                }
                DragState::Dragging { inv_index, ref mut touch_x, ref mut touch_y, from_quickbar_slot } => {
                    if let Some(td) = td {
                        // Update position
                        *touch_x = td.current_x;
                        *touch_y = td.current_y;
                    } else {
                        // Finger lifted — drop!
                        let (css_w, css_h) = window_css_size();
                        let mut gm = game.borrow_mut();

                        // Check if dropped on a quick-bar slot
                        let drop_slot = if is_landscape {
                            let panel_css_w = renderer.borrow().side_panel_css_w();
                            hit_test_side_panel_quickbar(*touch_x, *touch_y, css_w, panel_css_w, gm.inspected.is_some())
                        } else {
                            let bar_h_css = renderer.borrow().bottom_bar_height() / dpr;
                            let qbar_h_css = renderer.borrow().quickbar_height() / dpr;
                            hit_test_quick_bar(*touch_x, *touch_y, css_w, css_h, bar_h_css, qbar_h_css)
                        };

                        if let Some(slot) = drop_slot {
                            if inv_index < gm.inventory.len() {
                                let item = gm.inventory[inv_index].clone();
                                if let Some(from_slot) = from_quickbar_slot {
                                    // Drag from quick-bar slot to another slot → swap
                                    gm.quick_bar.swap(from_slot, slot);
                                } else {
                                    gm.quick_bar.assign(slot, inv_index, &item);
                                }
                            }
                        }
                        // Dropped outside a quick-bar slot — cancel (no change).
                        // Whether from inventory or from a quick-bar slot, just do nothing.

                        drop(gm);
                        *ds = DragState::Idle;
                    }
                }
            }

            *frame_counter.borrow_mut() = fc.wrapping_add(1);
        }

        // Process auto-move queue (one step every 8 frames ≈ 7.5 tiles/sec)
        {
            let mut ap = auto_path.borrow_mut();
            if !ap.is_empty() {
                let mut tick = auto_move_tick.borrow_mut();
                *tick += 1;
                if *tick >= 8 {
                    *tick = 0;
                    let mut gm = game.borrow_mut();
                    if gm.alive && !gm.won {
                        let (nx, ny) = ap[0];
                        let dx = nx - gm.player_x;
                        let dy = ny - gm.player_y;
                        let result = gm.move_player(dx, dy);
                        ap.remove(0);
                        if matches!(result, TurnResult::Blocked) {
                            // Blocked by enemy → attack and stop auto-move
                            if gm.enemies.iter().any(|e| e.x == nx && e.y == ny && e.hp > 0) {
                                gm.attack_adjacent(nx, ny);
                            }
                            ap.clear();
                        } else if gm.player_x != nx || gm.player_y != ny || matches!(result, TurnResult::MapChanged) {
                            ap.clear();
                        }
                    } else {
                        ap.clear();
                    }
                }
            }
        }

        // Update camera — follow the player
        {
            let gm = game.borrow();
            let map = gm.current_map();
            renderer.borrow_mut().camera.follow(
                gm.player_x as f64,
                gm.player_y as f64,
                map.width,
                map.height,
            );
        }

        // Tick animations (floating texts, bump anims, visual effects)
        {
            game.borrow_mut().tick_animations();
        }

        // Advance drawer animation + render
        {
            let gm = game.borrow();
            let mut r = renderer.borrow_mut();
            r.tick_drawer_anim(gm.drawer);
            r.tick_quickbar_flash();
            let ds = drag_state.borrow();
            let drag_info = if let DragState::Dragging { inv_index, touch_x, touch_y, .. } = &*ds {
                gm.inventory.get(*inv_index).map(|item| renderer::DragInfo {
                    item,
                    css_x: *touch_x,
                    css_y: *touch_y,
                })
            } else {
                None
            };
            r.draw(&gm, &preview_path.borrow(), drag_info.as_ref());
        }

        // Schedule next frame
        request_animation_frame(&window2, f.borrow().as_ref().expect("game loop closure missing"));
    }));

    let window3 = web_sys::window().expect("no global window");
    request_animation_frame(&window3, g.borrow().as_ref().expect("game loop closure missing"));

    Ok(())
}
