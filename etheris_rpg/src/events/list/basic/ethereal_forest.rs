use etheris_data::items;

use super::prelude::*;

make_event!(
    basic_ethereal_forest_digging,
    Event {
        identifier: "basic_ethereal_forest_digging",
        spawn: EventSpawn {
            weighted_regions: vec![(WorldRegion::Ethergrove, 4), (WorldRegion::Starbreeze, 2),],
            ..Default::default()
        },
        emoji: Emoji::from_unicode("🌍"),
        message: EventMessage::Multiple(&[
            "você encontrou um solo macio e facilmente escavável. O que deseja fazer?",
            "você achou uma terra escavável. Deseja cavar?",
        ]),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Cavar".to_string(),
                emoji: Some(items::tool::SHOVEL.emoji),
                conditions: vec![Condition::HasItem(items::tool::SHOVEL, 1)],
                consequences: vec![
                    common::consequence_didnt_find_anything(Probability::new(10)),
                    Consequence {
                        kind: ConsequenceKind::Rewards {
                            message: "você encontrou algumas coisas enterradas!".to_string(),
                            iterations: 5,
                            items: vec![
                                (Probability::new(60), items::material::STONE, (1, 5)),
                                (Probability::new(40), items::material::RAW_TRUNK, (1, 1)),
                                (Probability::new(40), items::material::PAPER, (1, 2)),
                                (Probability::new(20), items::material::TOOL_HANDLE, (1, 1)),
                                (Probability::new(40), items::consumable::APPLE, (0, 1)),
                                (Probability::new(40), items::consumable::WATER, (0, 1)),
                                (Probability::new(60), items::consumable::SALT, (0, 2)),
                                (Probability::new(60), items::consumable::SUGAR, (0, 2)),
                                (Probability::new(60), items::consumable::TOMATO, (0, 2)),
                                (Probability::new(60), items::consumable::CORN, (0, 1)),
                                (Probability::new(30), items::ore::COAL_ORE, (0, 3)),
                                (Probability::new(30), items::ore::COPPER_ORE, (0, 3)),
                            ],
                            orbs: (0, 30),
                            xp: XpReward {
                                health: (0, 15),
                                intelligence: (0, 12),
                                strength: (0, 15),
                                knowledge: (0, 8)
                            }
                        },
                        ..Default::default()
                    }
                ],
                extra_consequences: vec![Consequence {
                    kind: ConsequenceKind::RemoveItemDurability(items::tool::SHOVEL, 1),
                    ..Default::default()
                }],
                ..Default::default()
            },
        ]
    }
);

