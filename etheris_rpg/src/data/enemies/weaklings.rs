use etheris_data::items;

use super::*;

make_enemy!(
    giant_rat,
    Enemy {
        identifier: "giant_rat",
        name: "Rato Gigante",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        regions: &[(WorldRegion::Greenagis, 4),],
        personalities: &[Personality::Cowardice],
        strength: 2,
        intelligence: 1,
        resistance: 60,
        vitality: 80,
        ether: 15,
        weapon: None,
        allies: None,
        skills: vec![SkillKind::Bite],
        drop: EnemyReward {
            orbs: (10, 15),
            xp: (20, 40),
            items: vec![EnemyRewardItem {
                amount: (1, 1),
                item: items::material::PAPER,
                probability: Probability::new(30),
            }],
        },
    }
);

make_enemy!(
    greenagis_mutant,
    Enemy {
        identifier: "greenagis_mutant",
        name: "Mutante de Greenagis",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        regions: &[(WorldRegion::Greenagis, 3),],
        personalities: &[Personality::Cowardice],
        strength: 2,
        intelligence: 3,
        resistance: 90,
        vitality: 130,
        ether: 20,
        weapon: None,
        allies: None,
        skills: vec![SkillKind::ImbuedPunch, SkillKind::Charge],
        drop: EnemyReward {
            orbs: (15, 25),
            xp: (30, 45),
            items: vec![EnemyRewardItem {
                amount: (1, 1),
                item: items::tool::SHOVEL,
                probability: Probability::new(10),
            }],
        },
    }
);

make_enemy!(
    beginner_looter,
    Enemy {
        identifier: "beginner_looter",
        name: "Saqueador Iniciante",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        regions: &[
            (WorldRegion::Greenagis, 1),
            (WorldRegion::Emerelis, 4),
            (WorldRegion::Gloomwood, 3),
        ],
        personalities: &[Personality::Aggressiveness, Personality::Arrogance],
        strength: 5,
        intelligence: 2,
        resistance: 110,
        vitality: 180,
        ether: 25,
        weapon: Some(WeaponKind::Knife),
        allies: None,
        skills: vec![SkillKind::TornadoKick, SkillKind::DefensiveJump],
        drop: EnemyReward {
            orbs: (15, 30),
            xp: (30, 45),
            items: vec![
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::tool::SHOVEL,
                    probability: Probability::new(5),
                },
                EnemyRewardItem {
                    amount: (1, 2),
                    item: items::material::KNIFE,
                    probability: Probability::new(15),
                }
            ],
        },
    }
);

make_enemy!(
    weak_shredder,
    Enemy {
        identifier: "weak_shredder",
        name: "Retalhador Fraco",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        regions: &[(WorldRegion::Gloomwood, 1), (WorldRegion::Mudland, 1)],
        personalities: &[Personality::Cowardice, Personality::Aggressiveness],
        strength: 15,
        intelligence: 6,
        resistance: 230,
        vitality: 390,
        ether: 30,
        weapon: Some(WeaponKind::Knife),
        allies: None,
        skills: vec![
            SkillKind::SimpleCut,
            SkillKind::BloodTheft,
            SkillKind::WaterBlessing,
            SkillKind::Suplex
        ],
        drop: EnemyReward {
            orbs: (30, 80),
            xp: (50, 60),
            items: vec![
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::material::KNIFE,
                    probability: Probability::new(100),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::special::TRAP,
                    probability: Probability::new(2),
                }
            ],
        }
    }
);
