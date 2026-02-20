# Sprite Atlas Reference

Reference for all PNG sprite sheets from the 32rogues-0.5.0 tileset.
Coordinates use **0-based** (row, col) notation. The original txt files use
1-based row numbers and letter-indexed columns (a=0, b=1, ...).

---

## monsters.png (12 cols x 13 rows, 32x32 cells)

Source: `32rogues/monsters.txt` | Code: `src/sprite_atlas/monsters.rs`

| Row | Col | Sprite | Rust Enum |
|-----|-----|--------|-----------|
| 0 | 0 | Orc | `Orc` |
| 0 | 1 | Orc Wizard | `OrcWizard` |
| 0 | 2 | Goblin | `Goblin` |
| 0 | 3 | Orc Blademaster | `OrcBlademaster` |
| 0 | 4 | Orc Warchief | `OrcWarchief` |
| 0 | 5 | Goblin Archer | `GoblinArcher` |
| 0 | 6 | Goblin Mage | `GoblinMage` |
| 0 | 7 | Goblin Brute | `GoblinBrute` |
| 1 | 0 | Ettin | `Ettin` |
| 1 | 1 | Two Headed Ettin | `TwoHeadedEttin` |
| 1 | 2 | Troll | `Troll` |
| 2 | 0 | Small Slime | `SmallSlime` |
| 2 | 1 | Big Slime | `BigSlime` |
| 2 | 2 | Slimebody | *(not mapped)* |
| 2 | 3 | Merged Slimebodies | *(not mapped)* |
| 3 | 0 | Faceless Monk | `FacelessMonk` |
| 3 | 1 | Unholy Cardinal | `UnholyCardinal` |
| 4 | 0 | Skeleton | `Skeleton` |
| 4 | 1 | Skeleton Archer | `SkeletonArcher` |
| 4 | 2 | Lich | `Lich` |
| 4 | 3 | Death Knight | `DeathKnight` |
| 4 | 4 | Zombie | `Zombie` |
| 4 | 5 | Ghoul | `Ghoul` |
| 5 | 0 | Banshee | `Banshee` |
| 5 | 1 | Reaper | `Reaper` |
| 5 | 2 | Wraith | `Wraith` |
| 5 | 3 | Cultist | `Cultist` |
| 5 | 4 | Hag / Witch | `HagWitch` |
| 6 | 0 | Giant Centipede | `GiantCentipede` |
| 6 | 1 | Lampreymander | `Lampreymander` |
| 6 | 2 | Giant Earthworm | `GiantEarthworm` |
| 6 | 3 | Manticore | `Manticore` |
| 6 | 4 | Giant Ant | `GiantAnt` |
| 6 | 5 | Lycanthrope | `Lycanthrope` |
| 6 | 6 | Giant Bat | `GiantBat` |
| 6 | 7 | Lesser Giant Ant | `LesserGiantAnt` |
| 6 | 8 | Giant Spider | `GiantSpider` |
| 6 | 9 | Lesser Giant Spider | `LesserGiantSpider` |
| 6 | 10 | Warg / Dire Wolf | `WargDireWolf` |
| 6 | 11 | Giant Rat | `GiantRat` |
| 7 | 0 | Dryad | `Dryad` |
| 7 | 1 | Wendigo | `Wendigo` |
| 7 | 2 | Rock Golem | `RockGolem` |
| 7 | 3 | Centaur | `Centaur` |
| 7 | 4 | Naga | `Naga` |
| 7 | 5 | Forest Spirit | `ForestSpirit` |
| 7 | 6 | Satyr | `Satyr` |
| 7 | 7 | Minotaur | `Minotaur` |
| 7 | 8 | Harpy | `Harpy` |
| 7 | 9 | Gorgon / Medusa | `GorgonMedusa` |
| 8 | 0 | Lizardfolk / Kobold (reptile) | `LizardfolkKobold` |
| 8 | 1 | Drake / Lesser Dragon | `Drake` |
| 8 | 2 | Dragon | `Dragon` |
| 8 | 3 | Cockatrice | `Cockatrice` |
| 8 | 4 | Basilisk | `Basilisk` |
| 9 | 0 | Small Kobold (canine) | `SmallKoboldCanine` |
| 9 | 1 | Kobold (canine) | `KoboldCanine` |
| 10 | 0 | Small Myconid | `SmallMyconid` |
| 10 | 1 | Large Myconid | `LargeMyconid` |
| 11 | 0 | Angel / Archangel | `Angel` |
| 11 | 1 | Imp / Devil | `ImpDevil` |
| 12 | 0 | Small Writhing Mass | `SmallWrithingMass` |
| 12 | 1 | Large Writhing Mass | `LargeWrithingMass` |
| 12 | 2 | Writhing Humanoid | `WrithingHumanoid` |

