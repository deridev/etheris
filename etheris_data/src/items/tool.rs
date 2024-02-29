use super::*;
use etheris_discord::Emoji;

const TOOL_TAGS: &[ItemTag] = &[ItemTag::Tool];

pub const ALL_ITEMS: &[Item] = &[
    TRANSLATOR, SHOVEL, PICKAXE, HAMMER, AXE, BAT, SPEAR, KATANA, UMBRELLA,
];

pub const TRANSLATOR: Item = Item {
    identifier: "translator",
    display_name: "Tradutor",
    emoji: Emoji::from_unicode("📠"),
    tags: TOOL_TAGS,
    purchase_properties: PurchaseProperties {
        base_price: 3000,
        base_sell_price: 200,

        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const SHOVEL: Item = Item {
    identifier: "shovel",
    display_name: "Pá",
    emoji: Emoji::from_emote(Some("shovel"), 1212034121958686822),
    tags: TOOL_TAGS,
    purchase_properties: PurchaseProperties {
        base_price: 100,
        base_sell_price: 50,
        ..PurchaseProperties::default()
    },
    default_values: DefaultItemValues {
        values: &[DefaultItemValue::Durability(15)],
    },
    ..Item::default()
};

pub const PICKAXE: Item = Item {
    identifier: "pickaxe",
    display_name: "Picareta",
    emoji: Emoji::from_emote(Some("pickaxe"), 1212036853117489213),
    tags: TOOL_TAGS,
    purchase_properties: PurchaseProperties {
        base_price: 350,
        base_sell_price: 150,
        ..PurchaseProperties::default()
    },
    default_values: DefaultItemValues {
        values: &[DefaultItemValue::Durability(30)],
    },
    ..Item::default()
};

pub const HAMMER: Item = Item {
    identifier: "hammer",
    display_name: "Martelo",
    emoji: Emoji::from_emote(Some("hammer"), 1212390855705362465),
    tags: TOOL_TAGS,
    purchase_properties: PurchaseProperties {
        base_price: 500,
        base_sell_price: 200,
        ..PurchaseProperties::default()
    },
    default_values: DefaultItemValues {
        values: &[DefaultItemValue::Durability(15)],
    },
    ..Item::default()
};

pub const AXE: Item = Item {
    identifier: "axe",
    display_name: "Machado",
    emoji: Emoji::from_emote(Some("axe"), 1212806414402060419),
    tags: TOOL_TAGS,
    purchase_properties: PurchaseProperties {
        base_price: 300,
        base_sell_price: 100,
        ..PurchaseProperties::default()
    },
    default_values: DefaultItemValues {
        values: &[DefaultItemValue::Durability(20)],
    },
    ..Item::default()
};

pub const BAT: Item = Item {
    identifier: "bat",
    display_name: "Taco",
    emoji: Emoji::from_emote(Some("bat"), 1206601810404712480),
    tags: TOOL_TAGS,
    weapon: Some(WeaponKind::Bat),
    purchase_properties: PurchaseProperties {
        base_price: 700,
        base_sell_price: 50,

        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const SPEAR: Item = Item {
    identifier: "spear",
    display_name: "Lança",
    emoji: Emoji::from_emote(Some("spear"), 1207527321020399706),
    tags: TOOL_TAGS,
    weapon: Some(WeaponKind::Spear),
    purchase_properties: PurchaseProperties {
        base_price: 1150,
        base_sell_price: 80,

        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const KATANA: Item = Item {
    identifier: "katana",
    display_name: "Katana",
    emoji: Emoji::from_emote(Some("katana"), 1207539850098770011),
    tags: TOOL_TAGS,
    weapon: Some(WeaponKind::Katana),
    purchase_properties: PurchaseProperties {
        base_price: 3000,
        base_sell_price: 125,

        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const UMBRELLA: Item = Item {
    identifier: "umbrella",
    display_name: "Guarda-Chuva",
    emoji: Emoji::from_emote(Some("umbrella"), 1207543971644047390),
    tags: TOOL_TAGS,
    weapon: Some(WeaponKind::Umbrella),
    purchase_properties: PurchaseProperties {
        base_price: 400,
        base_sell_price: 30,

        ..PurchaseProperties::default()
    },
    ..Item::default()
};
