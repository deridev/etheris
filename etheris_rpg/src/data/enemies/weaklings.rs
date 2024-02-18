use etheris_data::items;

use super::*;

pub const GREENAGIS_MUTANT: Enemy = Enemy {
    identifier: "greenagis_mutant_base",
    name: "Mutante de Greenagis",
    regions: &[(WorldRegion::Greenagis, 8), (WorldRegion::Emerelis, 1)],
    personalities: &[Personality::Aggressiveness],
    strength: 1,
    intelligence: 1,
    resistance: 30,
    vitality: 50,
    ether: 0,
    weapon: None,
    skills: &[SkillKind::Bite],
    allies: Some(&[(Probability::new(40), "greenagis_mutant_base")]),
    drop: EnemyReward {
        orbs: (8, 15),
        xp: (3, 5),
        items: &[],
    },
};

pub const GIANT_RAT: Enemy = Enemy {
    identifier: "giant_rat",
    name: "Rato Gigante",
    regions: &[
        (WorldRegion::Greenagis, 4),
        (WorldRegion::Emerelis, 4),
        (WorldRegion::Gloomwood, 2),
        (WorldRegion::Mudland, 1),
    ],
    personalities: &[Personality::Cowardice],
    strength: 2,
    intelligence: 1,
    resistance: 60,
    vitality: 80,
    ether: 10,
    weapon: None,
    allies: Some(&[(Probability::new(30), "giant_rat")]),
    skills: &[SkillKind::Bite, SkillKind::BloodDonation],
    drop: EnemyReward {
        orbs: (10, 15),
        xp: (7, 10),
        items: &[EnemyRewardItem {
            amount: (1, 1),
            item: items::material::PAPER,
            probability: Probability::new(30),
        }],
    },
};

pub const BEGINNER_LOOTER: Enemy = Enemy {
    identifier: "beginner_looter",
    name: "Saqueador Iniciante",
    regions: &[
        (WorldRegion::Greenagis, 5),
        (WorldRegion::Emerelis, 4),
        (WorldRegion::Gloomwood, 2),
    ],
    personalities: &[Personality::Aggressiveness, Personality::Cowardice],
    strength: 1,
    intelligence: 1,
    resistance: 30,
    vitality: 80,
    ether: 15,
    weapon: None,
    allies: None,
    skills: &[SkillKind::SimpleCut],
    drop: EnemyReward {
        orbs: (5, 25),
        xp: (10, 15),
        items: &[],
    },
};

pub const BEGINNER_LOOTER_2: Enemy = Enemy {
    identifier: "beginner_looter_2",
    name: "Saqueador Iniciante",
    regions: &[
        (WorldRegion::Greenagis, 5),
        (WorldRegion::Emerelis, 4),
        (WorldRegion::Gloomwood, 2),
    ],
    personalities: &[Personality::Aggressiveness, Personality::Cowardice],
    strength: 1,
    intelligence: 1,
    resistance: 25,
    vitality: 90,
    ether: 10,
    weapon: Some(WeaponKind::Knife),
    allies: None,
    skills: &[SkillKind::SimpleCut],
    drop: EnemyReward {
        orbs: (15, 20),
        xp: (10, 15),
        items: &[],
    },
};

pub const NEWBIE_HUNTER: Enemy = Enemy {
    identifier: "newbie_hunter",
    name: "Caçador de Novatos",
    regions: &[
        (WorldRegion::Greenagis, 3),
        (WorldRegion::Emerelis, 3),
        (WorldRegion::Gloomwood, 1),
    ],
    personalities: &[Personality::Arrogance, Personality::Cowardice],
    strength: 2,
    intelligence: 1,
    resistance: 50,
    vitality: 120,
    ether: 20,
    weapon: Some(WeaponKind::Bat),
    allies: None,
    skills: &[
        SkillKind::MirrorDamage,
        SkillKind::ImbuedPunch,
        SkillKind::CyclonePush,
    ],
    drop: EnemyReward {
        orbs: (10, 30),
        xp: (5, 15),
        items: &[EnemyRewardItem {
            amount: (1, 1),
            item: items::tool::BAT,
            probability: Probability::new(60),
        }],
    },
};

