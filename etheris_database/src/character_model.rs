use std::{
    collections::HashSet,
    fmt::Display,
    hash::Hash,
    mem::discriminant,
    ops::{Add, Sub},
};

use chrono::Duration;
use etheris_common::{clear_string, config, Attribute};
use etheris_data::{
    appearance::CharacterAppearance, items::{self, Item}, personality::Personality, weapon::WeaponKind, world::regions::WorldRegion, BossKind, ItemValue, ItemValues, SkillKind
};
use etheris_discord::twilight_model::id::{marker::UserMarker, Id};
use mongodb::bson::oid::ObjectId;
use rand::{Rng, SeedableRng};
use serde::{Deserialize, Serialize};

use crate::common::*;

fn _default_born_at() -> DatabaseDateTime {
    let now = chrono::Utc::now() - Duration::from_std(config::YEAR_LENGTH * 18).unwrap();
    DatabaseDateTime(now)
}

fn _default_appearance() -> CharacterAppearance {
    CharacterAppearance::default()
}

// I hate so much the serde::default method. IT'S UGLY.
const fn _one() -> u32 {
    1
}
const fn _default_pl() -> i64 {
    1
}

const fn _default_attribute() -> StatAttribute {
    StatAttribute { max: 25, value: 25 }
}
const fn _default_resistance() -> StatAttribute {
    StatAttribute {
        max: 100,
        value: 100,
    }
}
const fn _default_vitality() -> StatAttribute {
    StatAttribute { max: 30, value: 30 }
}
const fn _default_ether() -> StatAttribute {
    StatAttribute { max: 20, value: 20 }
}
const fn _default_battle_stats() -> BattleStats {
    BattleStats {
        kills: 0,
        knockouts: 0,
        losses: 0,
        wins: 0,
        life_risks: 0,
        withdrawals: 0,
    }
}

