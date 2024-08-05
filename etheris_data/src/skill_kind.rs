use etheris_common::Identifiable;
use serde::{Deserialize, Serialize};

use crate::{personality::Personality, BrainKind};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct Soul {
    pub name: String,
    pub brain: Option<BrainKind>,
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
    Debug,
    // Boss Skills
    AgorathForcedDuel,
    GarhyanRatSummon,
    EthriaAdaptation,
    // Normal Skills
    ImbuedPunch,
    SimpleCut,
    DefensiveJump,
    TornadoKick,
    Bite,
    MirrorDamage,
    ElectricSlap,
    Intimidation,
    CyclonePush,
    Suplex,
    BloodTheft,
    InstinctiveReaction,
    Overcoming,
    FirePunch,
    Charge,
    FlamingBall,
    IcyShot,
    IcyBreath,
    WaterJet,
    BloodSpear,
    WaterBlessing,
    Refresh,
    ResplendentPunch,
    Earthquake,
    BloodDonation,
    WoundHealing,
    TenkuKikan(Option<Soul>),
    YinYang,
    AtomicHollow,
    ParalyzingBet,
    FinalCrucifix,
    EtherShadow,
    CursedBlood,
    Hakikotenchou,
    SkillMirror,
    EtherFlow,
    Pyrotransmutation,
    PoisonousGas,
    ThermalFists,
    DeepCut,
    ElectricalDischarge,
}

impl SkillKind {
    pub fn list() -> Vec<SkillKind> {
        vec![
            Self::Debug,
            Self::AgorathForcedDuel,
            Self::GarhyanRatSummon,
            Self::EthriaAdaptation,
            Self::ImbuedPunch,
            Self::SimpleCut,
            Self::DefensiveJump,
            Self::TornadoKick,
            Self::Bite,
            Self::MirrorDamage,
            Self::BloodTheft,
            Self::InstinctiveReaction,
            Self::Overcoming,
            Self::CyclonePush,
            Self::Suplex,
            Self::FirePunch,
            Self::Charge,
            Self::ResplendentPunch,
            Self::Earthquake,
            Self::ElectricSlap,
            Self::Intimidation,
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
            Self::FinalCrucifix,
            Self::EtherShadow,
            Self::AtomicHollow,
            Self::CursedBlood,
            Self::Hakikotenchou,
            Self::SkillMirror,
            Self::EtherFlow,
            Self::Pyrotransmutation,
            Self::PoisonousGas,
            Self::ThermalFists,
            Self::DeepCut,
            Self::ElectricalDischarge,
        ]
    }

