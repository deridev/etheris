use crate::weapon::WeaponKind;

use super::*;
use etheris_discord::Emoji;

const MATERIAL_TAGS: &[ItemTag] = &[ItemTag::Material];

pub const ALL_ITEMS: &[Item] = &[
    STONE,
    RAW_TRUNK,
    PLANK,
    STICK,
    PAPER,
    KNIFE,
    TOOL_HANDLE,
    BONE,
    SCORPION_FANG,
    COAL,
    IRON_BAR,
    COPPER_BAR,
    SILVER_BAR,
    GOLD_BAR,
];

pub const STONE: Item = Item {
    identifier: "stone",
    display_name: "Pedra",
    emoji: Emoji::from_emote(Some("stone"), 1098586680161730642),
    tags: MATERIAL_TAGS,
    purchase_properties: PurchaseProperties {
        base_price: 5,
        base_sell_price: 2,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const RAW_TRUNK: Item = Item {
    identifier: "raw_trunk",
    display_name: "Tronco Bruto",
    emoji: Emoji::from_emote(Some("raw_trunk"), 1074720130262630550),
    tags: MATERIAL_TAGS,
    purchase_properties: PurchaseProperties {
        base_price: 40,
        base_sell_price: 15,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const PLANK: Item = Item {
    identifier: "plank",
    display_name: "Tábua",
    emoji: Emoji::from_emote(Some("plank"), 1177293461863538698),
    tags: MATERIAL_TAGS,
    purchase_properties: PurchaseProperties {
        base_price: 15,
        base_sell_price: 4,
        is_sellable: true,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const STICK: Item = Item {
    identifier: "stick",
    display_name: "Graveto",
    emoji: Emoji::from_emote(Some("stick"), 1075463939804119111),
    tags: MATERIAL_TAGS,
    purchase_properties: PurchaseProperties {
        base_price: 12,
        base_sell_price: 6,
        is_sellable: true,
        ..PurchaseProperties::default()
    },
    weapon: Some(WeaponKind::Stick),
    ..Item::default()
};

pub const KNIFE: Item = Item {
    identifier: "knife",
    display_name: "Faca",
    emoji: Emoji::from_emote(Some("knife"), 1177287913369776229),
    tags: MATERIAL_TAGS,
    purchase_properties: PurchaseProperties {
        base_price: 60,
        base_sell_price: 12,
        is_sellable: true,
        ..PurchaseProperties::default()
    },
    weapon: Some(WeaponKind::Knife),
    ..Item::default()
};

pub const PAPER: Item = Item {
    identifier: "paper",
    display_name: "Papel",
    emoji: Emoji::from_emote(Some("paper"), 1174663109533778032),
    tags: MATERIAL_TAGS,
    purchase_properties: PurchaseProperties {
        base_price: 20,
        base_sell_price: 3,
        is_sellable: true,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const TOOL_HANDLE: Item = Item {
    identifier: "tool_handle",
    display_name: "Cabo de Ferramenta",
    emoji: Emoji::from_emote(Some("tool_handle"), 1212034381514809395),
    tags: MATERIAL_TAGS,
    purchase_properties: PurchaseProperties {
        base_price: 75,
        base_sell_price: 40,
        is_sellable: true,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const BONE: Item = Item {
    identifier: "bone",
    display_name: "Osso",
    emoji: Emoji::from_emote(Some("bone"), 1221804518421954571),
    tags: MATERIAL_TAGS,
    purchase_properties: PurchaseProperties {
        base_price: 30,
        base_sell_price: 15,
        is_sellable: true,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const SCORPION_FANG: Item = Item {
    identifier: "scorpion_fang",
    display_name: "Presa de Escorpião",
    emoji: Emoji::from_emote(Some("scorpion_fang"), 1260221716286799992),
    tags: MATERIAL_TAGS,
    purchase_properties: PurchaseProperties {
        base_price: 120,
        base_sell_price: 60,
        is_sellable: true,
        ..PurchaseProperties::default()
    },
    weapon: Some(WeaponKind::ScorpionFang),
    ..Item::default()
};

pub const COAL: Item = Item {
    identifier: "coal",
    display_name: "Carvão",
    emoji: Emoji::from_emote(Some("coal"), 1177300387598966856),
    tags: MATERIAL_TAGS,
    purchase_properties: PurchaseProperties {
        base_price: 50,
        base_sell_price: 12,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const IRON_BAR: Item = Item {
    identifier: "iron_bar",
    display_name: "Barra de Ferro",
    emoji: Emoji::from_emote(Some("iron_bar"), 1177299694997745735),
    tags: MATERIAL_TAGS,
    purchase_properties: PurchaseProperties {
        base_price: 120,
        base_sell_price: 20,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const COPPER_BAR: Item = Item {
    identifier: "copper_bar",
    display_name: "Barra de Cobre",
    emoji: Emoji::from_emote(Some("copper_bar"), 1177299656250769408),
    tags: MATERIAL_TAGS,
    purchase_properties: PurchaseProperties {
        base_price: 80,
        base_sell_price: 15,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const SILVER_BAR: Item = Item {
    identifier: "silver_bar",
    display_name: "Barra de Prata",
    emoji: Emoji::from_emote(Some("silver_bar"), 1177299735745400962),
    tags: MATERIAL_TAGS,
    purchase_properties: PurchaseProperties {
        base_price: 100,
        base_sell_price: 30,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const GOLD_BAR: Item = Item {
    identifier: "gold_bar",
    display_name: "Barra de Ouro",
    emoji: Emoji::from_emote(Some("gold_bar"), 1177299633437949992),
    tags: MATERIAL_TAGS,
    purchase_properties: PurchaseProperties {
        base_price: 800,
        base_sell_price: 125,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};
