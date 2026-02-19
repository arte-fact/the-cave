# Plan: Diablo-Style Quick-Use Bar (Drag & Drop)

## Concept

A new row of item slots that sits directly **above** the existing bottom action
bar (portrait) or at the bottom of the side panel (landscape). Inspired by
Diablo's belt/quick-use bar: the player drags consumable items (potions,
scrolls, food) from the inventory into numbered slots, then taps a slot during
gameplay to instantly use that item — no drawer required.

```
┌──────────────────────────────────┐
│          game world              │
│                                  │
│                                  │
├──────────────────────────────────┤  ← quick bar top edge
│ [slot 1] [slot 2] [slot 3] [4]  │  ← NEW quick-use bar row
├──────────────────────────────────┤  ← existing bottom bar top edge
│  Inventory  Stats  Sprint  Gear  │  ← existing bottom bar (unchanged)
└──────────────────────────────────┘
```

### Design principles

- **Mobile-first** — slots are large enough for thumbs (~44×44 CSS px).
- **Consumables only** — only Potion, Scroll, and Food items can be assigned
  to the quick bar. Equipment (weapons, armor, etc.) cannot.
- **Drag from inventory** — long-press an item in the inventory drawer to begin
  dragging, then drop it onto a quick-bar slot.
- **Tap to use** — during gameplay (drawer closed), tap a quick-bar slot to
  immediately consume the item. The item is removed from inventory and the slot
  empties.
- **No duplication** — the quick bar holds *references* (indices) into the
  inventory, not copies. When an item is consumed, dropped, or the inventory
  changes, stale slots are cleared automatically.

---

## Part 1: Data Model

### 1a. `QuickBar` struct in `game/types.rs`

```rust
/// Number of quick-bar slots.
pub const QUICKBAR_SLOTS: usize = 4;

/// A quick-use bar slot. Holds an inventory index for a consumable item.
/// `None` = empty slot.
pub type QuickBarSlot = Option<usize>;

/// The quick-use bar: fixed-size array of slots referencing inventory items.
#[derive(Clone, Debug)]
pub struct QuickBar {
    pub slots: [QuickBarSlot; QUICKBAR_SLOTS],
}
```

Default: all slots `None`.

### 1b. Add `quick_bar: QuickBar` to `Game` struct (`game/mod.rs`)

Initialize in `Game::new_overworld()` with all empty slots.

### 1c. Slot validity invariant

After any inventory mutation (use, equip, drop, pickup, reorder), call
`quick_bar.revalidate(&self.inventory)` to:

1. Clear slots whose index is now out of bounds.
2. Clear slots whose referenced item is no longer consumable (shouldn't happen,
   but defensive).
3. Remap indices when items shift due to `Vec::remove()`. This is the tricky
   part — when item at index `i` is removed, every slot pointing to `j > i`
   must decrement by 1.

```rust
impl QuickBar {
    /// Fix up slot indices after an item at `removed_idx` was removed from inventory.
    pub fn on_item_removed(&mut self, removed_idx: usize) {
        for slot in &mut self.slots {
            if let Some(idx) = slot {
                if *idx == removed_idx {
                    *slot = None;
                } else if *idx > removed_idx {
                    *idx -= 1;
                }
            }
        }
    }

    /// Assign an inventory item to a slot. Returns false if the item isn't consumable.
    pub fn assign(&mut self, slot: usize, inv_index: usize, item: &Item) -> bool {
        if slot >= QUICKBAR_SLOTS { return false; }
        match item.kind {
            ItemKind::Potion | ItemKind::Scroll | ItemKind::Food => {
                // Remove any existing assignment of this inventory index
                for s in &mut self.slots {
                    if *s == Some(inv_index) { *s = None; }
                }
                self.slots[slot] = Some(inv_index);
                true
            }
            _ => false,
        }
    }

    /// Clear a slot (drag item off, or manual clear).
    pub fn clear(&mut self, slot: usize) {
        if slot < QUICKBAR_SLOTS {
            self.slots[slot] = None;
        }
    }
}
```

### 1d. Tests (pure logic, `cargo test`)

