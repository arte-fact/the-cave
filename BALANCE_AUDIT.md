# Balance Audit — The Cave

Systematic analysis of every game system that affects difficulty, progression,
and player experience. Each section identifies the current state, flags issues,
and proposes concrete fixes with numbers.

---

## 1. Damage Formula

### Current
```
damage = max(1, attacker_stat - defender_stat)
```
Flat subtraction with a minimum of 1 (`game.rs:1091`, `game.rs:1119`, `game.rs:1297`).

### Problems
| Issue | Details |
|-------|---------|
| **Defense cliff** | Each point of defense is equally powerful until it hits the cap, then worthless. Going from 0 to 4 defense against a 5-ATK enemy takes damage from 5 → 1 (80% reduction). One more point adds nothing — you still take 1. |
| **Stacking is king** | Full Tier 2 gear gives +20 defense (armor 6 + helm 5 + shield 5 + boots 4). Against cave enemies (ATK 5-8), everything does 1 damage. The Dragon included. The final boss becomes a zero-threat punching bag. |
| **Offense mirrors the problem** | Player base 5 + Crystal Staff 7 + Gold Ring 4 + Strength points = 16+ attack. Against any enemy with 0 defense (all of them — enemies have no defense stat), every hit point of attack is equally lethal. |
| **No tension in late-game** | Flat subtraction creates a binary: either you outgear the floor and take minimum damage, or you don't and you melt. There is no middle ground where fights feel close. |

This is the single most impactful flaw in the game. Classic roguelike wisdom (Brogue, DCSS, Slay the Spire) universally avoids flat subtraction for this reason.

### Proposal: Ratio-based damage

Replace the flat formula with:
```
damage = attack * attack / (attack + defense)
```
This is the `c = defense` variant of `attack * c / (c + defense)`, which produces natural diminishing returns:

| ATK | DEF | Old Damage | New Damage |
|-----|-----|-----------|------------|
| 5   | 0   | 5         | 5          |
| 5   | 2   | 3         | 3.6        |
| 5   | 5   | 1 (min)   | 2.5        |
| 8   | 20  | 1 (min)   | 2.3        |
| 16  | 0   | 16        | 16         |
| 16  | 5   | 11        | 12.2       |

Defense always matters but never trivializes damage. The floor is naturally
positive (no min(1) needed since `attack > 0` guarantees `damage > 0`).

Implementation: `let dmg = (atk * atk + atk + def) / (atk + def);` using
integer math (the `+atk` rounds up to guarantee minimum 1).

---

## 2. Enemy Stats — No Defense Stat

### Current
Enemies have `hp` and `attack` only (`game.rs:72-81`). No defense field.
Player damage = `effective_attack()` applied directly to enemy HP.

### Problems
- Player attack is pure throughput with no diminishing returns — every +1 ATK
  is exactly +1 damage per hit. Stacking strength and weapon bonuses scales
  linearly forever.
- There is no mechanical difference between "armored" enemies (Skeleton, Death
  Knight, Troll) and "fragile" ones (Rat, Bat, Wraith) except HP.
- With 16+ effective attack in the late game, the Dragon (30 HP) dies in
  2 hits. A boss fight that lasts 2 turns is anticlimactic.

### Proposal: Add enemy defense

Add a `defense: i32` field to `Enemy`. Assign thematic values:

