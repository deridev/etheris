use etheris_data::SkillKind;
use once_cell::sync::Lazy;

pub mod prelude;

macro_rules! use_skill {
    ($name:ident) => {
        mod $name;
        pub use $name::*;
    };
}

use_skill!(debug);

use_skill!(imbued_punch);
use_skill!(cyclone_push);
use_skill!(charge);
use_skill!(bite);
use_skill!(tornado_kick);
use_skill!(fire_punch);
use_skill!(simple_cut);
use_skill!(electric_slap);
use_skill!(intimidation);
use_skill!(suplex);
use_skill!(icy_shot);
use_skill!(mirror_damage);
use_skill!(instinctive_reaction);
use_skill!(flaming_ball);
use_skill!(water_jet);
use_skill!(water_blessing);
use_skill!(refresh);
use_skill!(icy_breath);
use_skill!(earthquake);
use_skill!(blood_donation);
use_skill!(wound_healing);
use_skill!(tenku_kikan);
use_skill!(yinyang);
use_skill!(paralyzing_bet);
use_skill!(final_crucifix);
use_skill!(resplendent_punch);
use_skill!(ether_shadow);
use_skill!(atomic_hollow);
use_skill!(cursed_blood);
use_skill!(defensive_jump);
use_skill!(blood_theft);
use_skill!(blood_spear);
use_skill!(overcoming);
use_skill!(hakikotenchou);
use_skill!(skill_mirror);
use_skill!(ether_flow);
use_skill!(pyrotransmutation);
use_skill!(poisonous_gas);
use_skill!(thermal_fists);
use_skill!(deep_cut);
use_skill!(electrical_discharge);

mod specific;
pub use specific::*;

pub static ALL_SKILLS: Lazy<Vec<Box<dyn super::Skill + Send + Sync>>> = Lazy::new(|| {
    SkillKind::list()
        .into_iter()
        .map(get_boxed_skill_from_kind)
        .collect()
});

pub fn get_boxed_skill_from_kind(kind: SkillKind) -> Box<dyn super::Skill + Send + Sync> {
    // PS: for this function to work properly, SkillKind::list() must be CORRECT.

    match kind {
        SkillKind::Debug => Box::<Debug>::default(),
        SkillKind::AgorathForcedDuel => Box::<AgorathForcedDuel>::default(),
        SkillKind::GarhyanRatSummon => Box::<GarhyanRatSummon>::default(),
        SkillKind::EthriaAdaptation => Box::<EthriaAdaptation>::default(),
        SkillKind::ImbuedPunch => Box::<ImbuedPunch>::default(),
        SkillKind::SimpleCut => Box::<SimpleCut>::default(),
        SkillKind::TornadoKick => Box::<TornadoKick>::default(),
        SkillKind::Bite => Box::<Bite>::default(),
        SkillKind::CyclonePush => Box::<CyclonePush>::default(),
        SkillKind::FirePunch => Box::<FirePunch>::default(),
        SkillKind::MirrorDamage => Box::<MirrorDamage>::default(),
        SkillKind::InstinctiveReaction => Box::<InstinctiveReaction>::default(),
        SkillKind::IcyShot => Box::<IcyShot>::default(),
        SkillKind::ElectricSlap => Box::<ElectricSlap>::default(),
        SkillKind::Intimidation => Box::<Intimidation>::default(),
        SkillKind::Suplex => Box::<Suplex>::default(),
        SkillKind::Charge => Box::<Charge>::default(),
        SkillKind::FlamingBall => Box::<FlamingBall>::default(),
        SkillKind::IcyBreath => Box::<IcyBreath>::default(),
        SkillKind::Refresh => Box::<Refresh>::default(),
        SkillKind::ResplendentPunch => Box::<ResplendentPunch>::default(),
        SkillKind::Earthquake => Box::<Earthquake>::default(),
        SkillKind::WaterJet => Box::<WaterJet>::default(),
        SkillKind::BloodDonation => Box::<BloodDonation>::default(),
        SkillKind::WoundHealing => Box::<WoundHealing>::default(),
        SkillKind::WaterBlessing => Box::<WaterBlessing>::default(),
        SkillKind::TenkuKikan(soul) => Box::new(TenkuKikan::new(soul)),
        SkillKind::YinYang => Box::<YinYang>::default(),
        SkillKind::ParalyzingBet => Box::<ParalyzingBet>::default(),
        SkillKind::FinalCrucifix => Box::<FinalCrucifix>::default(),
        SkillKind::EtherShadow => Box::<EtherShadow>::default(),
        SkillKind::AtomicHollow => Box::<AtomicHollow>::default(),
        SkillKind::CursedBlood => Box::<CursedBlood>::default(),
        SkillKind::DefensiveJump => Box::<DefensiveJump>::default(),
        SkillKind::BloodTheft => Box::<BloodTheft>::default(),
        SkillKind::BloodSpear => Box::<BloodSpear>::default(),
        SkillKind::Overcoming => Box::<Overcoming>::default(),
        SkillKind::Hakikotenchou => Box::<Hakikotenchou>::default(),
        SkillKind::SkillMirror => Box::<SkillMirror>::default(),
        SkillKind::EtherFlow => Box::<EtherFlow>::default(),
        SkillKind::Pyrotransmutation => Box::<Pyrotransmutation>::default(),
        SkillKind::PoisonousGas => Box::<PoisonousGas>::default(),
        SkillKind::ThermalFists => Box::<ThermalFists>::default(),
        SkillKind::DeepCut => Box::<DeepCut>::default(),
        SkillKind::ElectricalDischarge => Box::<ElectricalDischarge>::default(),
    }
}