make_event!(
    ethereal_forest_whispering_trees,
    Event {
        identifier: "ethereal_forest_whispering_trees",
        spawn: EventSpawn {
            base_probability: Probability::new(30),
            weighted_regions: vec![(WorldRegion::Ethergrove, 2), (WorldRegion::Starbreeze, 2)],
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("🌳"),
        message: EventMessage::Single(
            "enquanto você caminha entre a floresta etérea, você ouve sussurros vindo das árvores. O que você quer fazer?"
        ),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Ouvir Cuidadosamente".to_string(),
                emoji: Some(Emoji::from_unicode("👂")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(70),
                        kind: ConsequenceKind::Rewards {
                            message: "você se concentrou e entendeu os sussurros, ganhando conhecimento etéreo!".to_string(),
                            iterations: 1,
                            items: vec![],
                            orbs: (0, 0),
                            xp: XpReward {
                                health: (0, 0),
                                intelligence: (30, 60),
                                strength: (0, 0),
                                knowledge: (50, 60)
                            }
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(30),
                        kind: ConsequenceKind::Message {
                            message: "os sussurros desapareceram antes que você pudesse entendê-los.".to_string(),
                            emoji: Some(Emoji::from_unicode("😔"))
                        },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Responder aos Sussurros".to_string(),
                emoji: Some(Emoji::from_unicode("🗣️")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(50),
                        kind: ConsequenceKind::Rewards {
                            message: "as árvores respondem favoravelmente, oferecendo você frutas etéreas!".to_string(),
                            iterations: 1,
                            items: vec![
                                (Probability::new(100), items::consumable::APPLE, (2, 4)),
                                (Probability::new(100), items::consumable::GREEN_APPLE, (2, 4)),
                                (Probability::new(20), items::special::INTELLIGENCE_CRYSTAL, (1, 1)),
                            ],
                            orbs: (20, 50),
                            xp: XpReward::default()
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(50),
                        kind: ConsequenceKind::FindARegionEnemy,
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        ]
    }
);

make_event!(
    ethereal_forest_glowing_pond,
    Event {
        identifier: "ethereal_forest_glowing_pond",
        spawn: EventSpawn {
            base_probability: Probability::new(25),
            weighted_regions: vec![(WorldRegion::Ethergrove, 2), (WorldRegion::Starbreeze, 2)],
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("🌟"),
        message: EventMessage::Single(
            "você encontra um pequeno lago emitindo uma luz etérea brilhante. O que você quer fazer?"
        ),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Beber do Lago".to_string(),
                emoji: Some(Emoji::from_unicode("🥤")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(70),
                        kind: ConsequenceKind::Rewards {
                            message: "A água brilhante enche você de energia etérea!".to_string(),
                            iterations: 1,
                            items: vec![],
                            orbs: (0, 0),
                            xp: XpReward {
                                health: (20, 40),
                                intelligence: (40, 80),
                                strength: (20, 40),
                                knowledge: (15, 30)
                            }
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(30),
                        kind: ConsequenceKind::Prejudice {
                            message: "A água faz você sentir nauseado e desorientado.".to_string(),
                            items_amount: (0, 0),
                            max_item_valuability: 0,
                            fixed_orbs: (0, 0),
                            orbs_percentage: 0.0,
                            specific_items: vec![],
                            damage_percentage: 0.1,
                            damage_limit: 50
                        },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Coletar Água Brilhante".to_string(),
                emoji: Some(Emoji::from_unicode("🧪")),
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Rewards {
                            message: "Você pegou com cuidado a água brilhante.".to_string(),
                            iterations: 1,
                            items: vec![
                                (Probability::new(100), items::consumable::WATER, (2, 4)),
                                (Probability::new(30), items::special::INVIGORATING_CRYSTAL, (1, 1)),
                            ],
                            orbs: (10, 30),
                            xp: XpReward::default()
                        },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        ]
    }
);

make_event!(
    basic_forest_wishing_tree,
    Event {
        identifier: "basic_forest_wishing_tree",
        spawn: EventSpawn {
            base_probability: Probability::new(10),
            weighted_regions: vec![(WorldRegion::Starbreeze, 1), (WorldRegion::Ethergrove, 1)],
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("🌳"),
        message: EventMessage::Single(
            "você encontra uma árvore antiga com folhas douradas. Uma voz sussurra: \"Faça um desejo, mas lembre-se: o universo mantém o equilíbrio.\" O que você deseja?"
        ),
        actions: vec![
            Action {
                name: "Desejo riquezas".to_string(),
                emoji: Some(Emoji::from_unicode("💰")),
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Rewards {
                            message: "galhos da árvore se curvam, derrubando moedas de ouro!".to_string(),
                            iterations: 1,
                            items: vec![],
                            orbs: (80, 300),
                            xp: XpReward::default()
                        },
                        extra_consequences: vec![
                            Consequence {
                                kind: ConsequenceKind::RemoveKarma(2),
                                ..Default::default()
                            },
                        ],
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Desejo sabedoria".to_string(),
                emoji: Some(Emoji::from_unicode("📚")),
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Rewards {
                            message: "você sente sua mente se expandir com conhecimentos antigos!".to_string(),
                            iterations: 1,
                            items: vec![(Probability::new(100), items::special::INTELLIGENCE_CRYSTAL, (1, 1))],
                            orbs: (0, 0),
                            xp: XpReward {
                                health: (0, 0),
                                intelligence: (30, 80),
                                strength: (0, 0),
                                knowledge: (15, 30)
                            }
                        },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Desejo o bem de todos".to_string(),
                emoji: Some(Emoji::from_unicode("🕊️")),
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::ConditionalConsequence {
                            condition: Condition::HasKarma(5),
                            consequence: Box::new(ConsequenceKind::Rewards {
                                message: "a árvore brilha intensamente. Você se sente revigorado e abençoado!".to_string(),
                                iterations: 1,
                                items: vec![(Probability::new(100), items::special::INVIGORATING_CRYSTAL, (1, 1))],
                                orbs: (100, 200),
                                xp: XpReward {
                                    health: (30, 60),
                                    intelligence: (30, 60),
                                    strength: (30, 60),
                                    knowledge: (10, 25)
                                }
                            }),
                            else_consequence: Some(Box::new(ConsequenceKind::Message {
                                message: "a árvore estremece levemente. Você sente que seu desejo era nobre, mas faltou algo...".to_string(),
                                emoji: Some(Emoji::from_unicode("😔"))
                            }))
                        },
                        extra_consequences: vec![
                            Consequence {
                                kind: ConsequenceKind::AddKarma(1),
                                ..Default::default()
                            }
                        ],
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        ]
    }
);
