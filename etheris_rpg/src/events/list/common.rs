use etheris_common::Probability;
use etheris_data::world::regions::WorldRegion;
use etheris_discord::Emoji;

use crate::{events::*, make_event};

pub fn all_regions(weight: i32) -> Vec<(WorldRegion, i32)> {
    let mut regions = vec![];
    for region in WorldRegion::LIST.iter() {
        regions.push((*region, weight));
    }

    regions
}

pub fn ignore_action() -> Action {
    Action {
        name: "Ignorar",
        probability: Probability::ALWAYS,
        emoji: None,
        conditions: Vec::new(),
        consequences: Vec::new(),
        extra_consequences: Vec::new(),
    }
}

pub fn consequence_didnt_find_anything(prob: Probability) -> Consequence {
    Consequence {
        conditions: Vec::new(),
        probability: prob,
        kind: ConsequenceKind::Event(common_didnt_find_anything),
        extra_consequences: vec![],
    }
}

make_event!(
    common_didnt_find_anything,
    Event {
        identifier: "common_didnt_find_anything",
        emoji: Emoji::from_unicode("😕"),
        spawn: EventSpawn {
            conditions: Vec::new(),
            base_probability: Probability::ALWAYS,
            weighted_regions: all_regions(1),
        },
        message: EventMessage::Single("você não encontrou nada de interessante!"),
        actions: Vec::new(),
    }
);
