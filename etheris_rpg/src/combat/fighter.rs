use std::{
    collections::HashMap,
    fmt::Debug,
    ops::{Add, Sub},
    sync::Arc,
};

use bitflags::bitflags;
use etheris_common::{calculate_power_level, Attribute, Probability};
use etheris_data::{
    items::{self, Item},
    personality::Personality,
    weapon::WeaponKind,
    BossKind, SkillKind,
};
use etheris_database::character_model::BattleAction;
use etheris_discord::{twilight_model::user::User, ButtonBuilder, Emoji};
use tokio::sync::Mutex;

use crate::{
    brain::{make_brain, Brain, BrainKind},
    common::{BoxedSkill, DamageSpecifier},
    data::{finishers::Finisher, Reward},
    list::*,
    FighterData, Modifiers,
};

use self::prelude::BattleItem;

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
    pub struct FighterFlags: u8 {
        const ASKED_TO_RISK_LIFE = 1 << 0;
        const HAD_A_NEAR_DEATH_EXPERIENCE = 1 << 1;
        const RISKING_LIFE = 1 << 2;
        const CANNOT_REGEN_ETHER = 1 << 3;
        const CANNOT_REGEN_ETHER_OVERLOAD = 1 << 4;
        const GAVE_UP = 1 << 5;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct FighterIndex(pub usize);

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum Composure {
    #[default]
    Standing,
    OnGround,
    OnAir(u8),
}

#[derive(Clone)]
pub struct FighterSkill {
    pub identifier: &'static str,
    pub dynamic_skill: Arc<Mutex<BoxedSkill>>,
    pub base_kind: SkillKind,
}

impl From<SkillKind> for FighterSkill {
    fn from(value: SkillKind) -> Self {
        let boxed_skill = get_boxed_skill_from_kind(value);
        Self::new(boxed_skill)
    }
}

impl FighterSkill {
    pub fn new(skill: BoxedSkill) -> Self {
        Self {
            identifier: skill.data(&Default::default()).identifier,
            base_kind: skill.kind(),
            dynamic_skill: Arc::new(Mutex::new(skill)),
        }
    }
}

impl Debug for FighterSkill {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[dyn Skill]")
    }
}

impl PartialEq for FighterSkill {
    fn eq(&self, other: &Self) -> bool {
        self.identifier == other.identifier
    }
}

#[derive(Clone)]
pub struct FighterBrain {
    pub kind: BrainKind,
    pub dynamic_brain: Arc<Mutex<Box<dyn Brain + Send + 'static>>>,
}

impl FighterBrain {
    pub fn new(brain: Box<dyn Brain + Send + 'static>) -> Self {
        Self {
            kind: brain.kind(),
            dynamic_brain: Arc::new(Mutex::new(brain)),
        }
    }
}

impl Debug for FighterBrain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[dyn Brain]")
    }
}

impl PartialEq for FighterBrain {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EffectKind {
    Flaming,
    Burning,
    Shocked,
    Ice,
    Wet,
    Frozen,
    Bleeding,
    Poisoned,
    Paralyzed,
    Curse,
    Exhausted,

