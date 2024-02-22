use etheris_data::items;

use super::*;

pub const CONSCIOUS_BEAR: Enemy = Enemy {
    identifier: "conscious_bear",
    name: "Urso Consciente",
    base_probability: Probability::ALWAYS,
    regions: &[(WorldRegion::Ethergrove, 6), (WorldRegion::Midgrass, 1)],
    personalities: &[
        Personality::Aggressiveness,
        Personality::Insanity,
        Personality::Arrogance,
    ],
    strength: 40,
    intelligence: 15,
    resistance: 200,
    vitality: 400,
    ether: 20,
    weapon: None,
    skills: &[
        SkillKind::Bite,
        SkillKind::Charge,
        SkillKind::ImbuedPunch,
        SkillKind::MirrorDamage,
    ],
    allies: None,
    drop: EnemyReward {
        orbs: (20, 50),
        xp: (30, 85),
        items: &[EnemyRewardItem {
            amount: (1, 1),
            item: items::special::INVIGORATING_CRYSTAL,
            probability: Probability::new(1),
        }],
    },
};

pub const SERPENTOID: Enemy = Enemy {
    identifier: "serpentoid_weak",
    name: "Serpentóide",
    base_probability: Probability::ALWAYS,
    regions: &[(WorldRegion::Tenypt, 8), (WorldRegion::Sandywater, 1)],
    personalities: &[Personality::Insanity, Personality::Courage],
    strength: 30,
    intelligence: 10,
    resistance: 300,
    vitality: 600,
    ether: 40,
    weapon: None,
    allies: None,
    skills: &[
        SkillKind::Bite,
        SkillKind::Charge,
        SkillKind::ElectricSlap,
        SkillKind::SimpleCut,
    ],
    drop: EnemyReward {
        orbs: (50, 70),
        xp: (30, 70),
        items: &[EnemyRewardItem {
            amount: (1, 1),
            item: items::tool::KATANA,
            probability: Probability::new(50),
        }],
    },
};

pub const CACTOID: Enemy = Enemy {
    identifier: "cactoid",
    name: "Cactóide",
    base_probability: Probability::ALWAYS,
    regions: &[(WorldRegion::Tenypt, 1), (WorldRegion::Sandywater, 5)],
    personalities: &[Personality::Insanity, Personality::Courage],
    strength: 20,
    intelligence: 8,
    resistance: 80,
    vitality: 300,
    ether: 40,
    weapon: Some(WeaponKind::Bat),
    skills: &[
        SkillKind::FlamingBall,
        SkillKind::FirePunch,
        SkillKind::MirrorDamage,
    ],
    allies: None,
    drop: EnemyReward {
        orbs: (30, 60),
        xp: (30, 70),
        items: &[],
    },
};

pub const DESERT_NOMAD: Enemy = Enemy {
    identifier: "desert_nomad",
    name: "Nômade do Deserto",
    base_probability: Probability::ALWAYS,
    regions: &[(WorldRegion::Tenypt, 7), (WorldRegion::Sandywater, 1)],
    personalities: &[
        Personality::Calm,
        Personality::Courage,
        Personality::Intelligence,
    ],
    strength: 8,
    intelligence: 14,
    resistance: 100,
    vitality: 185,
    ether: 30,
    weapon: Some(WeaponKind::Umbrella),
    skills: &[
        SkillKind::FlamingBall,
        SkillKind::Refresh,
        SkillKind::TornadoKick,
        SkillKind::WaterBlessing,
    ],
    allies: None,
    drop: EnemyReward {
        orbs: (30, 60),
        xp: (50, 65),
        items: &[EnemyRewardItem {
            amount: (1, 1),
            item: items::tool::UMBRELLA,
            probability: Probability::new(50),
        }],
    },
};

