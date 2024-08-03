use etheris_data::items;
use weaklings::greenagis_mutant;

use crate::ImmunityKind;

use super::*;

make_enemy!(
    average_looter,
    Enemy {
        identifier: "average_looter",
        name: "Saqueador",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        boss: None,
        regions: &[(WorldRegion::Tenypt, 6), (WorldRegion::Mudland, 3)],
        personalities: &[
            Personality::Aggressiveness,
            Personality::Intelligence,
            Personality::Cowardice,
        ],
        potential: EnemyPotential::High,
        immunities: BodyImmunities::new(),
        strength: 40,
        intelligence: 17,
        resistance: 621,
        vitality: 233,
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
        boss: None,
        regions: &[(WorldRegion::Tenypt, 7), (WorldRegion::Sandywater, 5)],
        personalities: &[Personality::Aggressiveness, Personality::Courage],
        potential: EnemyPotential::Medium,
        immunities: BodyImmunities::new().with_little_resistance(ImmunityKind::Poison),
        strength: 32,
        intelligence: 26,
        resistance: 450,
        vitality: 105,
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
        boss: None,
        regions: &[(WorldRegion::Tenypt, 8), (WorldRegion::Sandywater, 2)],
        personalities: &[Personality::Insanity, Personality::Aggressiveness,],
        potential: EnemyPotential::Low,
        immunities: BodyImmunities::new()
            .with_little_weakness(ImmunityKind::Fire)
            .with_resistance(ImmunityKind::Poison, 1.0),
        strength: 30,
        intelligence: 7,
        resistance: 380,
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
        boss: None,
        regions: &[(WorldRegion::Murkswamp, 8)],
        personalities: &[
            Personality::Arrogance,
            Personality::Courage,
            Personality::Aggressiveness,
        ],
        potential: EnemyPotential::Medium,
        immunities: BodyImmunities::new().with_resistance(ImmunityKind::Water, 0.6),
        strength: 23,
        intelligence: 8,
        resistance: 400,
        vitality: 160,
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
        boss: None,
        regions: &[(WorldRegion::Icefields, 8)],
        personalities: &[Personality::Courage, Personality::Arrogance],
        potential: EnemyPotential::High,
        immunities: BodyImmunities::new()
            .with_resistance(ImmunityKind::Ice, 0.9)
            .with_weakness(ImmunityKind::Fire, 0.3),
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
        boss: None,
        regions: &[(WorldRegion::Icefields, 3), (WorldRegion::Wornpeaks, 1)],
        personalities: &[Personality::Courage],
        potential: EnemyPotential::Medium,
        immunities: BodyImmunities::new()
            .with_resistance(ImmunityKind::Ice, 0.5)
            .with_little_resistance(ImmunityKind::Bleeding),
        strength: 17,
        intelligence: 5,
        resistance: 553,
        vitality: 221,
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
        boss: None,
        regions: &[(WorldRegion::Icefields, 3)],
        personalities: &[Personality::Courage],
        potential: EnemyPotential::Low,
        immunities: BodyImmunities::new().with_resistance(ImmunityKind::Electric, 0.5),
        strength: 6,
        intelligence: 68,
        resistance: 616,
        vitality: 299,
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
    trained_thief,
    Enemy {
        identifier: "trained_thief",
        name: "Ladrão Treinado",
        brain: BrainKind::Simple,
        boss: None,
        regions: &[],
        base_probability: Probability::ALWAYS,
        personalities: &[Personality::Cowardice],
        potential: EnemyPotential::Low,
        immunities: BodyImmunities::new().with_little_resistance(ImmunityKind::Bleeding),
        resistance: 480,
        vitality: 110,
        strength: 19,
        intelligence: 8,
        ether: 30,
        allies: None,
        weapon: None,
        skills: vec![
            SkillKind::TornadoKick,
            SkillKind::MirrorDamage,
            SkillKind::ResplendentPunch,
            SkillKind::Charge,
            SkillKind::IcyBreath,
        ],
        drop: EnemyReward {
            orbs: (10, 90),
            xp: (20, 70),
            items: vec![EnemyRewardItem {
                item: items::tool::PICKAXE,
                amount: (1, 1),
                probability: Probability::new(30),
            }],
        }
    }
);

make_enemy!(
    hunter,
    Enemy {
        identifier: "hunter",
        name: "Caçador",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        boss: None,
        regions: &[
            (WorldRegion::Mudland, 3),
            (WorldRegion::Emerelis, 6),
            (WorldRegion::Murkswamp, 1),
            (WorldRegion::Gloomwood, 4),
        ],
        personalities: &[Personality::Courage, Personality::Calm],
        potential: EnemyPotential::Medium,
        immunities: BodyImmunities::new(),
        strength: 8,
        intelligence: 6,
        resistance: 215,
        vitality: 70,
        ether: 30,
        weapon: None,
        allies: None,
        skills: vec![
            SkillKind::IcyBreath,
            SkillKind::IcyShot,
            SkillKind::WaterBlessing,
            SkillKind::Suplex,
        ],
        drop: EnemyReward {
            orbs: (20, 50),
            xp: (40, 70),
            items: vec![EnemyRewardItem {
                amount: (3, 6),
                item: items::material::STONE,
                probability: Probability::new(5),
            },],
        },
    }
);

make_enemy!(
    insane_fighter,
    Enemy {
        identifier: "insane_fighter",
        name: "Lutador Insano",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        boss: None,
        regions: &[
            (WorldRegion::Mudland, 1),
            (WorldRegion::Emerelis, 3),
            (WorldRegion::Murkswamp, 6),
            (WorldRegion::Gloomwood, 6),
        ],
        personalities: &[Personality::Insanity],
        potential: EnemyPotential::Medium,
        immunities: BodyImmunities::new(),
        strength: 36,
        intelligence: 12,
        resistance: 850,
        vitality: 320,
        ether: 30,
        weapon: None,
        allies: Some(vec![(Probability::new(5), Box::new(hunter()))]),
        skills: vec![
            SkillKind::ImbuedPunch,
            SkillKind::ElectricSlap,
            SkillKind::Suplex,
            SkillKind::Charge,
            SkillKind::Refresh,
        ],
        drop: EnemyReward {
            orbs: (60, 100),
            xp: (40, 70),
            items: vec![],
        },
    }
);

make_enemy!(
    wandering_mutant,
    Enemy {
        identifier: "wandering_mutant",
        name: "Mutante Vagante",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        boss: None,
        regions: &[
            (WorldRegion::Mudland, 2),
            (WorldRegion::Emerelis, 1),
            (WorldRegion::Murkswamp, 5),
            (WorldRegion::Gloomwood, 5),
            (WorldRegion::Sunreach, 3),
        ],
        personalities: &[Personality::Aggressiveness],
        potential: EnemyPotential::Low,
        immunities: BodyImmunities::new(),
        strength: 17,
        intelligence: 3,
        resistance: 430,
        vitality: 80,
        ether: 5,
        weapon: None,
        allies: Some(vec![(Probability::new(10), Box::new(greenagis_mutant()))]),
        skills: vec![SkillKind::Bite, SkillKind::MirrorDamage],
        drop: EnemyReward {
            orbs: (15, 60),
            xp: (30, 60),
            items: vec![EnemyRewardItem {
                amount: (1, 1),
                item: items::lore::GOLDEN_ROBOT_POEM,
                probability: Probability::new(5),
            },],
        },
    }
);

make_enemy!(
    tired_insane_wanderer,
    Enemy {
        identifier: "tired_insane_wanderer",
        name: "Vagante Insano Cansado",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        boss: None,
        regions: &[
            (WorldRegion::Mudland, 3),
            (WorldRegion::Murkswamp, 6),
            (WorldRegion::Ethergrove, 3),
            (WorldRegion::Sunreach, 3),
        ],
        personalities: &[Personality::Cowardice, Personality::Insanity],
        potential: EnemyPotential::Medium,
        immunities: BodyImmunities::new(),
        strength: 43,
        intelligence: 10,
        resistance: 690,
        vitality: 115,
        ether: 40,
        weapon: Some(WeaponKind::Spear),
        allies: None,
        skills: vec![
            SkillKind::TornadoKick,
            SkillKind::ElectricSlap,
            SkillKind::CyclonePush,
            SkillKind::ParalyzingBet,
            SkillKind::WaterBlessing,
        ],
        drop: EnemyReward {
            orbs: (20, 90),
            xp: (40, 60),
            items: vec![
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::tool::UMBRELLA,
                    probability: Probability::new(30),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::tool::SPEAR,
                    probability: Probability::new(5),
                },
            ],
        },
    }
);