    pub fn personalities_affinity(&self) -> &'static [Personality] {
        match self {
            Self::Debug => &[],
            Self::AgorathForcedDuel => &[Personality::Courage, Personality::Cowardice],
            Self::GarhyanRatSummon => &[Personality::Arrogance, Personality::Cowardice],
            Self::EthriaAdaptation => &[Personality::Courage, Personality::Insanity],
            Self::ImbuedPunch => &Personality::LIST,
            Self::SimpleCut => &[Personality::Cowardice, Personality::Arrogance],
            Self::DefensiveJump => &[Personality::Intelligence, Personality::Cowardice],
            Self::TornadoKick => &[Personality::Aggressiveness],
            Self::Bite => &[Personality::Aggressiveness, Personality::Insanity],
            Self::Charge => &[Personality::Insanity, Personality::Calm],
            Self::MirrorDamage => &[Personality::Arrogance],
            Self::ElectricSlap => &[Personality::Insanity, Personality::Aggressiveness],
            Self::Intimidation => &[Personality::Courage],
            Self::BloodTheft => &[Personality::Arrogance, Personality::Insanity],
            Self::Suplex => &[Personality::Courage, Personality::Arrogance],
            Self::FirePunch => &[Personality::Aggressiveness, Personality::Arrogance],
            Self::IcyBreath => &[
                Personality::Calm,
                Personality::Intelligence,
                Personality::Cowardice,
            ],
            Self::IcyShot => &[Personality::Aggressiveness, Personality::Cowardice],
            Self::WaterBlessing => &[Personality::Calm],
            Self::Refresh => &[Personality::Cowardice],
            Self::InstinctiveReaction => &[Personality::Calm, Personality::Arrogance],
            Self::Overcoming => &[Personality::Courage],
            Self::CyclonePush => &[Personality::Courage, Personality::Intelligence],
            Self::FlamingBall => &[Personality::Aggressiveness],
            Self::Earthquake => &[Personality::Insanity, Personality::Aggressiveness],
            Self::WaterJet => &[Personality::Intelligence, Personality::Cowardice],
            Self::BloodSpear => &[Personality::Arrogance, Personality::Insanity],
            Self::ResplendentPunch => &[Personality::Courage],
            Self::BloodDonation => &[Personality::Courage],
            Self::WoundHealing => &[Personality::Intelligence, Personality::Calm],
            Self::YinYang => &[Personality::Calm, Personality::Intelligence],
            Self::ParalyzingBet => &[Personality::Insanity],
            Self::FinalCrucifix => &[Personality::Insanity, Personality::Courage],
            Self::EtherShadow => &[
                Personality::Calm,
                Personality::Intelligence,
                Personality::Cowardice,
            ],
            Self::AtomicHollow => &[Personality::Arrogance],
            Self::TenkuKikan(_) => &[Personality::Arrogance],
            Self::CursedBlood => &[Personality::Courage],
            Self::Hakikotenchou => &[Personality::Calm, Personality::Intelligence],
            Self::SkillMirror => &[Personality::Insanity, Personality::Cowardice],
            Self::EtherFlow => &[Personality::Calm],
            Self::Pyrotransmutation => &[Personality::Aggressiveness, Personality::Cowardice],
            Self::PoisonousGas => &[Personality::Aggressiveness, Personality::Cowardice],
            Self::ThermalFists => &[Personality::Arrogance, Personality::Intelligence],
            Self::DeepCut => &[Personality::Insanity, Personality::Arrogance],
            Self::ElectricalDischarge => &[Personality::Cowardice, Personality::Aggressiveness],
        }
    }

    pub fn intelligence_requirement(&self) -> u32 {
        match self {
            Self::ImbuedPunch => 0,
            Self::CyclonePush => 1,
            Self::DefensiveJump => 2,
            Self::Bite => 3,
            Self::SimpleCut => 4,
            Self::Charge => 5,
            Self::TornadoKick => 6,
            Self::Suplex => 7,
            Self::ElectricSlap => 8,
            Self::FirePunch => 8,
            Self::MirrorDamage => 9,
            Self::IcyBreath => 10,
            Self::Intimidation => 11,
            Self::IcyShot => 12,
            Self::PoisonousGas => 12,
            Self::BloodDonation => 12,
            Self::WaterBlessing => 13,
            Self::EtherFlow => 13,
            Self::BloodTheft => 14,
            Self::InstinctiveReaction => 14,
            Self::Overcoming => 15,
            Self::ResplendentPunch => 16,
            Self::Refresh => 16,
            Self::WaterJet => 17,
            Self::FlamingBall => 20,
            Self::Pyrotransmutation => 21,
            Self::Earthquake => 22,
            Self::WoundHealing => 24,
            Self::BloodSpear => 25,
            Self::ThermalFists => 30,
            Self::ElectricalDischarge => 33,
            Self::CursedBlood => 35,
            Self::ParalyzingBet => 40,
            Self::EtherShadow => 45,
            Self::DeepCut => 50,
            Self::SkillMirror => 55,
            Self::AtomicHollow => 60,
            Self::YinYang => 70,
            Self::TenkuKikan(_) => 80,
            Self::FinalCrucifix => 130,
            Self::Hakikotenchou => 90,

            Self::GarhyanRatSummon => u32::MAX,
            Self::AgorathForcedDuel => u32::MAX,
            Self::EthriaAdaptation => u32::MAX,
            Self::Debug => u32::MAX,
        }
    }

    pub fn knowledge_cost(&self) -> u32 {
        match self {
            Self::ImbuedPunch
            | Self::SimpleCut
            | Self::CyclonePush
            | Self::TornadoKick
            | Self::Bite
            | Self::Intimidation
            | Self::Debug
            | Self::DefensiveJump => 1,
            Self::Charge
            | Self::IcyBreath
            | Self::MirrorDamage
            | Self::FirePunch
            | Self::Suplex
            | Self::BloodDonation
            | Self::Refresh
            | Self::WoundHealing
            | Self::ResplendentPunch
            | Self::BloodSpear
            | Self::EtherFlow
            | Self::PoisonousGas => 2,
            Self::IcyShot
            | Self::ElectricSlap
            | Self::WaterBlessing
            | Self::InstinctiveReaction
            | Self::Earthquake
            | Self::AtomicHollow
            | Self::CursedBlood
            | Self::BloodTheft
            | Self::Pyrotransmutation
            | Self::Overcoming
            | Self::GarhyanRatSummon
            | Self::ThermalFists
            | Self::DeepCut
            | Self::ElectricalDischarge => 3,
            Self::WaterJet | Self::FlamingBall | Self::EtherShadow | Self::SkillMirror => 4,
            Self::YinYang | Self::Hakikotenchou => 5,
            Self::TenkuKikan(..) => 6,
            Self::ParalyzingBet | Self::AgorathForcedDuel => 6,
            Self::FinalCrucifix | Self::EthriaAdaptation => 7,
        }
    }
}

impl Identifiable for SkillKind {
    fn identifier(&self) -> String {
        format!("{:?}", self)
    }
}

#[test]
fn count_personalities_affinities() {
    use std::collections::HashMap;

    let mut map: HashMap<Personality, u32> = HashMap::new();

    for skill in SkillKind::list() {
        for personality in Personality::LIST {
            if skill.personalities_affinity().contains(&personality) {
                let entry = map.entry(personality).or_insert(0);
                *entry += 1;
            }
        }
    }

    for (personality, count) in map {
        println!("{personality} has affinity with {count} skills");
    }
}
