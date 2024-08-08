use etheris_common::{Color, Identifiable};
use serde::{Deserialize, Serialize};

use crate::personality::Personality;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PactRarity {
    Common,
    Uncommon,
    Rare,
    VeryRare,
    Epic,
    Legendary,
    Mythic,
}

impl PactRarity {
    pub fn weight(&self) -> u32 {
        match self {
            Self::Common => 100,
            Self::Uncommon => 75,
            Self::Rare => 50,
            Self::VeryRare => 30,
            Self::Epic => 10,
            Self::Legendary => 5,
            Self::Mythic => 1,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Self::Common => Color::BLUE,
            Self::Uncommon => Color::BLURPLE,
            Self::Rare => Color::ORANGE,
            Self::VeryRare => Color::DARK_ORANGE,
            Self::Epic => Color::PURPLE,
            Self::Legendary => Color::YELLOW,
            Self::Mythic => Color::RED,
        }
    }

    pub const fn name(&self) -> &'static str {
        match self {
            Self::Common => "Comum",
            Self::Uncommon => "Incomum",
            Self::Rare => "Raro",
            Self::VeryRare => "Muito Raro",
            Self::Epic => "Épico",
            Self::Legendary => "Lendário",
            Self::Mythic => "Mítico",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum PactKind {
    Solidity,
    Hercules,
    Athena,
    Alchemist,
    Metis,
    Consistency,
    Phoenix,
    Hunter,
    FallenAngel,
    Coward,
    Ares,
    Insatiable,
    Rock,
    Courage,
    Thoth,
    Unshakable,
    Apollo,
}

impl PactKind {
    pub fn list() -> Vec<PactKind> {
        vec![
            Self::Solidity,
            Self::Hercules,
            Self::Athena,
            Self::Alchemist,
            Self::Metis,
            Self::Consistency,
            Self::Phoenix,
            Self::Hunter,
            Self::FallenAngel,
            Self::Coward,
            Self::Ares,
            Self::Insatiable,
            Self::Rock,
            Self::Courage,
            Self::Thoth,
            Self::Unshakable,
            Self::Apollo,
        ]
    }

    pub fn personalities_affinity(&self) -> &'static [Personality] {
        match self {
            Self::Solidity => &[Personality::Cowardice],
            Self::Hercules => &[Personality::Courage],
            Self::Athena => &[Personality::Intelligence],
            Self::Alchemist => &[Personality::Arrogance, Personality::Intelligence],
            Self::Metis => &[Personality::Calm],
            Self::Consistency => &[Personality::Insanity],
            Self::Phoenix => &[Personality::Arrogance],
            Self::Hunter => &[Personality::Courage],
            Self::FallenAngel => &[Personality::Courage],
            Self::Coward => &[Personality::Cowardice],
            Self::Ares => &[Personality::Insanity],
            Self::Insatiable => &[Personality::Aggressiveness],
            Self::Rock => &[Personality::Courage],
            Self::Courage => &[Personality::Courage],
            Self::Thoth => &[Personality::Calm],
            Self::Unshakable => &[Personality::Arrogance],
            Self::Apollo => &[Personality::Intelligence],
        }
    }

    pub fn rarity(&self) -> PactRarity {
        match self {
            Self::Solidity => PactRarity::Common,
            Self::Hercules => PactRarity::Common,
            Self::Athena => PactRarity::Common,
            Self::Rock => PactRarity::Common,
            Self::Alchemist => PactRarity::Uncommon,
            Self::Metis => PactRarity::Uncommon,
            Self::Thoth => PactRarity::Uncommon,
            Self::Consistency => PactRarity::Rare,
            Self::Apollo => PactRarity::Rare,
            Self::Hunter => PactRarity::Rare,
            Self::Courage => PactRarity::Rare,
            Self::Coward => PactRarity::VeryRare,
            Self::Unshakable => PactRarity::VeryRare,
            Self::Phoenix => PactRarity::Epic,
            Self::FallenAngel => PactRarity::Epic,
            Self::Ares => PactRarity::Epic,
            Self::Insatiable => PactRarity::Legendary,
        }
    }
}

impl Identifiable for PactKind {
    fn identifier(&self) -> String {
        format!("{:?}", self)
    }
}
