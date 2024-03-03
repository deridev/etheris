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
    InstinctiveReaction,
    FirePunch,
    Charge,
    FlamingBall,
    IcyShot,
    IcyBreath,
    WaterJet,
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
}

impl SkillKind {
    pub fn list() -> Vec<SkillKind> {
        vec![
            Self::ImbuedPunch,
            Self::SimpleCut,
            Self::DefensiveJump,
            Self::TornadoKick,
            Self::Bite,
            Self::MirrorDamage,
            Self::InstinctiveReaction,
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
        ]
    }

    pub fn personalities_affinity(&self) -> &'static [Personality] {
        match self {
            Self::ImbuedPunch => &Personality::LIST,
            Self::SimpleCut => &[Personality::Cowardice, Personality::Insanity],
            Self::DefensiveJump => &[Personality::Intelligence, Personality::Cowardice],
            Self::TornadoKick => &[Personality::Aggressiveness],
            Self::Bite => &[Personality::Aggressiveness, Personality::Insanity],
            Self::Charge => &[Personality::Insanity, Personality::Calm],
            Self::MirrorDamage => &[Personality::Arrogance],
            Self::ElectricSlap => &[Personality::Insanity, Personality::Aggressiveness],
            Self::Intimidation => &[Personality::Courage],
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
            Self::CyclonePush => &[Personality::Courage, Personality::Intelligence],
            Self::FlamingBall => &[Personality::Aggressiveness],
            Self::Earthquake => &[Personality::Arrogance, Personality::Aggressiveness],
            Self::WaterJet => &[Personality::Intelligence, Personality::Cowardice],
            Self::ResplendentPunch => &[Personality::Insanity, Personality::Courage],
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
            Self::WaterBlessing => 13,
            Self::InstinctiveReaction => 14,
            Self::ResplendentPunch => 15,
            Self::Refresh => 16,
            Self::WaterJet => 17,
            Self::FlamingBall => 20,
            Self::Earthquake => 21,
            Self::WoundHealing => 22,
            Self::BloodDonation => 25,
            Self::YinYang => 30,
            Self::CursedBlood => 35,
            Self::EtherShadow => 45,
            Self::ParalyzingBet => 50,
            Self::AtomicHollow => 55,
            Self::TenkuKikan(_) => 60,
            Self::FinalCrucifix => 70,
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
            | Self::DefensiveJump => 1,
            Self::Charge
            | Self::IcyBreath
            | Self::MirrorDamage
            | Self::FirePunch
            | Self::Suplex
            | Self::BloodDonation
            | Self::Refresh
            | Self::WoundHealing
            | Self::ResplendentPunch => 2,
            Self::IcyShot
            | Self::ElectricSlap
            | Self::WaterBlessing
            | Self::InstinctiveReaction
            | Self::Earthquake
            | Self::AtomicHollow
            | Self::CursedBlood => 3,
            Self::WaterJet | Self::FlamingBall | Self::EtherShadow => 4,
            Self::YinYang => 5,
            Self::TenkuKikan(..) => 6,
            Self::ParalyzingBet => 6,
            Self::FinalCrucifix => 7,
        }
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
