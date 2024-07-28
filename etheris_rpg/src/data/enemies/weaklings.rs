use etheris_data::items;

use crate::ImmunityKind;

use super::*;

make_enemy!(
    giant_rat,
    Enemy {
        identifier: "giant_rat",
        name: "Rato Gigante",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        boss: None,
        regions: &[(WorldRegion::Greenagis, 4),],
        personalities: &[Personality::Cowardice],
        potential: EnemyPotential::VeryLow,
        immunities: BodyImmunities::new().with_little_resistance(ImmunityKind::Poison),
        strength: 2,
        intelligence: 1,
        resistance: 60,
        vitality: 25,
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
        boss: None,
        regions: &[(WorldRegion::Greenagis, 6)],
        personalities: &[Personality::Cowardice],
        immunities: BodyImmunities::new(),
        potential: EnemyPotential::VeryLow,
        strength: 2,
        intelligence: 3,
        resistance: 90,
        vitality: 30,
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
                probability: Probability::new(1),
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
        boss: None,
        regions: &[
            (WorldRegion::Greenagis, 1),
            (WorldRegion::Emerelis, 4),
            (WorldRegion::Gloomwood, 3),
        ],
        personalities: &[Personality::Aggressiveness, Personality::Arrogance],
        potential: EnemyPotential::VeryLow,
        immunities: BodyImmunities::new(),
        strength: 5,
        intelligence: 2,
        resistance: 110,
        vitality: 43,
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
        boss: None,
        regions: &[(WorldRegion::Gloomwood, 1), (WorldRegion::Mudland, 1)],
        personalities: &[Personality::Cowardice, Personality::Aggressiveness],
        potential: EnemyPotential::Low,
        immunities: BodyImmunities::new().with_little_resistance(ImmunityKind::Fire),
        strength: 15,
        intelligence: 6,
        resistance: 230,
        vitality: 65,
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

make_enemy!(
    novice_bandit,
    Enemy {
        identifier: "novice_bandit",
        name: "Bandido Novato",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        boss: None,
        regions: &[(WorldRegion::Greenagis, 3), (WorldRegion::Emerelis, 5)],
        personalities: &[Personality::Aggressiveness, Personality::Cowardice],
        potential: EnemyPotential::Low,
        immunities: BodyImmunities::new(),
        strength: 4,
        intelligence: 2,
        resistance: 70,
        vitality: 35,
        ether: 10,
        weapon: Some(WeaponKind::Stick),
        allies: None,
        skills: vec![SkillKind::ImbuedPunch, SkillKind::Charge],
        drop: EnemyReward {
            orbs: (10, 20),
            xp: (20, 30),
            items: vec![
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::material::STICK,
                    probability: Probability::new(80),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::consumable::CORN,
                    probability: Probability::new(50),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::material::RAW_TRUNK,
                    probability: Probability::new(20),
                },
            ],
        },
    }
);

make_enemy!(
    newbie_hunter,
    Enemy {
        identifier: "newbie_hunter",
        name: "Caçador de Novatos",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        boss: None,
        regions: &[
            (WorldRegion::Greenagis, 3),
            (WorldRegion::Emerelis, 3),
            (WorldRegion::Gloomwood, 1),
        ],
        personalities: &[Personality::Arrogance, Personality::Cowardice],
        potential: EnemyPotential::Low,
        immunities: BodyImmunities::new(),
        strength: 5,
        intelligence: 2,
        resistance: 180,
        vitality: 75,
        ether: 20,
        weapon: Some(WeaponKind::Bat),
        allies: None,
        skills: vec![
            SkillKind::DefensiveJump,
            SkillKind::TornadoKick,
            SkillKind::ImbuedPunch,
            SkillKind::CyclonePush,
        ],
        drop: EnemyReward {
            orbs: (10, 25),
            xp: (30, 50),
            items: vec![EnemyRewardItem {
                amount: (1, 1),
                item: items::tool::BAT,
                probability: Probability::new(30),
            }],
        },
    }
);

make_enemy!(
    insane_wanderer,
    Enemy {
        identifier: "insane_wanderer",
        name: "Vagante Insano",
        base_probability: Probability::new(70),
        brain: BrainKind::Simple,
        boss: None,
        regions: &[
            (WorldRegion::Greenagis, 1),
            (WorldRegion::Emerelis, 1),
            (WorldRegion::Gloomwood, 2),
            (WorldRegion::Mudland, 5),
        ],
        personalities: &[Personality::Cowardice, Personality::Insanity],
        potential: EnemyPotential::Medium,
        immunities: BodyImmunities::new()
            .with_little_resistance(ImmunityKind::Poison)
            .with_little_resistance(ImmunityKind::Electric)
            .with_little_resistance(ImmunityKind::Bleeding),
        strength: 20,
        intelligence: 3,
        resistance: 250,
        vitality: 70,
        ether: 40,
        weapon: None,
        allies: None,
        skills: vec![
            SkillKind::TornadoKick,
            SkillKind::ElectricSlap,
            SkillKind::CyclonePush,
            SkillKind::Charge,
        ],
        drop: EnemyReward {
            orbs: (20, 40),
            xp: (50, 70),
            items: vec![
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::tool::BAT,
                    probability: Probability::new(80),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::tool::TRANSLATOR,
                    probability: Probability::new(1),
                },
            ],
        }
    }
);

