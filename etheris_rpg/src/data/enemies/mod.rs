use etheris_common::Probability;
use etheris_data::{
    items::Item, personality::Personality, weapon::WeaponKind, world::regions::WorldRegion,
    SkillKind,
};
use once_cell::sync::Lazy;
use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::{brain::BrainKind, FighterData};

use super::{Reward, RewardItem};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EnemyRewardItem {
    pub item: Item,
    pub amount: (i32, i32),
    pub probability: Probability,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct EnemyReward {
    pub orbs: (i32, i32),
    pub xp: (i32, i32),
    pub items: Vec<EnemyRewardItem>,
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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Enemy {
    pub identifier: &'static str,
    pub name: &'static str,
    pub base_probability: Probability,
    pub brain: BrainKind,
    pub regions: &'static [(WorldRegion, i32)],
    pub personalities: &'static [Personality],
    pub strength: u32,
    pub intelligence: u32,
    pub resistance: i32,
    pub vitality: i32,
    pub ether: i32,
    pub weapon: Option<WeaponKind>,
    pub allies: Option<Vec<(Probability, Box<Enemy>)>>,
    pub skills: Vec<SkillKind>,
    pub drop: EnemyReward,
}

impl Enemy {
    pub fn power_level(&self) -> i64 {
        FighterData::new_from_enemy(0, Default::default(), self.clone()).power_level()
    }
}

pub mod weaklings;
pub static ALL_ENEMIES: Lazy<Vec<Enemy>> = Lazy::new(|| [weaklings::giant_rat()].to_vec());

pub fn get_enemy_by_id(id: &str) -> Option<Enemy> {
    ALL_ENEMIES
        .iter()
        .find(|enemy| enemy.identifier == id)
        .cloned()
}

pub fn get_enemies_by_regions(regions: &[WorldRegion]) -> Vec<Enemy> {
    ALL_ENEMIES
        .iter()
        .filter(|e| e.regions.iter().any(|r| regions.contains(&r.0)))
        .cloned()
        .collect()
}

#[macro_export]
macro_rules! make_enemy {
    ($identifier:ident, $enemy:expr) => {
        pub fn $identifier() -> Enemy {
            assert_eq!($enemy.identifier, stringify!($identifier));
            $enemy
        }
    };
}

pub use make_enemy;