pub const BEAST_KILLER: Enemy = Enemy {
    identifier: "beast_killer",
    name: "Assassino de Bestas",
    base_probability: Probability::ALWAYS,
    regions: &[(WorldRegion::Ethergrove, 5), (WorldRegion::Wornpeaks, 1)],
    personalities: &[
        Personality::Calm,
        Personality::Intelligence,
        Personality::Arrogance,
    ],
    strength: 30,
    intelligence: 60,
    resistance: 300,
    vitality: 500,
    ether: 100,
    weapon: Some(WeaponKind::Spear),
    skills: &[
        SkillKind::MirrorDamage,
        SkillKind::YinYang,
        SkillKind::Suplex,
        SkillKind::SimpleCut,
        SkillKind::WoundHealing,
        SkillKind::IcyShot,
        SkillKind::CyclonePush,
        SkillKind::InstinctiveReaction,
        SkillKind::Bite,
    ],
    allies: None,
    drop: EnemyReward {
        orbs: (30, 70),
        xp: (80, 100),
        items: &[],
    },
};

pub const ETHEREAL_HUNTER: Enemy = Enemy {
    identifier: "ethereal_hunter",
    name: "Caçador Etéreo",
    base_probability: Probability::ALWAYS,
    regions: &[(WorldRegion::Ethergrove, 1), (WorldRegion::Wornpeaks, 2)],
    personalities: &[
        Personality::Calm,
        Personality::Insanity,
        Personality::Intelligence,
    ],
    strength: 20,
    intelligence: 60,
    resistance: 150,
    vitality: 400,
    ether: 100,
    weapon: Some(WeaponKind::Bat),
    skills: &[
        SkillKind::MirrorDamage,
        SkillKind::YinYang,
        SkillKind::IcyBreath,
        SkillKind::IcyShot,
        SkillKind::CyclonePush,
        SkillKind::Bite,
        SkillKind::ParalyzingBet,
    ],
    allies: None,
    drop: EnemyReward {
        orbs: (60, 100),
        xp: (80, 130),
        items: &[],
    },
};

pub const CORRUPT_MONK: Enemy = Enemy {
    identifier: "corrupt_monk",
    name: "Monge Corrupto",
    base_probability: Probability::ALWAYS,
    regions: &[(WorldRegion::Ethergrove, 1), (WorldRegion::Wornpeaks, 5)],
    personalities: &[
        Personality::Calm,
        Personality::Insanity,
        Personality::Arrogance,
    ],
    strength: 15,
    intelligence: 50,
    resistance: 150,
    vitality: 400,
    ether: 100,
    weapon: Some(WeaponKind::Bat),
    skills: &[
        SkillKind::MirrorDamage,
        SkillKind::YinYang,
        SkillKind::IcyBreath,
        SkillKind::IcyShot,
        SkillKind::CyclonePush,
        SkillKind::WoundHealing,
    ],
    allies: None,
    drop: EnemyReward {
        orbs: (30, 90),
        xp: (50, 150),
        items: &[EnemyRewardItem {
            amount: (1, 1),
            item: items::special::INTELLIGENCE_CRYSTAL,
            probability: Probability::new(5),
        }],
    },
};

pub const GRASS_GOLEM: Enemy = Enemy {
    identifier: "grass_golem",
    name: "Golem de Grama",
    base_probability: Probability::ALWAYS,
    regions: &[(WorldRegion::Midgrass, 2)],
    personalities: &[Personality::Insanity, Personality::Aggressiveness],
    strength: 40,
    intelligence: 10,
    resistance: 150,
    vitality: 500,
    ether: 50,
    weapon: None,
    skills: &[
        SkillKind::FirePunch,
        SkillKind::WaterBlessing,
        SkillKind::CyclonePush,
        SkillKind::WoundHealing,
    ],
    allies: None,
    drop: EnemyReward {
        orbs: (50, 112),
        xp: (60, 130),
        items: &[],
    },
};

