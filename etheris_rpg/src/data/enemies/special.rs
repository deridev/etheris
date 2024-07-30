use etheris_data::items;

use crate::ImmunityKind;

use super::*;

make_enemy!(
    debug,
    Enemy {
        identifier: "debug",
        name: "Debug",
        base_probability: Probability::new(100),
        brain: BrainKind::Simple,
        boss: None,
        regions: &[(WorldRegion::Greenagis, 5)],
        personalities: &[Personality::Cowardice, Personality::Intelligence],
        potential: EnemyPotential::Medium,
        immunities: BodyImmunities::new()
            .with_resistance(ImmunityKind::Physical, 1.0)
            .with_weakness(ImmunityKind::Cut, 2.0),
        strength: 1,
        intelligence: 1,
        resistance: 1000,
        vitality: 500,
        ether: 50,
        weapon: None,
        allies: None,
        skills: vec![],
        drop: EnemyReward {
            orbs: (0, 0),
            xp: (0, 0),
            items: vec![],
        },
    }
);

make_enemy!(
    miniorbs,
    Enemy {
        identifier: "miniorbs",
        name: "Miniorbs da Sorte",
        base_probability: Probability::new(1),
        brain: BrainKind::Simple,
        boss: None,
        regions: &[],
        personalities: &[Personality::Cowardice, Personality::Intelligence],
        potential: EnemyPotential::Medium,
        immunities: BodyImmunities::new()
            .with_little_resistance(ImmunityKind::Physical)
            .with_little_resistance(ImmunityKind::Cut),
        strength: 10,
        intelligence: 20,
        resistance: 450,
        vitality: 50,
        ether: 40,
        weapon: None,
        allies: None,
        skills: vec![
            SkillKind::Charge,
            SkillKind::DefensiveJump,
            SkillKind::TornadoKick,
            SkillKind::ElectricSlap,
        ],
        drop: EnemyReward {
            orbs: (80, 150),
            xp: (100, 150),
            items: vec![
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::special::INVIGORATING_CRYSTAL,
                    probability: Probability::new(50),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::special::GIFT,
                    probability: Probability::new(70),
                }
            ],
        },
    }
);
