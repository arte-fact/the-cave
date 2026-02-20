# Biome Design — The Cave

Biomes compose tile sets (walls, floors, props) with thematically matched enemy
rosters. This document defines 2 overworld biomes and 8 dungeon biomes built
entirely from sprites available in the 32rogues 0.5.0 sprite set.

## Sprite Audit

### Tile floors currently unused (available in tiles.png, not mapped to any DungeonStyle)

| Sprite set         | Rows   | Notes                        |
|--------------------|--------|------------------------------|
| GreenGrass1-3      | 14.b-d | Lush dark green grass        |
| GreenDirt1-3       | 13.b-d | Mossy green dirt             |
| BlueStoneFloor1-3  | 12.b-d | Blue-tinted stone            |
| DarkBrownBones1-3  | 15.b-d | Bones on dark brown ground   |

### Wall type unused

| Sprite             | Row    | Notes                        |
|--------------------|--------|------------------------------|
| RoughStoneWall     | 1.a-b  | Natural cave rock            |

### Monster sprites defined but never spawned

Harpy, Cockatrice, LizardfolkKobold, KoboldCanine, Lampreymander,
GiantEarthworm, LesserGiantSpider, WargDireWolf, Cultist, Angel,
SmallWrithingMass (only LargeWrithingMass spawns).

### Animal sprites in sheet but not in code

Primates (Chimpanzee, Gorilla, Orangutan, etc.), small mammals (Mink,
Mongoose, Marmot, Armadillo, etc.), Reindeer, Kangaroo, Koala, Penguin,
Cassowary, Emu, Duck, Swan, Turkey, Peacock, and more.

---

## Overworld Biomes

### 1. Temperate Forest (northern/central region)

The current overworld. European-style woodland with clearings and dirt roads.

| Element | Sprites                              |
|---------|--------------------------------------|
| Ground  | Grass1-3                             |
| Roads   | Dirt1-3                              |
| Trees   | Tree, SmallTree, Sapling             |
| Props   | Wheat, Buckwheat, SmallMushrooms     |

**Enemies — woodland wildlife + fey:**

| Tier      | Creatures                                          |
|-----------|----------------------------------------------------|
| Common    | Giant Rat, Fox, Badger, Buzzard, Coyote            |
| Uncommon  | Wolf, Boar, Giant Spider, Honey Badger             |
| Rare      | Bear, Lycanthrope, Wendigo                         |
| Mythical  | Dryad, Forest Spirit, Centaur                      |

### 2. Jungle / Tropical Forest (southern region)

Dense tropical canopy. Lush, humid, dangerous. Uses the currently-unused green
tile sets to create a visually distinct second overworld zone.

| Element | Sprites                                        |
|---------|------------------------------------------------|
| Ground  | GreenGrass1-3 *(currently unused)*             |
| Trails  | GreenDirt1-3 *(currently unused)*              |
| Trees   | Tree (dense), LargeMushroom                    |
| Props   | Rice, Sorghum, RedSpinach, SmallMushrooms      |

**Enemies — tropical predators + reptilians:**

| Tier      | Creatures                                          |
|-----------|----------------------------------------------------|
| Common    | Cobra, Black Mamba, Giant Centipede, Giant Ant     |
| Uncommon  | Cougar, Hyena, Lion, Alligator, Giant Spider       |
| Rare      | Manticore, Naga, Gorgon/Medusa                    |
| Mythical  | Satyr, Harpy *(unused)*, Dryad                     |

---

## Dungeon Biomes

Each dungeon placed on the overworld gets a type based on proximity to a biome
region and seeded RNG. The type determines wall/floor sprites, prop placement,
enemy tables per level, and the boss encounter.

### 3. Goblin Warren

Chaotic tunnels dug into dirt. Barrels of stolen loot. Orcs command the depths.

| Element | Sprites                              |
|---------|--------------------------------------|
| Walls   | DirtWall                             |
| Floors  | FloorStone1-3                        |
| Props   | Barrel, JarClosed, JarOpen           |
| Door    | Door1                                |

