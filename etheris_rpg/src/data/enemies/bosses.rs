use etheris_data::items;
use weaklings::giant_rat;

use crate::ImmunityKind;

use super::*;

make_enemy!(
    garhyan,
    Enemy {
        identifier: "garhyan",
        name: BossKind::Garhyan.name(),
        base_probability: Probability::NEVER,
        brain: BrainKind::Boss,
        boss: Some(BossKind::Garhyan),
        regions: &[],
        personalities: &[
            Personality::Cowardice,
            Personality::Aggressiveness,
            Personality::Arrogance
        ],
        potential: EnemyPotential::High,
        immunities: BodyImmunities::new()
            .with_resistance(ImmunityKind::Poison, 1.0)
            .with_resistance(ImmunityKind::Ice, 0.4)
            .with_resistance(ImmunityKind::Physical, 0.2)
            .with_little_weakness(ImmunityKind::Cut),
        strength: 28,
        intelligence: 52,
        resistance: 1280,
        vitality: 840,
        ether: 50,
        weapon: None,
        allies: Some(vec![
            (Probability::new(100), Box::new(giant_rat())),
            (Probability::new(100), Box::new(giant_rat())),
        ]),
        skills: vec![
            SkillKind::Intimidation,
            SkillKind::BloodTheft,
            SkillKind::Pyrotransmutation,
            SkillKind::Overcoming,
            SkillKind::Suplex,
            SkillKind::Refresh,
            SkillKind::GarhyanRatSummon,
        ],
        drop: EnemyReward {
            orbs: (300, 500),
            xp: (200, 300),
            items: vec![
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::special::INVIGORATING_CRYSTAL,
                    probability: Probability::new(100),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::special::INTELLIGENCE_CRYSTAL,
                    probability: Probability::new(25),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::special::GIFT,
                    probability: Probability::new(100),
                }
            ],
        },
    }
);

make_enemy!(
    agorath,
    Enemy {
        identifier: "agorath",
        name: BossKind::Agorath.name(),
        base_probability: Probability::NEVER,
        brain: BrainKind::Boss,
        boss: Some(BossKind::Agorath),
        regions: &[],
        personalities: &[Personality::Aggressiveness, Personality::Courage,],
        potential: EnemyPotential::High,
        immunities: BodyImmunities::new()
            .with_resistance(ImmunityKind::Physical, 0.5)
            .with_resistance(ImmunityKind::Cut, 0.3)
            .with_little_weakness(ImmunityKind::Electric),
        strength: 70,
        intelligence: 39,
        resistance: 2804,
        vitality: 1484,
        ether: 50,
        weapon: None,
        allies: None,
        skills: vec![
            SkillKind::AgorathForcedDuel,
            SkillKind::Intimidation,
            SkillKind::Overcoming,
            SkillKind::Charge,
            SkillKind::TornadoKick,
        ],
        drop: EnemyReward {
            orbs: (500, 800),
            xp: (250, 400),
            items: vec![
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::special::INVIGORATING_CRYSTAL,
                    probability: Probability::new(100),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::special::INTELLIGENCE_CRYSTAL,
                    probability: Probability::new(25),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::special::GIFT,
                    probability: Probability::new(100),
                }
            ],
        },
    }
);

make_enemy!(
    orsinium,
    Enemy {
        identifier: "orsinium",
        name: BossKind::Orsinium.name(),
        base_probability: Probability::NEVER,
        brain: BrainKind::Boss,
        boss: Some(BossKind::Orsinium),
        regions: &[],
        personalities: &[Personality::Courage, Personality::Calm],
        potential: EnemyPotential::High,
        immunities: BodyImmunities::new()
            .with_resistance(ImmunityKind::Physical, 0.8)
            .with_resistance(ImmunityKind::Electric, 0.6)
            .with_resistance(ImmunityKind::Bleeding, 0.8)
            .with_little_weakness(ImmunityKind::Water),
        strength: 112,
        intelligence: 25,
        resistance: 6934,
        vitality: 5800,
        ether: 60,
        weapon: None,
        allies: None,
        skills: vec![
            SkillKind::Refresh,
            SkillKind::InstinctiveReaction,
            SkillKind::FirePunch,
            SkillKind::CursedBlood,
            SkillKind::IcyShot,
        ],
        drop: EnemyReward {
            orbs: (500, 800),
            xp: (250, 400),
            items: vec![
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::special::INVIGORATING_CRYSTAL,
                    probability: Probability::new(100),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::special::INTELLIGENCE_CRYSTAL,
                    probability: Probability::new(25),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::special::GIFT,
                    probability: Probability::new(100),
                }
            ],
        },
    }
);