---

## animals.png (9 cols x 16 rows, 32x32 cells)

Source: `32rogues/animals.txt` | Code: `src/sprite_atlas/animals.rs`

> **Errata:** The txt file labels orangutan as "3.c." but the actual PNG has
> only 16 rows (512px tall). The orangutan is at row 1 col 2 (same row as
> chimp/gorilla). All rows from txt row 4 onward are -1 compared to the
> txt numbering. The row numbers below reflect the **actual image**.

| Row | Col | Sprite | Rust Enum |
|-----|-----|--------|-----------|
| 0 | 0 | Grizzly Bear | `GrizzlyBear` |
| 0 | 1 | Black Bear | `BlackBear` |
| 0 | 2 | Polar Bear | `PolarBear` |
| 0 | 3 | Panda | `Panda` |
| 1 | 0 | Chimpanzee | *(not mapped)* |
| 1 | 1 | Gorilla | *(not mapped)* |
| 1 | 2 | Orangutan | *(not mapped)* |
| 2 | 0 | Aye Aye | *(not mapped)* |
| 2 | 1 | Gibbon | *(not mapped)* |
| 2 | 2 | Mandrill | *(not mapped)* |
| 2 | 3 | Capuchin | *(not mapped)* |
| 2 | 4 | Langur | *(not mapped)* |
| 3 | 0 | Cat | `Cat` |
| 3 | 1 | Bobcat | `Bobcat` |
| 3 | 2 | Cougar | `Cougar` |
| 3 | 3 | Cheetah | `Cheetah` |
| 3 | 4 | Lynx | `Lynx` |
| 3 | 5 | Ocelot | `Ocelot` |
| 3 | 6 | Male Lion | `MaleLion` |
| 3 | 7 | Female Lion | `FemaleLion` |
| 4 | 0 | Dog | `Dog` |
| 4 | 1 | Puppy | *(not mapped)* |
| 4 | 2 | Hyena | `Hyena` |
| 4 | 3 | Fox | `Fox` |
| 4 | 4 | Jackal | `Jackal` |
| 4 | 5 | Coyote | `Coyote` |
| 4 | 6 | Wolf | `Wolf` |
| 5 | 0 | Capybara | `Capybara` |
| 5 | 1 | Beaver | `Beaver` |
| 5 | 2 | Mink | *(not mapped)* |
| 5 | 3 | Mongoose | *(not mapped)* |
| 5 | 4 | Marmot | *(not mapped)* |
| 5 | 5 | Groundhog | *(not mapped)* |
| 5 | 6 | Chinchilla | *(not mapped)* |
| 5 | 7 | Echidna | *(not mapped)* |
| 6 | 0 | Aardvark | *(not mapped)* |
| 6 | 1 | Armadillo | *(not mapped)* |
| 6 | 2 | Badger | `Badger` |
| 6 | 3 | Honeybadger | `Honeybadger` |
| 6 | 4 | Coati | *(not mapped)* |
| 6 | 5 | Opossum | *(not mapped)* |
| 6 | 6 | Rabbit | `Rabbit` |
| 6 | 7 | Hare | `Hare` |
| 6 | 8 | Rat | `Rat` |
| 7 | 0 | Snake | `Snake` |
| 7 | 1 | Cobra | `Cobra` |
| 7 | 2 | Kingsnake | `Kingsnake` |
| 7 | 3 | Black Mamba | `BlackMamba` |
| 8 | 0 | Alligator | `Alligator` |
| 8 | 1 | Monitor Lizard | `MonitorLizard` |
| 8 | 2 | Iguana | `Iguana` |
| 8 | 3 | Tortoise | `Tortoise` |
| 8 | 4 | Snapping Turtle | `SnappingTurtle` |
| 8 | 5 | Alligator Snapping Turtle | *(not mapped)* |
| 9 | 0 | Cow | `Cow` |
| 9 | 1 | Horse | `Horse` |
| 9 | 2 | Donkey | `Donkey` |
| 9 | 3 | Mule | *(not mapped)* |
| 9 | 4 | Alpaca | *(not mapped)* |
| 9 | 5 | Llama | *(not mapped)* |
| 9 | 6 | Pig | `Pig` |
| 9 | 7 | Boar | `Boar` |
| 10 | 0 | Camel | `Camel` |
| 10 | 1 | Reindeer / Caribou | *(not mapped)* |
| 10 | 2 | Water Buffalo | `WaterBuffalo` |
| 10 | 3 | Yak | `Yak` |
| 11 | 0 | Seagull | `Seagull` |
| 11 | 1 | Barn Owl | `BarnOwl` |
| 11 | 2 | Common Buzzard | `Buzzard` |
| 12 | 0 | Kangaroo | *(not mapped)* |
| 12 | 1 | Koala | *(not mapped)* |
| 13 | 0 | Penguin | *(not mapped)* |
| 13 | 1 | Little Penguin | *(not mapped)* |
| 13 | 2 | Cassowary | *(not mapped)* |
| 13 | 3 | Emu | *(not mapped)* |
| 14 | 0 | Chicken | `Chicken` |
| 14 | 1 | Rooster | `Rooster` |
| 14 | 2 | Mallard Duck | *(not mapped)* |
| 14 | 3 | Swan | *(not mapped)* |
| 14 | 4 | Turkey | *(not mapped)* |
| 14 | 5 | Guineafowl | *(not mapped)* |
| 14 | 6 | Peacock | *(not mapped)* |
| 15 | 0 | Goat | `Goat` |
| 15 | 1 | Mountain Goat | `MountainGoat` |
| 15 | 2 | Ibex | `Ibex` |
| 15 | 3 | Sheep (Ram) | `Sheep` |

