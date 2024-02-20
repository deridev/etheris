use super::*;
use etheris_common::Color;
use etheris_discord::Emoji;

const COSMETIC_TAGS: &[ItemTag] = &[ItemTag::Cosmetic];

pub const ALL_ITEMS: &[Item] = &[STRAWHAT, EYE_BANDANA, GLASSES];

pub const STRAWHAT: Item = Item {
    identifier: "strawhat",
    display_name: "Chapéu de Palha",
    emoji: Emoji::from_emote(Some("strawhat"), 1173587868153806858),
    tags: COSMETIC_TAGS,
    purchase_properties: PurchaseProperties {
        default_shop_sells: false,
        base_price: 700,
        base_sell_price: 300,
        ..PurchaseProperties::default()
    },
    cosmetic_properties: Some(CosmeticProperties {
        cosmetic_identifier: "strawhat",
        kind: CosmeticKind::Head,
        color: Color::WHITE,
    }),
    ..Item::default()
};

pub const EYE_BANDANA: Item = Item {
    identifier: "eye_bandana",
    display_name: "Bandana de Olho",
    emoji: Emoji::from_emote(Some("eye_bandana"), 1173620023391178843),
    tags: COSMETIC_TAGS,
    purchase_properties: PurchaseProperties {
        default_shop_sells: false,
        base_price: 500,
        base_sell_price: 200,
        ..PurchaseProperties::default()
    },
    cosmetic_properties: Some(CosmeticProperties {
        cosmetic_identifier: "eye_bandana",
        kind: CosmeticKind::Face,
        color: Color::WHITE,
    }),
    ..Item::default()
};

pub const GLASSES: Item = Item {
    identifier: "glasses",
    display_name: "Óculos",
    emoji: Emoji::from_unicode("👓"),
    tags: COSMETIC_TAGS,
    purchase_properties: PurchaseProperties {
        default_shop_sells: false,
        base_price: 150,
        base_sell_price: 80,
        ..PurchaseProperties::default()
    },
    cosmetic_properties: Some(CosmeticProperties {
        cosmetic_identifier: "glasses_1",
        kind: CosmeticKind::Face,
        color: Color::BLACK,
    }),
    ..Item::default()
};
