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
    desert_raider,
    Enemy {
        identifier: "desert_raider",
        name: "Saqueador do Deserto",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        regions: &[(WorldRegion::Tenypt, 7), (WorldRegion::Sandywater, 5)],
        personalities: &[Personality::Aggressiveness, Personality::Courage],
        potential: EnemyPotential::Medium,
        strength: 18,
        intelligence: 6,
        resistance: 350,
        vitality: 85,
        ether: 30,
        weapon: Some(WeaponKind::Spear),
        allies: None,
        skills: vec![
            SkillKind::SimpleCut,
            SkillKind::TornadoKick,
            SkillKind::CyclonePush,
        ],
        drop: EnemyReward {
            orbs: (30, 50),
            xp: (45, 70),
            items: vec![
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::tool::SPEAR,
                    probability: Probability::new(30),
                },
                EnemyRewardItem {
                    amount: (1, 3),
                    item: items::material::STICK,
                    probability: Probability::new(60),
                },
            ],
        },
    }
);

make_enemy!(
    small_scorpion,
    Enemy {
        identifier: "small_scorpion",
        name: "Escopião Pequeno",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        regions: &[(WorldRegion::Tenypt, 8), (WorldRegion::Sandywater, 2)],
        personalities: &[Personality::Insanity, Personality::Aggressiveness,],
        potential: EnemyPotential::Low,
        strength: 30,
        intelligence: 7,
        resistance: 180,
        vitality: 80,
        ether: 30,
        weapon: Some(WeaponKind::ScorpionFang),
        allies: None,
        skills: vec![
            SkillKind::SimpleCut,
            SkillKind::CyclonePush,
            SkillKind::Charge,
        ],
        drop: EnemyReward {
            orbs: (50, 70),
            xp: (50, 80),
            items: vec![EnemyRewardItem {
                amount: (1, 1),
                item: items::material::SCORPION_FANG,
                probability: Probability::new(10),
            }],
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
        resistance: 200,
        vitality: 60,
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

make_enemy!(
    ice_warrior,
    Enemy {
        identifier: "ice_warrior",
        name: "Guerreiro do Gelo",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        regions: &[(WorldRegion::Icefields, 8)],
        personalities: &[Personality::Courage, Personality::Arrogance],
        potential: EnemyPotential::High,
        strength: 25,
        intelligence: 10,
        resistance: 400,
        vitality: 110,
        ether: 50,
        weapon: Some(WeaponKind::Bat),
        allies: None,
        skills: vec![
            SkillKind::IcyBreath,
            SkillKind::IcyShot,
            SkillKind::ImbuedPunch,
            SkillKind::DefensiveJump,
        ],
        drop: EnemyReward {
            orbs: (40, 90),
            xp: (70, 110),
            items: vec![
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::tool::BAT,
                    probability: Probability::new(50),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::consumable::APPLE,
                    probability: Probability::new(30),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::consumable::WATER,
                    probability: Probability::new(60),
                },
            ],
        },
    }
);

make_enemy!(
    frost_wolf,
    Enemy {
        identifier: "frost_wolf",
        name: "Lobo do Gelo",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        regions: &[(WorldRegion::Icefields, 3), (WorldRegion::Wornpeaks, 1)],
        personalities: &[Personality::Courage],
        potential: EnemyPotential::Medium,
        strength: 17,
        intelligence: 5,
        resistance: 250,
        vitality: 80,
        ether: 30,
        weapon: None,
        allies: None,
        skills: vec![
            SkillKind::Bite,
            SkillKind::IcyBreath,
            SkillKind::DefensiveJump,
        ],
        drop: EnemyReward {
            orbs: (50, 80),
            xp: (30, 60),
            items: vec![
                EnemyRewardItem {
                    amount: (1, 3),
                    item: items::material::BONE,
                    probability: Probability::new(70),
                },
                EnemyRewardItem {
                    amount: (1, 2),
                    item: items::consumable::WATER,
                    probability: Probability::new(50),
                }
            ],
        },
    }
);

make_enemy!(
    abominable_maquiran,
    Enemy {
        identifier: "abominable_maquiran",
        name: "Abominável Ma'Quiran",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        regions: &[(WorldRegion::Icefields, 3)],
        personalities: &[Personality::Courage],
        potential: EnemyPotential::Low,
        strength: 6,
        intelligence: 41,
        resistance: 300,
        vitality: 100,
        ether: 30,
        weapon: None,
        allies: None,
        skills: vec![
            SkillKind::ElectricSlap,
            SkillKind::IcyBreath,
            SkillKind::WaterBlessing,
            SkillKind::Charge,
        ],
        drop: EnemyReward {
            orbs: (30, 90),
            xp: (30, 60),
            items: vec![
                EnemyRewardItem {
                    amount: (1, 2),
                    item: items::consumable::WATER,
                    probability: Probability::new(50),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::consumable::CHOCOLATE_MILK,
                    probability: Probability::new(50),
                }
            ],
        },
    }
);

make_enemy!(
    rocky_golem,
    Enemy {
        identifier: "rocky_golem",
        name: "Golem Rochoso",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        regions: &[(WorldRegion::Wornpeaks, 3)],
        personalities: &[Personality::Aggressiveness, Personality::Insanity],
        potential: EnemyPotential::High,
        strength: 38,
        intelligence: 3,
        resistance: 430,
        vitality: 120,
        ether: 45,
        weapon: None,
        allies: None,
        skills: vec![
            SkillKind::Charge,
            SkillKind::Suplex,
            SkillKind::Intimidation,
            SkillKind::IcyShot,
        ],
        drop: EnemyReward {
            orbs: (40, 100),
            xp: (30, 70),
            items: vec![
                EnemyRewardItem {
                    amount: (1, 3),
                    item: items::material::STONE,
                    probability: Probability::new(90),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::material::RAW_TRUNK,
                    probability: Probability::new(50),
                },
            ],
        },
    }
);
