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