| Level | Creatures                                                 |
|-------|-----------------------------------------------------------|
| L0    | Kobolds (canine), Giant Rats, Small Slimes, Goblins      |
| L1    | Goblin Archers, Goblin Mages, Big Slimes, Goblin Brutes  |
| L2    | Orcs, Orc Blademasters, Trolls                           |
| Boss  | **Orc Warchief**                                         |

*Found in: Temperate Forest*

### 4. Undead Crypt

Cold stone corridors. Coffins line the walls. The dead don't rest here.

| Element | Sprites                                                 |
|---------|---------------------------------------------------------|
| Walls   | CatacombsWall                                           |
| Floors  | BoneFloor1-3                                            |
| Props   | CoffinClosed, CoffinAjar, SarcophagusClosed/Open, Corpse1-2 |
| Door    | GratedDoor                                              |

| Level | Creatures                                                 |
|-------|-----------------------------------------------------------|
| L0    | Skeletons, Zombies, Giant Rats, Giant Bats               |
| L1    | Skeleton Archers, Ghouls, Wraiths, Hag Witch            |
| L2    | Banshees, Death Knights, Unholy Cardinals                |
| Boss  | **Lich**                                                 |

*Found in: Temperate Forest (old graveyards), Jungle (forgotten tombs)*

### 5. Fungal Grotto

Damp organic caves lit by bioluminescent fungi. Myconid colonies guard their
territory. Deeper levels reveal aberrant writhing masses consuming everything.

| Element | Sprites                                |
|---------|----------------------------------------|
| Walls   | RoughStoneWall *(currently unused)*    |
| Floors  | GreenDirt1-3 *(currently unused)*      |
| Props   | SmallMushrooms, LargeMushroom          |
| Door    | FramedDoorShut / FramedDoorOpen        |

| Level | Creatures                                                        |
|-------|------------------------------------------------------------------|
| L0    | Small Myconids, Small Slimes, Giant Centipedes, Giant Earthworms *(unused)* |
| L1    | Large Myconids, Big Slimes, Giant Ants, Lampreymanders *(unused)* |
| L2    | Writhing Masses (small + large), Writhing Humanoid               |
| Boss  | **Large Writhing Mass**                                          |

*Found in: Jungle (rainforest undergrowth), Temperate Forest (damp hollows)*

### 6. Orc Stronghold

Military fortress with organized patrols, armory rooms, and heavy stone walls.
Trolls and ettins serve as heavy infantry.

| Element | Sprites                              |
|---------|--------------------------------------|
| Walls   | LargeStoneWall                       |
| Floors  | StoneFloor1-3                        |
| Props   | Barrel, ChestClosed, ChestOpen       |
| Door    | FramedDoorShut                       |

| Level | Creatures                                                 |
|-------|-----------------------------------------------------------|
| L0    | Orcs, Kobolds (canine), Goblin Brutes                    |
| L1    | Orc Blademasters, Orc Wizards, Trolls, Giant Ants       |
| L2    | Ettins, Two-Headed Ettins, Orc Warchiefs                 |
| Boss  | **Two-Headed Ettin**                                     |

*Found in: Temperate Forest (mountain edges)*

### 7. Abyssal Temple

Dark temple to eldritch forces. Blue-lit stone floors with pentagrams and blood
sacrifices. Cultists on upper levels; summoned horrors in the depths.

| Element | Sprites                                |
|---------|----------------------------------------|
| Walls   | IgneousWall                            |
| Floors  | BlueStoneFloor1-3 *(currently unused)* |
| Props   | Pentagram, BloodSpatter1-2, Corpse1-2  |
| Door    | GratedDoor                             |

| Level | Creatures                                                 |
|-------|-----------------------------------------------------------|
| L0    | Cultists *(unused)*, Faceless Monks, Small Slimes        |
| L1    | Unholy Cardinals, Hag Witches, Wraiths                   |
| L2    | Writhing Humanoids, Imps, Nagas                          |
| Boss  | **Reaper**                                               |

*Found in: Jungle (hidden), Temperate Forest (remote)*

### 8. Dragon's Lair

Volcanic chamber. Red-hot stone. The ultimate end-game challenge.
Only appears as the deepest level of 4-level dungeons.