fn _default_stats() -> CharacterStats {
    CharacterStats {
        strength_level: _one(),
        health_level: _one(),
        intelligence_level: _one(),
        resistance: _default_resistance(),
        vitality: _default_vitality(),
        ether: _default_ether(),
        pve: _default_battle_stats(),
        pvp: _default_battle_stats(),
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum DeathCause {
    Vitality,
    Age,
    KilledBy(String),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct DeathInfo {
    pub cause: DeathCause,
    pub date: DatabaseDateTime,
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct StatAttribute {
    pub value: i32,
    pub max: i32,
}

impl From<StatAttribute> for Attribute {
    fn from(value: StatAttribute) -> Self {
        Self {
            value: value.value,
            max: value.max,
        }
    }
}

impl From<Attribute> for StatAttribute {
    fn from(value: Attribute) -> Self {
        Self {
            max: value.max,
            value: value.value,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize, Default)]
pub struct BattleStats {
    pub wins: u32,
    pub losses: u32,
    pub kills: u32,
    pub knockouts: u32,

    pub life_risks: u32,
    pub withdrawals: u32,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub struct CharacterStats {
    #[serde(default = "_one")]
    pub strength_level: u32,
    #[serde(default = "_one")]
    pub health_level: u32,
    #[serde(default = "_one")]
    pub intelligence_level: u32,

    #[serde(default = "_default_resistance")]
    pub resistance: StatAttribute,
    #[serde(default = "_default_vitality")]
    pub vitality: StatAttribute,
    #[serde(default = "_default_ether")]
    pub ether: StatAttribute,

    #[serde(default = "_default_battle_stats")]
    pub pvp: BattleStats,
    #[serde(default = "_default_battle_stats")]
    pub pve: BattleStats,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum SoulControlLevel {
    Rookie,
    Beginner,
    // TODO: /meditate
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct CharacterSettings {
    pub is_notifications_enabled: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub enum CharacterFlag {
    CanAknowledgeSkill,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum BattleAction {
    GiveUp,
    ControlPower,
}

impl BattleAction {
    pub fn identifier(&self) -> String {
        match self {
            Self::GiveUp => "give_up".to_string(),
            Self::ControlPower => "control_power".to_string(),
        }
    }

    pub fn name(&self) -> String {
        match self {
            Self::GiveUp => "Desistir".to_string(),
            Self::ControlPower => "Controlar Poder".to_string(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum MentalLevel {
    Laymen,
    Beginner,
    Novice,
    Accustomed,
    Spirited,
    Strong,
    Master,
    Champion,
    Legend,
}

impl MentalLevel {
    pub fn reward_multiplier(&self) -> f64 {
        match self {
            Self::Laymen => 0.3,
            Self::Beginner => 0.6,
            Self::Novice => 0.8,
            Self::Accustomed => 1.0,
            Self::Spirited => 1.1,
            Self::Strong => 1.2,
            Self::Master => 1.4,
            Self::Champion => 1.6,
            Self::Legend => 2.0,
        }
    }
}

impl Display for MentalLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Laymen => f.write_str("Leigo"),
            Self::Beginner => f.write_str("Iniciante"),
            Self::Novice => f.write_str("Novato"),
            Self::Accustomed => f.write_str("Acostumado"),
            Self::Spirited => f.write_str("Espirituoso"),
            Self::Strong => f.write_str("Forte"),
            Self::Master => f.write_str("Mestre"),
            Self::Champion => f.write_str("Campeão"),
            Self::Legend => f.write_str("Lenda"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct CharacterModel {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub user_id: String,
    pub name: String,
    pub settings: CharacterSettings,
    pub flags: HashSet<CharacterFlag>,
    pub orbs: i64,

    pub tags: HashSet<String>,
    pub inventory: Vec<InventoryItem>,
    pub battle_inventory: Vec<InventoryItem>,
    pub personalities: Vec<Personality>,
    pub actions: HashSet<BattleAction>,

    pub mental_level: MentalLevel,
    pub potential: f64,

    pub study_skills_cache: Vec<SkillKind>,
    pub skills: Vec<SkillKind>,
    pub learned_skills: Vec<SkillKind>,
    pub learnable_skills: Vec<SkillKind>,

    pub defeated_bosses: HashSet<BossKind>,

    pub region: WorldRegion,
    pub weapon: Option<WeaponKind>,

    pub action_points: u32,
    pub max_action_points: u32,

    pub warnings: Vec<String>,

    pub last_refill: DatabaseDateTime,
    pub refill_minutes: u32,

    #[serde(default = "_default_pl")]
    pub pl: i64,
    pub strength_xp: u32,
    pub health_xp: u32,
    pub intelligence_xp: u32,
    pub knowledge_xp: u32,
    pub knowledge_points: u32,

    pub karma: i32,
    pub life_expectancy: i32,
    pub alive: bool,
    pub death_info: Option<DeathInfo>,

    // Dates
    #[serde(default = "_default_now")]
    pub created_at: DatabaseDateTime,
    #[serde(default = "_default_born_at")]
    pub born_at: DatabaseDateTime,

    #[serde(default = "_default_stats")]
    pub stats: CharacterStats,
    #[serde(default = "_default_appearance")]
    pub appearance: CharacterAppearance,
}

impl CharacterModel {
    pub fn new(
        user_id: Id<UserMarker>,
        name: String,
        personalities: Vec<Personality>,
        skills: Vec<SkillKind>,
        appearance: CharacterAppearance,
    ) -> Self {
        let rng = &mut rand::rngs::StdRng::from_entropy();
        let life_expectancy = if rng.gen_bool(0.7) {
            rng.gen_range(70..100)
        } else {
            rng.gen_range(80..120)
        };

        Self {
            id: ObjectId::new(),
            name,
            user_id: user_id.get().to_string(),
            settings: CharacterSettings {
                is_notifications_enabled: true,
            },
            orbs: 0,
            life_expectancy,
            flags: HashSet::new(),
            alive: true,
            death_info: None,
            actions: [BattleAction::GiveUp].into_iter().collect(),

            tags: HashSet::new(),
            inventory: vec![],
            battle_inventory: vec![],

            mental_level: MentalLevel::Laymen,
            potential: 0.5,

            study_skills_cache: vec![],
            learned_skills: skills.clone(),
            learnable_skills: vec![SkillKind::ImbuedPunch],
            skills,
            personalities,

            region: WorldRegion::Greenagis,
            weapon: None,
            defeated_bosses: HashSet::new(),

            karma: 0,
            action_points: 30, // Start with a lot of action points to keep engagement at the start
            max_action_points: 10,

            warnings: Vec::default(),

            last_refill: _default_now(),
            refill_minutes: 60 * 5, // five hours

            pl: _default_pl(),
            strength_xp: 0,
            health_xp: 0,
            intelligence_xp: 0,
            knowledge_xp: 150,
            knowledge_points: 3,

            born_at: _default_born_at(),
            created_at: _default_now(),

            stats: _default_stats(),
            appearance,
        }
    }

    pub fn age(&self) -> i32 {
        let now = chrono::Utc::now();
        ((now.timestamp_millis() - self.born_at.timestamp_millis())
            / config::YEAR_LENGTH.as_millis() as i64) as i32
    }

    pub fn create_image_bufer(&self) -> Option<Vec<u8>> {
        if self.user_id == config::BOT_ID {
            None
        } else {
            Some(etheris_util::character_image::generate_character_image_buffer(&self.appearance))
        }
    }

    pub fn has_flag(&self, flag: CharacterFlag) -> bool {
        self.flags.contains(&flag)
    }

    pub fn insert_flag(&mut self, flag: CharacterFlag) {
        self.flags.insert(flag);
    }

    pub fn remove_flag(&mut self, flag: CharacterFlag) {
        self.flags.remove(&flag);
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(tag)
    }

    pub fn insert_tag(&mut self, tag: String) {
        self.tags.insert(tag);
    }

    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.remove(tag);
    }

    pub fn add_karma(&mut self, amount: i32) {
        self.karma = (self.karma + amount).clamp(-1000, 1000);
    }

    pub fn remove_karma(&mut self, amount: i32) {
        self.karma = (self.karma - amount).clamp(-1000, 1000);
    }

    pub fn heal(&mut self, amount: i32) {
        let resistance_heal = (amount + self.stats.vitality.value) - self.stats.vitality.max;
        if resistance_heal > 0 {
            self.stats.vitality.value = self.stats.vitality.max;
            self.stats.resistance.value = self
                .stats
                .resistance
                .value
                .add(resistance_heal)
                .min(self.stats.resistance.max);
        } else {
            self.stats.vitality.value = self
                .stats
                .vitality
                .value
                .add(amount)
                .min(self.stats.vitality.max);
        }
    }

    pub fn add_ether(&mut self, amount: i32) {
        self.stats.ether.value = self.stats.ether.value.add(amount).min(self.stats.ether.max);
    }

    pub fn check_for_death(&mut self) -> Option<DeathInfo> {
        match &self.death_info {
            Some(info) => Some(info.clone()),
            None => {
                let mut info = None;
                if self.stats.vitality.value <= 0 {
                    self.alive = false;
                    info = Some(DeathInfo {
                        cause: DeathCause::Vitality,
                        date: DatabaseDateTime::now(),
                    });
                }

                if !self.alive {
                    self.death_info = info.clone();
                }

                info
            }
        }
    }

    pub fn travel_to(&mut self, region: WorldRegion) {
        self.region = region;
    }

    pub fn already_knows_skill(&self, skill: SkillKind) -> bool {
        let skill = discriminant(&skill);

        self.learned_skills.iter().any(|s| skill == discriminant(s))
            || self.skills.iter().any(|s| skill == discriminant(s))
            || self
                .learnable_skills
                .iter()
                .any(|s| skill == discriminant(s))
    }

    pub fn learn_skill(&mut self, skill: SkillKind) {
        if self
            .learned_skills
            .iter()
            .any(|s| discriminant(s) == discriminant(&skill))
        {
            return;
        }

        self.learnable_skills
            .retain(|s| discriminant(s) != discriminant(&skill));
        self.learned_skills.push(skill);
    }

    pub fn equip_skill(&mut self, skill: SkillKind) {
        if self
            .skills
            .iter()
            .any(|s| discriminant(s) == discriminant(&skill))
        {
            return;
        }

        self.learnable_skills.retain(|s| *s != skill);
        self.skills.push(skill);
    }

    pub fn unequip_skill(&mut self, skill: SkillKind) {
        self.learnable_skills.retain(|s| *s != skill);
        self.skills.retain(|s| *s != skill);

        if !self.learned_skills.contains(&skill) {
            self.learned_skills.push(skill);
        }
    }

    pub fn aknowledge_skill(&mut self, skill: SkillKind) {
        if self
            .learned_skills
            .iter()
            .any(|s| discriminant(s) == discriminant(&skill))
            || self
                .learnable_skills
                .iter()
                .any(|s| discriminant(s) == discriminant(&skill))
        {
            return;
        }

        self.learnable_skills.push(skill);
    }

    pub fn add_orbs(&mut self, orbs: i64) {
        self.orbs += orbs;
    }

    pub fn remove_orbs(&mut self, orbs: i64) {
        self.orbs -= orbs;
    }

    pub fn take_damage(&mut self, damage: i32) {
        let vital_damage = damage - self.stats.resistance.value;

        if vital_damage > 0 {
            self.stats.resistance.value = 0;
            self.stats.vitality.value = self.stats.vitality.value.sub(vital_damage).max(0);
        } else {
            self.stats.resistance.value = self.stats.resistance.value.sub(damage).max(0);
        }
    }

    pub fn get_inventory_item(&self, item: &Item) -> Option<&InventoryItem> {
        self.inventory
            .iter()
            .find(|it| it.identifier == item.identifier)
    }

    pub fn get_inventory_item_mut(&mut self, item: &Item) -> Option<&mut InventoryItem> {
        self.inventory
            .iter_mut()
            .find(|it| it.identifier == item.identifier)
    }

    pub fn get_battle_inventory_item(&self, item: &Item) -> Option<&InventoryItem> {
        self.battle_inventory
            .iter()
            .find(|it| it.identifier == item.identifier)
    }

    pub fn get_inventory_item_by_name(&self, name: &str) -> Option<&InventoryItem> {
        internal_get_inventory_item_by_name(&self.inventory, name)
    }

    pub fn get_battle_inventory_item_by_name(&self, name: &str) -> Option<&InventoryItem> {
        internal_get_inventory_item_by_name(&self.battle_inventory, name)
    }

    pub fn add_recipe(&mut self, recipe: String) {
        for item in self.inventory.iter_mut() {
            for value in item.values.values.iter_mut() {
                if let ItemValue::Recipes(recipes) = value {
                    recipes.push(recipe.clone())
                }
            }
        }
    }

    pub fn has_item_by_name(&self, item_name: &str, quantity: usize) -> bool {
        if let Some(item) = self.get_inventory_item_by_name(item_name) {
            item.quantity >= quantity
        } else {
            false
        }
    }

    pub fn has_item(&self, item: &Item, quantity: usize) -> bool {
        if let Some(item) = self.get_inventory_item(item) {
            item.quantity >= quantity
        } else {
            false
        }
    }

    pub fn add_item(&mut self, item: Item, quantity: usize, values: Option<ItemValues>) {
        internal_add_item(&mut self.inventory, item, quantity, values);
    }

    pub fn remove_item(&mut self, item: Item, quantity: usize) {
        internal_remove_item(&mut self.inventory, item, quantity);
    }

    pub fn add_battle_item(&mut self, item: Item, quantity: usize, values: Option<ItemValues>) {
        internal_add_item(&mut self.battle_inventory, item, quantity, values);
    }

    pub fn remove_battle_item(&mut self, item: Item, quantity: usize) {
        internal_remove_item(&mut self.battle_inventory, item, quantity);
    }
}

pub(crate) fn internal_get_inventory_item_by_name<'a>(
    inventory: &'a [InventoryItem],
    item_name: &str,
) -> Option<&'a InventoryItem> {
    let item_name = clear_string(item_name);
    for inventory_item in inventory.iter() {
        let alternative_names = inventory_item.values.alternative_names();
        if !alternative_names.is_empty()
            && alternative_names
                .iter()
                .map(|n| clear_string(n))
                .any(|n| n == item_name)
        {
            return Some(inventory_item);
        }

        if let Some(item) = items::get_item(&inventory_item.identifier) {
            if clear_string(item.display_name) == item_name {
                return Some(inventory_item);
            }
        }
    }

    None
}

pub(crate) fn internal_add_item(
    inventory: &mut Vec<InventoryItem>,
    item: Item,
    quantity: usize,
    values: Option<ItemValues>,
) {
    if !item.stackable {
        for _ in 0..quantity {
            inventory.push(InventoryItem {
                identifier: item.identifier.to_string(),
                quantity: 1,
                values: values.clone().unwrap_or(item.default_values.into()),
            });
        }

        return;
    }

    if let Some(inv_item) = inventory
        .iter_mut()
        .find(|i| i.identifier == item.identifier)
    {
        inv_item.quantity += quantity;
    } else {
        inventory.push(InventoryItem {
            identifier: item.identifier.to_string(),
            quantity,
            values: values.unwrap_or(item.default_values.into()),
        });
    }
}

pub(crate) fn internal_remove_item(
    inventory: &mut Vec<InventoryItem>,
    item: Item,
    quantity: usize,
) {
    if let Some((index, inv_item)) = inventory
        .iter_mut()
        .enumerate()
        .find(|(_, i)| i.identifier == item.identifier)
    {
        inv_item.quantity = inv_item.quantity.saturating_sub(quantity);

        if inv_item.quantity == 0 {
            inventory.remove(index);
        }
    }
}
