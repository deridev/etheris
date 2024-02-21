use etheris_data::items;

use super::*;

const ALL_REGIONS: &[(WorldRegion, i32)] = &[
    (WorldRegion::Greenagis, 1),
    (WorldRegion::Emerelis, 1),
    (WorldRegion::Gloomwood, 1),
    (WorldRegion::Ethergrove, 1),
    (WorldRegion::Murkswamp, 1),
    (WorldRegion::Tenypt, 1),
    (WorldRegion::Sandywater, 1),
    (WorldRegion::Wornpeaks, 1),
    (WorldRegion::Midgrass, 1),
];

pub const MINIORBS: Enemy = Enemy {
    identifier: "miniorbs",
    name: "Miniorbs da Sorte",
    base_probability: Probability::new(5),
    regions: ALL_REGIONS,
    personalities: &[Personality::Intelligence, Personality::Courage],
    strength: 6,
    intelligence: 6,
    resistance: 100,
    vitality: 200,
    ether: 40,
    weapon: None,
    allies: None,
    skills: &[
        SkillKind::MirrorDamage,
        SkillKind::ImbuedPunch,
        SkillKind::CyclonePush,
        SkillKind::Charge,
        SkillKind::Refresh,
    ],
    drop: EnemyReward {
        orbs: (40, 80),
        xp: (30, 50),
        items: &[EnemyRewardItem {
            amount: (1, 1),
            item: items::special::INVIGORATING_CRYSTAL,
            probability: Probability::new(50),
        }],
    },
};

pub const AVEWORBS: Enemy = Enemy {
    identifier: "aveworbs",
    name: "Aveworbs da Fortuna",
    base_probability: Probability::new(1),
    regions: ALL_REGIONS,
    personalities: &[Personality::Intelligence, Personality::Courage],
    strength: 17,
    intelligence: 17,
    resistance: 150,
    vitality: 300,
    ether: 50,
    weapon: None,
    allies: None,
    skills: &[
        SkillKind::MirrorDamage,
        SkillKind::ImbuedPunch,
        SkillKind::CyclonePush,
        SkillKind::Charge,
        SkillKind::Refresh,
        SkillKind::YinYang,
        SkillKind::InstinctiveReaction,
    ],
    drop: EnemyReward {
        orbs: (60, 200),
        xp: (80, 150),
        items: &[
            EnemyRewardItem {
                amount: (1, 1),
                item: items::special::INVIGORATING_CRYSTAL,
                probability: Probability::new(80),
            },
            EnemyRewardItem {
                amount: (1, 1),
                item: items::special::INTELLIGENCE_CRYSTAL,
                probability: Probability::new(5),
            },
        ],
    },
};
