mod item;

pub mod cosmetic;
pub mod lore;
pub mod material;
pub mod ore;
pub mod special;
pub mod tool;

pub mod recipes;

use etheris_common::clear_string;
pub use item::*;
use once_cell::sync::Lazy;

use crate::weapon::WeaponKind;

pub static ALL_ITEMS: Lazy<Vec<Item>> = Lazy::new(|| {
    let mut all_items = material::ALL_ITEMS.to_vec();
    all_items.extend_from_slice(ore::ALL_ITEMS);
    all_items.extend_from_slice(cosmetic::ALL_ITEMS);
    all_items.extend_from_slice(lore::ALL_ITEMS);
    all_items.extend_from_slice(tool::ALL_ITEMS);
    all_items.extend_from_slice(special::ALL_ITEMS);

    all_items
});

pub fn get_item(identifier: &str) -> Option<Item> {
    ALL_ITEMS
        .iter()
        .find(|i| i.identifier == identifier)
        .copied()
}

pub fn get_item_by_name(display_name: &str) -> Option<Item> {
    ALL_ITEMS
        .iter()
        .find(|i| clear_string(i.display_name) == clear_string(display_name))
        .copied()
}

pub fn get_item_by_weapon(weapon: WeaponKind) -> Item {
    ALL_ITEMS
        .iter()
        .find(|i| i.weapon == Some(weapon))
        .copied()
        .expect("Every weapon should be also a item")
}
