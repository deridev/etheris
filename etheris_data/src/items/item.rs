use etheris_common::Color;
use etheris_discord::Emoji;

use crate::weapon::WeaponKind;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ConsumptionProperties {
    pub health_regenation: i32,
    pub ether_regeneration: i32,
    pub scale_factor: u8,
}

impl std::ops::Mul<i32> for ConsumptionProperties {
    type Output = ConsumptionProperties;

    fn mul(self, rhs: i32) -> Self::Output {
        Self {
            health_regenation: self.health_regenation * rhs,
            ether_regeneration: self.ether_regeneration * rhs,
            scale_factor: self.scale_factor.saturating_mul(rhs as u8),
        }
    }
}

impl ConsumptionProperties {
    pub const fn default() -> Self {
        Self {
            health_regenation: 0,
            ether_regeneration: 0,
            scale_factor: 100,
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
    pub is_buyable: bool,
    pub is_sellable: bool,
    pub base_price: i64,
    pub base_sell_price: i64,
}

impl PurchaseProperties {
    pub const fn default() -> Self {
        Self {
            base_price: 1,
            base_sell_price: 1,
            is_buyable: true,
            is_sellable: true,
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
    Specific,
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
    Durability(u32),
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
    pub consumption_properties: Option<ConsumptionProperties>,
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
            consumption_properties: None,
            purchase_properties: PurchaseProperties {
                base_price: 1,
                base_sell_price: 0,
                is_buyable: false,
                is_sellable: false,
            },
            weapon: None,
            tags: &[],
            cosmetic_properties: None,
            pages: &[],
            default_values: DefaultItemValues { values: &[] },
        }
    }

    pub const fn useable_in_battle(&self) -> bool {
        self.consumption_properties.is_some()
    }
}