make_enemy!(
    mummified_swordsman,
    Enemy {
        identifier: "mummified_swordsman",
        name: "Espadachim Mumificado",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        boss: None,
        regions: &[(WorldRegion::Tenypt, 8), (WorldRegion::Sandywater, 2)],
        personalities: &[Personality::Arrogance, Personality::Courage],
        potential: EnemyPotential::High,
        immunities: BodyImmunities::new()
            .with_resistance(ImmunityKind::Cut, 0.6)
            .with_resistance(ImmunityKind::Poison, 1.0),
        strength: 30,
        intelligence: 15,
        resistance: 675,
        vitality: 238,
        ether: 50,
        weapon: Some(WeaponKind::Katana),
        allies: None,
        skills: vec![
            SkillKind::SimpleCut,
            SkillKind::CyclonePush,
            SkillKind::TornadoKick,
            SkillKind::YinYang,
        ],
        drop: EnemyReward {
            orbs: (50, 70),
            xp: (50, 80),
            items: vec![EnemyRewardItem {
                amount: (1, 1),
                item: items::tool::KATANA,
                probability: Probability::new(50),
            },],
        },
    }
);

make_enemy!(
    mad_scientist,
    Enemy {
        identifier: "mad_scientist",
        name: "Cientista Louco",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        boss: None,
        regions: &[(WorldRegion::Ethergrove, 1), (WorldRegion::Starbreeze, 3)],
        personalities: &[Personality::Intelligence, Personality::Arrogance],
        potential: EnemyPotential::High,
        immunities: BodyImmunities::new().with_weakness(ImmunityKind::Poison, 0.5),
        strength: 10,
        intelligence: 50,
        resistance: 415,
        vitality: 150,
        ether: 100,
        weapon: None,
        allies: None,
        skills: vec![
            SkillKind::IcyShot,
            SkillKind::WaterBlessing,
            SkillKind::WaterJet,
            SkillKind::ElectricSlap,
            SkillKind::Refresh,
            SkillKind::TenkuKikan(None),
        ],
        drop: EnemyReward {
            orbs: (60, 160),
            xp: (50, 100),
            items: vec![
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::lore::ENTITY_039_REPORT,
                    probability: Probability::new(10),
                },
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
    mountain_goat,
    Enemy {
        identifier: "mountain_goat",
        name: "Cabra da Montanha",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        boss: None,
        regions: &[(WorldRegion::Wornpeaks, 8)],
        personalities: &[Personality::Courage, Personality::Aggressiveness],
        potential: EnemyPotential::Medium,
        immunities: BodyImmunities::new()
            .with_resistance(ImmunityKind::Physical, 0.2)
            .with_resistance(ImmunityKind::Cut, 0.3),
        strength: 25,
        intelligence: 6,
        resistance: 320,
        vitality: 95,
        ether: 15,
        weapon: None,
        allies: None,
        skills: vec![
            SkillKind::Charge,
            SkillKind::DefensiveJump,
            SkillKind::TornadoKick,
        ],
        drop: EnemyReward {
            orbs: (40, 70),
            xp: (50, 80),
            items: vec![EnemyRewardItem {
                amount: (1, 1),
                item: items::consumable::MILK,
                probability: Probability::new(30),
            },],
        },
    }
);

make_enemy!(
    mountain_hermit,
    Enemy {
        identifier: "mountain_hermit",
        name: "Eremita da Montanha",
        base_probability: Probability::new(70),
        brain: BrainKind::Simple,
        boss: None,
        regions: &[(WorldRegion::Wornpeaks, 5)],
        personalities: &[
            Personality::Intelligence,
            Personality::Calm,
            Personality::Cowardice
        ],
        potential: EnemyPotential::High,
        immunities: BodyImmunities::new()
            .with_resistance(ImmunityKind::Ice, 0.7)
            .with_little_resistance(ImmunityKind::Fire),
        strength: 10,
        intelligence: 20,
        resistance: 350,
        vitality: 120,
        ether: 80,
        weapon: Some(WeaponKind::Umbrella),
        allies: None,
        skills: vec![
            SkillKind::IcyBreath,
            SkillKind::WaterBlessing,
            SkillKind::Refresh,
            SkillKind::MirrorDamage,
            SkillKind::InstinctiveReaction,
        ],
        drop: EnemyReward {
            orbs: (60, 100),
            xp: (70, 110),
            items: vec![
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::tool::UMBRELLA,
                    probability: Probability::new(40),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::consumable::CHOCOLATE_MILK,
                    probability: Probability::new(30),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::special::INVIGORATING_CRYSTAL,
                    probability: Probability::new(1),
                },
            ],
        },
    }
);

make_enemy!(
    echo_mimic,
    Enemy {
        identifier: "echo_mimic",
        name: "Eco Mímico",
        base_probability: Probability::new(60),
        brain: BrainKind::Simple,
        boss: None,
        regions: &[(WorldRegion::Wornpeaks, 2)],
        personalities: &[Personality::Intelligence, Personality::Cowardice],
        potential: EnemyPotential::High,
        immunities: BodyImmunities::new()
            .with_resistance(ImmunityKind::Water, 1.0)
            .with_resistance(ImmunityKind::Ice, 1.0)
            .with_weakness(ImmunityKind::Fire, 0.5),
        strength: 15,
        intelligence: 25,
        resistance: 300,
        vitality: 110,
        ether: 90,
        weapon: None,
        allies: None,
        skills: vec![
            SkillKind::MirrorDamage,
            SkillKind::CyclonePush,
            SkillKind::InstinctiveReaction,
            SkillKind::Refresh,
        ],
        drop: EnemyReward {
            orbs: (50, 90),
            xp: (60, 100),
            items: vec![EnemyRewardItem {
                amount: (1, 1),
                item: items::special::INVIGORATING_CRYSTAL,
                probability: Probability::new(10),
            },],
        },
    }
);
