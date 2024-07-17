use etheris_data::{items, personality::Personality, ShopItem};

use super::prelude::*;

pub fn basic_desert_exploration(_state: EventBuildState) -> Event {
    Event {
        identifier: "basic_desert_exploration",
        spawn: EventSpawn {
            weighted_regions: vec![(WorldRegion::Tenypt, 10), (WorldRegion::Sandywater, 10)],
            ..Default::default()
        },
        emoji: Emoji::from_unicode("🗺️"),
        message: EventMessage::Multiple(&[
            "você está no meio de um árido deserto! Como você vai explorar?",
            "você só vê areia e dunas por todo o horizonte. Como você quer explorar essa vastidão desértica?",
        ]),
        actions: vec![
            Action {
                name: "Procurar Ameaças".to_string(),
                emoji: Some(Emoji::from_unicode("⚔️")),
                consequences: vec![
                    common::consequence_didnt_find_anything(Probability::new(5)),
                    Consequence {
                        kind: ConsequenceKind::FindARegionEnemy,
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Caminhar".to_string(),
                emoji: Some(Emoji::from_unicode("🚶")),
                consequences: vec![
                    common::consequence_didnt_find_anything(Probability::new(5)),
                    Consequence {
                        probability: Probability::new(50),
                        kind: ConsequenceKind::Rewards {
                            message: "você achou algumas coisas pelo caminho".to_string(),
                            iterations: 1,
                            items: vec![],
                            orbs: (12, 24),
                            xp: XpReward::default()
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(5),
                        kind: ConsequenceKind::FindARegionEnemy,
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(10),
                        kind: ConsequenceKind::Event(basic_desert_beginner_nomad_merchant),
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
        ]
    }
}

make_event!(
    basic_desert_digging,
    Event {
        identifier: "basic_desert_digging",
        spawn: EventSpawn {
            weighted_regions: vec![(WorldRegion::Tenypt, 4), (WorldRegion::Sandywater, 5)],
            ..Default::default()
        },
        emoji: Emoji::from_unicode("🌍"),
        message: EventMessage::Multiple(&[
            "você encontrou uma areia úmida e macia. O que deseja fazer?",
            "você achou uma área com areia menos densa. Deseja cavar?",
        ]),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Cavar".to_string(),
                emoji: Some(items::tool::SHOVEL.emoji),
                conditions: vec![Condition::HasItem(items::tool::SHOVEL, 1)],
                consequences: vec![
                    common::consequence_didnt_find_anything(Probability::new(5)),
                    Consequence {
                        kind: ConsequenceKind::Rewards {
                            message: "você encontrou algumas coisas enterradas!".to_string(),
                            iterations: 6,
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
                                (Probability::new(30), items::ore::COAL_ORE, (0, 3)),
                            ],
                            orbs: (0, 30),
                            xp: XpReward {
                                health: (0, 15),
                                intelligence: (0, 10),
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
    basic_desert_beginner_nomad_merchant,
    Event {
        identifier: "basic_desert_beginner_nomad_merchant",
        spawn: EventSpawn::never(),
        emoji: Emoji::from_unicode("💸"),
        message: EventMessage::Multiple(&[
            "um vendedor gritou de longe te chamando para comprar algo. O que você quer fazer?",
            "alguém tocou no seu ombro. Quando você olhou para trás, era um vendedor nômade. Quer dar uma olhada nos seus itens à venda?",
        ]),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Ver Loja".to_string(),
                emoji: Some(Emoji::from_unicode("🏪")),
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Shop {
                            name: "Vendedor Nômade do Deserto".to_string(),
                            items: vec![
                                ShopItem::new_item(4, items::consumable::WATER, 1.1),
                                ShopItem::new_item(15, items::consumable::WHEAT, 1.2),
                                ShopItem::new_item(6, items::consumable::APPLE, 1.1),
                                ShopItem::new_item(8, items::consumable::CHEESE, 1.2),
                                ShopItem::new_item(2, items::consumable::CHOCOLATE, 1.2),
                                ShopItem::new_item(3, items::consumable::SALT, 1.2),
                                ShopItem::new_item(3, items::consumable::SUGAR, 1.1),
                                ShopItem::new_item(1, items::tool::SHOVEL, 0.7),
                                ShopItem::new_item(1, items::tool::PICKAXE, 1.3),
                                ShopItem::new_item(1, items::tool::HAMMER, 1.1),
                                ShopItem::new_item(1, items::tool::AXE, 1.1),
                                ShopItem::new_sellable_item(23, items::material::STONE, 1.2, 0.6),
                                ShopItem::new_sellable_item(15, items::material::STICK, 1.2, 0.7),
                                ShopItem::new_sellable_item(15, items::material::PAPER, 1.2, 0.7),
                                ShopItem::new_sellable_item(1, items::material::KNIFE, 1.3, 0.7),
                                ShopItem::new_sellable_item(5, items::ore::COAL_ORE, 1.4, 0.7),
                                ShopItem::new_sellable_item(0, items::ore::IRON_ORE, 1.4, 0.7),
                                ShopItem::new_sellable_item(0, items::ore::COPPER_ORE, 1.4, 0.7),

                                if Probability::new(5).generate_random_bool() {
                                    ShopItem::new_item(1, items::special::GIFT, 0.7)
                                } else {
                                    ShopItem::new_item(1, items::cosmetic::GLASSES, 1.2)
                                }
                            ]
                        },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
        ],
    }
);

make_event!(
    basic_desert_sandstorm,
    Event {
        identifier: "basic_desert_sandstorm",
        spawn: EventSpawn {
            weighted_regions: vec![(WorldRegion::Tenypt, 5), (WorldRegion::Sandywater, 3)],
            ..Default::default()
        },
        emoji: Emoji::from_unicode("🌪️"),
        message: EventMessage::Multiple(&[
            "uma violenta tempestade de areia se aproxima rapidamente. Você tem poucos momentos para decidir como reagir!",
            "o céu escurece com uma parede de areia avançando em sua direção. A tempestade parece implacável. O que você faz?",
        ]),
        actions: vec![
            Action {
                name: "Buscar Abrigo".to_string(),
                emoji: Some(Emoji::from_unicode("🏜️")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(60),
                        kind: ConsequenceKind::Rewards {
                            message: "você encontra uma formação rochosa que oferece proteção. Enquanto espera a tempestade passar, você descobre itens deixados por outros viajantes.".to_string(),
                            iterations: 3,
                            items: vec![
                                (Probability::new(80), items::consumable::WATER, (1, 3)),
                                (Probability::new(80), items::consumable::WATERMELON, (1, 3)),
                                (Probability::new(60), items::material::KNIFE, (0, 1)),
                                (Probability::new(40), items::ore::IRON_ORE, (0, 1)),
                                (Probability::new(40), items::ore::COPPER_ORE, (1, 2)),
                            ],
                            orbs: (5, 30),
                            xp: XpReward {
                                intelligence: (5, 15),
                                knowledge: (10, 20),
                                ..Default::default()
                            }
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(40),
                        kind: ConsequenceKind::Prejudice {
                            message: "você não consegue encontrar um abrigo adequado a tempo. A tempestade de areia causa alguns danos.".to_string(),
                            items_amount: (0, 0),
                            max_item_valuability: 0,
                            fixed_orbs: (0, 0),
                            orbs_percentage: 0.0,
                            specific_items: vec![],
                            damage_percentage: 0.15,
                            damage_limit: 100
                        },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Usar Ether para Criar Barreira".to_string(),
                emoji: Some(Emoji::from_unicode("🛡️")),
                conditions: vec![Condition::HasEther(20)],
                consequences: vec![
                    Consequence {
                        probability: Probability::new(70),
                        kind: ConsequenceKind::Rewards {
                            message: "sua barreira de ether repele a tempestade. Você nota objetos trazidos pelo vento se acumulando ao redor.".to_string(),
                            iterations: 3,
                            items: vec![
                                (Probability::new(90), items::material::STONE, (2, 5)),
                                (Probability::new(70), items::ore::IRON_ORE, (1, 3)),
                                (Probability::new(2), items::special::INTELLIGENCE_CRYSTAL, (0, 1)),
                            ],
                            orbs: (20, 50),
                            xp: XpReward {
                                intelligence: (20, 40),
                                strength: (10, 20),
                                ..Default::default()
                            }
                        },
                        extra_consequences: vec![
                            Consequence {
                                kind: ConsequenceKind::RemoveEther(20),
                                ..Default::default()
                            }
                        ],
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(30),
                        kind: ConsequenceKind::Message {
                            message: "sua barreira de ether falha sob a força da tempestade, mas você escapa com apenas alguns arranhões.".to_string(),
                            emoji: Some(Emoji::from_unicode("😓"))
                        },
                        extra_consequences: vec![
                            Consequence {
                                kind: ConsequenceKind::RemoveEther(20),
                                ..Default::default()
                            },
                            Consequence {
                                kind: ConsequenceKind::Prejudice {
                                    message: "você sofreu danos severos.".to_string(),
                                    items_amount: (0, 0),
                                    max_item_valuability: 0,
                                    fixed_orbs: (0, 0),
                                    orbs_percentage: 0.0,
                                    specific_items: vec![],
                                    damage_percentage: 0.25,
                                    damage_limit: 300
                                },
                                ..Default::default()
                            }
                        ],
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Enfrentar a Tempestade".to_string(),
                emoji: Some(Emoji::from_unicode("💪")),
                conditions: vec![Condition::HasPersonality(Personality::Courage)],
                consequences: vec![
                    Consequence {
                        probability: Probability::new(40),
                        kind: ConsequenceKind::Rewards {
                            message: "com coragem sobre-humana, você atravessa a tempestade e descobre um antigo tesouro desenterrado pela areia.".to_string(),
                            iterations: 1,
                            items: vec![
                                (Probability::new(100), items::special::GIFT, (1, 1)),
                                (Probability::new(80), items::ore::GOLD_ORE, (2, 4)),
                            ],
                            orbs: (50, 100),
                            xp: XpReward {
                                strength: (30, 50),
                                health: (20, 40),
                                ..Default::default()
                            }
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(60),
                        kind: ConsequenceKind::Prejudice {
                            message: "a tempestade prova ser mais forte que sua determinação. Você sofre danos e perde alguns itens na confusão.".to_string(),
                            items_amount: (1, 3),
                            max_item_valuability: 200,
                            fixed_orbs: (20, 50),
                            orbs_percentage: 0.1,
                            specific_items: vec![],
                            damage_percentage: 0.25,
                            damage_limit: 200
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
    basic_desert_oasis,
    Event {
        identifier: "basic_desert_oasis",
        spawn: EventSpawn {
            base_probability: Probability::new(10),
            weighted_regions: vec![(WorldRegion::Tenypt, 1), (WorldRegion::Sandywater, 1)],
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("🏝️"),
        message: EventMessage::Single(
            "você avista um oásis no meio do deserto. Parece refrescante, mas pode ser uma miragem."
        ),
        actions: vec![
            Action {
                name: "Investigar".to_string(),
                emoji: Some(Emoji::from_unicode("🔍")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(70),
                        kind: ConsequenceKind::Rewards {
                            message: "o oásis é real! Você encontra água fresca e algumas frutas.".to_string(),
                            iterations: 3,
                            items: vec![
                                (Probability::new(100), items::consumable::WATER, (1, 2)),
                                (Probability::new(80), items::consumable::APPLE, (1, 3)),
                                (Probability::new(80), items::consumable::ORANGE, (1, 3)),
                                (Probability::new(80), items::consumable::TOMATO, (1, 3)),
                            ],
                            orbs: (0, 0),
                            xp: XpReward {
                                health: (10, 20),
                                ..Default::default()
                            }
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(30),
                        kind: ConsequenceKind::Message {
                            message: "era apenas uma miragem. Você perde tempo e energia perseguindo uma ilusão.".to_string(),
                            emoji: Some(Emoji::from_unicode("😞"))
                        },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Ignorar".to_string(),
                emoji: Some(Emoji::from_unicode("🚶")),
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Message {
                            message: "você decide não arriscar e continua sua jornada pelo deserto.".to_string(),
                            emoji: None
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
    basic_desert_ancient_ruins,
    Event {
        identifier: "basic_desert_ancient_ruins",
        spawn: EventSpawn {
            base_probability: Probability::new(8),
            weighted_regions: vec![(WorldRegion::Tenypt, 1), (WorldRegion::Sandywater, 2)],
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("🏛️"),
        message: EventMessage::Single(
            "você descobre ruínas antigas parcialmente enterradas na areia. Elas parecem conter segredos do passado."
        ),
        actions: vec![
            Action {
                name: "Explorar as Ruínas".to_string(),
                emoji: Some(Emoji::from_unicode("🔍")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(60),
                        kind: ConsequenceKind::Rewards {
                            message: "você encontra uma câmara secreta com artefatos antigos!".to_string(),
                            iterations: 2,
                            items: vec![
                                (Probability::new(80), items::lore::GOLDEN_ROBOT_POEM, (0, 1)),
                                (Probability::new(60), items::ore::COPPER_ORE, (0, 2)),
                                (Probability::new(50), items::ore::GOLD_ORE, (1, 3)),
                                (Probability::new(30), items::special::GIFT, (0, 1)),
                                (Probability::new(15), items::special::INVIGORATING_CRYSTAL, (1, 1)),
                            ],
                            orbs: (25, 100),
                            xp: XpReward {
                                intelligence: (20, 40),
                                knowledge: (10, 20),
                                ..Default::default()
                            }
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(40),
                        kind: ConsequenceKind::FindARegionEnemy,
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Estudar as Inscrições".to_string(),
                emoji: Some(Emoji::from_unicode("📜")),
                conditions: vec![
                    Condition::Or(
                        Box::new(Condition::HasPersonality(Personality::Intelligence)),
                        Box::new(Condition::HasItem(items::tool::TRANSLATOR, 1))
                    )
                ],
                consequences: vec![
                    Consequence {
                        probability: Probability::new(80),
                        kind: ConsequenceKind::Rewards {
                            message: "você decifra as inscrições e ganha conhecimento valioso sobre a história da região.".to_string(),
                            iterations: 1,
                            items: vec![
                                (Probability::new(100), items::lore::HAKIKO_LEGEND, (1, 1)),
                            ],
                            orbs: (20, 50),
                            xp: XpReward {
                                intelligence: (40, 60),
                                knowledge: (50, 70),
                                ..Default::default()
                            }
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(20),
                        kind: ConsequenceKind::Message {
                            message: "as inscrições são muito antigas e confusas para serem decifradas.".to_string(),
                            emoji: Some(Emoji::from_unicode("😕"))
                        },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            common::ignore_action(),
        ]
    }
);

make_event!(
    basic_desert_mirage_merchant,
    Event {
        identifier: "basic_desert_mirage_merchant",
        spawn: EventSpawn {
            base_probability: Probability::new(10),
            weighted_regions: vec![(WorldRegion::Tenypt, 1), (WorldRegion::Sandywater, 1)],
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("🌇"),
        message: EventMessage::Single(
            "você vê um mercador misterioso se aproximando em meio a uma miragem do deserto. Será real ou uma ilusão?"
        ),
        actions: vec![
            Action {
                name: "Negociar".to_string(),
                emoji: Some(Emoji::from_unicode("🤝")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(70),
                        kind: ConsequenceKind::Shop {
                            name: "Mercador da Miragem".to_string(),
                            items: vec![
                                ShopItem::new_item(3, items::consumable::WATERMELON, 1.2),
                                ShopItem::new_item(2, items::consumable::LEMONADE, 1.3),
                                ShopItem::new_item(1, items::cosmetic::EYE_BANDANA, 1.5).with_description("A bandana do mais forte. Ha, ha, ha!"),
                                ShopItem::new_item(1, items::special::INVIGORATING_CRYSTAL, 2.0),
                                ShopItem::new_sellable_item(5, items::ore::DIAMOND_ORE, 1.5, 0.8),
                                if Probability::new(10).generate_random_bool() {
                                    ShopItem::new_item(1, items::special::GIFT, 0.9).with_description("Presenteie algum ente querido com essa beleza!")
                                } else {
                                    ShopItem::new_item(1, items::tool::UMBRELLA, 1.1)
                                }
                            ]
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(30),
                        kind: ConsequenceKind::Message {
                            message: "o mercador desaparece assim que você se aproxima. Era apenas uma miragem.".to_string(),
                            emoji: Some(Emoji::from_unicode("💨"))
                        },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Ignorar".to_string(),
                emoji: Some(Emoji::from_unicode("🚶")),
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Message {
                            message: "você decide não arriscar e continua sua jornada, deixando a miragem para trás.".to_string(),
                            emoji: None
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
    basic_desert_scorpion_nest,
    Event {
        identifier: "basic_desert_scorpion_nest",
        spawn: EventSpawn {
            base_probability: Probability::new(30),
            weighted_regions: vec![(WorldRegion::Tenypt, 1), (WorldRegion::Sandywater, 3)],
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("🦂"),
        message: EventMessage::Single(
            "você tropeça em um ninho de escorpiões gigantes! Eles parecem agressivos, mas suas presas podem ser valiosas."
        ),
        actions: vec![
            Action {
                name: "Enfrentar os Escorpiões".to_string(),
                emoji: Some(Emoji::from_unicode("⚔️")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(60),
                        kind: ConsequenceKind::Rewards {
                            message: "você derrota os escorpiões e coleta suas presas!".to_string(),
                            iterations: 2,
                            items: vec![
                                (Probability::new(100), items::material::SCORPION_FANG, (1, 3)),
                                (Probability::new(50), items::special::INVIGORATING_CRYSTAL, (0, 1)),
                            ],
                            orbs: (30, 70),
                            xp: XpReward {
                                strength: (30, 50),
                                health: (20, 40),
                                ..Default::default()
                            }
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(40),
                        kind: ConsequenceKind::Prejudice {
                            message: "os escorpiões são mais fortes do que você esperava. Você sofre danos e recua.".to_string(),
                            items_amount: (0, 3),
                            max_item_valuability: 100,
                            fixed_orbs: (10, 30),
                            orbs_percentage: 0.05,
                            specific_items: vec![(items::material::SCORPION_FANG, 3)],
                            damage_percentage: 0.2,
                            damage_limit: 250
                        },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Usar Ether para Acalmar".to_string(),
                emoji: Some(Emoji::from_unicode("✨")),
                conditions: vec![Condition::HasEther(15)],
                consequences: vec![
                    Consequence {
                        probability: Probability::new(80),
                        kind: ConsequenceKind::Rewards {
                            message: "você usa seu ether para acalmar os escorpiões e coletar suas presas pacificamente.".to_string(),
                            iterations: 2,
                            items: vec![
                                (Probability::new(100), items::material::SCORPION_FANG, (1, 3)),
                                (Probability::new(40), items::special::INVIGORATING_CRYSTAL, (1, 1)),
                            ],
                            orbs: (40, 80),
                            xp: XpReward {
                                intelligence: (40, 60),
                                ..Default::default()
                            }
                        },
                        extra_consequences: vec![
                            Consequence {
                                kind: ConsequenceKind::RemoveEther(15),
                                ..Default::default()
                            }
                        ],
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(20),
                        kind: ConsequenceKind::Message {
                            message: "seu ether não é suficiente para acalmar todos os escorpiões. Você recua em segurança.".to_string(),
                            emoji: Some(Emoji::from_unicode("😓"))
                        },
                        extra_consequences: vec![
                            Consequence {
                                kind: ConsequenceKind::RemoveEther(15),
                                ..Default::default()
                            }
                        ],
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Fugir".to_string(),
                emoji: Some(Emoji::from_unicode("🏃")),
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Message {
                            message: "você decide que não vale a pena o risco e foge rapidamente do ninho de escorpiões.".to_string(),
                            emoji: None
                        },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        ]
    }
);
