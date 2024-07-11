#![allow(clippy::single_match)]
use std::collections::HashSet;

use crate::EffectKind;

#[derive(Debug, Clone, PartialEq)]
pub struct Modifier {
    pub kind: ModKind,
    pub tags: HashSet<String>,
    pub turns_remaining: Option<u32>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ModKind {
    DmgMultiplier(f32),
    DefenseMultiplier(f32),
    EtherRegenMultiplier(f32),
    EffectImmunity(EffectKind),
}

impl Modifier {
    pub fn new(kind: ModKind, turns_remaining: Option<u32>) -> Self {
        Self {
            kind,
            tags: HashSet::new(),
            turns_remaining,
        }
    }

    pub fn with_tag(mut self, tag: impl ToString) -> Self {
        self.tags.insert(tag.to_string());
        self
    }

    pub fn with_tags(mut self, tags: Vec<impl ToString>) -> Self {
        self.tags.extend(tags.into_iter().map(|t| t.to_string()));
        self
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct Modifiers {
    pub list: Vec<Modifier>,
}

impl Modifiers {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, modifier: Modifier) {
        self.list.push(modifier);
    }

    pub fn remove_all_with_tag(&mut self, tag: impl ToString) {
        let tag = tag.to_string();
        self.list.retain(|m| !m.tags.contains(&tag));
    }

    pub fn remove(&mut self, modifier: Modifier) {
        self.list.retain(|m| m.kind != modifier.kind);
    }

    pub fn get_mut_with_tag(&mut self, tag: impl ToString) -> Option<&mut Modifier> {
        let tag = tag.to_string();
        self.list.iter_mut().find(|m| m.tags.contains(&tag))
    }

    pub fn overall_dmg_multiplier(&self) -> f32 {
        let mut overall_multiplier = 1.0;
        for modifier in self.list.iter() {
            match modifier.kind {
                ModKind::DmgMultiplier(multiplier) => overall_multiplier *= multiplier,
                _ => {}
            }
        }

        overall_multiplier
    }

    pub fn overall_defense_multiplier(&self) -> f32 {
        let mut overall_multiplier = 1.0;
        for modifier in self.list.iter() {
            match modifier.kind {
                ModKind::DefenseMultiplier(multiplier) => overall_multiplier *= multiplier,
                _ => {}
            }
        }

        overall_multiplier
    }

    pub fn overall_ether_regen_multiplier(&self) -> f32 {
        let mut overall_multiplier = 1.0;
        for modifier in self.list.iter() {
            match modifier.kind {
                ModKind::EtherRegenMultiplier(multiplier) => overall_multiplier *= multiplier,
                _ => {}
            }
        }

        overall_multiplier
    }
}