---

## tiles.png (17 cols x 26 rows, 32x32 cells)

Source: `32rogues/tiles.txt` | Code: `src/sprite_atlas/tiles.rs`

### Walls

| Row | Col | Sprite | Rust Enum |
|-----|-----|--------|-----------|
| 0 | 0 | Dirt Wall (top) | `DirtWallTop` |
| 0 | 1 | Dirt Wall (side) | `DirtWallSide` |
| 0 | 2 | Inner Wall | *(not mapped)* |
| 1 | 0 | Rough Stone Wall (top) | `RoughStoneWallTop` |
| 1 | 1 | Rough Stone Wall (side) | `RoughStoneWallSide` |
| 2 | 0 | Stone Brick Wall (top) | `StoneBrickWallTop` |
| 2 | 1 | Stone Brick Wall (side 1) | `StoneBrickWallSide1` |
| 2 | 2 | Stone Brick Wall (side 2) | `StoneBrickWallSide2` |
| 3 | 0 | Igneous Wall (top) | `IgneousWallTop` |
| 3 | 1 | Igneous Wall (side) | `IgneousWallSide` |
| 4 | 0 | Large Stone Wall (top) | `LargeStoneWallTop` |
| 4 | 1 | Large Stone Wall (side) | `LargeStoneWallSide` |
| 5 | 0 | Catacombs Wall (top) | `CatacombsWallTop` |
| 5 | 1 | Catacombs Wall (side) | `CatacombsWallSide` |

