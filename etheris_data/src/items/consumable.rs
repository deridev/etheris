use super::*;
use etheris_discord::Emoji;

const CONSUMABLE_TAGS: &[ItemTag] = &[ItemTag::Consumable];

pub const ALL_ITEMS: &[Item] = &[
    SALT,
    SUGAR,
    COFFEE_POWDER,
    WATER,
    MILK,
    WHEAT,
    SLICE_OF_BREAD,
    BREAD,
    EGG,
    FRIED_EGG,
    CHEESE,
    APPLE,
    GREEN_APPLE,
    CORN,
    ORANGE,
    LEMON,
    WATERMELON,
    TOMATO,
    CHOCOLATE,
    CHOCOLATE_MILK,
    ORANGE_JUICE,
    LEMONADE,
    COFFEE,
    BEEF,
    CHICKEN_MEAT,
    BACON,
    COMMON_FISH,
    TROPICAL_FISH,
    GREEN_FISH,
];

pub const SALT: Item = Item {
    identifier: "salt",
    display_name: "Sal",
    emoji: Emoji::from_emote(Some("salt"), 1076475138305953857),
    tags: CONSUMABLE_TAGS,
    consumption_properties: Some(ConsumptionProperties {
        health_regenation: 1,
        ether_regeneration: 0,
        ..ConsumptionProperties::default()
    }),
    purchase_properties: PurchaseProperties {
        base_price: 2,
        base_sell_price: 1,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const SUGAR: Item = Item {
    identifier: "sugar",
    display_name: "Açúcar",
    emoji: Emoji::from_emote(Some("sugar"), 1075463702683328665),
    tags: CONSUMABLE_TAGS,
    consumption_properties: Some(ConsumptionProperties {
        ether_regeneration: 1,
        health_regenation: 0,
        ..ConsumptionProperties::default()
    }),
    purchase_properties: PurchaseProperties {
        base_price: 2,
        base_sell_price: 1,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const COFFEE_POWDER: Item = Item {
    identifier: "coffee_powder",
    display_name: "Pó de Café",
    emoji: Emoji::from_emote(Some("coffee_powder"), 1271195392079429765),
    tags: CONSUMABLE_TAGS,
    consumption_properties: Some(ConsumptionProperties {
        ether_regeneration: 3,
        health_regenation: 1,
        ..ConsumptionProperties::default()
    }),
    purchase_properties: PurchaseProperties {
        base_price: 48,
        base_sell_price: 7,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const WHEAT: Item = Item {
    identifier: "wheat",
    display_name: "Trigo",
    emoji: Emoji::from_emote(Some("wheat"), 1076480042277797918),
    tags: CONSUMABLE_TAGS,
    consumption_properties: Some(ConsumptionProperties {
        ether_regeneration: 1,
        health_regenation: 1,
        ..ConsumptionProperties::default()
    }),
    purchase_properties: PurchaseProperties {
        base_price: 8,
        base_sell_price: 3,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const SLICE_OF_BREAD: Item = Item {
    identifier: "slice_of_bread",
    display_name: "Fatia de Pão",
    emoji: Emoji::from_emote(Some("slice_of_bread"), 1074706585022115940),
    tags: CONSUMABLE_TAGS,
    consumption_properties: Some(ConsumptionProperties {
        health_regenation: 10,
        ether_regeneration: 0,
        ..ConsumptionProperties::default()
    }),
    purchase_properties: PurchaseProperties {
        base_price: 18,
        base_sell_price: 9,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const BREAD: Item = Item {
    identifier: "bread",
    display_name: "Pão",
    emoji: Emoji::from_emote(Some("bread"), 1074706636767240202),
    tags: CONSUMABLE_TAGS,
    consumption_properties: Some(ConsumptionProperties {
        health_regenation: 60,
        ether_regeneration: 0,
        ..ConsumptionProperties::default()
    }),
    purchase_properties: PurchaseProperties {
        base_price: 40,
        base_sell_price: 15,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const WATER: Item = Item {
    identifier: "water",
    display_name: "Água",
    emoji: Emoji::from_emote(Some("water"), 1076492433497542696),
    tags: CONSUMABLE_TAGS,
    consumption_properties: Some(ConsumptionProperties {
        health_regenation: 0,
        ether_regeneration: 5,
        ..ConsumptionProperties::default()
    }),
    purchase_properties: PurchaseProperties {
        base_price: 10,
        base_sell_price: 2,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const MILK: Item = Item {
    identifier: "milk",
    display_name: "Leite",
    emoji: Emoji::from_emote(Some("milk"), 1076492468645797989),
    tags: CONSUMABLE_TAGS,
    consumption_properties: Some(ConsumptionProperties {
        health_regenation: 1,
        ether_regeneration: 8,
        ..ConsumptionProperties::default()
    }),
    purchase_properties: PurchaseProperties {
        base_price: 20,
        base_sell_price: 5,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const EGG: Item = Item {
    identifier: "egg",
    display_name: "Ovo",
    emoji: Emoji::from_emote(Some("egg"), 1075468833697050664),
    tags: CONSUMABLE_TAGS,
    consumption_properties: Some(ConsumptionProperties {
        health_regenation: 8,
        ether_regeneration: 1,
        ..ConsumptionProperties::default()
    }),
    purchase_properties: PurchaseProperties {
        base_price: 5,
        base_sell_price: 2,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const FRIED_EGG: Item = Item {
    identifier: "fried_egg",
    display_name: "Ovo Frito",
    emoji: Emoji::from_emote(Some("fried_egg"), 1075468874121740429),
    tags: CONSUMABLE_TAGS,
    consumption_properties: Some(ConsumptionProperties {
        health_regenation: 20,
        ether_regeneration: 2,
        ..ConsumptionProperties::default()
    }),
    purchase_properties: PurchaseProperties {
        base_price: 20,
        base_sell_price: 5,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const CHEESE: Item = Item {
    identifier: "cheese",
    display_name: "Queijo",
    emoji: Emoji::from_emote(Some("cheese"), 1173970030619283489),
    tags: CONSUMABLE_TAGS,
    consumption_properties: Some(ConsumptionProperties {
        health_regenation: 30,
        ether_regeneration: 3,
        ..ConsumptionProperties::default()
    }),
    purchase_properties: PurchaseProperties {
        base_price: 30,
        base_sell_price: 9,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const APPLE: Item = Item {
    identifier: "apple",
    display_name: "Maçã",
    emoji: Emoji::from_emote(Some("apple"), 1074675013619290132),
    tags: CONSUMABLE_TAGS,
    consumption_properties: Some(ConsumptionProperties {
        health_regenation: 15,
        ether_regeneration: 0,
        ..ConsumptionProperties::default()
    }),
    purchase_properties: PurchaseProperties {
        base_price: 12,
        base_sell_price: 5,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const GREEN_APPLE: Item = Item {
    identifier: "green_apple",
    display_name: "Maçã Verde",
    emoji: Emoji::from_emote(Some("green_apple"), 1075464813288894677),
    tags: CONSUMABLE_TAGS,
    consumption_properties: Some(ConsumptionProperties {
        health_regenation: 10,
        ether_regeneration: 2,
        ..ConsumptionProperties::default()
    }),
    purchase_properties: PurchaseProperties {
        base_price: 12,
        base_sell_price: 5,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const CORN: Item = Item {
    identifier: "corn",
    display_name: "Milho",
    emoji: Emoji::from_emote(Some("corn"), 1075474488352837692),
    tags: CONSUMABLE_TAGS,
    consumption_properties: Some(ConsumptionProperties {
        health_regenation: 20,
        ether_regeneration: 0,
        ..ConsumptionProperties::default()
    }),
    purchase_properties: PurchaseProperties {
        base_price: 15,
        base_sell_price: 6,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const ORANGE: Item = Item {
    identifier: "orange",
    display_name: "Laranja",
    emoji: Emoji::from_emote(Some("orange"), 1074675044426469377),
    tags: CONSUMABLE_TAGS,
    consumption_properties: Some(ConsumptionProperties {
        health_regenation: 25,
        ether_regeneration: 0,
        ..ConsumptionProperties::default()
    }),
    purchase_properties: PurchaseProperties {
        base_price: 15,
        base_sell_price: 6,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const LEMON: Item = Item {
    identifier: "lemon",
    display_name: "Limão",
    emoji: Emoji::from_emote(Some("lemon"), 1074677203750633482),
    tags: CONSUMABLE_TAGS,
    consumption_properties: Some(ConsumptionProperties {
        health_regenation: 15,
        ether_regeneration: 3,
        ..ConsumptionProperties::default()
    }),
    purchase_properties: PurchaseProperties {
        base_price: 18,
        base_sell_price: 7,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const WATERMELON: Item = Item {
    identifier: "watermelon",
    display_name: "Melancia",
    emoji: Emoji::from_emote(Some("watermelon"), 1076499133420023818),
    tags: CONSUMABLE_TAGS,
    consumption_properties: Some(ConsumptionProperties {
        health_regenation: 40,
        ether_regeneration: 5,
        ..ConsumptionProperties::default()
    }),
    purchase_properties: PurchaseProperties {
        base_price: 25,
        base_sell_price: 10,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const TOMATO: Item = Item {
    identifier: "tomato",
    display_name: "Tomate",
    emoji: Emoji::from_emote(Some("tomato"), 1074687074587902042),
    tags: CONSUMABLE_TAGS,
    consumption_properties: Some(ConsumptionProperties {
        health_regenation: 30,
        ether_regeneration: 0,
        ..ConsumptionProperties::default()
    }),
    purchase_properties: PurchaseProperties {
        base_price: 13,
        base_sell_price: 4,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const CHOCOLATE: Item = Item {
    identifier: "chocolate",
    display_name: "Chocolate",
    emoji: Emoji::from_emote(Some("chocolate"), 1075483317891235900),
    tags: CONSUMABLE_TAGS,
    consumption_properties: Some(ConsumptionProperties {
        health_regenation: 80,
        ether_regeneration: 5,
        ..ConsumptionProperties::default()
    }),
    purchase_properties: PurchaseProperties {
        base_price: 23,
        base_sell_price: 11,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const CHOCOLATE_MILK: Item = Item {
    identifier: "chocolate_milk",
    display_name: "Achocolatado",
    emoji: Emoji::from_emote(Some("chocolate_milk"), 1076492726071218227),
    tags: CONSUMABLE_TAGS,
    consumption_properties: Some(ConsumptionProperties {
        health_regenation: 10,
        ether_regeneration: 20,
        ..ConsumptionProperties::default()
    }),
    purchase_properties: PurchaseProperties {
        base_price: 26,
        base_sell_price: 12,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const ORANGE_JUICE: Item = Item {
    identifier: "orange_juice",
    display_name: "Suco de Laranja",
    emoji: Emoji::from_emote(Some("orange_juice"), 1076492513097023589),
    tags: CONSUMABLE_TAGS,
    consumption_properties: Some(ConsumptionProperties {
        health_regenation: 30,
        ether_regeneration: 10,
        ..ConsumptionProperties::default()
    }),
    purchase_properties: PurchaseProperties {
        base_price: 32,
        base_sell_price: 18,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const LEMONADE: Item = Item {
    identifier: "lemonade",
    display_name: "Limonada",
    emoji: Emoji::from_emote(Some("lemonade"), 1076495924433539173),
    tags: CONSUMABLE_TAGS,
    consumption_properties: Some(ConsumptionProperties {
        health_regenation: 10,
        ether_regeneration: 30,
        ..ConsumptionProperties::default()
    }),
    purchase_properties: PurchaseProperties {
        base_price: 37,
        base_sell_price: 21,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const COFFEE: Item = Item {
    identifier: "coffee",
    display_name: "Café",
    emoji: Emoji::from_emote(Some("coffee_mug"), 1271195435188355145),
    tags: CONSUMABLE_TAGS,
    consumption_properties: Some(ConsumptionProperties {
        health_regenation: 20,
        ether_regeneration: 30,
        ..ConsumptionProperties::default()
    }),
    purchase_properties: PurchaseProperties {
        base_price: 90,
        base_sell_price: 25,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const BEEF: Item = Item {
    identifier: "beef",
    display_name: "Bife",
    emoji: Emoji::from_emote(Some("beef"), 1076502264753168545),
    tags: CONSUMABLE_TAGS,
    consumption_properties: Some(ConsumptionProperties {
        health_regenation: 95,
        ether_regeneration: 0,
        ..ConsumptionProperties::default()
    }),
    purchase_properties: PurchaseProperties {
        base_price: 80,
        base_sell_price: 30,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const CHICKEN_MEAT: Item = Item {
    identifier: "chicken_meat",
    display_name: "Carne de Galinha",
    emoji: Emoji::from_unicode("🍗"),
    tags: CONSUMABLE_TAGS,
    consumption_properties: Some(ConsumptionProperties {
        health_regenation: 60,
        ether_regeneration: 10,
        ..ConsumptionProperties::default()
    }),
    purchase_properties: PurchaseProperties {
        base_price: 110,
        base_sell_price: 21,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const BACON: Item = Item {
    identifier: "bacon",
    display_name: "Bacon",
    emoji: Emoji::from_emote(Some("bacon"), 1076974186762862673),
    tags: CONSUMABLE_TAGS,
    consumption_properties: Some(ConsumptionProperties {
        health_regenation: 120,
        ether_regeneration: 0,
        ..ConsumptionProperties::default()
    }),
    purchase_properties: PurchaseProperties {
        base_price: 329,
        base_sell_price: 115,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const COMMON_FISH: Item = Item {
    identifier: "common_fish",
    display_name: "Peixe",
    emoji: Emoji::from_emote(Some("common_fish"), 1076978186413408348),
    tags: CONSUMABLE_TAGS,
    consumption_properties: Some(ConsumptionProperties {
        health_regenation: 40,
        ether_regeneration: 10,
        ..ConsumptionProperties::default()
    }),
    purchase_properties: PurchaseProperties {
        base_price: 25,
        base_sell_price: 10,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const TROPICAL_FISH: Item = Item {
    identifier: "tropical_fish",
    display_name: "Peixe-Tropical",
    emoji: Emoji::from_emote(Some("tropical_fish"), 1076980239160332329),
    tags: CONSUMABLE_TAGS,
    consumption_properties: Some(ConsumptionProperties {
        health_regenation: 60,
        ether_regeneration: 15,
        ..ConsumptionProperties::default()
    }),
    purchase_properties: PurchaseProperties {
        base_price: 45,
        base_sell_price: 22,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};

pub const GREEN_FISH: Item = Item {
    identifier: "green_fish",
    display_name: "Peixe-Verde",
    emoji: Emoji::from_emote(Some("green_fish"), 1076979486073028720),
    tags: CONSUMABLE_TAGS,
    consumption_properties: Some(ConsumptionProperties {
        health_regenation: 80,
        ether_regeneration: 15,
        ..ConsumptionProperties::default()
    }),
    purchase_properties: PurchaseProperties {
        base_price: 35,
        base_sell_price: 15,
        ..PurchaseProperties::default()
    },
    ..Item::default()
};
