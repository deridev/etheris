use etheris_common::{calculate_power_level, Attribute};
use etheris_data::{personality::Personality, weapon::WeaponKind, SkillKind};
use etheris_database::character_model::CharacterModel;
use etheris_discord::twilight_model::user::User;

use crate::data::{enemies::Enemy, Reward};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FighterData {
    pub team: u8,
    pub name: String,
    pub user: Option<User>,

    pub personalities: Vec<Personality>,
    pub skills: Vec<SkillKind>,

    pub strength_level: u32,
    pub intelligence_level: u32,

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

            personalities: character.personalities.clone(),
            skills: character.skills.clone(),

            strength_level: character.stats.strength_level,
            intelligence_level: character.stats.intelligence_level,

            resistance: character.stats.resistance.into(),
            vitality: character.stats.vitality.into(),
            ether: character.stats.ether.into(),

            weapon: character.weapon,

            drop,
        }
    }

    pub fn new_from_enemy(team: u8, drop: Reward, enemy: Enemy) -> Self {
        FighterData {
            team,
            personalities: enemy.personalities.to_owned(),
            drop,
            user: None,
            name: enemy.name.to_string(),
            intelligence_level: enemy.intelligence,
            strength_level: enemy.strength,
            ether: Attribute::from(enemy.ether),
            resistance: Attribute::from(enemy.resistance),
            vitality: Attribute::from(enemy.vitality),
            weapon: enemy.weapon,
            skills: enemy.skills.to_vec(),
        }
    }

    pub fn power_level(&self) -> i64 {
        let weighted_skills = {
            let mut weight = 0.0;
            for skill in self.skills.iter() {
                let cost = skill.knowledge_cost();
                weight += (cost as f64) / 0.2;
            }

            weight / (self.skills.len() as f64)
        };

        calculate_power_level(
            self.vitality,
            self.resistance,
            self.ether,
            self.strength_level,
            self.intelligence_level,
            weighted_skills,
        )
    }
}