### Floors

| Row | Col | Sprite | Rust Enum |
|-----|-----|--------|-----------|
| 6 | 0 | Blank Floor (dark grey) | `BlankFloorDark` |
| 6 | 1-3 | Floor Stone 1-3 | `FloorStone1`..`FloorStone3` |
| 7 | 0 | Blank Floor (grass bg) | `BlankFloorGrass` |
| 7 | 1-3 | Grass 1-3 | `Grass1`..`Grass3` |
| 8 | 1-3 | Dirt 1-3 | `Dirt1`..`Dirt3` |
| 9 | 1-3 | Stone Floor 1-3 | `StoneFloor1`..`StoneFloor3` |
| 10 | 1-3 | Bone Floor 1-3 | `BoneFloor1`..`BoneFloor3` |
| 11 | 0 | Blank Red Floor | `BlankRedFloor` |
| 11 | 1-3 | Red Stone Floor 1-3 | `RedStoneFloor1`..`RedStoneFloor3` |
| 12 | 0 | Blank Blue Floor | `BlankBlueFloor` |
| 12 | 1-3 | Blue Stone Floor 1-3 | `BlueStoneFloor1`..`BlueStoneFloor3` |
| 13 | 1-3 | Green Dirt 1-3 | `GreenDirt1`..`GreenDirt3` |
| 14 | 1-3 | Green Grass 1-3 | `GreenGrass1`..`GreenGrass3` |
| 15 | 1-3 | Dark Brown Bones 1-3 | `DarkBrownBones1`..`DarkBrownBones3` |

### Doors, Stairs, Traps

| Row | Col | Sprite | Rust Enum |
|-----|-----|--------|-----------|
| 16 | 0 | Door 1 | `Door1` |
| 16 | 1 | Door 2 | `Door2` |
| 16 | 2 | Framed Door (shut) | `FramedDoorShut` |
| 16 | 3 | Framed Door (open) | `FramedDoorOpen` |
| 16 | 4 | Framed Door 2 (shut) | *(not mapped)* |
| 16 | 5 | Framed Door 2 (open) | *(not mapped)* |
| 16 | 6 | Grated Door | `GratedDoor` |
| 16 | 7 | Staircase Down | `StaircaseDown` |
| 16 | 8 | Staircase Up | `StaircaseUp` |
| 16 | 9 | Pressure Plate (up) | `PressurePlateUp` |
| 16 | 10 | Pressure Plate (down) | `PressurePlateDown` |
| 16 | 11 | Chute | *(not mapped)* |
| 16 | 12 | Pit | *(not mapped)* |
| 16 | 13 | Trap Door | *(not mapped)* |
| 16 | 14 | Pentagram | `Pentagram` |
| 16 | 15 | Spikes (down) | *(not mapped)* |
| 16 | 16 | Spikes (up) | *(not mapped)* |

### Containers, Objects, Vegetation

