use serde::{Deserialize, Serialize};

use crate::personality::Personality;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Soul {
    pub name: String,
    pub strength: u32,
    pub intelligence: u32,
    pub ether: i32,
    pub vitality: i32,
    pub resistance: i32,
    pub skills: Vec<SkillKind>,
    pub personalities: Vec<Personality>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum SkillKind {
    ImbuedPunch,
    SimpleCut,
    TornadoKick,
    Bite,
    MirrorDamage,
    ElectricSlap,
    CyclonePush,
    Suplex,
    InstinctiveReaction,
    FirePunch,
    Charge,
    FlamingBall,
    IcyShot,
    IcyBreath,
    WaterJet,
    WaterBlessing,
    Refresh,
    BloodDonation,
    WoundHealing,
    TenkuKikan(Option<Soul>),
    YinYang,
    ParalyzingBet,
}

impl SkillKind {
    pub fn list() -> Vec<SkillKind> {
        vec![
            Self::ImbuedPunch,
            Self::SimpleCut,
            Self::TornadoKick,
            Self::Bite,
            Self::MirrorDamage,
            Self::InstinctiveReaction,
            Self::CyclonePush,
            Self::Suplex,
            Self::FirePunch,
            Self::Charge,
            Self::ElectricSlap,
            Self::FlamingBall,
            Self::IcyShot,
            Self::IcyBreath,
            Self::WaterJet,
            Self::WaterBlessing,
            Self::Refresh,
            Self::BloodDonation,
            Self::WoundHealing,
            Self::TenkuKikan(None),
            Self::YinYang,
            Self::ParalyzingBet,
        ]
    }

    pub fn intelligence_requirement(&self) -> u32 {
        match self {
            Self::ImbuedPunch => 0,
            Self::CyclonePush => 1,
            Self::Bite => 2,
            Self::Charge => 3,
            Self::SimpleCut => 4,
            Self::ElectricSlap => 5,
            Self::Suplex => 6,
            Self::TornadoKick => 7,
            Self::FirePunch => 8,
            Self::MirrorDamage => 9,
            Self::IcyBreath => 10,
            Self::IcyShot => 12,
            Self::WaterBlessing => 13,
            Self::InstinctiveReaction => 14,
            Self::Refresh => 15,
            Self::WaterJet => 17,
            Self::FlamingBall => 20,
            Self::WoundHealing => 22,
            Self::BloodDonation => 25,
            Self::YinYang => 30,
            Self::ParalyzingBet => 50,
            Self::TenkuKikan(_) => 60,
        }
    }

    pub fn knowledge_cost(&self) -> u32 {
        match self {
            Self::ImbuedPunch
            | Self::SimpleCut
            | Self::CyclonePush
            | Self::TornadoKick
            | Self::Bite => 1,
            Self::Charge
            | Self::IcyBreath
            | Self::MirrorDamage
            | Self::FirePunch
            | Self::Suplex
            | Self::BloodDonation
            | Self::Refresh
            | Self::WoundHealing => 2,
            Self::IcyShot
            | Self::ElectricSlap
            | Self::WaterBlessing
            | Self::InstinctiveReaction => 3,
            Self::WaterJet | Self::FlamingBall => 4,
            Self::YinYang => 5,
            Self::TenkuKikan(..) => 6,
            Self::ParalyzingBet => 6,
        }
    }
}
