pub mod appearance;
pub mod building;
pub mod emojis;
pub mod items;
pub mod jobs;
pub mod personality;
pub mod util;
pub mod weapon;
pub mod world;
mod boss;
pub use boss::*;

mod skill_kind;
pub use skill_kind::*;

mod brain;
pub use brain::*;

use items::{DefaultItemValue, DefaultItemValues, Item};

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize,
)]
pub struct ShopItem {
    pub identifier: String,
    pub quantity: i32,
    pub price: i64,
    pub description: Option<String>,
    pub sellable_price: Option<i64>,
}

impl ShopItem {
    pub fn new(quantity: i32, identifier: impl ToString, price: i64) -> Self {
        Self {
            identifier: identifier.to_string(),
            quantity,
            price,
            sellable_price: None,
            description: None,
        }
    }

    pub fn new_item(quantity: i32, item: Item, price_multiplier: f64) -> Self {
        Self {
            identifier: item.identifier.to_string(),
            quantity,
            price: (item.purchase_properties.base_price as f64 * price_multiplier) as i64,
            sellable_price: None,
            description: None,
        }
    }

    pub fn new_sellable_item(
        quantity: i32,
        item: Item,
        price_multiplier: f64,
        sell_multiplier: f64,
    ) -> Self {
        Self {
            identifier: item.identifier.to_string(),
            quantity,
            price: (item.purchase_properties.base_price as f64 * price_multiplier) as i64,
            sellable_price: Some(
                (item.purchase_properties.base_sell_price as f64 * sell_multiplier) as i64,
            ),
            description: None,
        }
    }

    pub fn with_description(mut self, description: impl ToString) -> Self {
        self.description = Some(description.to_string());
        self
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize, Hash,
)]
pub enum ItemValue {
    AlternativeName(String),
    Recipes(Vec<String>),
    Durability(u32),
}

impl From<DefaultItemValue> for ItemValue {
    fn from(value: DefaultItemValue) -> Self {
        match value {
            DefaultItemValue::AlternativeName(name) => Self::AlternativeName(name.to_owned()),
            DefaultItemValue::Recipes(recipes) => {
                Self::Recipes(recipes.iter().map(|s| (*s).to_owned()).collect())
            }
            DefaultItemValue::Durability(durability) => Self::Durability(durability),
        }
    }
}
#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize, Hash,
)]
pub struct ItemValues {
    pub values: Vec<ItemValue>,
}

impl ItemValues {
    pub const EMPTY: Self = Self::empty();

    pub const fn empty() -> Self {
        Self { values: Vec::new() }
    }

    pub fn new(values: Vec<ItemValue>) -> Self {
        Self { values }
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn alternative_names(&self) -> Vec<String> {
        self.values
            .iter()
            .filter_map(|value| match value {
                ItemValue::AlternativeName(value) => Some(value),
                _ => None,
            })
            .cloned()
            .collect()
    }

    pub fn recipes(&self) -> Vec<String> {
        self.values
            .iter()
            .find_map(|value| match value {
                ItemValue::Recipes(value) => Some(value),
                _ => None,
            })
            .cloned()
            .unwrap_or_default()
    }
}

impl From<DefaultItemValues> for ItemValues {
    fn from(value: DefaultItemValues) -> Self {
        Self {
            values: value.values.iter().map(|v| (*v).into()).collect(),
        }
    }
}