| Enemy | HP | ATK | DEF | Rationale |
|-------|------|-----|-----|-----------|
| Giant Rat | 3 | 1 | 0 | Fragile pest |
| Giant Bat | 4 | 2 | 0 | Fragile |
| Wolf | 5 | 2 | 1 | Light natural armor |
| Giant Spider | 6 | 3 | 0 | Dangerous but fragile |
| Boar | 8 | 2 | 2 | Thick hide |
| Bear | 12 | 4 | 2 | Tough hide |
| Lycanthrope | 14 | 5 | 3 | Supernatural toughness |
| Kobold | 4 | 2 | 1 | Light armor |
| Small Slime | 4 | 1 | 0 | Blob |
| Goblin | 5 | 2 | 1 | Leather scraps |
| Skeleton | 6 | 3 | 2 | Bone hardness |
| Goblin Archer | 6 | 3 | 1 | Light armor |
| Zombie | 10 | 2 | 1 | Slow meat shield |
| Skeleton Archer | 7 | 4 | 2 | Bone armor |
| Big Slime | 10 | 2 | 0 | HP sponge, not armored |
| Orc | 10 | 4 | 3 | Plate scraps |
| Ghoul | 10 | 5 | 2 | Undead hide |
| Orc Blademaster | 14 | 5 | 4 | Full orc plate |
| Wraith | 8 | 6 | 0 | Glass cannon — ethereal, no physical armor |
| Naga | 12 | 6 | 3 | Scales |
| Troll | 16 | 5 | 3 | Thick hide, regeneration (future) |
| Death Knight | 20 | 7 | 5 | Heavy plate |
| Lich | 15 | 8 | 2 | Magical barrier, fragile body |
| Dragon | 40 | 10 | 6 | Boss-tier, legendary |

Note: Dragon HP bumped 30 → 40 and ATK 8 → 10 to make it a proper capstone.
With the ratio formula and 6 DEF, a fully geared player (16 ATK) deals
~12 damage/hit, meaning the fight lasts 3-4 turns — dangerous but fair.

---

## 3. Combat Action Economy — Simultaneous Retaliation

### Current
When the player attacks an enemy, the enemy retaliates **in the same turn**
(`game.rs:1117-1126`). The player cannot attack without taking damage. Then on
the enemy's own turn, all nearby enemies move/attack again
(`game.rs:1147-1149`).

### Problems
- The player takes damage **twice per turn cycle** from a melee enemy: once as
  retaliation, once during enemy_turn. This is extremely punishing and makes
  melee combat a pure attrition race.
- There is no way to "hit and run" in melee. The only escape from damage is
  sprinting (which skips the entire enemy_turn, including other enemies).
- This double-damage loop creates a **death spiral**: taking damage forces
  potion use, which fills inventory, which limits loot collection, which
  limits progression.

### Proposal: Remove retaliation, use standard turn alternation

Remove the retaliation block at `game.rs:1117-1126`. When the player moves
into an enemy, deal player damage. The enemy then gets its normal turn in
`enemy_turn()` where it can attack back. This is the standard roguelike model
(Brogue, DCSS, NetHack all use alternating turns, not simultaneous exchange).

Benefits:
- Corridor fighting becomes strategic (attack, step back, attack).
- Ranged weapons become a genuine advantage (currently they miss and the
  enemy still gets its turn, while melee guarantees a hit but also guarantees
  taking damage — ranged is arguably worse).
- Sprint becomes "move faster" rather than "the only way to not die."

If you want to keep retaliation as a special ability, make it a trait on
specific enemies (e.g., Troll regeneration, Skeleton Archer return fire)
rather than universal.

---

## 4. XP Curve and Leveling

### Current
- XP to next level: `20 * level^1.5` (`game.rs:881-882`)
- Level up: +2 max HP, +2 skill points, full heal (`game.rs:885-897`)
- Skill points: +1 STR (= +1 ATK), +1 VIT (= +3 max HP), +1 DEX (range/accuracy), +1 STA (+10 max stamina) (`game.rs:907-926`)

### XP budget analysis

| Level | XP Needed | Cumulative XP | Rats Needed | Wolves | Orcs |
|-------|-----------|---------------|-------------|--------|------|
| 1→2 | 20 | 20 | 7 | 4 | 2 |
| 2→3 | 57 | 77 | 19 | 12 | 6 |
| 3→4 | 104 | 181 | 35 | 21 | 11 |
| 4→5 | 160 | 341 | 54 | 32 | 16 |
| 5→6 | 224 | 565 | 75 | 45 | 23 |

