use etheris_data::items;

use super::prelude::*;

make_event!(
    basic_rock_mining,
    Event {
        identifier: "basic_rock_mining",
        spawn: EventSpawn {
            weighted_regions: vec![
                (WorldRegion::Greenagis, 4),
                (WorldRegion::Emerelis, 4),
                (WorldRegion::Gloomwood, 3),
                (WorldRegion::Mudland, 3),
                (WorldRegion::Murkswamp, 2),
                (WorldRegion::Midgrass, 1),
                (WorldRegion::Sunreach, 4)
            ],
            ..Default::default()
        },
        emoji: Emoji::from_unicode("🪨"),
        message: EventMessage::Multiple(&[
            "você encontrou uma rocha de tamanho médio. O que deseja fazer?",
            "uma rocha chamou sua atenção. O que quer fazer com ela?",
        ]),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Minerar",
                emoji: Some(items::tool::PICKAXE.emoji),
                conditions: vec![Condition::HasItem(items::tool::PICKAXE, 1)],
                consequences: vec![Consequence {
                    probability: Probability::ALWAYS,
                    kind: ConsequenceKind::Rewards {
                        iterations: 4,
                        items: vec![
                            (Probability::ALWAYS, items::material::STONE, (1, 3)),
                            (Probability::new(40), items::ore::COAL_ORE, (0, 3)),
                            (Probability::new(30), items::ore::IRON_ORE, (0, 2)),
                            (Probability::new(15), items::ore::COPPER_ORE, (0, 1)),
                        ],
                        orbs: (0, 30),
                        xp: XpReward {
                            strength: (5, 15),
                            health: (2, 13),
                            intelligence: (0, 5),
                            knowledge: (0, 2),
                        }
                    },
                    ..Default::default()
                }],
                extra_consequences: vec![Consequence {
                    kind: ConsequenceKind::RemoveItemDurability(items::tool::PICKAXE, 1),
                    ..Default::default()
                }],
                ..Default::default()
            }
        ]
    }
);
