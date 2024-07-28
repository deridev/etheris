use super::*;
use etheris_discord::Emoji;

const ORE_TAGS: &[ItemTag] = &[ItemTag::Ore];

pub const ALL_ITEMS: &[Item] = &[
    COAL_ORE,
    IRON_ORE,
    COPPER_ORE,
    TIN_ORE,
    LEAD_ORE,
    GOLD_ORE,
    DIAMOND_ORE,
];

pub const COAL_ORE: Item = Item {
    identifier: "coal_ore",
    display_name: "Minério de Carvão",
    emoji: Emoji::from_emote(Some("coal_ore"), 1104795286129352774),
    tags: ORE_TAGS,
    purchase_properties: PurchaseProperties {
        base_price: 30,
        base_sell_price: 4,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const IRON_ORE: Item = Item {
    identifier: "iron_ore",
    display_name: "Minério de Ferro",
    emoji: Emoji::from_emote(Some("iron_ore"), 1098586725799952522),
    tags: ORE_TAGS,
    purchase_properties: PurchaseProperties {
        base_price: 90,
        base_sell_price: 7,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const LEAD_ORE: Item = Item {
    identifier: "lead_ore",
    display_name: "Minério de Chumbo",
    emoji: Emoji::from_emote(Some("lead_ore"), 1104794112252715068),
    tags: ORE_TAGS,
    purchase_properties: PurchaseProperties {
        base_price: 75,
        base_sell_price: 10,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const COPPER_ORE: Item = Item {
    identifier: "copper_ore",
    display_name: "Minério de Cobre",
    emoji: Emoji::from_emote(Some("copper_ore"), 1098586777511526420),
    tags: ORE_TAGS,
    purchase_properties: PurchaseProperties {
        base_price: 60,
        base_sell_price: 15,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const TIN_ORE: Item = Item {
    identifier: "tin_ore",
    display_name: "Minério de Estranho",
    emoji: Emoji::from_emote(Some("tin_ore"), 1104794061191249920),
    tags: ORE_TAGS,
    purchase_properties: PurchaseProperties {
        base_price: 70,
        base_sell_price: 16,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const GOLD_ORE: Item = Item {
    identifier: "gold_ore",
    display_name: "Minério de Ouro",
    emoji: Emoji::from_emote(Some("gold_ore"), 1104794155261100092),
    tags: ORE_TAGS,
    purchase_properties: PurchaseProperties {
        base_price: 520,
        base_sell_price: 46,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const DIAMOND_ORE: Item = Item {
    identifier: "diamond_ore",
    display_name: "Minério de Diamante",
    emoji: Emoji::from_emote(Some("diamond_ore"), 1104794226933375099),
    tags: ORE_TAGS,
    purchase_properties: PurchaseProperties {
        base_price: 2300,
        base_sell_price: 128,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};
