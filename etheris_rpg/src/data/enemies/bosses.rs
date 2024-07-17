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
        resistance: 680,
        vitality: 180,
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
                    item: items::special::GIFT,
                    probability: Probability::new(100),
                }
            ],
        },
    }
);