make_enemy!(
    conscious_beast,
    Enemy {
        identifier: "conscious_beast",
        name: "Besta Consciente",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        boss: None,
        regions: &[
            (WorldRegion::Emerelis, 3),
            (WorldRegion::Gloomwood, 4),
            (WorldRegion::Murkswamp, 5),
        ],
        personalities: &[Personality::Aggressiveness, Personality::Insanity],
        potential: EnemyPotential::Medium,
        immunities: BodyImmunities::new()
            .with_little_resistance(ImmunityKind::Cut)
            .with_little_resistance(ImmunityKind::Bleeding),
        strength: 11,
        intelligence: 6,
        resistance: 315,
        vitality: 80,
        ether: 20,
        weapon: None,
        allies: None,
        skills: vec![
            SkillKind::ImbuedPunch,
            SkillKind::Charge,
            SkillKind::TornadoKick,
        ],
        drop: EnemyReward {
            orbs: (20, 60),
            xp: (30, 50),
            items: vec![EnemyRewardItem {
                amount: (1, 7),
                item: items::material::STICK,
                probability: Probability::new(80),
            },],
        },
    }
);

make_enemy!(
    forest_guardian,
    Enemy {
        identifier: "forest_guardian",
        name: "Guardião da Floresta",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        boss: None,
        regions: &[(WorldRegion::Mudland, 1), (WorldRegion::Gloomwood, 4)],
        personalities: &[Personality::Courage, Personality::Intelligence],
        potential: EnemyPotential::Medium,
        immunities: BodyImmunities::new().with_little_resistance(ImmunityKind::Physical),
        strength: 10,
        intelligence: 12,
        resistance: 160,
        vitality: 50,
        ether: 60,
        weapon: Some(WeaponKind::Bat),
        allies: None,
        skills: vec![
            SkillKind::ImbuedPunch,
            SkillKind::WaterBlessing,
            SkillKind::Refresh,
            SkillKind::DefensiveJump,
        ],
        drop: EnemyReward {
            orbs: (20, 40),
            xp: (35, 55),
            items: vec![
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::tool::AXE,
                    probability: Probability::new(10),
                },
                EnemyRewardItem {
                    amount: (1, 2),
                    item: items::consumable::APPLE,
                    probability: Probability::new(50),
                },
            ],
        },
    }
);

