use crate::weapon::WeaponKind;

use super::*;
use etheris_discord::Emoji;

const MATERIAL_TAGS: &[ItemTag] = &[ItemTag::Consumable];

pub const ALL_ITEMS: &[Item] = &[STONE, RAW_TRUNK, PLANK, STICK, PAPER, KNIFE, TOOL_HANDLE];

pub const STONE: Item = Item {
    identifier: "stone",
    display_name: "Pedra",
    emoji: Emoji::from_emote(Some("stone"), 1098586680161730642),
    tags: MATERIAL_TAGS,
    purchase_properties: PurchaseProperties {
        base_price: 5,
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
        base_price: 10,
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
        base_price: 130,
        base_sell_price: 30,
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
        base_sell_price: 5,
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
