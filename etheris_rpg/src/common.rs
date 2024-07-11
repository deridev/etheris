use etheris_data::{items::Item, ItemValues};

use crate::{skills::Skill, Effect, FighterIndex};

pub type BoxedSkill = Box<dyn Skill + Send + 'static>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct DamageSpecifier {
    pub kind: DamageKind,
    pub amount: i32,
    pub balance_effectiveness: u8,
    pub accuracy: u8,
    pub effect: Option<Effect>,
    pub culprit: FighterIndex,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum DamageKind {
    #[default]
    Physical,
    /// Physical damage with ether envolved
    SpecialPhysical,
    /// Special damage (like bleeding)
    Special,
    /// Wind damage
    Wind,
    /// Fire damage
    Fire,
    /// Ice damage
    Ice,
    /// Electric damage
    Electric,
    /// Water damage
    Water,
    /// Poisonous damage
    Poisonous,
    /// Cut damage made with ether
    Cut,
    /// Physycal cut damage (like knifes)
    PhysicalCut,
}

impl DamageKind {
    pub const fn is_physical(&self) -> bool {
        matches!(
            self,
            Self::PhysicalCut | Self::Physical | Self::SpecialPhysical
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BattleItem {
    pub item: Item,
    pub quantity: usize,
    pub values: ItemValues,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct XpReward {
    pub strength: (i32, i32),
    pub health: (i32, i32),
    pub intelligence: (i32, i32),
    pub knowledge: (i32, i32),
}

impl XpReward {
    pub fn is_empty(&self) -> bool {
        self == &Default::default()
    }
}
