use etheris_data::items;

use super::*;

pub const INSANE_FIGHTER: Enemy = Enemy {
    identifier: "insane_fighter",
    name: "Lutador Insano",
    base_probability: Probability::ALWAYS,
    regions: &[
        (WorldRegion::Mudland, 1),
        (WorldRegion::Emerelis, 3),
        (WorldRegion::Murkswamp, 6),
        (WorldRegion::Gloomwood, 6),
    ],
    personalities: &[Personality::Insanity],
    strength: 18,
    intelligence: 5,
    resistance: 150,
    vitality: 350,
    ether: 30,
    weapon: None,
    skills: &[
        SkillKind::ImbuedPunch,
        SkillKind::ElectricSlap,
        SkillKind::Suplex,
        SkillKind::Charge,
        SkillKind::Refresh,
    ],
    allies: Some(&[(Probability::new(5), "hunter")]),
    drop: EnemyReward {
        orbs: (30, 40),
        xp: (40, 70),
        items: &[EnemyRewardItem {
            amount: (1, 1),
            item: items::lore::OLD_ABANDONED_BASEMENT_DIARY,
            probability: Probability::new(5),
        }],
    },
};

pub const WANDERING_MUTANT: Enemy = Enemy {
    identifier: "wandering_mutant_base",
    name: "Mutante Vagante",
    base_probability: Probability::ALWAYS,
    regions: &[
        (WorldRegion::Mudland, 2),
        (WorldRegion::Emerelis, 1),
        (WorldRegion::Murkswamp, 5),
        (WorldRegion::Gloomwood, 5),
    ],
    personalities: &[Personality::Aggressiveness],
    strength: 13,
    intelligence: 3,
    resistance: 100,
    vitality: 200,
    ether: 5,
    weapon: None,
    skills: &[SkillKind::Bite, SkillKind::MirrorDamage],
    allies: Some(&[(Probability::new(10), "greenagis_mutant_base")]),
    drop: EnemyReward {
        orbs: (15, 30),
        xp: (30, 60),
        items: &[EnemyRewardItem {
            amount: (1, 1),
            item: items::lore::GOLDEN_ROBOT_POEM,
            probability: Probability::new(5),
        }],
    },
};

pub const HUNTER: Enemy = Enemy {
    identifier: "hunter",
    name: "Caçador",
    base_probability: Probability::ALWAYS,
    regions: &[
        (WorldRegion::Mudland, 3),
        (WorldRegion::Emerelis, 6),
        (WorldRegion::Murkswamp, 1),
        (WorldRegion::Gloomwood, 4),
    ],
    personalities: &[Personality::Courage, Personality::Calm],
    strength: 8,
    intelligence: 6,
    resistance: 80,
    vitality: 175,
    ether: 30,
    weapon: None,
    skills: &[
        SkillKind::IcyBreath,
        SkillKind::IcyShot,
        SkillKind::WaterBlessing,
        SkillKind::Suplex,
    ],
    allies: Some(&[(Probability::new(10), "wandering_mutant_base")]),
    drop: EnemyReward {
        orbs: (20, 30),
        xp: (40, 70),
        items: &[EnemyRewardItem {
            amount: (3, 6),
            item: items::material::STONE,
            probability: Probability::new(5),
        }],
    },
};

pub const TIRED_INSANE_WANDERER: Enemy = Enemy {
    identifier: "tired_insane_wanderer",
    name: "Vagante Insano Cansado",
    base_probability: Probability::ALWAYS,
    regions: &[
        (WorldRegion::Mudland, 3),
        (WorldRegion::Murkswamp, 6),
        (WorldRegion::Ethergrove, 3),
    ],
    personalities: &[Personality::Cowardice, Personality::Insanity],
    strength: 15,
    intelligence: 10,
    resistance: 150,
    vitality: 350,
    ether: 40,
    weapon: Some(WeaponKind::Spear),
    allies: None,
    skills: &[
        SkillKind::TornadoKick,
        SkillKind::ElectricSlap,
        SkillKind::CyclonePush,
        SkillKind::ParalyzingBet,
        SkillKind::WaterBlessing,
    ],
    drop: EnemyReward {
        orbs: (20, 40),
        xp: (40, 60),
        items: &[
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
};

pub const MUMMIFIED_SWORDSMAN: Enemy = Enemy {
    identifier: "mummified_swordsman",
    name: "Espadachim Mumificado",
    base_probability: Probability::ALWAYS,
    regions: &[(WorldRegion::Tenypt, 8), (WorldRegion::Sandywater, 2)],
    personalities: &[Personality::Arrogance, Personality::Courage],
    strength: 30,
    intelligence: 15,
    resistance: 200,
    vitality: 500,
    ether: 50,
    weapon: Some(WeaponKind::Katana),
    allies: None,
    skills: &[
        SkillKind::SimpleCut,
        SkillKind::CyclonePush,
        SkillKind::TornadoKick,
        SkillKind::YinYang,
    ],
    drop: EnemyReward {
        orbs: (50, 70),
        xp: (50, 80),
        items: &[EnemyRewardItem {
            amount: (1, 1),
            item: items::tool::KATANA,
            probability: Probability::new(50),
        }],
    },
};

pub const LOOTER: Enemy = Enemy {
    identifier: "average_looter",
    name: "Saqueador",
    base_probability: Probability::ALWAYS,
    regions: &[(WorldRegion::Tenypt, 6), (WorldRegion::Mudland, 3)],
    personalities: &[
        Personality::Aggressiveness,
        Personality::Intelligence,
        Personality::Cowardice,
    ],
    strength: 15,
    intelligence: 7,
    resistance: 100,
    vitality: 250,
    ether: 40,
    weapon: Some(WeaponKind::Knife),
    allies: None,
    skills: &[
        SkillKind::SimpleCut,
        SkillKind::FlamingBall,
        SkillKind::TornadoKick,
    ],
    drop: EnemyReward {
        orbs: (30, 50),
        xp: (50, 80),
        items: &[
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
};

pub const CORRUPT_SHAMAN: Enemy = Enemy {
    identifier: "corrupt_shaman",
    name: "Xamã Corrupto",
    base_probability: Probability::ALWAYS,
    regions: &[(WorldRegion::Ethergrove, 6), (WorldRegion::Wornpeaks, 1)],
    personalities: &[
        Personality::Intelligence,
        Personality::Cowardice,
        Personality::Arrogance,
    ],
    strength: 5,
    intelligence: 30,
    resistance: 120,
    vitality: 300,
    ether: 80,
    weapon: None,
    allies: Some(&[(Probability::new(50), "wandering_mutant_base")]),
    skills: &[
        SkillKind::BloodDonation,
        SkillKind::WaterJet,
        SkillKind::ElectricSlap,
        SkillKind::TenkuKikan(None),
    ],
    drop: EnemyReward {
        orbs: (30, 50),
        xp: (50, 85),
        items: &[
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
};
