use etheris_data::items;

use super::*;

make_enemy!(
    average_looter,
    Enemy {
        identifier: "average_looter",
        name: "Saqueador",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        regions: &[(WorldRegion::Tenypt, 6), (WorldRegion::Mudland, 3)],
        personalities: &[
            Personality::Aggressiveness,
            Personality::Intelligence,
            Personality::Cowardice,
        ],
        potential: EnemyPotential::High,
        strength: 20,
        intelligence: 7,
        resistance: 421,
        vitality: 133,
        ether: 40,
        weapon: Some(WeaponKind::Knife),
        allies: None,
        skills: vec![
            SkillKind::SimpleCut,
            SkillKind::FlamingBall,
            SkillKind::TornadoKick,
            SkillKind::Charge,
            SkillKind::Suplex,
        ],
        drop: EnemyReward {
            orbs: (30, 50),
            xp: (50, 80),
            items: vec![
                EnemyRewardItem {
                    amount: (1, 2),
                    item: items::material::KNIFE,
                    probability: Probability::new(90),
                },
                EnemyRewardItem {
                    amount: (3, 5),
                    item: items::material::PAPER,
                    probability: Probability::new(70),
                },
                EnemyRewardItem {
                    amount: (1, 5),
                    item: items::material::STICK,
                    probability: Probability::new(40),
                },
            ],
        },
    }
);

make_enemy!(
    marsh_marauder,
    Enemy {
        identifier: "marsh_marauder",
        name: "Saqueador do Pântano",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        regions: &[(WorldRegion::Murkswamp, 8)],
        personalities: &[
            Personality::Arrogance,
            Personality::Courage,
            Personality::Aggressiveness,
        ],
        potential: EnemyPotential::Medium,
        strength: 23,
        intelligence: 8,
        resistance: 100,
        vitality: 30,
        ether: 60,
        weapon: Some(WeaponKind::Spear),
        allies: Some(vec![(Probability::new(40), Box::new(average_looter()))]),
        skills: vec![
            SkillKind::SimpleCut,
            SkillKind::CyclonePush,
            SkillKind::ElectricSlap,
            SkillKind::WaterJet,
        ],
        drop: EnemyReward {
            orbs: (50, 70),
            xp: (50, 80),
            items: vec![EnemyRewardItem {
                amount: (1, 1),
                item: items::tool::SPEAR,
                probability: Probability::new(30),
            }],
        },
    }
);