| Element | Sprites                              |
|---------|--------------------------------------|
| Walls   | IgneousWall                          |
| Floors  | RedStoneFloor1-3                     |
| Props   | LargeRock1-2, ChestClosed (hoard)    |
| Door    | none (open cave)                     |

| Level | Creatures                                                 |
|-------|-----------------------------------------------------------|
| Cave  | Drakes, Basilisks, Imps, Manticores, Rock Golems         |
| Boss  | **Dragon** (unique, hp:40, atk:10, def:6)                |

*Found in: either biome (rare, end-game)*

### 9. Beast Den

Natural cave system. Animal bones carpet the floor. Pack hunters in the upper
levels, apex predators below. Beware the Wendigo.

| Element | Sprites                                |
|---------|----------------------------------------|
| Walls   | RoughStoneWall *(currently unused)*    |
| Floors  | DarkBrownBones1-3 *(currently unused)* |
| Props   | Corpse1-2, BloodSpatter1-2, LargeRock1-2 |
| Door    | none (natural cave opening)            |

| Level | Creatures                                                            |
|-------|----------------------------------------------------------------------|
| L0    | Giant Rats, Giant Bats, Lesser Giant Spiders *(unused)*, Cobras     |
| L1    | Giant Spiders, Dire Wolves *(unused)*, Lycanthropes                 |
| L2    | Wendigos, Manticores                                                |
| Boss  | **Wendigo**                                                         |

*Found in: Temperate Forest (wilderness edges)*

### 10. Serpent Pit

Winding serpentine tunnels. Warm and damp. Reptilian creatures guard their
nesting grounds. Deeper chambers are petrification zones.

| Element | Sprites                                |
|---------|----------------------------------------|
| Walls   | DirtWall                               |
| Floors  | GreenDirt1-3 *(currently unused)*      |
| Props   | SmallMushrooms, LargeRock1-2           |
| Door    | none (narrow passage)                  |

| Level | Creatures                                                            |
|-------|----------------------------------------------------------------------|
| L0    | Cobras, Lizardfolk/Kobold *(unused)*, Giant Centipedes              |
| L1    | Black Mambas, Cockatrices *(unused)*, Nagas                        |
| L2    | Basilisks, Gorgon/Medusa                                           |
| Boss  | **Basilisk**                                                        |

*Found in: Jungle (primary), Temperate Forest (rare)*

---

## Summary Matrix

| # | Biome              | Walls       | Floors          | Boss             | Region    |
|---|--------------------|-------------|-----------------|------------------|-----------|
| 1 | Temperate Forest   | —           | Grass           | —                | Overworld |
| 2 | Jungle             | —           | GreenGrass*     | —                | Overworld |
| 3 | Goblin Warren      | Dirt        | FloorStone      | Orc Warchief     | Temperate |
| 4 | Undead Crypt       | Catacombs   | Bone            | Lich             | Both      |
| 5 | Fungal Grotto      | RoughStone* | GreenDirt*      | Writhing Mass    | Both      |
| 6 | Orc Stronghold     | LargeStone  | Stone           | Two-Headed Ettin | Temperate |
| 7 | Abyssal Temple     | Igneous     | BlueStone*      | Reaper           | Both      |
| 8 | Dragon's Lair      | Igneous     | RedStone        | Dragon           | Both      |
| 9 | Beast Den          | RoughStone* | DarkBrownBones* | Wendigo          | Temperate |
| 10| Serpent Pit        | Dirt        | GreenDirt*      | Basilisk         | Jungle    |

\* = sprite set currently defined but unused — this design gives it a purpose.

## Unused sprites activated by this design

- **Tile floors:** GreenGrass1-3, GreenDirt1-3, BlueStoneFloor1-3, DarkBrownBones1-3
- **Tile walls:** RoughStoneWall (top + side)
- **Monsters:** Harpy, Cockatrice, LizardfolkKobold, KoboldCanine, Lampreymander, GiantEarthworm, LesserGiantSpider, WargDireWolf, Cultist, SmallWrithingMass
- **Props:** Pentagram, Corpse1-2, BloodSpatter1-2, CoffinClosed/Ajar, SarcophagusClosed/Open, LargeMushroom, multiple crop sprites