pub const INSANE_WANDERER: Enemy = Enemy {
    identifier: "insane_wanderer",
    name: "Vagante Insano",
    regions: &[
        (WorldRegion::Greenagis, 1),
        (WorldRegion::Emerelis, 1),
        (WorldRegion::Gloomwood, 2),
        (WorldRegion::Mudland, 5),
    ],
    personalities: &[Personality::Cowardice, Personality::Insanity],
    strength: 3,
    intelligence: 1,
    resistance: 70,
    vitality: 200,
    ether: 35,
    weapon: None,
    allies: None,
    skills: &[
        SkillKind::TornadoKick,
        SkillKind::ElectricSlap,
        SkillKind::CyclonePush,
    ],
    drop: EnemyReward {
        orbs: (20, 40),
        xp: (9, 15),
        items: &[
            EnemyRewardItem {
                amount: (1, 1),
                item: items::tool::BAT,
                probability: Probability::new(10),
            },
            EnemyRewardItem {
                amount: (1, 1),
                item: items::tool::TRANSLATOR,
                probability: Probability::new(10),
            },
        ],
    },
};

pub const WEAK_MERCENARY: Enemy = Enemy {
    identifier: "weak_mercenary",
    name: "Mercenário",
    regions: &[
        (WorldRegion::Greenagis, 2),
        (WorldRegion::Emerelis, 5),
        (WorldRegion::Gloomwood, 3),
        (WorldRegion::Mudland, 2),
    ],
    personalities: &[Personality::Calm, Personality::Courage],
    strength: 2,
    intelligence: 2,
    resistance: 60,
    vitality: 120,
    ether: 20,
    weapon: Some(WeaponKind::Knife),
    allies: None,
    skills: &[
        SkillKind::SimpleCut,
        SkillKind::Charge,
        SkillKind::ImbuedPunch,
    ],
    drop: EnemyReward {
        orbs: (20, 30),
        xp: (10, 15),
        items: &[EnemyRewardItem {
            amount: (1, 1),
            item: items::material::KNIFE,
            probability: Probability::new(60),
        }],
    },
};

pub const CONSCIOUS_BEAST: Enemy = Enemy {
    identifier: "conscious_beast",
    name: "Besta Consciente",
    regions: &[
        (WorldRegion::Emerelis, 3),
        (WorldRegion::Gloomwood, 4),
        (WorldRegion::Murkswamp, 5),
    ],
    personalities: &[Personality::Aggressiveness, Personality::Insanity],
    strength: 6,
    intelligence: 6,
    resistance: 100,
    vitality: 250,
    ether: 20,
    weapon: None,
    allies: None,
    skills: &[
        SkillKind::ImbuedPunch,
        SkillKind::Charge,
        SkillKind::TornadoKick,
    ],
    drop: EnemyReward {
        orbs: (20, 60),
        xp: (10, 15),
        items: &[EnemyRewardItem {
            amount: (1, 7),
            item: items::material::STICK,
            probability: Probability::new(80),
        }],
    },
};

pub const SHREDDER_WEAK: Enemy = Enemy {
    identifier: "shredder_weak",
    name: "Retalhador",
    regions: &[
        (WorldRegion::Emerelis, 3),
        (WorldRegion::Gloomwood, 7),
        (WorldRegion::Murkswamp, 4),
    ],
    personalities: &[Personality::Aggressiveness, Personality::Insanity],
    strength: 11,
    intelligence: 2,
    resistance: 80,
    vitality: 300,
    ether: 40,
    weapon: Some(WeaponKind::Knife),
    allies: None,
    skills: &[
        SkillKind::SimpleCut,
        SkillKind::ElectricSlap,
        SkillKind::ImbuedPunch,
    ],
    drop: EnemyReward {
        orbs: (25, 40),
        xp: (10, 20),
        items: &[
            EnemyRewardItem {
                amount: (1, 1),
                item: items::material::KNIFE,
                probability: Probability::new(20),
            },
            EnemyRewardItem {
                amount: (1, 2),
                item: items::material::TOOL_HANDLE,
                probability: Probability::new(40),
            },
            EnemyRewardItem {
                amount: (1, 2),
                item: items::cosmetic::EYE_BANDANA,
                probability: Probability::new(5),
            },
        ],
    },
};