### Problems
| Issue | Details |
|-------|---------|
| **Overworld grinding is optimal** | 200x200 map × ~45% walkable × 3% spawn = ~540 overworld enemies. Killing all of them before entering any dungeon yields ~2,700 XP — enough for level 7+. |
| **Level-up heal is a full heal** | This makes level-up timing a strategic resource. A player about to die can kill one more rat and fully heal. This is a common exploit in RPGs and undermines the survival loop. |
| **Skill points per level (2) are stingy** | With 4 skills competing, it takes 5 levels to get +5 in one stat. By level 5 the player has 8 points — enough for some STR and some VIT but not enough to feel meaningfully different from level 1. |
| **Stamina skill is a trap** | +10 max stamina per point is nearly useless. Sprint costs 15/turn and regens 5/turn. The base 100 stamina already gives 6 sprint turns. Adding 10 (one skill point) gives 0.67 more sprint turns. Compared to +1 ATK or +3 HP, this is never worth taking. |
| **Dexterity is niche** | Only matters if a ranged weapon is equipped. A melee player gets zero value from DEX. This makes it a trap choice for 3 of 4 builds. |

### Proposals

1. **Cap overworld XP or introduce diminishing returns**: After killing N
   overworld enemies, XP gain drops (e.g., halved after 50 kills, quartered
   after 100). This prevents overworld grinding from trivializing dungeons.

2. **Level-up heal → partial heal**: Heal 50% of missing HP instead of full.
   `self.player_hp += (self.player_max_hp - self.player_hp) / 2;`

3. **Give 3 skill points per level** instead of 2. This lets players feel
   meaningful progression faster and reduces the penalty for "wrong" choices.

4. **Rework Stamina skill**: Instead of +10 max stamina, make it reduce sprint
   cost by 1 (min 5). At 5 points invested, sprint costs 10 instead of 15 —
   a meaningful tactical difference.

5. **Give Dexterity a melee benefit**: +1 DEX = +2% dodge chance (capped at
   20%). This makes DEX universally useful while keeping its ranged synergy.

---

## 5. Item Economy

### Current drop rates
- Overworld: no item drops except meat from beasts
- Dungeon floor: 5% per tile, 3% in cave (`game.rs:719-720`)
- Dungeon L0 (40×30 = 1200 tiles × ~40% floor × 5%): ~24 items
- Dungeon L1 (50×35 = 1750 tiles × ~40% floor × 5%): ~35 items
- Dungeon L2+ (60×40 = 2400 tiles × ~40% floor × 5%): ~48 items
- Cave (80×60 = 4800 tiles × ~50% floor × 3%): ~72 items

### Problems
| Issue | Details |
|-------|---------|
| **Item flood** | ~24 items on L0 alone, with a 10-slot inventory, means the player is constantly full. Excess items on the ground are wasted since there's no stash or shop. |
| **No scarcity = no tension** | In classic roguelikes (Brogue, DCSS), finding a good weapon is a pivotal moment. Here, the player will find 2-3 weapons per floor. Equipment is disposable rather than exciting. |
| **Tier locking is too clean** | L0 = tier 0, L1 = tier 1, L2 = tier 2. There's zero chance of finding a good item early or a bad item late. This removes the thrill of "lucky find" that drives roguelike engagement. |
| **Potions dominate** | 28% of T0 items are Health Potions (heal 5). With ~24 items per floor, that's ~7 potions. Combined with food healing, the player has massive sustain. |
| **No consumable pressure** | Scrolls of Fire (AOE 8) are strong but there's no urgency to use them. No expiration, no degradation, no monster that demands them. Players hoard them (the "potion hoarding problem" from roguelike design literature). |
| **Equipment slot saturation** | By mid-dungeon 1, the player likely has every slot filled. From that point, items are pure upgrades with no tradeoffs. |

### Proposals

1. **Reduce drop rate to 2%** (1% in cave). This cuts item count to ~10
   per floor — meaningful finds without flood. Specific numbers:
   - L0: ~10 items
   - L1: ~14 items
   - L2: ~19 items
   - Cave: ~24 items

2. **Allow tier bleed**: 20% chance an item rolls one tier higher, 10% chance
   one tier lower. Finding an Iron Sword on L0 is exciting. Finding a Rusty
   Sword on L2 is a tough-luck moment that adds texture.

3. **Reduce potion drop rate within tiers**: Cap healing items (potions + food)
   at 30% combined (currently potions alone are 18-28% plus 16-20% food =
   34-48% healing). Target: 25% total consumables (healing + scrolls + food).