- Assign a consumable → slot populated.
- Assign a weapon → rejected (returns false).
- Remove item at index 2 → slot pointing to 2 cleared, slot pointing to 4
  becomes 3.
- Assign same item to second slot → first slot cleared (no duplicates).
- Assign to out-of-bounds slot → no panic, returns false.
- `on_item_removed` with empty bar → no panic.

---

## Part 2: Drag & Drop State Machine

Touch-based drag-and-drop on mobile requires careful state management to
distinguish taps, swipes, and long-press-to-drag.

### 2a. `DragState` enum in `input.rs` (or new `drag.rs`)

```rust
pub enum DragState {
    /// No drag in progress.
    Idle,
    /// Long-press detected, item is being dragged. The ghost follows the finger.
    Dragging {
        /// Index of the item in inventory.
        inv_index: usize,
        /// Current finger position (CSS pixels) for ghost rendering.
        touch_x: f64,
        touch_y: f64,
    },
}
```

### 2b. Drag lifecycle (touch events)

| Event | Current state | Condition | Action |
|-------|--------------|-----------|--------|
| `touchstart` | Idle | Tap lands inside an inventory item row (drawer open) | Start a 300ms long-press timer. Record item index + start position. |
| `touchmove` | Timer running | Finger moves > 8px before 300ms | Cancel timer → fall through to normal swipe/scroll. |
| Timer fires | Timer running | 300ms elapsed, finger still near start | Transition to `Dragging { inv_index, touch_x, touch_y }`. Haptic feedback (`navigator.vibrate(30)` if available). |
| `touchmove` | Dragging | — | Update `touch_x/y`. Renderer draws ghost icon at finger. |
| `touchend` | Dragging | Finger over a quick-bar slot | Call `quick_bar.assign(slot, inv_index, &item)`. Show brief confirmation. |
| `touchend` | Dragging | Finger NOT over a quick-bar slot | Cancel drag — nothing happens. |
| `touchend` | Dragging | Finger over an *occupied* quick-bar slot | Replace: old assignment cleared, new item assigned. |

### 2c. Long-press timer implementation

Use `web_sys::Window::set_timeout_with_callback_and_timeout_and_arguments_0`
or, simpler, track elapsed time in the game loop:

- On `touchstart` in an inventory row: store `drag_candidate: Option<(usize, f64, f64, f64)>` = `(inv_index, x, y, timestamp)`.
- On each `touchmove`: if distance > 8px from start, clear `drag_candidate`.
- In `tick()` (called every frame): if `drag_candidate` is set and
  `now - timestamp > 300ms`, promote to `Dragging`.

This avoids Web API timer complexity and keeps everything in the game loop.

### 2d. Drag from quick-bar slot (reorder / clear)

Long-press on an *occupied* quick-bar slot (when drawer is closed) starts a drag
from the bar itself:

- Drop on another quick-bar slot → swap.
- Drop anywhere else (game world) → clear the slot.
- This allows reorganizing without opening inventory.

### 2e. Tests

- Long-press timer: simulate press at t=0, check no drag at t=200ms, drag at
  t=350ms.
- Move cancels drag candidate if distance > threshold.
- Drop on valid slot assigns item.
- Drop outside bar cancels cleanly.
- Drag from bar slot, drop on another slot → swap contents.

---

## Part 3: Quick-Bar Rendering (Portrait)

### 3a. Layout constants

```rust
const QUICKBAR_BASE: f64 = 44.0;  // Quick-bar height in CSS pixels
```

The quick bar sits between the message area and the bottom bar:

```
canvas_h - bottom_h - quickbar_h  →  quick bar top
canvas_h - bottom_h               →  quick bar bottom / bottom bar top
canvas_h                           →  canvas bottom
```

### 3b. `draw_quick_bar()` in `renderer/hud.rs`

- **Background**: Same style as bottom bar — `rgba(12,12,18,0.92)` with
  `rgba(255,255,255,0.08)` accent line on top.
- **Slots**: 4 evenly-spaced rounded rectangles, each ~40×40 CSS px.
- **Empty slot**: Dashed border, dim color, small "+" or slot number.
- **Occupied slot**: Item sprite (via `sprites::item_sprite(name)`) drawn at
  slot size. Thin colored border matching item kind color. Small quantity badge
  if we later add stacking.