| Row | Col | Sprite | Rust Enum |
|-----|-----|--------|-----------|
| 17 | 0 | Chest (closed) | `ChestClosed` |
| 17 | 1 | Chest (open) | `ChestOpen` |
| 17 | 2 | Jar (closed) | `JarClosed` |
| 17 | 3 | Jar (open) | `JarOpen` |
| 17 | 4 | Barrel | `Barrel` |
| 17 | 5 | Ore Sack | *(not mapped)* |
| 17 | 6 | Log Pile | *(not mapped)* |
| 18 | 0-1 | Large Rock 1-2 | `LargeRock1`..`LargeRock2` |
| 19 | 0 | Buckwheat | `Buckwheat` |
| 19 | 1 | Flax | `Flax` |
| 19 | 2-5 | Papyrus, Kenaf, Ramie, Jute | *(not mapped)* |
| 19 | 6 | Rice | `Rice` |
| 19 | 7 | Wheat | `Wheat` |
| 19 | 8 | Maize / Corn | `MaizeCorn` |
| 19 | 9 | Amaranth | `Amaranth` |
| 19 | 10 | Quinoa | `Quinoa` |
| 19 | 11 | Bitter Vetch | `BitterVetch` |
| 19 | 12 | Sorghum | `Sorghum` |
| 19 | 13 | Red Spinach | `RedSpinach` |
| 19 | 14-15 | Cotton, Alfalfa | *(not mapped)* |
| 20 | 0 | Small Mushrooms | `SmallMushrooms` |
| 20 | 1 | Large Mushroom | `LargeMushroom` |
| 21 | 0-1 | Corpse 1-2 | `Corpse1`..`Corpse2` |
| 22 | 0-1 | Blood Spatter 1-2 | `BloodSpatter1`..`BloodSpatter2` |
| 22 | 2-3 | Slime (small/large) | *(not mapped)* |
| 23 | 0 | Coffin (closed) | `CoffinClosed` |
| 23 | 1 | Coffin (ajar) | `CoffinAjar` |
| 23 | 2 | Coffin (open) | *(not mapped)* |
| 23 | 3 | Sarcophagus (closed) | `SarcophagusClosed` |
| 23 | 4 | Sarcophagus (ajar) | *(not mapped)* |
| 23 | 5 | Sarcophagus (open) | `SarcophagusOpen` |
| 25 | 0 | Sapling | `Sapling` |
| 25 | 1 | Small Tree | `SmallTree` |
| 25 | 2 | Tree | `Tree` |
| 25 | 3 | Two Tile Tree | *(not mapped)* |

---

## items.png (11 cols x 26 rows, 32x32 cells)

Source: `32rogues/items.txt` | Code: `src/sprite_atlas/items.rs`

### Weapons