make_enemy!(
    weak_mercenary,
    Enemy {
        identifier: "weak_mercenary",
        name: "Mercenário",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        boss: None,
        regions: &[
            (WorldRegion::Greenagis, 2),
            (WorldRegion::Emerelis, 5),
            (WorldRegion::Gloomwood, 3),
            (WorldRegion::Mudland, 2),
            (WorldRegion::Starbreeze, 1),
        ],
        personalities: &[Personality::Calm, Personality::Courage],
        potential: EnemyPotential::Medium,
        immunities: BodyImmunities::new().with_little_weakness(ImmunityKind::Ice),
        strength: 9,
        intelligence: 5,
        resistance: 130,
        vitality: 45,
        ether: 20,
        weapon: Some(WeaponKind::Knife),
        allies: None,
        skills: vec![
            SkillKind::SimpleCut,
            SkillKind::Charge,
            SkillKind::ImbuedPunch,
        ],
        drop: EnemyReward {
            orbs: (20, 30),
            xp: (30, 40),
            items: vec![EnemyRewardItem {
                amount: (1, 1),
                item: items::material::KNIFE,
                probability: Probability::new(60),
            }],
        },
    }
);

make_enemy!(
    mud_golem,
    Enemy {
        identifier: "mud_golem",
        name: "Golem de Lama",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        boss: None,
        regions: &[(WorldRegion::Mudland, 1), (WorldRegion::Murkswamp, 4)],
        personalities: &[Personality::Calm, Personality::Insanity],
        potential: EnemyPotential::Medium,
        immunities: BodyImmunities::new()
            .with_little_weakness(ImmunityKind::Water)
            .with_little_weakness(ImmunityKind::Ice),
        strength: 22,
        intelligence: 10,
        resistance: 315,
        vitality: 60,
        ether: 50,
        weapon: Some(WeaponKind::Stick),
        allies: None,
        skills: vec![
            SkillKind::DefensiveJump,
            SkillKind::WaterBlessing,
            SkillKind::WaterJet,
            SkillKind::Bite,
            SkillKind::Suplex,
        ],
        drop: EnemyReward {
            orbs: (45, 60),
            xp: (30, 60),
            items: vec![EnemyRewardItem {
                amount: (1, 1),
                item: items::material::KNIFE,
                probability: Probability::new(60),
            }],
        },
    }
);

make_enemy!(
    swamp_executioner,
    Enemy {
        identifier: "swamp_executioner",
        name: "Carrasco do Pântano",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        boss: None,
        regions: &[(WorldRegion::Murkswamp, 3),],
        personalities: &[Personality::Courage, Personality::Insanity],
        potential: EnemyPotential::Medium,
        immunities: BodyImmunities::new().with_little_resistance(ImmunityKind::Physical),
        strength: 32,
        intelligence: 5,
        resistance: 364,
        vitality: 111,
        ether: 30,
        weapon: Some(WeaponKind::Knife),
        allies: None,
        skills: vec![
            SkillKind::CursedBlood,
            SkillKind::SimpleCut,
            SkillKind::Refresh,
            SkillKind::InstinctiveReaction,
        ],
        drop: EnemyReward {
            orbs: (25, 60),
            xp: (40, 90),
            items: vec![
                EnemyRewardItem {
                    amount: (1, 3),
                    item: items::material::BONE,
                    probability: Probability::new(20),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::cosmetic::EYE_BANDANA,
                    probability: Probability::new(5),
                },
            ],
        },
    }
);

make_enemy!(
    swamp_master,
    Enemy {
        identifier: "swamp_master",
        name: "Mestre do Pântano",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        boss: None,
        regions: &[
            (WorldRegion::Mudland, 3),
            (WorldRegion::Murkswamp, 5),
            (WorldRegion::Midgrass, 1),
        ],
        personalities: &[Personality::Intelligence, Personality::Insanity],
        potential: EnemyPotential::Medium,
        immunities: BodyImmunities::new(),
        strength: 8,
        intelligence: 15,
        resistance: 208,
        vitality: 93,
        ether: 50,
        weapon: Some(WeaponKind::Umbrella),
        allies: None,
        skills: vec![
            SkillKind::WaterJet,
            SkillKind::WaterBlessing,
            SkillKind::ElectricSlap,
        ],
        drop: EnemyReward {
            orbs: (25, 35),
            xp: (40, 70),
            items: vec![
                EnemyRewardItem {
                    amount: (1, 3),
                    item: items::material::RAW_TRUNK,
                    probability: Probability::new(20),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::cosmetic::STRAWHAT,
                    probability: Probability::new(5),
                },
            ],
        },
    }
);

