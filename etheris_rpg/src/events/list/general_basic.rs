use etheris_data::items;

use self::common::all_regions;

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
                (WorldRegion::Sunreach, 4),
                (WorldRegion::Wornpeaks, 5),
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
                name: "Minerar".to_string(),
                emoji: Some(items::tool::PICKAXE.emoji),
                conditions: vec![Condition::HasItem(items::tool::PICKAXE, 1)],
                consequences: vec![Consequence {
                    probability: Probability::ALWAYS,
                    kind: ConsequenceKind::Rewards {
                        message: "a pedra quebrou e você pegou alguns materiais!".to_string(),
                        iterations: 4,
                        items: vec![
                            (Probability::ALWAYS, items::material::STONE, (1, 3)),
                            (Probability::new(40), items::ore::COAL_ORE, (0, 3)),
                            (Probability::new(30), items::ore::IRON_ORE, (0, 2)),
                            (Probability::new(15), items::ore::COPPER_ORE, (0, 1)),
                            (Probability::new(2), items::ore::GOLD_ORE, (0, 1)),
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

make_event!(general_basic_place_to_meditate, Event {
    identifier: "general_basic_place_to_meditate",
    spawn: EventSpawn {
        base_probability: Probability::new(30),
        weighted_regions: all_regions(1),
        conditions: vec![]
    },
    emoji: Emoji::from_unicode("🌅"),
    message: EventMessage::Single("você encontrou um lugar que parece ser um lugar de meditação. Você sente que a luz do sol é mais aconchegante aqui. Quer se concentrar e meditar?"),
    actions: vec![
        common::ignore_action(),
        Action {
            name: "Meditar".to_string(),
            emoji: None,
            conditions: vec![],
            consequences: vec![
                Consequence {
                    probability: Probability::new(70),
                    kind: ConsequenceKind::Rewards {
                        message: "você passou algumas horas meditando e sente seu corpo leve. Você ganhou pontos de ação!".to_string(), 
                        iterations: 1, items: vec![], orbs: (0, 0),
                        xp: XpReward {
                            health: (10, 20), intelligence: (10, 15), strength: (0, 5), knowledge: (10, 30)
                        }
                    },
                    extra_consequences: vec![Consequence {
                        kind: ConsequenceKind::AddActionPoint(3),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                Consequence {
                    probability: Probability::new(30),
                    kind: ConsequenceKind::Prejudice {
                        message: "alguém te roubou enquanto você meditava!".to_string(),
                        items_amount: (1, 5),
                        max_item_valuability: 200,
                        fixed_orbs: (50, 100),
                        orbs_percentage: 0.3,
                        specific_items: vec![],
                        damage_percentage: 0.0,
                        damage_limit: 0,
                    },
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    ],
});
