use etheris_data::items;

use super::*;

pub const CONSCIOUS_BEAR: Enemy = Enemy {
    identifier: "conscious_bear",
    name: "Urso Consciente",
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
    regions: &[(WorldRegion::Tenypt, 7), (WorldRegion::Sandywater, 1)],
    personalities: &[Personality::Calm, Personality::Courage, Personality::Intelligence],
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
            probability: Probability::new(50)
        }],
    },
};

pub const BEAST_KILLER: Enemy = Enemy {
    identifier: "beast_killer",
    name: "Assassino de Bestas",
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
        SkillKind::IcyShot,
        SkillKind::CyclonePush,
        SkillKind::InstinctiveReaction,
        SkillKind::Bite,
    ],
    allies: None,
    drop: EnemyReward {
        orbs: (30, 60),
        xp: (60, 70),
        items: &[],
    },
};

pub const ETHEREAL_HUNTER: Enemy = Enemy {
    identifier: "ethereal_hunter",
    name: "Caçador Etéreo",
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
        orbs: (30, 60),
        xp: (30, 70),
        items: &[],
    },
};

pub const CORRUPT_MONK: Enemy = Enemy {
    identifier: "corrupt_monk",
    name: "Monge Corrupto",
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
    ],
    allies: None,
    drop: EnemyReward {
        orbs: (30, 60),
        xp: (50, 70),
        items: &[EnemyRewardItem {
            amount: (1, 1),
            item: items::special::INTELLIGENCE_CRYSTAL,
            probability: Probability::new(5),
        }],
    },
};