make_enemy!(
    stone_golem,
    Enemy {
        identifier: "stone_golem",
        name: "Golem de Pedregulho",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        boss: None,
        regions: &[(WorldRegion::Tenypt, 1), (WorldRegion::Murkswamp, 6)],
        personalities: &[Personality::Intelligence, Personality::Insanity],
        potential: EnemyPotential::High,
        immunities: BodyImmunities::new()
            .with_resistance(ImmunityKind::Physical, 0.8)
            .with_resistance(ImmunityKind::Cut, 0.6)
            .with_resistance(ImmunityKind::Poison, 1.0)
            .with_resistance(ImmunityKind::Water, 0.5)
            .with_resistance(ImmunityKind::Bleeding, 1.0)
            .with_weakness(ImmunityKind::Ice, 1.0),
        strength: 6,
        intelligence: 5,
        resistance: 280,
        vitality: 112,
        ether: 45,
        weapon: None,
        allies: None,
        skills: vec![
            SkillKind::WaterJet,
            SkillKind::WaterBlessing,
            SkillKind::ElectricSlap,
        ],
        drop: EnemyReward {
            orbs: (55, 80),
            xp: (40, 70),
            items: vec![
                EnemyRewardItem {
                    amount: (1, 3),
                    item: items::material::RAW_TRUNK,
                    probability: Probability::new(20),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::cosmetic::STRAWHAT,
                    probability: Probability::new(5),
                },
            ],
        },
    }
);

make_enemy!(
    wood_golem,
    Enemy {
        identifier: "wood_golem",
        name: "Golem de Madeira",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        boss: None,
        regions: &[
            (WorldRegion::Gloomwood, 1),
            (WorldRegion::Ethergrove, 5),
            (WorldRegion::Murkswamp, 1),
        ],
        personalities: &[
            Personality::Calm,
            Personality::Courage,
            Personality::Intelligence,
        ],
        potential: EnemyPotential::High,
        immunities: BodyImmunities::new()
            .with_resistance(ImmunityKind::Physical, 0.6)
            .with_resistance(ImmunityKind::Poison, 1.0)
            .with_resistance(ImmunityKind::Water, 0.8)
            .with_resistance(ImmunityKind::Bleeding, 1.0)
            .with_weakness(ImmunityKind::Fire, 1.0),
        strength: 12,
        intelligence: 6,
        resistance: 320,
        vitality: 120,
        ether: 50,
        weapon: Some(WeaponKind::Stick),
        allies: None,
        skills: vec![
            SkillKind::FirePunch,
            SkillKind::IcyShot,
            SkillKind::WaterBlessing,
        ],
        drop: EnemyReward {
            orbs: (50, 80),
            xp: (40, 70),
            items: vec![
                EnemyRewardItem {
                    amount: (1, 5),
                    item: items::material::RAW_TRUNK,
                    probability: Probability::new(90),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::material::TOOL_HANDLE,
                    probability: Probability::new(60),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::cosmetic::EYE_BANDANA,
                    probability: Probability::new(5),
                },
            ],
        },
    }
);

