use super::*;
use etheris_discord::Emoji;

pub const ALL_ITEMS: &[Item] = &[RECIPE_BOOK, INTELLIGENCE_CRYSTAL, INVIGORATING_CRYSTAL];

pub const RECIPE_BOOK: Item = Item {
    identifier: "recipe_book",
    display_name: "Livro de Receitas",
    emoji: Emoji::from_unicode("📙"),
    tags: &[ItemTag::Specific, ItemTag::Special],
    stackable: false,
    default_values: DefaultItemValues {
        values: &[DefaultItemValue::Recipes(&[])],
    },
    ..Item::default()
};

pub const INTELLIGENCE_CRYSTAL: Item = Item {
    identifier: "intelligence_crystal",
    display_name: "Cristal da Inteligência",
    emoji: Emoji::from_emote(Some("intelligence_crystal"), 1175173728561270965),
    tags: &[ItemTag::Consumable, ItemTag::Special],
    has_consumption_function: true,
    purchase_properties: PurchaseProperties {
        base_price: 3000,
        base_sell_price: 450,

        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const INVIGORATING_CRYSTAL: Item = Item {
    identifier: "invigorating_crystal",
    display_name: "Cristal Revigorante",
    emoji: Emoji::from_emote(Some("invigorating_crystal"), 1175175892453052540),
    tags: &[ItemTag::Consumable, ItemTag::Special],
    has_consumption_function: true,
    purchase_properties: PurchaseProperties {
        base_price: 700,
        base_sell_price: 300,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};
