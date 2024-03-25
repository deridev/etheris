use etheris_data::items;

use super::*;

make_enemy!(
    miniorbs,
    Enemy {
        identifier: "miniorbs",
        name: "Miniorbs da Sorte",
        base_probability: Probability::new(1),
        brain: BrainKind::Simple,
        regions: &[],
        personalities: &[Personality::Cowardice, Personality::Intelligence],
        strength: 10,
        intelligence: 20,
        resistance: 200,
        vitality: 300,
        ether: 40,
        weapon: None,
        allies: None,
        skills: vec![
            SkillKind::Charge,
            SkillKind::DefensiveJump,
            SkillKind::TornadoKick,
            SkillKind::ElectricSlap,
        ],
        drop: EnemyReward {
            orbs: (80, 150),
            xp: (100, 150),
            items: vec![EnemyRewardItem {
                amount: (1, 1),
                item: items::special::INVIGORATING_CRYSTAL,
                probability: Probability::new(50),
            }],
        },
    }
);