4. **Add scroll target pressure**: Give Scrolls of Fire/Lightning/Storm a use
   case players can't ignore — e.g., enemies that split if killed by melee
   but die permanently to magic, or enemy groups where AOE is the only
   efficient answer.

---

## 6. Survival System — Hunger

### Current
- Hunger 100/100, drains 1 per 5 turns (`game.rs:761`)
- Starvation at 0: 1 HP/turn (`game.rs:777`)
- HP regen when hunger > 50: +1 HP per 5 turns, costs 2 hunger (`game.rs:767-773`)
- Food values: 5-25 hunger per item

### Time-to-starve analysis
- 100 hunger ÷ (1 per 5 turns) = 500 turns to starve from full
- Average dungeon floor has ~30-50 rooms, each requiring ~5-15 steps = ~150-750 turns
- So one floor can drain 30-100% of hunger depending on exploration thoroughness
- Food drops are generous: ~16-20% of items are food, so 4-10 food items per floor

### Problems
| Issue | Details |
|-------|---------|
| **Hunger is a non-threat** | With 500 turns of runway and abundant food, starvation is essentially impossible in normal play. It only kills players who AFK or get lost. |
| **Regen threshold (50) is generous** | The player regens HP for the first 250 turns of each food cycle. Since floors take ~200 turns, regen is almost always active. |
| **Hunger doesn't create decisions** | There's no meaningful choice about when to eat — you eat when below 50 (to maintain regen) or when food is about to be wasted from a full stack. |
| **No differentiation between areas** | Hunger drains identically on the overworld and in deep dungeons. The overworld should feel safer; dungeons should feel oppressive. |

### Proposals

1. **Increase hunger drain in dungeons**: Base rate stays 1/5 turns on the
   overworld. In dungeons, drain 1/3 turns. In the cave, drain 1/2 turns.
   This creates resource pressure that scales with depth.

2. **Lower the regen threshold to 30**: This shortens the "free healing" window
   and makes food timing more deliberate.

3. **Reduce base hunger to 80**: Shorter runway forces earlier food decisions.
   Combined with faster dungeon drain, the player must plan around food supply
   rather than ignoring it.

---

## 7. Sprint System

### Current
- Toggle on/off (`game.rs:732`)
- Cost: 15 stamina/turn, regen 5/turn when not sprinting (`game.rs:7-8`)
- **Enemies skip their turn entirely when player sprints** (`game.rs:1147-1149`)
- 100 base stamina → 6 consecutive sprint turns

