use std::{
    fmt::Debug,
    ops::{Add, Sub},
    sync::Arc,
};

use bitflags::bitflags;
use etheris_common::{Attribute, Probability};
use etheris_data::{
    items::{self, Item},
    personality::Personality,
    weapon::WeaponKind,
    SkillKind,
};
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
            identifier: skill.data().identifier,
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
    Paralyzed,
    Curse,

    LowProtection,
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
            WeaponKind::Umbrella => "Bater",
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
pub struct AiState {
    pub focused_in: FighterIndex,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Fighter {
    pub team: u8,
    pub index: FighterIndex,
    pub target: FighterIndex,
    pub name: String,
    pub user: Option<User>,
    pub ai_state: Option<AiState>,
    pub brain: Option<FighterBrain>,

    pub pl: i64,
    pub finishers: Vec<Finisher>,

    pub image: Option<Arc<Vec<u8>>>,

    pub flags: FighterFlags,
    pub inventory: Vec<BattleItem>,
    pub personalities: Vec<Personality>,
    pub skills: Vec<FighterSkill>,
    pub effects: Vec<Effect>,
    pub modifiers: Modifiers,

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

            image: None,

            finishers,

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

    pub fn strength_multiplier(&self) -> f32 {
        1.0 + (self.strength_level as f32) * 0.25
    }

    pub fn intelligence_multiplier(&self) -> f32 {
        1.0 + (self.intelligence_level as f32) * 0.225
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