// MERCENARY DUO
make_enemy!(
    weak_mercenary_leader,
    Enemy {
        identifier: "weak_mercenary_leader",
        name: "Mercenário Chefe",
        base_probability: Probability::ALWAYS,
        brain: BrainKind::Simple,
        boss: None,
        regions: &[
            (WorldRegion::Gloomwood, 2),
            (WorldRegion::Mudland, 2),
            (WorldRegion::Murkswamp, 2),
            (WorldRegion::Midgrass, 4),
            (WorldRegion::Sandywater, 1),
            (WorldRegion::Starbreeze, 1),
        ],
        personalities: &[Personality::Calm, Personality::Courage],
        potential: EnemyPotential::Medium,
        immunities: BodyImmunities::new().with_little_resistance(ImmunityKind::Bleeding),
        strength: 25,
        intelligence: 8,
        resistance: 250,
        vitality: 73,
        ether: 50,
        weapon: Some(WeaponKind::Bat),
        allies: Some(vec![(Probability::new(100), Box::new(weak_mercenary()))]),
        skills: vec![
            SkillKind::FlamingBall,
            SkillKind::ParalyzingBet,
            SkillKind::ImbuedPunch,
            SkillKind::BloodDonation,
            SkillKind::IcyBreath,
            SkillKind::Refresh,
        ],
        drop: EnemyReward {
            orbs: (40, 110),
            xp: (10, 50),
            items: vec![
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::tool::BAT,
                    probability: Probability::new(60),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::consumable::APPLE,
                    probability: Probability::new(60),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: items::lore::OLD_ABANDONED_BASEMENT_DIARY,
                    probability: Probability::new(2),
                },
            ],
        },
    }
);

make_enemy!(
    dangerous_bear,
    Enemy {
        identifier: "dangerous_bear",
        name: "Urso Perigoso",
        base_probability: Probability::NEVER,
        brain: BrainKind::Simple,
        boss: None,
        regions: &[(WorldRegion::Murkswamp, 1), (WorldRegion::Mudland, 1)],
        personalities: &[Personality::Aggressiveness],
        potential: EnemyPotential::Low,
        immunities: BodyImmunities::new().with_little_resistance(ImmunityKind::Bleeding),
        strength: 40,
        intelligence: 5,
        resistance: 400,
        vitality: 100,
        ether: 40,
        weapon: None,
        allies: None,
        skills: vec![
            SkillKind::ImbuedPunch,
            SkillKind::Charge,
            SkillKind::ElectricSlap,
            SkillKind::Refresh,
            SkillKind::ResplendentPunch
        ],
        drop: EnemyReward {
            orbs: (40, 90),
            xp: (80, 120),
            items: vec![],
        }
    }
);

make_enemy!(
    weak_thief,
    Enemy {
        identifier: "weak_thief",
        name: "Ladrão Comum",
        brain: BrainKind::Simple,
        boss: None,
        regions: &[],
        base_probability: Probability::ALWAYS,
        personalities: &[Personality::Cowardice],
        potential: EnemyPotential::Low,
        immunities: BodyImmunities::new(),
        resistance: 180,
        vitality: 55,
        intelligence: 3,
        strength: 6,
        ether: 15,
        allies: None,
        weapon: None,
        skills: vec![
            SkillKind::TornadoKick,
            SkillKind::MirrorDamage,
            SkillKind::ImbuedPunch,
        ],
        drop: EnemyReward {
            orbs: (10, 15),
            xp: (20, 50),
            items: vec![EnemyRewardItem {
                item: items::tool::SHOVEL,
                amount: (1, 1),
                probability: Probability::new(30),
            }],
        },
    }
);

make_enemy!(
    desert_coward,
    Enemy {
        identifier: "desert_coward",
        name: "Covarde do Deserto",
        brain: BrainKind::Simple,
        boss: None,
        regions: &[(WorldRegion::Sandywater, 2), (WorldRegion::Tenypt, 4)],
        base_probability: Probability::ALWAYS,
        personalities: &[Personality::Cowardice],
        potential: EnemyPotential::Low,
        immunities: BodyImmunities::new(),
        strength: 15,
        intelligence: 4,
        resistance: 214,
        vitality: 58,
        ether: 30,
        allies: None,
        weapon: None,
        skills: vec![SkillKind::ResplendentPunch,],
        drop: EnemyReward {
            orbs: (10, 42),
            xp: (20, 50),
            items: vec![EnemyRewardItem {
                item: items::material::KNIFE,
                amount: (1, 1),
                probability: Probability::new(30),
            }],
        },
    }
);
