use etheris_common::Probability;
use etheris_data::{
    items::Item, personality::Personality, weapon::WeaponKind, world::regions::WorldRegion,
    BossKind, PactKind, SkillKind,
};
use once_cell::sync::Lazy;
use rand::Rng;

use crate::{brain::BrainKind, BodyImmunities, FighterData};

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
    pub fn to_reward<RNG: Rng>(
        &self,
        rng: &mut RNG,
        player_pl: i64,
        enemy_pl: i64,
        is_boss: bool,
    ) -> Reward {
        let base_orbs = rng.gen_range(self.orbs.0..=self.orbs.1) as i64;
        let base_xp = rng.gen_range(self.xp.0..=self.xp.1) as i64;

        Reward {
            orbs: if is_boss {
                base_orbs as i32
            } else {
                calculate_orbs_gain(player_pl, enemy_pl, base_orbs) as i32
            },
            xp: if is_boss {
                base_xp as i32
            } else {
                calculate_xp_gain(player_pl, enemy_pl, base_xp) as i32
            },
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
        let scaled_difference = (pl_difference / 15.0).min(100.0); // Scale and cap the difference
        1.0 / (1.0 + scaled_difference.powf(1.75) * multiplier) // Gradual decrease
    } else {
        // Player is weaker
        1.0 + ((multiplier * 0.8) / 10.0) * pl_difference.abs()
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EnemyPotential {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
    Master,
}

impl EnemyPotential {
    pub fn to_f64(&self) -> f64 {
        match self {
            Self::VeryLow => 0.5,
            Self::Low => 0.75,
            Self::Medium => 1.0,
            Self::High => 1.5,
            Self::VeryHigh => 2.0,
            Self::Master => 4.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Enemy {
    pub identifier: &'static str,
    pub name: &'static str,
    pub base_probability: Probability,
    pub boss: Option<BossKind>,
    pub brain: BrainKind,
    pub regions: &'static [(WorldRegion, i32)],
    pub personalities: &'static [Personality],
    pub potential: EnemyPotential,
    pub immunities: BodyImmunities,
    pub strength: u32,
    pub intelligence: u32,
    pub resistance: i32,
    pub vitality: i32,
    pub ether: i32,
    pub weapon: Option<WeaponKind>,
    pub allies: Option<Vec<(Probability, Box<Enemy>)>>,
    pub skills: Vec<SkillKind>,
    pub drop: EnemyReward,
    pub pacts: Vec<PactKind>,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            identifier: "invalid_enemy",
            name: "Inimigo",
            base_probability: Probability::NEVER,
            brain: BrainKind::Simple,
            boss: None,
            regions: &[],
            personalities: &[],
            potential: EnemyPotential::Low,
            immunities: BodyImmunities::new(),
            strength: 0,
            intelligence: 0,
            resistance: 0,
            vitality: 0,
            ether: 0,
            weapon: None,
            allies: None,
            skills: vec![],
            drop: EnemyReward::default(),
            pacts: vec![],
        }
    }
}

impl Enemy {
    pub fn power_level(&self) -> i64 {
        FighterData::new_from_enemy(0, Default::default(), self.clone()).power_level()
    }
}

pub mod bosses;
pub mod special;
pub mod weaklings;
pub mod weaklings_plus;
pub mod weaks;
pub static ALL_ENEMIES: Lazy<Vec<Enemy>> = Lazy::new(|| {
    [
        //special::debug(),
        special::miniorbs(),
        //  BOSSES
        bosses::garhyan(),
        bosses::agorath(),
        bosses::orsinium(),
        bosses::ethria(),
        bosses::microlord_bedialus(),
        bosses::macrolord_vastorrant(),
        // Weaklings
        weaklings::giant_rat(),
        weaklings::greenagis_mutant(),
        weaklings::beginner_looter(),
        weaklings::insane_wanderer(),
        weaklings::weak_shredder(),
        weaklings::newbie_hunter(),
        weaklings::novice_bandit(),
        weaklings::conscious_beast(),
        weaklings::weak_mercenary(),
        weaklings::mud_golem(),
        weaklings::swamp_master(),
        weaklings::swamp_executioner(),
        weaklings::weak_mercenary_leader(),
        weaklings::stone_golem(),
        weaklings::wood_golem(),
        weaklings::forest_guardian(),
        weaklings::dangerous_bear(),
        weaklings::weak_thief(),
        weaklings::desert_coward(),
        // Weaklings+
        weaklings_plus::average_looter(),
        weaklings_plus::small_scorpion(),
        weaklings_plus::marsh_marauder(),
        weaklings_plus::ice_warrior(),
        weaklings_plus::desert_raider(),
        weaklings_plus::abominable_maquiran(),
        weaklings_plus::frost_wolf(),
        weaklings_plus::trained_thief(),
        weaklings_plus::hunter(),
        weaklings_plus::insane_fighter(),
        weaklings_plus::wandering_mutant(),
        weaklings_plus::tired_insane_wanderer(),
        weaklings_plus::mummified_swordsman(),
        weaklings_plus::mad_scientist(),
        weaklings_plus::mountain_goat(),
        weaklings_plus::mountain_hermit(),
        weaklings_plus::echo_mimic(),
        // Weaks
        weaks::conscious_bear(),
        weaks::serpentoid_weak(),
        weaks::cactoid(),
        weaks::desert_nomad(),
        weaks::midgrass_centaur(),
        weaks::thornbeast(),
        weaks::frost_shaman(),
        weaks::rock_thrower(),
        weaks::ethereal_hunter(),
        weaks::corrupt_monk(),
        weaks::etherking(),
    ]
    .to_vec()
});

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