make_enemy!(
    ethria,
    Enemy {
        identifier: "ethria",
        name: BossKind::Ethria.name(),
        base_probability: Probability::NEVER,
        brain: BrainKind::Boss,
        boss: Some(BossKind::Ethria),
        regions: &[],
        personalities: &[Personality::Arrogance, Personality::Intelligence],
        potential: EnemyPotential::High,
        immunities: BodyImmunities::new()
            .with_resistance(ImmunityKind::Cut, 0.4)
            .with_resistance(ImmunityKind::Ice, 0.2)
            .with_little_weakness(ImmunityKind::Electric),
        strength: 70,
        intelligence: 180,
        resistance: 4085,
        vitality: 3951,
        ether: 150,
        weapon: Some(WeaponKind::EthriaKatana),
        allies: None,
        skills: vec![
            SkillKind::EthriaAdaptation,
            SkillKind::CursedBlood,
            SkillKind::AtomicHollow,
            SkillKind::Hakikotenchou,
            SkillKind::EtherFlow,
            SkillKind::FlamingBall,
            SkillKind::Suplex,
            SkillKind::BloodDonation,
        ],
        drop: EnemyReward {
            orbs: (500, 800),
            xp: (250, 400),
            items: vec![
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::special::INVIGORATING_CRYSTAL,
                    probability: Probability::new(100),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::special::INTELLIGENCE_CRYSTAL,
                    probability: Probability::new(50),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::special::GIFT,
                    probability: Probability::new(100),
                }
            ],
        },
    }
);

make_enemy!(
    microlord_diabolius,
    Enemy {
        identifier: "microlord_diabolius",
        name: BossKind::MicrolordDiabolius.name(),
        base_probability: Probability::NEVER,
        brain: BrainKind::Boss,
        boss: Some(BossKind::MicrolordDiabolius),
        regions: &[],
        personalities: &[Personality::Cowardice, Personality::Arrogance],
        potential: EnemyPotential::Medium,
        immunities: BodyImmunities::new()
            .with_resistance(ImmunityKind::Water, 1.0)
            .with_resistance(ImmunityKind::Electric, 1.0)
            .with_resistance(ImmunityKind::Ice, 0.8)
            .with_little_weakness(ImmunityKind::Cut)
            .with_little_weakness(ImmunityKind::Physical),
        strength: 28,
        intelligence: 160,
        resistance: 2931,
        vitality: 421,
        ether: 150,
        weapon: None,
        allies: None,
        skills: vec![
            SkillKind::CursedBlood,
            SkillKind::WoundHealing,
            SkillKind::Refresh,
            SkillKind::EtherFlow,
            SkillKind::YinYang,
            SkillKind::WaterBlessing,
            SkillKind::WaterJet,
            SkillKind::ResplendentPunch,
        ],
        drop: EnemyReward {
            orbs: (300, 500),
            xp: (150, 300),
            items: vec![
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::special::INVIGORATING_CRYSTAL,
                    probability: Probability::new(100),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::special::GIFT,
                    probability: Probability::new(100),
                }
            ],
        },
    }
);

make_enemy!(
    macrolord_vastorrant,
    Enemy {
        identifier: "macrolord_vastorrant",
        name: BossKind::MacrolordVastorrant.name(),
        base_probability: Probability::NEVER,
        brain: BrainKind::Boss,
        boss: Some(BossKind::MacrolordVastorrant),
        regions: &[],
        personalities: &[Personality::Insanity, Personality::Arrogance],
        potential: EnemyPotential::VeryHigh,
        immunities: BodyImmunities::new()
            .with_resistance(ImmunityKind::Electric, 1.0)
            .with_resistance(ImmunityKind::Fire, 0.6)
            .with_little_resistance(ImmunityKind::Cut)
            .with_little_resistance(ImmunityKind::Physical)
            .with_little_weakness(ImmunityKind::Ice),
        strength: 190,
        intelligence: 115,
        resistance: 1750,
        vitality: 1199,
        ether: 80,
        weapon: None,
        allies: None,
        skills: vec![
            SkillKind::Earthquake,
            SkillKind::ResplendentPunch,
            SkillKind::BloodSpear,
            SkillKind::ElectricSlap,
            SkillKind::WaterJet,
            SkillKind::Charge,
            SkillKind::Intimidation,
            SkillKind::DefensiveJump,
            SkillKind::InstinctiveReaction,
            SkillKind::Suplex,
            SkillKind::AtomicHollow,
        ],
        drop: EnemyReward {
            orbs: (700, 1000),
            xp: (40, 500),
            items: vec![
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::special::INTELLIGENCE_CRYSTAL,
                    probability: Probability::new(100),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::special::TRAP,
                    probability: Probability::new(100),
                }
            ],
        },
    }
);
