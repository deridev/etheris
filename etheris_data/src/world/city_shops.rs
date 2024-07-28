use once_cell::sync::Lazy;

use crate::{items, ShopItem};

pub static METROLIS: Lazy<Vec<ShopItem>> = Lazy::new(|| {
    vec![
        // Consumables
        ShopItem::new_item(200, items::consumable::SALT, 1.2),
        ShopItem::new_item(200, items::consumable::SUGAR, 1.2),
        ShopItem::new_item(150, items::consumable::WATER, 1.2),
        ShopItem::new_item(150, items::consumable::MILK, 1.3),
        ShopItem::new_item(100, items::consumable::WHEAT, 1.2),
        ShopItem::new_item(100, items::consumable::SLICE_OF_BREAD, 1.2),
        ShopItem::new_item(100, items::consumable::BREAD, 1.3),
        ShopItem::new_item(50, items::consumable::EGG, 1.4),
        ShopItem::new_item(35, items::consumable::FRIED_EGG, 1.4),
        ShopItem::new_item(35, items::consumable::CHEESE, 1.3),
        ShopItem::new_item(25, items::consumable::APPLE, 1.2),
        ShopItem::new_item(25, items::consumable::GREEN_APPLE, 1.2),
        ShopItem::new_item(25, items::consumable::CORN, 1.2),
        ShopItem::new_item(25, items::consumable::ORANGE, 1.2),
        ShopItem::new_item(25, items::consumable::LEMON, 1.2),
        ShopItem::new_item(10, items::consumable::WATERMELON, 1.2),
        ShopItem::new_item(25, items::consumable::TOMATO, 1.2),
        ShopItem::new_item(50, items::consumable::CHOCOLATE, 1.5),
        // Materials
        ShopItem::new_sellable_item(150, items::material::STONE, 1.2, 0.8),
        ShopItem::new_sellable_item(80, items::material::RAW_TRUNK, 1.4, 0.8),
        ShopItem::new_sellable_item(120, items::material::PLANK, 1.1, 0.8),
        ShopItem::new_sellable_item(100, items::material::STICK, 1.1, 0.8),
        ShopItem::new_sellable_item(80, items::material::PAPER, 1.1, 0.8),
        ShopItem::new_sellable_item(25, items::material::KNIFE, 1.3, 0.8),
        ShopItem::new_sellable_item(50, items::material::TOOL_HANDLE, 1.2, 0.6),
        // Tools
        ShopItem::new_item(80, items::tool::SHOVEL, 0.8),
        ShopItem::new_item(50, items::tool::AXE, 1.4),
        ShopItem::new_item(30, items::tool::PICKAXE, 1.1),
        ShopItem::new_item(5, items::tool::HAMMER, 1.5),
        // Cosmetics
        ShopItem::new_item(135, items::cosmetic::GLASSES, 0.8),
        // Lore
        ShopItem::new_item(338, items::lore::METROLIS_LAW_338, 1.0),
    ]
});

pub static SWORD_TOWN: Lazy<Vec<ShopItem>> = Lazy::new(|| {
    vec![
        // Consumables
        ShopItem::new_item(100, items::consumable::SALT, 1.3),
        ShopItem::new_item(100, items::consumable::SUGAR, 1.3),
        ShopItem::new_item(300, items::consumable::WATER, 1.1),
        ShopItem::new_item(300, items::consumable::MILK, 1.2),
        ShopItem::new_item(250, items::consumable::WHEAT, 1.2),
        ShopItem::new_item(200, items::consumable::SLICE_OF_BREAD, 1.0),
        ShopItem::new_item(150, items::consumable::BREAD, 1.1),
        ShopItem::new_item(125, items::consumable::EGG, 1.0),
        ShopItem::new_item(100, items::consumable::FRIED_EGG, 1.1),
        ShopItem::new_item(80, items::consumable::CHEESE, 1.2),
        ShopItem::new_item(15, items::consumable::APPLE, 1.3),
        ShopItem::new_item(15, items::consumable::GREEN_APPLE, 1.3),
        ShopItem::new_item(15, items::consumable::CORN, 1.3),
        ShopItem::new_item(15, items::consumable::ORANGE, 1.4),
        ShopItem::new_item(15, items::consumable::LEMON, 1.4),
        ShopItem::new_item(10, items::consumable::WATERMELON, 1.6),
        ShopItem::new_item(30, items::consumable::TOMATO, 1.4),
        ShopItem::new_item(50, items::consumable::CHOCOLATE, 1.3),
        ShopItem::new_item(80, items::consumable::COMMON_FISH, 1.5),
        ShopItem::new_item(40, items::consumable::TROPICAL_FISH, 1.2),
        // Materials
        ShopItem::new_sellable_item(150, items::material::STONE, 1.3, 0.6),
        ShopItem::new_sellable_item(80, items::material::RAW_TRUNK, 1.3, 0.9),
        ShopItem::new_sellable_item(120, items::material::PLANK, 1.0, 0.9),
        ShopItem::new_sellable_item(100, items::material::STICK, 1.0, 0.9),
        ShopItem::new_sellable_item(80, items::material::PAPER, 1.0, 0.9),
        ShopItem::new_sellable_item(25, items::material::KNIFE, 0.9, 0.6),
        ShopItem::new_sellable_item(50, items::material::TOOL_HANDLE, 1.1, 0.8),
        // Tools
        ShopItem::new_item(120, items::tool::SHOVEL, 0.9),
        ShopItem::new_item(80, items::tool::AXE, 1.2),
        ShopItem::new_item(50, items::tool::PICKAXE, 1.2),
        ShopItem::new_item(30, items::tool::HAMMER, 1.3),
        // Weapon Tools
        ShopItem::new_item(500, items::tool::KATANA, 0.9),
        // Cosmetics
        ShopItem::new_item(100, items::cosmetic::GLASSES, 1.2),
        // Lore
        ShopItem::new_item(11, items::lore::THE_MAN_WHO_SEES_SOUNDS, 1.2),
    ]
});
