use crate::map::DungeonBiome;
use super::types::*;

/// The four legendary equipment slots.
pub const LEGENDARY_SLOTS: [ItemKind; 4] = [
    ItemKind::Helmet,
    ItemKind::Armor,
    ItemKind::Shield,
    ItemKind::Boots,
];

/// Generate a legendary item for a given biome and equipment slot.
/// Each biome has a unique name prefix; each slot has a unique suffix.
///
/// Stats: Helmet +5, Armor +6, Shield +5, Boots +4 DEF.
/// Durability 0 (indestructible — existing `wear_armor()` only decrements when `durability > 0`).
pub fn legendary_item(biome: DungeonBiome, slot: &ItemKind) -> Item {
    let def = match slot {
        ItemKind::Helmet => 5,
        ItemKind::Armor  => 6,
        ItemKind::Shield => 5,
        ItemKind::Boots  => 4,
        _ => 0,
    };
    let glyph = match slot {
        ItemKind::Helmet => '^',
        ItemKind::Armor  => '[',
        ItemKind::Shield => ')',
        ItemKind::Boots  => '{',
        _ => '?',
    };
    let name = legendary_name(biome, slot);
    Item {
        kind: slot.clone(),
        name,
        glyph,
        effect: ItemEffect::BuffDefense(def),
        weight: 0,
        durability: 0,
        legendary: true,
    }
}

/// Static name for a legendary item: biome prefix + slot suffix.
/// 7 biomes x 4 slots = 28 unique names.
fn legendary_name(biome: DungeonBiome, slot: &ItemKind) -> &'static str {
    match (biome, slot) {
        // Goblin Warren — Warchief's
        (DungeonBiome::GoblinWarren, ItemKind::Helmet) => "Warchief's Helm",
        (DungeonBiome::GoblinWarren, ItemKind::Armor)  => "Warchief's Hauberk",
        (DungeonBiome::GoblinWarren, ItemKind::Shield) => "Warchief's Bulwark",
        (DungeonBiome::GoblinWarren, ItemKind::Boots)  => "Warchief's Greaves",

        // Undead Crypt — Lich
        (DungeonBiome::UndeadCrypt, ItemKind::Helmet) => "Lich Crown",
        (DungeonBiome::UndeadCrypt, ItemKind::Armor)  => "Lich Robes",
        (DungeonBiome::UndeadCrypt, ItemKind::Shield) => "Lich Phylactery",
        (DungeonBiome::UndeadCrypt, ItemKind::Boots)  => "Lich Sandals",

        // Fungal Grotto — Sporeguard
        (DungeonBiome::FungalGrotto, ItemKind::Helmet) => "Sporeguard Helm",
        (DungeonBiome::FungalGrotto, ItemKind::Armor)  => "Sporeguard Carapace",
        (DungeonBiome::FungalGrotto, ItemKind::Shield) => "Sporeguard Buckler",
        (DungeonBiome::FungalGrotto, ItemKind::Boots)  => "Sporeguard Treads",

        // Orc Stronghold — Ettin-themed
        (DungeonBiome::OrcStronghold, ItemKind::Helmet) => "Ettin-Skull Helm",
        (DungeonBiome::OrcStronghold, ItemKind::Armor)  => "Ettin-Hide Armor",
        (DungeonBiome::OrcStronghold, ItemKind::Shield) => "Ettin-Bone Shield",
        (DungeonBiome::OrcStronghold, ItemKind::Boots)  => "Ettin-Skin Boots",

        // Abyssal Temple — Reaper's
        (DungeonBiome::AbyssalTemple, ItemKind::Helmet) => "Reaper's Cowl",
        (DungeonBiome::AbyssalTemple, ItemKind::Armor)  => "Reaper's Vestment",
        (DungeonBiome::AbyssalTemple, ItemKind::Shield) => "Reaper's Aegis",
        (DungeonBiome::AbyssalTemple, ItemKind::Boots)  => "Reaper's Treads",

        // Beast Den — Wendigo
        (DungeonBiome::BeastDen, ItemKind::Helmet) => "Wendigo Skull",
        (DungeonBiome::BeastDen, ItemKind::Armor)  => "Wendigo Hide",
        (DungeonBiome::BeastDen, ItemKind::Shield) => "Wendigo Claw Shield",
        (DungeonBiome::BeastDen, ItemKind::Boots)  => "Wendigo Hooves",

        // Serpent Pit — Basilisk
        (DungeonBiome::SerpentPit, ItemKind::Helmet) => "Basilisk Crest",
        (DungeonBiome::SerpentPit, ItemKind::Armor)  => "Basilisk Scalemail",
        (DungeonBiome::SerpentPit, ItemKind::Shield) => "Basilisk Eye Shield",
        (DungeonBiome::SerpentPit, ItemKind::Boots)  => "Basilisk Scale Boots",

        _ => "Legendary Relic",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legendary_items_have_correct_stats() {
        for biome in DungeonBiome::PLACEABLE {
            for slot in &LEGENDARY_SLOTS {
                let item = legendary_item(biome, slot);
                assert!(item.legendary, "{} should be legendary", item.name);
                assert_eq!(item.durability, 0, "{} should be indestructible", item.name);
                assert_eq!(item.kind, *slot, "{} has wrong kind", item.name);
            }
        }
    }

    #[test]
    fn legendary_defense_values() {
        let biome = DungeonBiome::GoblinWarren;
        let h = legendary_item(biome, &ItemKind::Helmet);
        let a = legendary_item(biome, &ItemKind::Armor);
        let s = legendary_item(biome, &ItemKind::Shield);
        let b = legendary_item(biome, &ItemKind::Boots);
        assert_eq!(h.effect, ItemEffect::BuffDefense(5));
        assert_eq!(a.effect, ItemEffect::BuffDefense(6));
        assert_eq!(s.effect, ItemEffect::BuffDefense(5));
        assert_eq!(b.effect, ItemEffect::BuffDefense(4));
    }

    #[test]
    fn all_28_names_unique() {
        let mut names = std::collections::HashSet::new();
        for biome in DungeonBiome::PLACEABLE {
            for slot in &LEGENDARY_SLOTS {
                let item = legendary_item(biome, slot);
                assert!(names.insert(item.name),
                    "duplicate legendary name: {}", item.name);
            }
        }
        assert_eq!(names.len(), 28);
    }
}
