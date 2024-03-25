use std::fmt::Debug;

use etheris_common::Probability;
use etheris_data::{items::Item, personality::Personality, world::regions::WorldRegion, ShopItem};
use etheris_discord::Emoji;
use etheris_framework::CommandContext;

use crate::{data::enemies::Enemy, list::prelude::XpReward};

use super::{
    list::{EventBuildState, EventBuilder},
    ControllerAction, ControllerFlag,
};
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
    SingleString(String),
    Multiple(&'static [&'static str]),
    MultipleString(Vec<String>),
    Conditional(Vec<(Condition, String)>),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Condition {
    None,
    Not(Box<Condition>),
    Or(Box<Condition>, Box<Condition>),
    IsFlagSet(ControllerFlag),
    HasOrbs(i64),
    HasItem(Item, usize),
    HasTag(&'static str),
    HasPersonality(Personality),
    SimilarPowerTo(Enemy),
    StrongerThan(Enemy),
    WeakerThan(Enemy),
    Probability(Probability),
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
    pub name: String,
    pub probability: Probability,
    pub emoji: Option<Emoji<'static>>,
    pub conditions: Vec<Condition>,
    pub consequences: Vec<Consequence>,
    pub extra_consequences: Vec<Consequence>,
}

impl Default for Action {
    fn default() -> Self {
        Self {
            name: "default".to_string(),
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

#[derive(Debug, Clone, PartialEq, PartialOrd, Default)]
pub struct BattleConsequence {
    pub enemies: Vec<Enemy>,
    pub prompt: bool,
    pub on_win_knockout_event: Option<EventBuilder>,
    pub on_win_kill_event: Option<EventBuilder>,
    pub on_lose_knockout_event: Option<EventBuilder>,
    pub on_lose_die_event: Option<EventBuilder>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ConsequenceKind {
    Message {
        message: String,
        emoji: Option<Emoji<'static>>,
    },
    Event(EventBuilder),
    Action(ControllerAction),
    Battle(BattleConsequence),
    Encounter(Enemy),
    InstantBattle(Enemy),
    MultiplePossibleEncounters(Vec<Enemy>),
    FindARegionEnemy,
    Rewards {
        message: String,
        iterations: usize,
        items: Vec<(Probability, Item, (i32, i32))>,
        orbs: (i64, i64),
        xp: XpReward,
    },
    Prejudice {
        message: String,
        items_amount: (usize, usize),
        max_item_valuability: usize,
        fixed_orbs: (i64, i64),
        orbs_percentage: f64,
        specific_items: Vec<(Item, usize)>,
        damage_percentage: f64,
        damage_limit: i32,
    },
    Shop {
        name: String,
        items: Vec<ShopItem>,
    },
    RemoveItemDurability(Item, u32),
    RemoveItem(Item, usize),
    AddActionPoint(u32),
    AddTag(String),
    RemoveTag(String),
    AddKarma(i32),
    RemoveKarma(i32),
}

impl Default for ConsequenceKind {
    fn default() -> Self {
        Self::Event(make_default_event)
    }
}

fn make_default_event(_: EventBuildState) -> Event {
    Event::default()
}