    LowProtection,
}

impl EffectKind {
    pub fn affected_immunity(&self) -> Option<ImmunityKind> {
        match self {
            Self::Flaming => Some(ImmunityKind::Fire),
            Self::Burning => Some(ImmunityKind::Fire),
            Self::Shocked => Some(ImmunityKind::Electric),
            Self::Bleeding => Some(ImmunityKind::Bleeding),
            Self::Poisoned => Some(ImmunityKind::Poison),
            Self::Ice => Some(ImmunityKind::Ice),
            Self::Wet => Some(ImmunityKind::Water),
            _ => None,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Effect {
    pub kind: EffectKind,
    pub amount: i32,
    pub culprit: FighterIndex,
}

impl Effect {
    pub fn new(kind: EffectKind, amount: i32, culprit: FighterIndex) -> Effect {
        Effect {
            kind,
            amount,
            culprit,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FighterWeapon {
    pub kind: WeaponKind,
}

impl From<WeaponKind> for FighterWeapon {
    fn from(value: WeaponKind) -> Self {
        Self { kind: value }
    }
}

impl FighterWeapon {
    pub fn action(&self) -> &'static str {
        match self.kind {
            WeaponKind::Knife => "Cortar",
            WeaponKind::Stick => "Atacar",
            WeaponKind::Bat => "Bater",
            WeaponKind::Spear => "Perfurar",
            WeaponKind::Katana => "Cortar",
            WeaponKind::EthriaKatana => "Cortar",
            WeaponKind::Umbrella => "Bater",
            WeaponKind::ScorpionFang => "Atacar",
        }
    }

    pub fn emoji(&self) -> Emoji<'_> {
        items::get_item_by_weapon(self.kind).emoji
    }

    pub fn input_button(&self) -> ButtonBuilder {
        ButtonBuilder::new()
            .set_label(self.action())
            .set_emoji(self.emoji())
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ImmunityKind {
    Fire,
    Ice,
    Water,
    Poison,
    Bleeding,
    Physical,
    Cut,
    Electric,
    Special,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct BodyImmunities {
    /// Resistance is a number from 0.0 to 1.0, where 1.0 is the maximum resistance (immune) and 0.0 is no resistance at all (default)
    pub resistance_map: HashMap<ImmunityKind, f64>,
    /// Weakness is a number from 0.0 to 2.0, where 2.0 is the maximum weakness (totally vulnerable) and 0.0 is no weakness at all (default)
    pub weakness_map: HashMap<ImmunityKind, f64>,
}

impl BodyImmunities {
    pub fn new() -> Self {
        Self {
            resistance_map: HashMap::new(),
            weakness_map: HashMap::new(),
        }
    }

    /// Resistance is a number from 0.0 to 1.0, where 1.0 is the maximum resistance (immune) and 0.0 is no resistance at all (default)
    pub fn with_resistance(mut self, immunity: ImmunityKind, resistance: f64) -> Self {
        self.resistance_map.insert(immunity, resistance);
        self
    }

    pub fn with_little_resistance(mut self, immunity: ImmunityKind) -> Self {
        self.resistance_map.insert(immunity, 0.1);
        self
    }

    /// Weakness is a number from 0.0 to 2.0, where 2.0 is the maximum weakness (totally vulnerable) and 0.0 is no weakness at all (default)
    pub fn with_weakness(mut self, immunity: ImmunityKind, weakness: f64) -> Self {
        self.weakness_map.insert(immunity, weakness);
        self
    }

    pub fn with_little_weakness(mut self, immunity: ImmunityKind) -> Self {
        self.weakness_map.insert(immunity, 0.1);
        self
    }

    pub fn add_resistance(&mut self, immunity: ImmunityKind, amount: f64) {
        let resistance = self.resistance_map.get(&immunity).copied().unwrap_or(0.0) + amount;
        self.resistance_map
            .insert(immunity, resistance.clamp(0.0, 1.0));
    }

    pub fn remove_resistance(&mut self, immunity: ImmunityKind, amount: f64) {
        let resistance = self.resistance_map.get(&immunity).copied().unwrap_or(0.0) - amount;
        self.resistance_map
            .insert(immunity, resistance.clamp(0.0, 1.0));
    }

    pub fn add_weakness(&mut self, immunity: ImmunityKind, amount: f64) {
        let weakness = self.weakness_map.get(&immunity).copied().unwrap_or(0.0) + amount;
        self.weakness_map.insert(immunity, weakness.clamp(0.0, 2.0));
    }

    pub fn remove_weakness(&mut self, immunity: ImmunityKind, amount: f64) {
        let weakness = self.weakness_map.get(&immunity).copied().unwrap_or(0.0) - amount;
        self.weakness_map.insert(immunity, weakness.clamp(0.0, 2.0));
    }

    pub fn increase_resistance(&mut self, immunity: ImmunityKind, amount: f64) {
        let weakness = self.weakness_map.get(&immunity).copied().unwrap_or(0.0);
        if weakness - amount < 0.0 {
            self.weakness_map.remove(&immunity);
            self.add_resistance(immunity, amount);
        } else {
            self.remove_weakness(immunity, amount);
        }
    }

    pub fn get_resistance(&self, immunity: ImmunityKind) -> f64 {
        self.resistance_map.get(&immunity).copied().unwrap_or(0.0)
    }

    pub fn get_weakness(&self, immunity: ImmunityKind) -> f64 {
        self.weakness_map.get(&immunity).copied().unwrap_or(0.0)
    }

    pub fn dmg_multiplier_from_immunity(&self, immunity: ImmunityKind) -> f64 {
        let resistance = self.get_resistance(immunity);
        let weakness = self.get_weakness(immunity);

        // Damage multiplier is affected by both resistance and weakness
        // Start with the base multiplier of 1.0 (no change in damage)
        let base_multiplier = 1.0;

        // Apply resistance (resistance decreases damage, hence we subtract)
        let after_resistance = base_multiplier - (base_multiplier * resistance);

        // Apply weakness (weakness increases damage, hence we add)
        let final_multiplier = after_resistance + (after_resistance * weakness);

        final_multiplier
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AiState {
    pub focused_in: FighterIndex,
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Fighter {
    pub team: u8,
    pub index: FighterIndex,
    pub target: FighterIndex,
    pub name: String,
    pub user: Option<User>,
    pub ai_state: Option<AiState>,
    pub brain: Option<FighterBrain>,
    pub boss: Option<BossKind>,

    pub pl: i64,
    pub finishers: Vec<Finisher>,
    pub actions: Vec<BattleAction>,
    pub power: f64,
    pub potential: f64,

    pub image: Option<Arc<Vec<u8>>>,

    pub flags: FighterFlags,
    pub inventory: Vec<BattleItem>,
    pub personalities: Vec<Personality>,
    pub skills: Vec<FighterSkill>,
    pub effects: Vec<Effect>,
    pub modifiers: Modifiers,
    pub body_immunities: BodyImmunities,

    pub is_defeated: bool,
    pub defeated_by: Option<FighterIndex>,
    pub killed_by: Option<FighterIndex>,

    pub strength_level: u32,
    pub intelligence_level: u32,

    pub weapon: Option<FighterWeapon>,

    pub overload: f64,
    pub resistance: Attribute,
    pub vitality: Attribute,
    pub ether: Attribute,

    pub defense: u32,
    pub composure: Composure,
    pub balance: u8,

    pub drop: Reward,
}

impl Fighter {
    pub fn dummy(data: FighterData) -> Self {
        Self::new(0, FighterIndex(0), FighterIndex(0), data)
    }

    pub fn new(team: u8, index: FighterIndex, target: FighterIndex, data: FighterData) -> Self {
        let mut finishers = vec![Finisher::Knockout, Finisher::BreakNeck];
        if let Some(weapon) = data.weapon {
            finishers.extend_from_slice(Finisher::get_weapon_finishers(weapon));
        }

        let is_ai = data.brain.is_some();

        Self {
            team,
            index,
            target,
            pl: data.power_level(),
            name: data.name,
            user: data.user,
            ai_state: if is_ai {
                Some(AiState { focused_in: target })
            } else {
                None
            },
            brain: data.brain.map(|b| FighterBrain::new(make_brain(b))),
            boss: data.boss,

            image: None,

            finishers,
            actions: data.actions.clone(),
            power: data.potential.min(1.0),
            potential: data.potential,

            is_defeated: false,
            defeated_by: None,
            killed_by: None,

            flags: FighterFlags::empty(),
            personalities: data.personalities.clone(),
            inventory: data.inventory.clone(),
            skills: data
                .skills
                .iter()
                .map(|s| FighterSkill::from(s.clone()))
                .collect(),
            effects: Vec::new(),
            modifiers: Modifiers::new(),
            body_immunities: data.immunities,

            strength_level: data.strength_level,
            intelligence_level: data.intelligence_level,

            weapon: data.weapon.map(From::from),

            overload: 0.0,
            resistance: data.resistance,
            vitality: data.vitality,
            ether: data.ether,

            defense: 0,
            composure: Composure::Standing,
            balance: 100,

            drop: data.drop,
        }
    }

    pub fn health(&self) -> Attribute {
        Attribute::new(
            self.resistance.value + self.vitality.value,
            self.resistance.max + self.vitality.max,
        )
    }

    pub fn height_above_ground(&self) -> u8 {
        match self.composure {
            Composure::OnAir(n) => n,
            _ => 0,
        }
    }

    pub fn strength_multiplier(&self) -> f32 {
        let base = 1.5 + (self.strength_level as f32);
        base * (self.power as f32) * 0.3
    }

    pub fn intelligence_multiplier(&self) -> f32 {
        let base = 1.5 + (self.intelligence_level as f32);
        base * (self.power as f32) * 0.2
    }

    pub fn mixed_multiplier(&self, strength_weight: f32, intelligence_weight: f32) -> f32 {
        let multiplier = self.strength_multiplier() * strength_weight
            + self.intelligence_multiplier() * intelligence_weight;

        multiplier / (strength_weight + intelligence_weight)
    }

    pub fn weapon_multiplier(&self) -> f32 {
        let Some(weapon) = &self.weapon else {
            return 0.0;
        };

        self.mixed_multiplier(
            weapon.kind.strength_weight(),
            weapon.kind.intelligence_weight(),
        )
    }

    pub fn add_balance(&mut self, balance: u8) {
        self.balance = self.balance.saturating_add(balance).min(100);
    }

    pub fn has_effect(&self, kind: EffectKind) -> bool {
        self.effects.iter().any(|e| e.kind == kind)
    }

    pub fn convert_effect(&mut self, from: EffectKind, to: Effect) {
        if let Some(effect) = self.effects.iter_mut().find(|e| e.kind == from) {
            *effect = to;
        }
    }

    pub fn apply_effect(&mut self, effect: Effect) -> bool {
        if let Some(e) = self.effects.iter_mut().find(|ef| ef.kind == effect.kind) {
            e.amount = e.amount.add(effect.amount).min(100);
            false
        } else {
            self.effects.push(effect);
            true
        }
    }

    pub fn delete_effect(&mut self, effect_kind: EffectKind) {
        self.effects.retain(|ef| ef.kind != effect_kind);
    }

    pub fn remove_effect(&mut self, effect: Effect) -> bool {
        if let Some(e) = self.effects.iter_mut().find(|ef| ef.kind == effect.kind) {
            e.amount = e.amount.sub(effect.amount).max(0);
            if e.amount <= 0 {
                self.effects.retain(|ef| ef.kind != effect.kind);
                return true;
            }
        }

        false
    }

    pub fn get_effect(&self, effect: EffectKind) -> Option<Effect> {
        self.effects.iter().find(|e| e.kind == effect).copied()
    }

    pub fn has_personality(&self, personality: Personality) -> bool {
        self.personalities.contains(&personality)
    }

    pub fn recalculate_pl(&mut self) {
        let weighted_skills = {
            let mut weight = 0.0;
            for skill in self.skills.iter() {
                let cost = skill.base_kind.knowledge_cost();
                weight += (cost as f64) / 0.2;
            }

            weight / 5.0
        };

        self.pl = calculate_power_level(
            self.vitality,
            self.resistance,
            self.ether,
            self.strength_level,
            self.intelligence_level,
            self.power,
            weighted_skills,
        );
    }

    pub fn regenerate_all(&mut self) {
        self.vitality.value = self.vitality.max;
        self.resistance.value = self.resistance.max;
        self.ether.value = self.ether.max;
    }

    pub fn remove_item(&mut self, item: Item, amount: usize) {
        self.inventory.iter_mut().for_each(|i| {
            if i.item.identifier == item.identifier {
                i.quantity = i.quantity.saturating_sub(amount);
            }
        });

        self.inventory.retain(|i| i.quantity > 0);
    }

    pub fn heal(&mut self, _culprit: FighterIndex, amount: i32) {
        let resistance_heal = (amount + self.vitality.value) - self.vitality.max;
        if resistance_heal > 0 {
            if self.flags.contains(FighterFlags::RISKING_LIFE) {
                self.flags.remove(FighterFlags::RISKING_LIFE);
                self.flags.remove(FighterFlags::ASKED_TO_RISK_LIFE);
            }

            self.vitality.value = self.vitality.max;
            self.resistance.value = self
                .resistance
                .value
                .add(resistance_heal)
                .min(self.resistance.max);
        } else {
            self.vitality.value = self.vitality.value.add(amount).min(self.vitality.max);
        }
    }

    pub fn take_damage(&mut self, culprit: FighterIndex, damage: DamageSpecifier) {
        let vital_damage = damage.amount - self.resistance.value;

        if vital_damage > 0 {
            self.resistance.value = 0;
            self.vitality.value = self.vitality.value.sub(vital_damage).max(0);
        } else {
            self.resistance.value = self.resistance.value.sub(damage.amount).max(0);
        }

        if damage.amount > 0 && culprit != self.index && Probability::new(50).generate_random_bool()
        {
            if let Some(mut state) = self.ai_state {
                state.focused_in = culprit;
            }
        }

        if !self
            .flags
            .contains(FighterFlags::HAD_A_NEAR_DEATH_EXPERIENCE)
            && self.vitality.value == 0
            && vital_damage > 0
            && vital_damage < (self.vitality.max / 4)
            && Probability::new(80).generate_random_bool()
        {
            self.flags.insert(FighterFlags::HAD_A_NEAR_DEATH_EXPERIENCE);
            self.vitality.value += (vital_damage % 4) + 1;
        }

        if self.resistance.value <= 0 && !self.flags.contains(FighterFlags::ASKED_TO_RISK_LIFE) {
            self.defeated_by = Some(culprit);
        }

        if self.vitality.value <= 0 {
            self.is_defeated = true;
            self.killed_by = Some(culprit);
            self.defeated_by = Some(culprit);
        }
    }
}