pub const FRANTIC: Enemy = Enemy {
    identifier: "frantic",
    name: "Frenético",
    base_probability: Probability::ALWAYS,
    regions: &[
        (WorldRegion::Ethergrove, 3),
        (WorldRegion::Sandywater, 1),
        (WorldRegion::Midgrass, 2),
    ],
    personalities: &[Personality::Insanity, Personality::Aggressiveness],
    strength: 25,
    intelligence: 5,
    resistance: 100,
    vitality: 300,
    ether: 80,
    weapon: None,
    skills: &[
        SkillKind::ElectricSlap,
        SkillKind::WaterJet,
        SkillKind::Bite,
    ],
    allies: None,
    drop: EnemyReward {
        orbs: (50, 90),
        xp: (60, 80),
        items: &[EnemyRewardItem {
            amount: (1, 3),
            item: items::material::STONE,
            probability: Probability::new(80),
        }],
    },
};

pub const ICE_MASTER: Enemy = Enemy {
    identifier: "ice_master",
    name: "Mestre do Gelo",
    base_probability: Probability::ALWAYS,
    regions: &[(WorldRegion::Ethergrove, 2), (WorldRegion::Wornpeaks, 4)],
    personalities: &[
        Personality::Calm,
        Personality::Intelligence,
        Personality::Arrogance,
    ],
    strength: 5,
    intelligence: 30,
    resistance: 180,
    vitality: 250,
    ether: 80,
    weapon: None,
    skills: &[
        SkillKind::IcyBreath,
        SkillKind::IcyShot,
        SkillKind::WaterBlessing,
        SkillKind::WaterJet,
        SkillKind::WoundHealing,
    ],
    allies: None,
    drop: EnemyReward {
        orbs: (40, 90),
        xp: (50, 170),
        items: &[EnemyRewardItem {
            amount: (1, 1),
            item: items::material::STONE,
            probability: Probability::new(5),
        }],
    },
};

pub const ETHERKING: Enemy = Enemy {
    identifier: "etherking",
    name: "Etherking",
    base_probability: Probability::ALWAYS,
    regions: &[(WorldRegion::Ethergrove, 1)],
    personalities: &[
        Personality::Calm,
        Personality::Insanity,
        Personality::Arrogance,
    ],
    strength: 50,
    intelligence: 70,
    resistance: 500,
    vitality: 800,
    ether: 175,
    weapon: None,
    skills: &[
        SkillKind::WoundHealing,
        SkillKind::YinYang,
        SkillKind::IcyBreath,
        SkillKind::IcyShot,
        SkillKind::CyclonePush,
        SkillKind::FirePunch,
        SkillKind::FlamingBall,
        SkillKind::ElectricSlap,
        SkillKind::Refresh,
        SkillKind::InstinctiveReaction,
    ],
    allies: None,
    drop: EnemyReward {
        orbs: (150, 400),
        xp: (150, 350),
        items: &[EnemyRewardItem {
            amount: (1, 1),
            item: items::special::INTELLIGENCE_CRYSTAL,
            probability: Probability::new(5),
        }],
    },
};


pub const CORRUPTED_PHARAOH: Enemy = Enemy {
    identifier: "corrupted_pharaoh",
    name: "Faraó Corrompido",
    base_probability: Probability::ALWAYS,
    regions: &[(WorldRegion::Sandywater, 2)],
    personalities: &[
        Personality::Insanity,
        Personality::Aggressiveness,
        Personality::Arrogance,
    ],
    strength: 32,
    intelligence: 21,
    resistance: 250,
    vitality: 600,
    ether: 110,
    weapon: None,
    skills: &[
        SkillKind::FirePunch,
        SkillKind::FlamingBall,
        SkillKind::TornadoKick,
        SkillKind::WoundHealing,
        SkillKind::SimpleCut,
    ],
    allies: None,
    drop: EnemyReward {
        orbs: (50, 250),
        xp: (80, 150),
        items: &[EnemyRewardItem {
            amount: (1, 1),
            item: items::tool::KATANA,
            probability: Probability::new(30),
        }],
    },
};