- **Slot number labels**: Tiny "1" "2" "3" "4" in top-left corner of each
  slot (useful if we add keyboard shortcuts 1-4 later).

### 3c. Drag ghost rendering

When `DragState::Dragging`, draw the item sprite at `(touch_x, touch_y)` with
slight transparency (alpha 0.7) and a drop shadow. Draw it in `draw()` as the
very last element (above everything, including drawers and HUD).

When dragging over a valid quick-bar slot, highlight that slot with a glow
border (e.g., `rgba(255,200,80,0.6)` stroke).

### 3d. Camera padding adjustment

The quick bar adds height below the game viewport. Update `resize()`:

```rust
// Portrait padding now includes quick bar
self.camera.pad_bottom = (self.bottom_bar_h() + self.quickbar_h() + self.msg_area_h()) / cell;
```

This ensures the camera doesn't render tiles behind the quick bar.

### 3e. Inventory drawer coexistence

When the inventory drawer is open, the quick bar remains visible between the
drawer and the bottom bar. This is intentional — the player drags items *down*
from the inventory drawer *into* the quick-bar slots.

If the drawer covers the quick-bar area, shift the quick bar to remain visible
or render it at the bottom edge of the drawer.

---

## Part 4: Quick-Bar Rendering (Landscape)

### 4a. Layout in side panel

In landscape mode, the quick bar renders as a horizontal row at the bottom of
the side panel, just above the 2x2 button grid:

```
┌─────────────────────┐
│   Stats bars        │
│   Tile detail       │
│ ┌───┬───┬───┬───┐  │  ← quick bar
│ │ 1 │ 2 │ 3 │ 4 │  │
│ └───┴───┴───┴───┘  │
│ [Inv] [Stats]       │  ← 2x2 button grid
│ [Sprint] [Settings] │
└─────────────────────┘
```

### 4b. Side panel inventory drag

When the inventory is shown in the side panel (landscape), dragging works the
same way — long-press an item row → drag ghost → drop on quick-bar slot.

---

## Part 5: Quick-Bar Tap-to-Use

### 5a. Hit testing in `lib.rs`

Add `hit_test_quick_bar(css_x, css_y) -> Option<usize>` that returns the slot
index (0–3) if the tap lands within a quick-bar slot.

In the tap processing pipeline (portrait mode), check quick bar *before*
checking the game world:

```
1. Bottom bar buttons (Inventory, Stats, Sprint, Settings)
2. Quick-bar slots          ← NEW
3. Drawer content (if open)
4. Game world tap
```

### 5b. Tap action

When a quick-bar slot is tapped and contains a valid item:

1. Look up `inv_index` from `quick_bar.slots[slot]`.
2. Bounds-check against `game.inventory`.
3. Call `game.use_item(inv_index)` — existing logic handles potions, scrolls,
   food.
4. `use_item` calls `inventory.remove()`, which triggers
   `quick_bar.on_item_removed(inv_index)` to fix up all slot indices.
5. Advance game turn (enemies move, etc.) — same as using an item from the
   drawer.

### 5c. Empty slot tap

Tapping an empty slot does nothing (or optionally opens the inventory drawer).

### 5d. Visual feedback

On successful use:
- Brief flash/pulse on the slot (200ms white overlay fading out).
- Item's floating text effect ("+12 HP", "AOE blast!", etc.) shows as usual.
- Slot transitions to empty state.

### 5e. Keyboard shortcut (future)

Keys `1`, `2`, `3`, `4` mapped to `InputAction::QuickUse(slot)` for keyboard
players. Not required for initial implementation but the data model supports it.

---

## Part 6: Inventory Integration

### 6a. Wiring `on_item_removed` into existing code

Every place that calls `self.inventory.remove(index)` must also call
`self.quick_bar.on_item_removed(index)`. These call sites are:

| Function | File:Line | Trigger |
|----------|-----------|---------|
| `use_item()` (Potion) | `game/items.rs:174` | Drinking a potion |
| `use_item()` (Scroll) | `game/items.rs:184` | Reading a scroll |
| `eat_food()` | `game/items.rs:~212` | Eating food |
| `equip_item()` | `game/items.rs:232` | Equipping gear |
| `drop_item()` | `game/items.rs:249` | Dropping an item |

