use etheris_common::Color;
use etheris_discord::Emoji;

use crate::weapon::WeaponKind;

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub struct ConsumptionProperties {
    pub thirst_regeneration: i32,
    pub hunger_regeneration: i32,
    pub stamina_regeneration: i32,
    pub salt_level: f32,
    pub sugar_level: f32,
    pub calories: f32,
}

impl std::ops::Mul<i32> for ConsumptionProperties {
    type Output = ConsumptionProperties;

    fn mul(self, rhs: i32) -> Self::Output {
        let rhs_f32 = rhs as f32;
        Self {
            hunger_regeneration: self.hunger_regeneration * rhs,
            thirst_regeneration: self.thirst_regeneration * rhs,
            stamina_regeneration: self.stamina_regeneration * rhs,
            salt_level: self.salt_level * rhs_f32,
            sugar_level: self.sugar_level * rhs_f32,
            calories: self.calories * rhs_f32,
        }
    }
}

impl ConsumptionProperties {
    pub const fn default() -> Self {
        Self {
            hunger_regeneration: 0,
            thirst_regeneration: 0,
            stamina_regeneration: 0,
            salt_level: 0.0,
            sugar_level: 0.0,
            calories: 0.0,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CosmeticKind {
    Head,
    Face,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CosmeticProperties {
    pub cosmetic_identifier: &'static str,
    pub kind: CosmeticKind,
    pub color: Color,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PurchaseProperties {
    pub default_shop_sells: bool,
    pub is_buyable: bool,
    pub is_sellable: bool,
    pub base_price: i64,
}

impl PurchaseProperties {
    pub const fn default() -> Self {
        Self {
            base_price: 1,
            is_buyable: true,
            is_sellable: true,
            default_shop_sells: true,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ItemTag {
    Consumable,
    Tool,
    Material,
    Ore,
    Cosmetic,
    Special,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Page {
    pub title: &'static str,
    pub content: &'static str,
    pub translate: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DefaultItemValue {
    AlternativeName(&'static str),
    Recipes(&'static [&'static str]),
    Durability(u16),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DefaultItemValues {
    pub values: &'static [DefaultItemValue],
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Item {
    pub identifier: &'static str,
    pub display_name: &'static str,
    pub emoji: Emoji<'static>,
    pub stackable: bool,
    pub has_consumption_function: bool,
    pub purchase_properties: PurchaseProperties,
    pub cosmetic_properties: Option<CosmeticProperties>,
    pub weapon: Option<WeaponKind>,
    pub tags: &'static [ItemTag],
    pub pages: &'static [Page],
    pub default_values: DefaultItemValues,
}

impl Item {
    pub const fn default() -> Self {
        Self {
            identifier: "_invalid_item",
            display_name: "Invalid Item",
            emoji: Emoji::from_unicode("❔"),
            stackable: true,
            has_consumption_function: false,
            purchase_properties: PurchaseProperties {
                base_price: 1,
                is_buyable: false,
                is_sellable: false,
                default_shop_sells: false,
            },
            weapon: None,
            tags: &[],
            cosmetic_properties: None,
            pages: &[],
            default_values: DefaultItemValues { values: &[] },
        }
    }
}
