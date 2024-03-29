use std::fmt::Display;

use etheris_macros::List;
use serde::{Deserialize, Serialize};

use crate::SkillKind;

#[derive(
    List, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub enum Personality {
    Calm,
    Courage,
    Cowardice,
    Aggressiveness,
    Arrogance,
    Intelligence,
    Insanity,
}

impl Display for Personality {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Calm => f.write_str("Calma"),
            Self::Courage => f.write_str("Coragem"),
            Self::Cowardice => f.write_str("Covardia"),
            Self::Aggressiveness => f.write_str("Agressividade"),
            Self::Arrogance => f.write_str("Arrogância"),
            Self::Intelligence => f.write_str("Inteligência"),
            Self::Insanity => f.write_str("Insanidade"),
        }
    }
}

impl Personality {
    pub fn exclusive_personality(&self) -> &'static [Personality] {
        const EMPTY: &[Personality] = &[];

        match self {
            Self::Courage => &[Self::Cowardice],
            Self::Cowardice => &[Self::Courage],

            Self::Aggressiveness => &[Self::Calm],
            Self::Calm => &[Self::Aggressiveness],

            Self::Insanity => &[Self::Intelligence],
            Self::Intelligence => &[Self::Insanity],

            _ => EMPTY,
        }
    }

    pub fn initial_learnable_skills(&self) -> Vec<SkillKind> {
        match self {
            Self::Calm => vec![SkillKind::CyclonePush],
            Self::Courage => vec![SkillKind::Charge],
            Self::Cowardice => vec![SkillKind::MirrorDamage],
            Self::Aggressiveness => vec![SkillKind::TornadoKick],
            Self::Arrogance => vec![SkillKind::SimpleCut],
            Self::Intelligence => vec![SkillKind::CyclonePush],
            Self::Insanity => vec![SkillKind::Bite],
        }
    }
}
