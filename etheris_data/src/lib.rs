pub mod appearance;
pub mod building;
pub mod emojis;
pub mod items;
pub mod jobs;
pub mod personality;
pub mod util;
pub mod weapon;
pub mod world;

mod skill_kind;
pub use skill_kind::*;

use items::{DefaultItemValue, DefaultItemValues};

#[derive(Debug, Clone, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct ShopItem {
    pub identifier: String,
    pub quantity: i32,
    pub price: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
pub enum ItemValue {
    AlternativeName(String),
    Recipes(Vec<String>),
    Durability(u16),
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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, serde::Deserialize, serde::Serialize)]
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
