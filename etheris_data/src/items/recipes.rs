use super::{get_item, Item};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ingredient {
    pub item: &'static str,
    pub quantity: usize,
}

impl Ingredient {
    pub const fn new(item: &'static str, quantity: usize) -> Self {
        Self { item, quantity }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Recipe {
    pub output: &'static str,
    pub output_quantity: usize,
    pub ingredients: &'static [Ingredient],
}

fn verify_ingredients(a: &[Ingredient], b: &[Ingredient]) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut a = a.to_vec();
    a.sort();
    let mut b = b.to_vec();
    b.sort();

    a.iter()
        .zip(b.iter())
        .all(|(a, b)| a.item == b.item && a.quantity >= b.quantity)
}

#[rustfmt::skip]
pub fn get_item_by_recipe(ingredients: &[Ingredient]) -> Option<(Item, usize, Vec<Ingredient>)> {
    let recipe = *ALL_RECIPES.iter().find(|r| verify_ingredients(ingredients, r.ingredients))?;
    let output_item = get_item(recipe.output)?;

    let mut recipe_ingredients = recipe.ingredients.to_vec();
    recipe_ingredients.sort();

    let mut ingredients = ingredients.to_vec();
    ingredients.sort();

    let mut used_ingredients = ingredients.iter().map(|i| Ingredient { item: i.item, quantity: 0 }).collect::<Vec<_>>();
    let mut output_quantity = 0;

    loop {
        for (i, ingredient) in ingredients.iter_mut().enumerate() {
            if ingredient.quantity < recipe_ingredients[i].quantity {
                for i in 0..i {
                    used_ingredients[i].quantity -= recipe_ingredients[i].quantity;
                }

                return Some((output_item, output_quantity, used_ingredients));
            }
            
            ingredient.quantity -= recipe_ingredients[i].quantity;
            used_ingredients[i].quantity += recipe_ingredients[i].quantity;
        }

        output_quantity += recipe.output_quantity;  
    }
}

pub const ALL_RECIPES: &[Recipe] = &[
    Recipe {
        output: "bread",
        output_quantity: 1,
        ingredients: &[Ingredient::new("wheat", 9), Ingredient::new("milk", 1)],
    },
    Recipe {
        output: "slice_of_bread",
        output_quantity: 6,
        ingredients: &[Ingredient::new("knife", 1), Ingredient::new("bread", 1)],
    },
    Recipe {
        output: "fried_egg",
        output_quantity: 1,
        ingredients: &[Ingredient::new("egg", 1), Ingredient::new("salt", 1)],
    },
    Recipe {
        output: "chocolate_milk",
        output_quantity: 1,
        ingredients: &[Ingredient::new("chocolate", 1), Ingredient::new("milk", 1)],
    },
    Recipe {
        output: "popcorn",
        output_quantity: 1,
        ingredients: &[Ingredient::new("corn", 1), Ingredient::new("salt", 1)],
    },
    Recipe {
        output: "orange_juice",
        output_quantity: 1,
        ingredients: &[
            Ingredient::new("water", 1),
            Ingredient::new("orange", 1),
            Ingredient::new("sugar", 1),
        ],
    },
    Recipe {
        output: "plank",
        output_quantity: 3,
        ingredients: &[Ingredient::new("knife", 1), Ingredient::new("raw_trunk", 1)],
    },
    Recipe {
        output: "tool_handle",
        output_quantity: 1,
        ingredients: &[
            Ingredient::new("knife", 1),
            Ingredient::new("stick", 1),
            Ingredient::new("plank", 1),
        ],
    },
    Recipe {
        output: "coal",
        output_quantity: 1,
        ingredients: &[Ingredient::new("knife", 1), Ingredient::new("coal_ore", 2)],
    },
    Recipe {
        output: "iron_bar",
        output_quantity: 1,
        ingredients: &[
            Ingredient::new("knife", 1),
            Ingredient::new("iron_ore", 2),
            Ingredient::new("coal", 1),
        ],
    },
    Recipe {
        output: "copper_bar",
        output_quantity: 1,
        ingredients: &[
            Ingredient::new("knife", 1),
            Ingredient::new("copper_ore", 2),
            Ingredient::new("coal", 1),
        ],
    },
    Recipe {
        output: "silver_bar",
        output_quantity: 1,
        ingredients: &[
            Ingredient::new("knife", 1),
            Ingredient::new("tin_ore", 1),
            Ingredient::new("lead_ore", 1),
            Ingredient::new("coal", 1),
        ],
    },
    Recipe {
        output: "gold_bar",
        output_quantity: 1,
        ingredients: &[
            Ingredient::new("knife", 1),
            Ingredient::new("gold_ore", 2),
            Ingredient::new("coal", 1),
        ],
    },
    Recipe {
        output: "pickaxe",
        output_quantity: 1,
        ingredients: &[
            Ingredient::new("tool_handle", 1),
            Ingredient::new("iron_bar", 1),
        ],
    },
    Recipe {
        output: "shovel",
        output_quantity: 1,
        ingredients: &[
            Ingredient::new("tool_handle", 1),
            Ingredient::new("iron_bar", 1),
            Ingredient::new("stone", 1),
        ],
    },
    Recipe {
        output: "axe",
        output_quantity: 1,
        ingredients: &[
            Ingredient::new("tool_handle", 1),
            Ingredient::new("silver_bar", 1),
        ],
    },
    Recipe {
        output: "hammer",
        output_quantity: 1,
        ingredients: &[
            Ingredient::new("tool_handle", 1),
            Ingredient::new("iron_bar", 1),
            Ingredient::new("copper_bar", 1),
        ],
    },
];
