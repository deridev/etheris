use etheris_common::Probability;
use etheris_data::{
    items::Item, personality::Personality, weapon::WeaponKind, world::regions::WorldRegion,
    SkillKind,
};
use once_cell::sync::Lazy;
use rand::Rng;

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

impl EnemyReward {
    pub fn to_reward<RNG: Rng>(&self, rng: &mut RNG, player_pl: i64, enemy_pl: i64) -> Reward {
        let base_orbs = rng.gen_range(self.orbs.0..=self.orbs.1) as i64;
        let base_xp = rng.gen_range(self.xp.0..=self.xp.1) as i64;

        Reward {
            orbs: calculate_orbs_gain(player_pl, enemy_pl, base_orbs) as i32,
            xp: calculate_xp_gain(player_pl, enemy_pl, base_xp) as i32,
            items: self
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

fn calculate_gain(
    player_pl: i64,
    enemy_pl: i64,
    base_value: i64,
    multiplier: f64,
    max_factor: f64,
) -> i64 {
    let pl_difference = (player_pl - enemy_pl) as f64;

    let mut reduction_factor = if pl_difference >= 0.0 {
        // Player is stronger
        1.0 / (1.0 + multiplier * pl_difference)
    } else {
        // Player is weaker
        1.0 + ((multiplier * 0.98) / 10.0) * pl_difference.abs()
    };

    if reduction_factor > max_factor {
        reduction_factor = max_factor;
    }

    (base_value as f64 * reduction_factor) as i64
}

fn calculate_xp_gain(player_pl: i64, enemy_pl: i64, base_xp: i64) -> i64 {
    calculate_gain(player_pl, enemy_pl, base_xp, 0.03, 5.0)
}

fn calculate_orbs_gain(player_pl: i64, enemy_pl: i64, base_orbs: i64) -> i64 {
    calculate_gain(player_pl, enemy_pl, base_orbs, 0.3, 2.0)
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
