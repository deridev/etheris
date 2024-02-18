use etheris_data::items::Item;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RewardItem {
    pub item: Item,
    pub amount: i32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Reward {
    pub orbs: i32,
    pub xp: i32,
    pub items: Vec<RewardItem>,
}

impl Reward {
    pub fn add(&self, rhs: Reward) -> Self {
        let mut items = self.items.clone();

        for it in rhs.items {
            if let Some(item) = items
                .iter_mut()
                .find(|i| i.item.identifier == it.item.identifier)
            {
                item.amount += it.amount;
            } else {
                items.push(it);
            }
        }

        Self {
            orbs: self.orbs + rhs.orbs,
            xp: self.xp + rhs.xp,
            items,
        }
    }

    pub fn is_empty(&self) -> bool {
        self == &Self::default()
    }
}
