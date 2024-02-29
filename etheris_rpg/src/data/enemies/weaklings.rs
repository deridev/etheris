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
        ether: 10,
        weapon: None,
        allies: None,
        skills: vec![SkillKind::Bite, SkillKind::BloodDonation],
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