| Row | Col | Sprite | Rust Enum |
|-----|-----|--------|-----------|
| 0 | 0 | Dagger | `Dagger` |
| 0 | 1 | Short Sword | `ShortSword` |
| 0 | 2 | Short Sword 2 | `ShortSword2` |
| 0 | 3 | Long Sword | `LongSword` |
| 0 | 4 | Bastard Sword | `BastardSword` |
| 0 | 5 | Zweihander | `Zweihander` |
| 0 | 6 | Sanguine Dagger | *(not mapped)* |
| 0 | 7 | Magic Dagger | `MagicDagger` |
| 0 | 8 | Crystal Sword | `CrystalSword` |
| 0 | 9 | Evil Sword | `EvilSword` |
| 0 | 10 | Flame Sword | `FlameSword` |
| 1 | 0 | Wide Short Sword | `WideShortSword` |
| 1 | 1 | Wide Long Sword | `WideLongSword` |
| 1 | 2 | Rapier | `Rapier` |
| 1 | 3 | Long Rapier | `LongRapier` |
| 1 | 4 | Flamberge | `Flamberge` |
| 1 | 5 | Large Flamberge | *(not mapped)* |
| 1 | 6 | Great Sword | `GreatSword` |
| 2 | 0 | Shotel | *(not mapped)* |
| 2 | 1 | Scimitar | `Scimitar` |
| 2 | 2 | Large Scimitar | `LargeScimitar` |
| 2 | 3 | Great Scimitar | `GreatScimitar` |
| 2 | 4 | Kukri | `Kukri` |
| 3 | 0-6 | Axes (Hand Axe..Woodcutter's) | `HandAxe`..`WoodcuttersAxe` |
| 4 | 0-4 | Hammers | `BlacksmithHammer`..`GreatHammer` |
| 5 | 0-3 | Maces | `Mace1`..`SpikedBat` |
| 6 | 0-4 | Spears | `Spear`..`MagicSpear` |
| 7 | 0-2 | Flails | `Flail1`..`Flail3` |
| 8 | 0-2 | Clubs | `Club`..`GreatClub` |
| 9 | 0-4 | Ranged (Crossbow..LargeCrossbow) | `Crossbow`..`LargeCrossbow` |
| 10 | 0-6 | Staves | `CrystalStaff`..`FlameStaff` |

### Equipment

| Row | Col | Sprite | Rust Enum |
|-----|-----|--------|-----------|
| 11 | 0-6 | Shields | `Buckler`..`LargeShield` |
| 12 | 0-5 | Body Armor | `ClothArmor`..`ChestPlate` |
| 13 | 0-3 | Gloves | `ClothGloves`..`Gauntlets` |
| 14 | 0-3 | Boots | `Shoes`..`Greaves` |
| 15 | 0-7 | Helmets | `ClothHood`..`PlateHelm2` |
| 16 | 0-2 | Pendants | `RedPendant`..`CrystalPendant` |
| 17 | 0-5 | Rings (row 1) | `GoldEmeraldRing`..`OnyxRing` |
| 18 | 0-5 | Rings (row 2) | `GoldSignetRing`..`TwistedMetalRing` |

### Consumables & Misc

| Row | Col | Sprite | Rust Enum |
|-----|-----|--------|-----------|
| 19 | 0-4 | Potions (row 1) | `PurplePotion`..`GreenPotion` |
| 20 | 0-4 | Potions (row 2) | `BlackPotion`..`OrangePotion` |
| 21 | 0-6 | Scrolls & Books | `Scroll`..`Scroll2` |
| 22 | 0-2 | Keys | `GoldKey`..`MetalKey` |
| 23 | 0-3 | Ammo | `Arrow`..`Bolts` |
| 24 | 0-3 | Coins | `Coin`..`CoinPurse` |
| 25 | 0 | Cheese | `Cheese` |
| 25 | 1 | Bread | `Bread` |
| 25 | 2 | Apple | `Apple` |
| 25 | 3 | Bottle of Beer | `BottleOfBeer` |
| 25 | 4 | Bottle of Water | `BottleOfWater` |

> **Note:** The original `items.txt` has a typo `"25.e. bottle of water"` which
> should be `"26.e."` (it follows `"26.d. bottle of beer"` in the food row).

---

## rogues.png (7 cols x 8 rows, 32x32 cells)

Source: `32rogues/rogues.txt` | Code: `src/sprite_atlas/rogues.rs`

| Row | Col | Sprite | Rust Enum |
|-----|-----|--------|-----------|
| 0 | 0 | Dwarf | *(not mapped)* |
| 0 | 1 | Elf | *(not mapped)* |
| 0 | 2 | Ranger | *(not mapped)* |
| 0 | 3 | Rogue | `Rogue` |
| 0 | 4 | Bandit | *(not mapped)* |
| 1 | 0-4 | Knights / Fighters | *(not mapped)* |
| 2 | 0-6 | Monks / Clerics | *(not mapped)* |
| 3 | 0-5 | Barbarians / Swordsmen | *(not mapped)* |
| 4 | 0-4 | Wizards / Druids | *(not mapped)* |
| 5 | 0-5 | Warlock + more | *(not mapped)* |
| 6 | 0-5 | Farmers / NPCs | *(not mapped)* |
| 7 | 0-4 | Peasants / Shopkeeps | *(not mapped)* |

---

## Not yet deployed

These sheets exist in `32rogues-0.5.0.zip` but are not yet in `web/assets/`:

- **animated-tiles.png** — Braziers, fire pits, torches, lamps, fire, water waves, poison bubbles (12 items)
- **autotiles.png** — Water tiles, poison swamp tiles
- **items-palette-swaps.png** — Color variants of item sprites
