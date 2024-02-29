use etheris_data::items;

use super::prelude::*;

pub fn basic_forest_exploration(state: EventBuildState) -> Event {
    Event {
        identifier: "basic_forest_exploration",
        spawn: EventSpawn {
            weighted_regions: vec![(WorldRegion::Mudland, 6), (WorldRegion::Gloomwood, 6)],
            ..Default::default()
        },
        emoji: Emoji::from_unicode("🗺️"),
        message: EventMessage::Multiple(&[
            "você está no meio de uma densa floresta! Como você vai explorar?",
            "a floresta em sua frente se extende até onde seus olhos conseguem enxergar. Como você quer explorar a planície?",
        ]),
        actions: vec![
            Action {
                name: "Procurar Ameaças",
                emoji: Some(Emoji::from_unicode("⚔️")),
                consequences: vec![
                    common::consequence_didnt_find_anything(Probability::new(20)),
                    Consequence {
                        kind: ConsequenceKind::MultiplePossibleEncounters(get_enemies_by_regions(&[state.character.region])),
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Caminhar",
                emoji: Some(Emoji::from_unicode("🚶")),
                consequences: vec![
                    common::consequence_didnt_find_anything(Probability::new(5)),
                    Consequence {
                        probability: Probability::new(50),
                        kind: ConsequenceKind::Rewards { iterations: 1, items: vec![], orbs: (8, 16), xp: XpReward::default() },
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
            Action {
                name: "Cortar Árvore",
                emoji: Some(items::tool::AXE.emoji),
                conditions: vec![Condition::HasItem(items::tool::AXE, 1)],
                consequences: vec![Consequence {
                    kind: ConsequenceKind::Rewards {
                        iterations: 2,
                        items: vec![(Probability::new(100), items::material::RAW_TRUNK, (1, 3)),],
                        orbs: (0, 0),
                        xp: XpReward {
                            health: (0, 8),
                            intelligence: (0, 6),
                            strength: (0, 7),
                            knowledge: (0, 7)
                        }
                    },
                    extra_consequences: vec![Consequence {
                        kind: ConsequenceKind::RemoveItemDurability(items::tool::AXE, 1),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }
        ]
    }
}
