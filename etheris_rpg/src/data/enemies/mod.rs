use etheris_common::Probability;
use etheris_data::{
    items::Item, personality::Personality, weapon::WeaponKind, world::regions::WorldRegion,
    SkillKind,
};
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{Fighter, FighterData, FighterIndex};

use super::{Reward, RewardItem};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnemyRewardItem {
    pub item: Item,
    pub amount: (i32, i32),
    pub probability: Probability,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct EnemyReward {
    pub orbs: (i32, i32),
    pub xp: (i32, i32),
    pub items: &'static [EnemyRewardItem],
}

impl From<EnemyReward> for Reward {
    fn from(value: EnemyReward) -> Self {
        let rng = &mut StdRng::from_entropy();

        Self {
            orbs: rng.gen_range(value.orbs.0..=value.orbs.1),
            xp: rng.gen_range(value.xp.0..=value.xp.1),
            items: value
                .items
                .iter()
                .filter(|i| i.probability.generate_random_bool())
                .map(|i| RewardItem {
                    amount: rng.gen_range(i.amount.0..=i.amount.1),
                    item: i.item,
                })
                .collect(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Enemy {
    pub identifier: &'static str,
    pub name: &'static str,
    pub base_probability: Probability,
    pub regions: &'static [(WorldRegion, i32)],
    pub personalities: &'static [Personality],
    pub strength: u32,
    pub intelligence: u32,
    pub resistance: i32,
    pub vitality: i32,
    pub ether: i32,
    pub weapon: Option<WeaponKind>,
    pub allies: Option<&'static [(Probability, &'static str)]>,
    pub skills: &'static [SkillKind],
    pub drop: EnemyReward,
}

impl Enemy {
    pub async fn power_level(&self) -> i64 {
        Fighter::new(
            0,
            FighterIndex(0),
            FighterIndex(0),
            FighterData::new_from_enemy(0, Default::default(), *self),
        )
        .power_level()
        .await
    }
}

pub mod special;
pub mod weaklings;
pub mod weaklings_2;
pub mod weaks;
pub const ALL_ENEMIES: &[Enemy] = &[
    special::MINIORBS,
    weaklings::GREENAGIS_MUTANT,
    weaklings::GIANT_RAT,
    weaklings::BEGINNER_LOOTER,
    weaklings::BEGINNER_LOOTER_2,
    weaklings::NEWBIE_HUNTER,
    weaklings::INSANE_WANDERER,
    weaklings::WEAK_MERCENARY,
    weaklings::CONSCIOUS_BEAST,
    weaklings::SHREDDER_WEAK,
    weaklings::WOOD_GOLEM,
    weaklings::STONE_GOLEM,
    weaklings_2::WANDERING_MUTANT,
    weaklings_2::INSANE_FIGHTER,
    weaklings_2::HUNTER,
    weaklings_2::LOOTER,
    weaklings_2::TIRED_INSANE_WANDERER,
    weaklings_2::MUMMIFIED_SWORDSMAN,
    weaklings_2::CORRUPT_SHAMAN,
    weaks::CONSCIOUS_BEAR,
    weaks::CORRUPT_MONK,
    weaks::ETHEREAL_HUNTER,
    weaks::SERPENTOID,
    weaks::CACTOID,
    weaks::DESERT_NOMAD,
    weaks::BEAST_KILLER,
    Enemy {
        identifier: "insane_legend",
        name: "Lenda Insana",
        base_probability: Probability::new(50),
        regions: &[(WorldRegion::Midgrass, 1), (WorldRegion::Wornpeaks, 1)],
        personalities: &[
            Personality::Aggressiveness,
            Personality::Insanity,
            Personality::Intelligence,
            Personality::Courage,
        ],
        strength: 150,
        intelligence: 120,
        resistance: 800,
        vitality: 1500,
        ether: 250,
        weapon: Some(WeaponKind::Katana),
        skills: &[
            SkillKind::ImbuedPunch,
            SkillKind::SimpleCut,
            SkillKind::MirrorDamage,
            SkillKind::FirePunch,
            SkillKind::Charge,
            SkillKind::ElectricSlap,
            SkillKind::FlamingBall,
            SkillKind::IcyShot,
            SkillKind::WaterJet,
            SkillKind::WaterBlessing,
        ],
        allies: None,
        drop: EnemyReward {
            orbs: (600, 800),
            xp: (400, 600),
            items: &[
                EnemyRewardItem {
                    amount: (1, 1),
                    item: etheris_data::items::lore::OLD_ABANDONED_BASEMENT_DIARY,
                    probability: Probability::new(50),
                },
                EnemyRewardItem {
                    amount: (1, 1),
                    item: etheris_data::items::lore::ENTITY_039_REPORT,
                    probability: Probability::new(100),
                },
            ],
        },
    },
];

pub fn get_enemy_by_id(id: &str) -> Option<Enemy> {
    ALL_ENEMIES
        .iter()
        .find(|enemy| enemy.identifier == id)
        .copied()
}
