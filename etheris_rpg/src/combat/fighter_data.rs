use etheris_common::{calculate_power_level, Attribute};
use etheris_data::{
    items, personality::Personality, weapon::WeaponKind, BossKind, PactKind, SkillKind,
};
use etheris_database::character_model::{BattleAction, CharacterModel};
use etheris_discord::twilight_model::user::User;

use crate::{
    brain::BrainKind,
    data::{enemies::Enemy, Reward},
    list::prelude::BattleItem,
    BodyImmunities,
};

#[derive(Debug, Clone, PartialEq)]
pub struct FighterData {
    pub team: u8,
    pub name: String,
    pub user: Option<User>,
    pub boss: Option<BossKind>,

    pub brain: Option<BrainKind>,
    pub actions: Vec<BattleAction>,

    pub inventory: Vec<BattleItem>,
    pub personalities: Vec<Personality>,
    pub skills: Vec<SkillKind>,
    pub pacts: Vec<PactKind>,

    pub strength_level: u32,
    pub intelligence_level: u32,
    pub potential: f64,
    pub immunities: BodyImmunities,

    pub weapon: Option<WeaponKind>,

    pub resistance: Attribute,
    pub vitality: Attribute,
    pub ether: Attribute,

    pub drop: Reward,
}

impl FighterData {
    pub fn new_from_character(
        team: u8,
        character: &CharacterModel,
        user: User,
        drop: Reward,
    ) -> Self {
        Self {
            team,
            name: character.name.to_owned(),
            user: Some(user),
            boss: None,

            brain: None,
            actions: character.actions.iter().copied().collect(),

            inventory: character
                .battle_inventory
                .iter()
                .map(|i| BattleItem {
                    item: items::get_item(&i.identifier).unwrap(),
                    quantity: i.quantity,
                    values: i.values.clone(),
                })
                .collect(),
            personalities: character.personalities.clone(),
            skills: character.skills.clone(),
            pacts: character.pacts.clone().into_iter().collect(),

            strength_level: character.stats.strength_level,
            intelligence_level: character.stats.intelligence_level,
            potential: character.potential,

            resistance: character.stats.resistance.into(),
            vitality: character.stats.vitality.into(),
            ether: character.stats.ether.into(),

            weapon: character.weapon,
            immunities: BodyImmunities::new(),

            drop,
        }
    }

    pub fn new_from_enemy(team: u8, drop: Reward, enemy: Enemy) -> Self {
        FighterData {
            team,
            personalities: enemy.personalities.to_owned(),
            boss: enemy.boss,
            drop,
            brain: Some(enemy.brain),
            actions: vec![BattleAction::ControlPower],
            user: None,
            name: enemy.name.to_string(),
            inventory: vec![],
            potential: enemy.potential.to_f64(),
            intelligence_level: enemy.intelligence,
            strength_level: enemy.strength,
            ether: Attribute::from(enemy.ether),
            resistance: Attribute::from(enemy.resistance),
            vitality: Attribute::from(enemy.vitality),
            weapon: enemy.weapon,
            skills: enemy.skills.to_vec(),
            pacts: enemy.pacts.clone().into_iter().collect(),
            immunities: enemy.immunities,
        }
    }

    pub fn power_level(&self) -> i64 {
        let weighted_skills = {
            let mut weight = 0.0;
            for skill in self.skills.iter() {
                let cost = skill.knowledge_cost();
                weight += (cost as f64) / 0.2;
            }

            weight / 5.0
        };

        calculate_power_level(
            self.vitality,
            self.resistance,
            self.ether,
            self.strength_level,
            self.intelligence_level,
            self.potential,
            weighted_skills,
        )
    }
}
