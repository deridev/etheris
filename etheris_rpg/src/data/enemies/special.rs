use etheris_data::items;

use super::*;

pub const MINIORBS: Enemy = Enemy {
    identifier: "miniorbs",
    name: "Miniorbs da Sorte",
    base_probability: Probability::new(30),
    regions: &[
        (WorldRegion::Greenagis, 1),
        (WorldRegion::Emerelis, 1),
        (WorldRegion::Gloomwood, 1),
        (WorldRegion::Ethergrove, 1),
        (WorldRegion::Murkswamp, 1),
        (WorldRegion::Metrolis, 1),
    ],
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