pub const STONE_GOLEM: Enemy = Enemy {
    identifier: "stone_golem",
    name: "Golem de Pedregulho",
    regions: &[(WorldRegion::Tenypt, 1), (WorldRegion::Murkswamp, 6)],
    personalities: &[Personality::Intelligence, Personality::Insanity],
    strength: 15,
    intelligence: 10,
    resistance: 130,
    vitality: 300,
    ether: 50,
    weapon: Some(WeaponKind::Umbrella),
    allies: None,
    skills: &[
        SkillKind::ImbuedPunch,
        SkillKind::InstinctiveReaction,
        SkillKind::Charge,
        SkillKind::Suplex,
    ],
    drop: EnemyReward {
        orbs: (25, 50),
        xp: (10, 25),
        items: &[
            EnemyRewardItem {
                amount: (1, 6),
                item: items::material::STONE,
                probability: Probability::new(80),
            },
            EnemyRewardItem {
                amount: (1, 1),
                item: items::cosmetic::STRAWHAT,
                probability: Probability::new(5),
            },
        ],
    },
};

pub const WOOD_GOLEM: Enemy = Enemy {
    identifier: "wood_golem",
    name: "Golem de Madeira",
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
    strength: 25,
    intelligence: 15,
    resistance: 180,
    vitality: 450,
    ether: 50,
    weapon: Some(WeaponKind::Stick),
    allies: None,
    skills: &[
        SkillKind::FirePunch,
        SkillKind::IcyShot,
        SkillKind::WaterBlessing,
    ],
    drop: EnemyReward {
        orbs: (25, 45),
        xp: (10, 25),
        items: &[
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
};

pub const SWAMP_MASTER: Enemy = Enemy {
    identifier: "swamp_master",
    name: "Mestre do Pântano",
    regions: &[
        (WorldRegion::Mudland, 3),
        (WorldRegion::Murkswamp, 5),
        (WorldRegion::Midgrass, 1),
    ],
    personalities: &[Personality::Intelligence, Personality::Insanity],
    strength: 8,
    intelligence: 8,
    resistance: 100,
    vitality: 500,
    ether: 50,
    weapon: Some(WeaponKind::Umbrella),
    allies: None,
    skills: &[
        SkillKind::WaterJet,
        SkillKind::WaterBlessing,
        SkillKind::ElectricSlap,
    ],
    drop: EnemyReward {
        orbs: (25, 35),
        xp: (10, 25),
        items: &[
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
};

/// MERCENARY_DUO
pub const WEAK_MERCENARY_LEADER: Enemy = Enemy {
    identifier: "weak_mercenary_leader",
    name: "Mercenário Chefe",
    regions: &[
        (WorldRegion::Emerelis, 1),
        (WorldRegion::Gloomwood, 2),
        (WorldRegion::Mudland, 2),
        (WorldRegion::Murkswamp, 2),
        (WorldRegion::Midgrass, 4),
        (WorldRegion::Sandywater, 1),
    ],
    personalities: &[Personality::Calm, Personality::Courage],
    strength: 10,
    intelligence: 15,
    resistance: 130,
    vitality: 230,
    ether: 50,
    weapon: Some(WeaponKind::Bat),
    allies: Some(&[(Probability::ALWAYS, "weak_mercenary")]),
    skills: &[
        SkillKind::FlamingBall,
        SkillKind::ParalyzingBet,
        SkillKind::ImbuedPunch,
        SkillKind::BloodDonation,
        SkillKind::IcyBreath,
    ],
    drop: EnemyReward {
        orbs: (40, 50),
        xp: (10, 30),
        items: &[
            EnemyRewardItem {
                amount: (1, 1),
                item: items::tool::BAT,
                probability: Probability::new(60),
            },
            EnemyRewardItem {
                amount: (1, 1),
                item: items::lore::OLD_ABANDONED_BASEMENT_DIARY,
                probability: Probability::new(10),
            },
        ],
    },
};