### 6b. Visual indicator in inventory drawer

Items that are assigned to a quick-bar slot show a small badge in their
inventory row — the slot number (1–4) in a colored circle. This tells the
player "this potion is on quick slot 2" without opening any extra UI.

### 6c. Drag-out from quick bar back to inventory

Dragging an item *off* the quick bar (not onto another slot) simply clears the
assignment. The item stays in inventory — nothing is consumed or dropped.

---

## Part 7: Edge Cases & Polish

### 7a. Inventory full after pickup

When the player picks up items and the inventory changes, `on_item_removed`
isn't needed (items are appended). Quick-bar slots remain valid since
`Vec::push` doesn't shift existing indices.

### 7b. Item consumed via drawer while assigned to quick bar

If the player opens the inventory drawer and taps "Use" on an item that's also
on the quick bar, the normal `use_item` + `on_item_removed` flow handles it —
the quick-bar slot clears automatically.

### 7c. Inventory at capacity

Assigning an item to the quick bar doesn't move it — it stays in inventory. No
capacity issues.

### 7d. Multiple identical items

If the player has 3 Health Potions and assigns one to slot 1, only *that
specific inventory index* is referenced. Using it clears slot 1. The other
potions are unaffected. The player can assign a different potion to slot 1
afterward.

### 7e. Auto-refill (optional, not in initial implementation)

A "sticky" mode where using a potion from slot 1 automatically assigns the
next potion of the same kind to slot 1. Diablo II does this with the belt.
Defer to a follow-up — requires scanning inventory for matching `ItemKind` +
`name` after consumption.

### 7f. Drawer animation interaction

The quick bar should remain visible during drawer open/close animations. The
drawer slides up *above* the quick bar, not over it. The quick bar acts as a
fixed element between the drawer area and the bottom bar.

---

## Implementation Order

1. **Data model** — `QuickBar` struct, `Game` field, `on_item_removed`,
   `assign`, `clear`. Unit tests. (`game/types.rs`, `game/mod.rs`)
2. **Wire `on_item_removed`** — Add calls at every `inventory.remove()` site.
   Existing tests must still pass. (`game/items.rs`)
3. **Rendering (portrait)** — `draw_quick_bar()`, layout constants, camera
   padding update. Visual only — slots always empty at this stage.
   (`renderer/hud.rs`, `renderer/mod.rs`)
4. **Rendering (landscape)** — Quick bar row in side panel. (`renderer/hud.rs`)
5. **Tap-to-use** — Hit test for quick-bar slots, `use_item` call, turn
   advance. (`lib.rs`)
6. **Drag & drop state machine** — `DragState`, long-press timer, touch event
   integration. (`input.rs` or new `drag.rs`)
7. **Drag rendering** — Ghost sprite, slot highlight on hover. (`renderer/`)
8. **Inventory badge** — Show slot number on assigned items in drawer.
   (`renderer/drawers.rs`)
9. **Polish** — Use feedback animation, haptic, edge case hardening.

Each step produces a working build. Steps 1–2 are pure logic with unit tests.
Steps 3–4 are visual-only. Step 5 makes slots functional. Steps 6–9 add the
drag interaction.

---

## Files Modified

| File | Changes |
|------|---------|
| `src/game/types.rs` | `QuickBar` struct, `QUICKBAR_SLOTS` const |
| `src/game/mod.rs` | `quick_bar` field on `Game`, init in constructors |
| `src/game/items.rs` | `on_item_removed()` calls after every `inventory.remove()` |
| `src/renderer/mod.rs` | `QUICKBAR_BASE` const, `quickbar_h()` helper, camera padding |
| `src/renderer/hud.rs` | `draw_quick_bar()` (portrait + landscape) |
| `src/renderer/drawers.rs` | Slot-number badge on assigned inventory items |
| `src/input.rs` | `DragState` enum, long-press timer, drag touch events |
| `src/lib.rs` | Quick-bar hit testing, tap-to-use action, drag drop handling |
