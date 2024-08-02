use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum WeaponKind {
    Stick,
    Knife,
    Bat,
    Umbrella,
    Spear,
    Katana,
    EthriaKatana,
    ScorpionFang,
}

impl WeaponKind {
    pub fn strength_weight(&self) -> f32 {
        match self {
            Self::Stick => 0.5,
            Self::Knife => 0.8,
            Self::Bat => 1.2,
            Self::Umbrella => 0.7,
            Self::Spear => 0.5,
            Self::Katana => 0.8,
            Self::ScorpionFang => 1.1,
            Self::EthriaKatana => 0.6,
        }
    }

    pub fn intelligence_weight(&self) -> f32 {
        match self {
            Self::Stick => 0.3,
            Self::Knife => 0.1,
            Self::Bat => 0.1,
            Self::Umbrella => 0.3,
            Self::Spear => 0.7,
            Self::Katana => 0.8,
            Self::ScorpionFang => 0.4,
            Self::EthriaKatana => 1.4,
        }
    }
}
