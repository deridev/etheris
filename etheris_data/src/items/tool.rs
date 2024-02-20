use super::*;
use etheris_discord::Emoji;

const TOOL_TAGS: &[ItemTag] = &[ItemTag::Tool];

pub const ALL_ITEMS: &[Item] = &[TRANSLATOR, BAT, SPEAR, KATANA, UMBRELLA];

pub const TRANSLATOR: Item = Item {
    identifier: "translator",
    display_name: "Tradutor",
    emoji: Emoji::from_unicode("📠"),
    tags: TOOL_TAGS,
    purchase_properties: PurchaseProperties {
        base_price: 3000,
        default_shop_sells: false,
        ..PurchaseProperties::default()
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
        default_shop_sells: false,
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
        default_shop_sells: false,
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
        default_shop_sells: false,
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
        default_shop_sells: false,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};