### Problems
| Issue | Details |
|-------|---------|
| **Sprint is god mode** | Enemies literally do not act when the player sprints. This means sprint is not "move faster" — it is "become invincible." Any dangerous situation can be escaped with 1 sprint turn. |
| **Sprint trivializes dungeon navigation** | Sprint through corridors, toggle off in rooms to fight. There is no risk in movement. |
| **Sprint kills ranged combat value** | Why use a bow when you can sprint to melee range without being hit? |
| **No cost to sprint-fighting** | Sprint on, attack enemy (enemies don't retaliate OR act), sprint off. Wait for stamina to regen. Repeat. This is a dominant strategy that bypasses all combat. |

Actually, re-reading: sprint skips enemy_turn but the player can't attack
while sprinting (attack is on move_player which calls enemy_turn conditionally).
But the player CAN sprint past enemies with zero risk, making positioning
trivial.

### Proposals

1. **Sprint should reduce enemy actions, not skip them**: Enemies still get
   their turn, but the player gets an extra move before it. Alternatively,
   enemies move at half speed (skip every other turn) while the player sprints.
   This makes sprint a tactical advantage, not invincibility.

2. **Alternatively**: Enemies still chase and attack during sprint, but the
   player moves 2 tiles per turn (covering more ground). Sprint is for
   repositioning under pressure, not for safety.

---

## 8. Ranged Combat

### Current
- Hit chance: `(90 - distance * 70 / max_range).max(20) + dexterity * 2`, capped 95% (`game.rs:1351-1357`)
- Damage: same as melee (`effective_attack()`) (`game.rs:1414`)
- Misses waste a turn (enemies still move) (`game.rs:1440-1447`)
- No ammo system

### Problems
| Issue | Details |
|-------|---------|
| **Ranged is strictly worse than melee** | Melee has 100% hit rate. Ranged has 20-95%. Both deal the same damage. Melee always hits; ranged sometimes misses and wastes a turn. The only advantage is range, but enemies close the gap in 1-3 turns. |
| **No ammo = no resource management** | Infinite shots remove what should be the core tension of ranged play: "do I have enough arrows for this fight?" |
| **Dexterity scaling is weak** | +2% hit chance per DEX point. Going from 3 DEX to 8 DEX (5 skill points!) improves hit chance by 10%. Compare: 5 points in STR gives +5 damage per hit, every hit. |
| **Ranged weapon attack bonus is redundant** | The weapon provides a BuffAttack bonus, but this applies equally to melee if the player switches weapons. There's no "ranged-only damage." |

### Proposals

1. **Give ranged attacks a damage bonus at distance**: `ranged_damage = effective_attack() + distance / 2`. This rewards staying at range and differentiates ranged from melee.

2. **Increase DEX scaling**: +3% hit chance per point (instead of +2%), and add
   +1 ranged damage per 2 DEX. This makes DEX investment worthwhile for
   ranged builds.

3. **Add ammo (optional, higher complexity)**: Arrows drop alongside bows.
   Each shot costs 1 arrow. Start with 15. This adds meaningful resource
   tension without over-complicating mobile UX.

4. **Ranged miss should not waste the full turn**: On a miss, enemies move but
   the player gets a "half turn" — enemy movement is halved. This reduces the
   punishment for the already-risky ranged attack.

---

## 9. Enemy AI

### Current
- Chase range: 5 tiles Manhattan distance (`game.rs:1289`)
- Movement: try X-axis first, then Y-axis toward player (`game.rs:1290-1316`)
- No fleeing, no pack tactics, no ranged enemies (despite "Archer" names)

### Problems
| Issue | Details |
|-------|---------|
| **5-tile chase is short** | FOV radius is 6-8 tiles. The player sees enemies before they aggro. Combined with sprint, every engagement is optional. |
| **All enemies behave identically** | Rats, Dragons, Archers, Wraiths — all walk toward the player and hit in melee. Names like "Goblin Archer" and "Skeleton Archer" promise ranged behavior but deliver melee. |
| **No threat from groups** | Enemies don't coordinate. Fighting 3 goblins is fighting 1 goblin three times. Corridor chokepoints trivialize multi-enemy encounters. |
| **Movement is predictable** | Always tries X first, then Y. The player can exploit this by approaching from diagonal. |

### Proposals

1. **Increase chase range to 8** (matching overworld FOV). Enemies spot the
   player when the player spots them. In dungeons (FOV 6), enemies would
   sometimes aggro from outside vision — adding tension.

2. **Archer enemies should shoot**: Give "Archer" enemies a ranged attack (2-4
   tile range, 60-80% hit chance, deal `attack - 1` damage). They should
   prefer to maintain distance and shoot rather than chase into melee.

3. **Randomize movement axis**: 50/50 chance to try Y-axis first vs X-axis.
   Removes exploitable predictability.

4. **Pack bonus (simple)**: If 2+ enemies of the same type are adjacent to the
   player, each gets +1 ATK. This rewards the player for using corridors and
   punishes open-field fighting against groups.

---

## 10. Progression Pacing — Depth vs. Power

### Current path to victory
1. Spawn on overworld road
2. Explore overworld, kill forest enemies (earn ~2000+ XP)
3. Enter dungeon, clear L0 (tier 0 loot)
4. Descend L1 (tier 1 loot)
5. Descend L2 (tier 2 loot)
6. Find dragon dungeon, descend to cave (L3)
7. Kill Dragon → win

### Problems
| Issue | Details |
|-------|---------|
| **Overworld is an XP piñata** | ~540 enemies with no structure. The player can grind to level 7+ before entering any dungeon. |
| **All dungeons are equal** | Every dungeon has the same 3 levels with the same enemy tiers. Only one has the cave. There's no reason to explore multiple dungeons except loot (which floods). |
| **No difficulty signaling** | All dungeon entrances look identical. The player can't tell which dungeon has the dragon. |
| **Backtracking is free** | Exit dungeon → full overworld restore. No enemies respawn in cleared areas, but the player can heal to full via regen on the overworld with no time pressure. |
| **Power plateau** | By Dungeon L2, the player has T2 gear in every slot. From that point, more loot is meaningless. The cave is a gear-check victory lap, not a challenge. |

### Proposals

1. **Differentiate dungeons by difficulty**: Assign each dungeon a tier (easy,
   medium, hard). Easy dungeons have L0-L1 enemies on all floors. Hard
   dungeons have L1-L2+ enemies. Only the dragon dungeon has all 4 tiers.
   This gives the player a choice of where to start and a clear path of
   escalation.

2. **Limit overworld grinding**: Cap overworld enemy count at ~100 (reduce
   spawn rate to 1.5%) or introduce diminishing XP returns.

3. **Add a visual tell for dungeon difficulty**: Change the entrance glyph or
   description. "A dark passage" (easy), "A foreboding chasm" (medium), "A
   scorched opening radiating heat" (dragon).

4. **Cost to backtracking**: Enemies respawn in dungeons when you re-enter,
   but at reduced count (50%). Cleared floors are partially repopulated. This
   prevents consequence-free retreat.

---

## 11. Skill System Balance

### Current stat values per skill point

| Skill | Benefit per point | Effective combat value |
|-------|-------------------|-----------------------|
| Strength | +1 ATK | +1 damage/hit (linear, never diminishes) |
| Vitality | +3 Max HP, +3 current HP | ~1 extra hit survived |
| Dexterity | +1 DEX (ranged accuracy +2%, range +0.33) | Near-zero for melee builds |
| Stamina | +10 max stamina | +0.67 sprint turns |

### Problems
- **Strength dominates**: With flat damage, every STR point is equally
  valuable at all stages. 5 STR = +5 damage = one-shotting most L0-L1 enemies.
- **Vitality is the only alternative**: +3 HP per point is decent but
  linear. HP is less valuable when the player can avoid damage via sprint.
- **Dexterity is a trap for melee**: Zero benefit unless wielding a ranged
  weapon. The game doesn't tell you this.
- **Stamina is always wrong**: No scenario makes +10 stamina better than +1
  ATK or +3 HP. It is a noob trap.

### Proposals

1. **Make each skill useful in multiple systems**:
   - **Strength**: +1 ATK, +1 carry weight (if inventory weight is added), or
     +1 melee damage with diminishing returns via the ratio formula.
   - **Vitality**: +3 max HP, +1 hunger capacity (+1 max_hunger). Ties into
     survival system.
   - **Dexterity**: +2% dodge chance (melee and ranged), +2% ranged accuracy,
     +0.33 range. Now useful for everyone.
   - **Stamina**: -1 sprint cost (min 5) instead of +10 max. Or: +1 moves per
     turn when sprinting. Fundamentally changes sprint dynamics.

2. **Display stat tooltips**: On the skill allocation screen, show exactly
   what each point does. "STR 3 → 4: Attack 8 → 9. Damage vs Goblin: 8 → 9."
   Brogue-style transparency.

---

## 12. Dragon Boss Fight

### Current
- HP: 30, ATK: 8, no defense
- With full T2 gear: player deals 16+ damage/hit, takes 1 damage/hit
- Fight lasts 2 turns. Player takes 2 damage.

### Problems
This is the final boss and the win condition. A 2-turn fight with negligible
damage is profoundly anticlimactic.

### Proposals

1. **Dragon stats** (with new formula): HP 40, ATK 10, DEF 6.
   With 16 ATK vs 6 DEF using ratio formula: `16*16/(16+6) ≈ 12` damage/hit.
   Fight lasts 3-4 turns. Dragon deals `10*10/(10+20) ≈ 3-4` damage/hit
   (against 20 DEF). Player takes 12-16 total damage — dangerous but fair.

2. **Dragon special ability**: Every 3 turns, the Dragon uses a breath attack
   hitting all tiles in a cone (3 tiles wide, 4 tiles deep). Damage: 8.
   Player must position to avoid it. This adds tactical depth to the boss
   beyond "hit it until it dies."

3. **Dragon lair layout**: Guarantee the cave has an open room (at least 10×10)
   where the Dragon spawns. This gives the breath attack room to matter and
   prevents trivial corridor-cheesing.

---

## 13. Food & Meat Economy

### Current meat drops
| Animal | Hunger | Side Effect |
|--------|--------|-------------|
| Wolf | +15 | +10 stamina |
| Boar | +25 | +3 HP |
| Bear | +35 | +5 HP |

### Problems
- Only 3 of 7 forest animals drop meat. Rats, Bats, Spiders, and Lycanthropes
  drop nothing.
- Bear meat (+35 hunger, +5 HP) is extremely strong — a full meal plus
  a potion. Bears are rare (8%) but not rare enough for this reward.
- No dungeon food drops from enemies (only floor items). Dungeon-only players
  rely entirely on RNG item spawns for food.

### Proposals

1. **More animals drop meat**: Giant Rat drops "Rat Meat" (+5 hunger, Sicken
   effect −5 stamina). Giant Spider drops nothing (inedible). Boar stays as-is.
   Bear meat reduced to +25 hunger, +3 HP (still the best, but not absurd).

2. **Dungeon enemies occasionally drop rations**: Goblins have a 20% chance
   to drop "Stolen Rations" (+10 hunger). This provides a food trickle in
   dungeons without flooding.

---

## 14. Summary — Priority Matrix

| # | Issue | Severity | Effort | Impact |
|---|-------|----------|--------|--------|
| 1 | Flat damage formula | Critical | Medium | Fixes combat scaling across the entire game |
| 2 | No enemy defense | High | Low | Enables tactical enemy variety |
| 3 | Retaliation double-hit | High | Low | Fixes melee combat pacing |
| 4 | Sprint = invincibility | High | Low | Restores risk to movement |
| 5 | Item flood | Medium | Low | Creates scarcity and meaningful loot |
| 6 | Overworld XP grinding | Medium | Low | Prevents dungeon trivialization |
| 7 | Skill balance (STA/DEX traps) | Medium | Medium | Removes noob traps |
| 8 | Archers don't shoot | Medium | Medium | Delivers on enemy fantasy |
| 9 | Dragon is trivial | Medium | Medium | Satisfying finale |
| 10 | Hunger is a non-threat | Low | Low | Adds survival tension |
| 11 | Ranged < melee | Low | Medium | Opens a viable second playstyle |
| 12 | Dungeon differentiation | Low | Medium | Adds strategic exploration |

### Recommended implementation order

1. **Damage formula + enemy defense** (items 1-2) — fixes the foundation
2. **Remove retaliation** (item 3) — fixes combat feel
3. **Sprint nerf** (item 4) — restores risk
4. **Item drop rate reduction** (item 5) — creates scarcity
5. **Dragon rework** (item 9) — satisfying endgame
6. **Skill rebalancing** (item 7) — removes traps
7. Everything else

---

## References

- [Brogue combat mechanics](https://brogue.fandom.com/wiki/Combat) — Multiplicative enchantment, hit probability curves
- [DCSS damage formula](http://crawl.chaosforge.org/Damage_formula) — Layered random reduction, GDR
- [Strategy headroom in roguelikes](http://nethack4.org/blog/strategy-headroom.html) — High vs low headroom design
- [Damage formulas survey (yujiri)](https://yujiri.xyz/game-design/damage-formulas.gmi) — Flat vs ratio vs exponential comparison
- [Simplest non-problematic damage formula](https://tung.github.io/posts/simplest-non-problematic-damage-formula/) — `atk^2/(atk+def)` analysis
- [Cogmind design philosophy](https://www.gridsagegames.com/blog/2022/05/kyzratis-game-design-philosophy/) — Item attrition, resilient player
- [Eight rules of roguelike design](https://www.gamedeveloper.com/game-platforms/analysis-the-eight-rules-of-roguelike-design) — No-cyanide, no-beheading rules
- [Stat balancing in roguelikes](https://blog.roguetemple.com/articles/stat-balancing-in-roguelikes/) — Polynomial regression approach
- [Power curve (RogueBasin)](https://www.roguebasin.com/index.php/Power_Curve) — Depth-scaling frameworks
