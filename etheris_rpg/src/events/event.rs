use std::fmt::Debug;

use etheris_common::Probability;
use etheris_data::{items::Item, world::regions::WorldRegion};
use etheris_discord::Emoji;
use etheris_framework::CommandContext;

use crate::{data::enemies::Enemy, list::prelude::XpReward};

use super::list::{EventBuildState, EventBuilder};
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventSpawn {
    pub conditions: Vec<Condition>,
    pub base_probability: Probability,
    pub weighted_regions: Vec<(WorldRegion, i32)>,
}

impl EventSpawn {
    pub const fn never() -> Self {
        Self {
            conditions: Vec::new(),
            base_probability: Probability::NEVER,
            weighted_regions: Vec::new(),
        }
    }
}

impl Default for EventSpawn {
    fn default() -> Self {
        Self {
            conditions: Vec::new(),
            base_probability: Probability::ALWAYS,
            weighted_regions: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum EventMessage {
    Single(&'static str),
    Multiple(&'static [&'static str]),
    Conditional(Vec<(Condition, String)>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Condition {
    Not(Box<Condition>),
    HasItem(Item, usize),
    SimilarPowerTo(Enemy),
    StrongerThan(Enemy),
    WeakerThan(Enemy),
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Event {
    pub identifier: &'static str,
    pub emoji: Emoji<'static>,
    pub spawn: EventSpawn,
    pub message: EventMessage,
    pub actions: Vec<Action>,
}

impl Default for Event {
    fn default() -> Self {
        Self {
            identifier: "default_event",
            actions: Vec::new(),
            emoji: Emoji::Unicode("🌍"),
            message: EventMessage::Single("default_event_message"),
            spawn: EventSpawn::default(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Action {
    pub name: &'static str,
    pub probability: Probability,
    pub emoji: Option<Emoji<'static>>,
    pub conditions: Vec<Condition>,
    pub consequences: Vec<Consequence>,
    pub extra_consequences: Vec<Consequence>,
}

impl Default for Action {
    fn default() -> Self {
        Self {
            name: "default",
            conditions: Vec::new(),
            consequences: Vec::new(),
            emoji: None,
            extra_consequences: Vec::new(),
            probability: Probability::ALWAYS,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Consequence {
    pub probability: Probability,
    pub conditions: Vec<Condition>,
    pub kind: ConsequenceKind,
    pub extra_consequences: Vec<Consequence>,
}

impl Default for Consequence {
    fn default() -> Self {
        Self {
            probability: Probability::ALWAYS,
            conditions: Vec::new(),
            kind: ConsequenceKind::default(),
            extra_consequences: Vec::new(),
        }
    }
}

#[async_trait::async_trait]
pub trait CustomConsequence {
    async fn execute<'a>(&self, ctx: &'a mut CommandContext) -> anyhow::Result<()>;
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ConsequenceKind {
    Event(EventBuilder),
    Encounter(Enemy),
    InstantBattle(Enemy),
    MultiplePossibleEncounters(Vec<Enemy>),
    Rewards {
        iterations: usize,
        items: Vec<(Probability, Item, (i32, i32))>,
        orbs: (i64, i64),
        xp: XpReward,
    },
    Prejudice {
        items_amount: (usize, usize),
        max_item_valuability: usize,
        fixed_orbs: (i64, i64),
        orbs_percentage: f64,
        specific_items: Vec<(Item, usize)>,
    },
    RemoveItemDurability(Item, u32),
}

impl Default for ConsequenceKind {
    fn default() -> Self {
        Self::Event(make_default_event)
    }
}

fn make_default_event(_: EventBuildState) -> Event {
    Event::default()
}
