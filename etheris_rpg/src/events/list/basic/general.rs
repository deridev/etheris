use etheris_data::items;
use rand::SeedableRng;

use self::common::all_regions;

use super::prelude::*;

make_event!(
    basic_general_rock_mining,
    Event {
        identifier: "basic_general_rock_mining",
        spawn: EventSpawn {
            base_probability: Probability::new(80),
            weighted_regions: vec![
                (WorldRegion::Greenagis, 2),
                (WorldRegion::Emerelis, 3),
                (WorldRegion::Gloomwood, 3),
                (WorldRegion::Mudland, 3),
                (WorldRegion::Murkswamp, 2),
                (WorldRegion::Midgrass, 1),
                (WorldRegion::Sunreach, 3),
                (WorldRegion::Wornpeaks, 3),
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

make_event!(
    basic_general_big_rock_mining,
    Event {
        identifier: "basic_general_big_rock_mining",
        spawn: EventSpawn {
            base_probability: Probability::new(40),
            weighted_regions: vec![
                (WorldRegion::Wornpeaks, 4),
                (WorldRegion::Sunreach, 2),
                (WorldRegion::Tenypt, 2),
                (WorldRegion::Sandywater, 3),
            ],
            ..Default::default()
        },
        emoji: Emoji::from_unicode("🗻"),
        message: EventMessage::Multiple(&[
            "você encontrou uma rocha massiva. O que deseja fazer?",
            "uma rocha gigante chamou sua atenção. O que quer fazer com ela?",
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
                            (Probability::ALWAYS, items::material::STONE, (1, 2)),
                            (Probability::new(60), items::ore::COAL_ORE, (0, 4)),
                            (Probability::new(50), items::ore::IRON_ORE, (0, 3)),
                            (Probability::new(50), items::ore::COPPER_ORE, (0, 3)),
                            (Probability::new(30), items::ore::LEAD_ORE, (0, 3)),
                            (Probability::new(30), items::ore::TIN_ORE, (0, 3)),
                            (Probability::new(5), items::ore::GOLD_ORE, (0, 1)),
                            (Probability::new(2), items::ore::DIAMOND_ORE, (0, 1)),
                        ],
                        orbs: (0, 40),
                        xp: XpReward {
                            strength: (15, 20),
                            health: (15, 20),
                            intelligence: (0, 7),
                            knowledge: (0, 2),
                        }
                    },
                    ..Default::default()
                }],
                extra_consequences: vec![Consequence {
                    kind: ConsequenceKind::RemoveItemDurability(items::tool::PICKAXE, 4),
                    ..Default::default()
                }],
                ..Default::default()
            }
        ]
    }
);

make_event!(basic_general_place_to_meditate, Event {
    identifier: "basic_general_place_to_meditate", 
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
                    probability: Probability::new(80),
                    kind: ConsequenceKind::Rewards {
                        message: "você passou algumas horas meditando e sente seu corpo leve. Você ganhou pontos de ação!".to_string(), 
                        iterations: 1,
                        items: vec![],
                        orbs: (0, 0),
                        xp: XpReward {
                            health: (10, 20),
                            intelligence: (10, 15),
                            strength: (0, 5),
                            knowledge: (10, 30)
                        }
                    },
                    extra_consequences: vec![Consequence {
                        kind: ConsequenceKind::AddActionPoint(3),
                        ..Default::default()
                    }],
                    ..Default::default()
                },
                Consequence {
                    probability: Probability::new(20),
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

make_event!(
    basic_general_mysterious_chest,
    Event {
        identifier: "basic_general_mysterious_chest",
        spawn: EventSpawn {
            base_probability: Probability::new(15),
            weighted_regions: all_regions(1),
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("🧰"),
        message: EventMessage::Single("você encontrou um baú misterioso. Ele parece estar trancado. O que você quer fazer?"),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Tentar Abrir".to_string(),
                emoji: Some(Emoji::from_unicode("🔓")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(70),
                        kind: ConsequenceKind::Rewards {
                            message: "você conseguiu abrir o baú e encontrou alguns itens!".to_string(),
                            iterations: 3,
                            items: vec![
                                (Probability::new(80), items::consumable::WATER, (1, 2)),
                                (Probability::new(70), items::consumable::APPLE, (1, 3)),
                                (Probability::new(50), items::material::STONE, (2, 5)),
                                (Probability::new(30), items::material::STICK, (2, 4)),
                                (Probability::new(20), items::ore::COAL_ORE, (1, 3)),
                                (Probability::new(10), items::ore::IRON_ORE, (1, 2)),
                                (Probability::new(10), items::ore::TIN_ORE, (1, 2)),
                                (Probability::new(10), items::ore::LEAD_ORE, (1, 2)),
                                (Probability::new(10), items::special::TRAP, (1, 1)),
                                (Probability::new(2), items::special::INVIGORATING_CRYSTAL, (1, 1)),
                            ],
                            orbs: (20, 50),
                            xp: XpReward {
                                intelligence: (5, 15),
                                strength: (5, 10),
                                ..Default::default()
                            }
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(30),
                        kind: ConsequenceKind::Message {
                            message: "o baú estava protegido por uma armadilha! Você não conseguiu abri-lo e machucou sua mão no processo.".to_string(),
                            emoji: Some(Emoji::from_unicode("💥"))
                        },
                        extra_consequences: vec![
                            Consequence {
                                kind: ConsequenceKind::Prejudice {
                                    message: "você se feriu!".to_string(),
                                    items_amount: (0, 0),
                                    max_item_valuability: 0,
                                    fixed_orbs: (0, 0),
                                    orbs_percentage: 0.0,
                                    specific_items: vec![],
                                    damage_percentage: 0.05,
                                    damage_limit: 80
                                },
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

pub fn basic_general_traveller_riddle(_: EventBuildState) -> Event {
    const RIDDLES: [(&str, &str, Emoji); 5] = [
        (
            "O que tem raízes que ninguém vê, é mais alto que as árvores, sobe, sobe e contudo não cresce?",
            "Montanha",
            Emoji::from_unicode("🏔️")
        ),
        (
            "O que é que quanto mais se tira, mais se aumenta?",
            "Buraco",
            Emoji::from_unicode("🕳️")
        ),
        (
            "O que é que quanto mais seca, mais molhada fica?",
            "Toalha",
            Emoji::from_unicode("🧻")
        ),
        (
            "O que é que está sempre à sua frente, mas não pode ser visto?",
            "Futuro",
            Emoji::from_unicode("🔮")
        ),
        (
            "O cai em pé e corre deitado?",
            "Chuva",
            Emoji::from_unicode("🚿")
        ),
    ];

    let mut rng = rand::rngs::StdRng::from_entropy();

    use rand::prelude::SliceRandom;
    let (riddle, correct_answer, ..) = RIDDLES.choose(&mut rng).unwrap();

    let mut actions = RIDDLES
        .iter()
        .map(|(_, answer, emoji)| Action {
            name: answer.to_string(),
            emoji: Some(*emoji),
            consequences: vec![Consequence {
                conditions: vec![],
                kind: if answer == correct_answer {
                    ConsequenceKind::Rewards {
                        message: format!(
                            "\"Correto!\", exclama o viajante. Ele lhe entrega uma recompensa."
                        ),
                        iterations: 1,
                        items: vec![(Probability::new(100), items::special::GIFT, (1, 1))],
                        orbs: (50, 100),
                        xp: XpReward {
                            intelligence: (20, 40),
                            knowledge: (20, 40),
                            ..Default::default()
                        },
                    }
                } else {
                    ConsequenceKind::Message {
                        message:
                            "\"Incorreto\", diz o viajante, desaparecendo em uma nuvem de fumaça."
                                .to_string(),
                        emoji: Some(Emoji::from_unicode("💨")),
                    }
                },
                ..Default::default()
            }],
            ..Default::default()
        })
        .collect::<Vec<_>>();

    actions.shuffle(&mut rng);

    let correct_action = actions
        .iter()
        .find(|a| &&a.name == correct_answer)
        .unwrap()
        .clone();
    actions = actions.into_iter().take(3).collect();

    if !actions.iter().any(|a| &a.name == correct_answer) {
        actions.push(correct_action);
    }

    Event {
        identifier: "basic_general_traveller_riddle",
        spawn: EventSpawn {
            base_probability: Probability::new(10),
            weighted_regions: all_regions(1),
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("🧙"),
        message: EventMessage::MultipleString(vec![
            format!("você encontra um viajante misterioso que lhe propõe um enigma. \"Responda corretamente e será recompensado\", diz ele. \"{}\"", riddle),
            format!("uma figura estranha te para e solta uma frase misteriosa. \"Responda corretamente o enigma e será recompensado\", diz ele. \"{}\"", riddle),
        ]),
        actions,
    }
}

make_event!(
    basic_general_gambler_encounter,
    Event {
        identifier: "basic_general_gambler_encounter",
        spawn: EventSpawn {
            base_probability: Probability::new(15),
            weighted_regions: all_regions(1),
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("🎲"),
        message: EventMessage::Single("um apostador te encontrou e ofereceu um jogo. Ambos apostam 100 orbs, e quem vencer leva tudo. Você quer jogar?"),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Jogar".to_string(),
                emoji: Some(Emoji::from_unicode("🪙")),
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::ConditionalConsequence {
                            condition: Condition::HasOrbs(100), // Assuming a minimum bet of 50 orbs
                            consequence: Box::new(ConsequenceKind::Event(gambler_coin_toss)),
                            else_consequence: Some(Box::new(ConsequenceKind::Message {
                                message: "você não tem orbs suficientes para apostar.".to_string(),
                                emoji: Some(Emoji::from_unicode("💸"))
                            }))
                        },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        ]
    }
);

pub fn gambler_coin_toss(_: EventBuildState) -> Event {
    Event {
        identifier: "gambler_coin_toss",
        spawn: EventSpawn::never(),
        emoji: Emoji::from_unicode("🪙"),
        message: EventMessage::Single("o apostador joga a moeda. Escolha cara ou coroa:"),
        actions: vec![
            Action {
                name: "Cara".to_string(),
                emoji: Some(Emoji::from_unicode("👑")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(50),
                        kind: ConsequenceKind::Rewards {
                            message: "deu cara! Você ganhou a aposta!".to_string(),
                            iterations: 1,
                            items: vec![],
                            orbs: (100, 100),
                            xp: XpReward::default(),
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(50),
                        kind: ConsequenceKind::Prejudice {
                            message: "deu coroa. Você perdeu a aposta.".to_string(),
                            items_amount: (0, 0),
                            max_item_valuability: 0,
                            fixed_orbs: (100, 100),
                            orbs_percentage: 0.0,
                            specific_items: vec![],
                            damage_percentage: 0.0,
                            damage_limit: 0,
                        },
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
            Action {
                name: "Coroa".to_string(),
                emoji: Some(Emoji::from_unicode("👑")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(50),
                        kind: ConsequenceKind::Rewards {
                            message: "deu coroa! Você ganhou a aposta!".to_string(),
                            iterations: 1,
                            items: vec![],
                            orbs: (100, 100),
                            xp: XpReward::default(),
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(50),
                        kind: ConsequenceKind::Prejudice {
                            message: "deu cara. Você perdeu a aposta.".to_string(),
                            items_amount: (0, 0),
                            max_item_valuability: 0,
                            fixed_orbs: (100, 100),
                            orbs_percentage: 0.0,
                            specific_items: vec![],
                            damage_percentage: 0.0,
                            damage_limit: 0,
                        },
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
        ],
    }
}

make_event!(
    basic_general_lost_pet,
    Event {
        identifier: "basic_general_lost_pet",
        spawn: EventSpawn {
            base_probability: Probability::new(20),
            weighted_regions: all_regions(1),
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("🐾"),
        message: EventMessage::Single("você encontra um animal de estimação perdido com uma coleira. O que você quer fazer?"),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Ajudar o animal".to_string(),
                emoji: Some(Emoji::from_unicode("🤲")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(70),
                        kind: ConsequenceKind::Rewards {
                            message: "você conseguiu encontrar o dono do animal! Eles ficaram muito gratos e te recompensaram.".to_string(),
                            iterations: 2,
                            items: vec![
                                (Probability::new(50), items::consumable::APPLE, (1, 2)),
                                (Probability::new(30), items::special::GIFT, (1, 1)),
                            ],
                            orbs: (50, 100),
                            xp: XpReward {
                                intelligence: (10, 20),
                                knowledge: (5, 15),
                                ..Default::default()
                            }
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(30),
                        kind: ConsequenceKind::Message {
                            message: "você não conseguiu encontrar o dono, mas sentiu que o animal ficou grato pela sua ajuda.".to_string(),
                            emoji: Some(Emoji::from_unicode("😺"))
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

make_event!(
    basic_general_broken_cart,
    Event {
        identifier: "basic_general_broken_cart",
        spawn: EventSpawn {
            base_probability: Probability::new(15),
            weighted_regions: all_regions(1),
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("🛒"),
        message: EventMessage::Single("você encontra uma carroça quebrada. O que você faz?"),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Tentar consertar".to_string(),
                emoji: Some(items::tool::HAMMER.emoji),
                conditions: vec![Condition::HasItem(items::tool::HAMMER, 1)],
                consequences: vec![Consequence {
                    kind: ConsequenceKind::Rewards {
                        message: "você consertou a carroça e o dono te recompensou!".to_string(),
                        iterations: 2,
                        items: vec![
                            (Probability::new(80), items::material::STICK, (1, 3)),
                            (Probability::new(60), items::material::RAW_TRUNK, (1, 2)),
                            (Probability::new(20), items::consumable::WATERMELON, (1, 2)),
                        ],
                        orbs: (10, 30),
                        xp: XpReward {
                            health: (0, 5),
                            intelligence: (5, 10),
                            strength: (5, 15),
                            knowledge: (5, 10)
                        }
                    },
                    ..Default::default()
                }],
                extra_consequences: vec![Consequence {
                    kind: ConsequenceKind::RemoveItemDurability(items::tool::HAMMER, 1),
                    ..Default::default()
                }],
                ..Default::default()
            },
        ]
    }
);

make_event!(
    basic_general_item_hole,
    Event {
        identifier: "basic_general_item_hole",
        spawn: EventSpawn {
            base_probability: Probability::new(40),
            weighted_regions: all_regions(1),
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("🕳️"),
        message: EventMessage::Single("você encontrou um buraco suspeito no chão. O que você faz?"),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Investigar".to_string(),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(30),
                        kind: ConsequenceKind::Prejudice {
                            message: "um animal mordeu sua mão.".to_string(),
                            items_amount: (0, 0),
                            max_item_valuability: 0,
                            fixed_orbs: (0, 0),
                            orbs_percentage: 0.0,
                            specific_items: vec![],
                            damage_percentage: 0.2,
                            damage_limit: 100,
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(70),
                        kind: ConsequenceKind::Rewards {
                            message: "você encontrou algumas coisas no buraco!".to_string(),
                            iterations: 3,
                            items: vec![
                                (Probability::new(80), items::material::STICK, (1, 3)),
                                (Probability::new(80), items::material::STONE, (1, 3)),
                                (Probability::new(70), items::material::BONE, (1, 2)),
                                (Probability::new(60), items::material::RAW_TRUNK, (1, 1)),
                                (Probability::new(50), items::material::KNIFE, (1, 1)),
                                (Probability::new(50), items::consumable::MILK, (1, 1)),
                                (Probability::new(40), items::consumable::WHEAT, (1, 2)),
                                (Probability::new(30), items::material::PLANK, (1, 1)),
                                (Probability::new(20), items::consumable::WATERMELON, (1, 2)),
                                (Probability::new(5), items::special::TRAP, (1, 1)),
                            ],
                            orbs: (10, 30),
                            xp: XpReward {
                                health: (0, 5),
                                intelligence: (5, 10),
                                strength: (5, 15),
                                knowledge: (5, 10)
                            }
                        },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
        ]
    }
);

make_event!(
    basic_general_old_well,
    Event {
        identifier: "basic_general_old_well",
        spawn: EventSpawn {
            base_probability: Probability::new(20),
            weighted_regions: all_regions(1),
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("⛲"),
        message: EventMessage::Single("você encontra um velho poço abandonado. O que você faz?"),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Coletar água".to_string(),
                emoji: None,
                consequences: vec![Consequence {
                    kind: ConsequenceKind::Rewards {
                        message: "você coletou água fresca do poço!".to_string(),
                        iterations: 1,
                        items: vec![(Probability::new(100), items::consumable::WATER, (2, 4)),],
                        orbs: (0, 5),
                        xp: XpReward {
                            health: (5, 10),
                            intelligence: (0, 5),
                            strength: (5, 10),
                            knowledge: (0, 5)
                        }
                    },
                    ..Default::default()
                }],
                ..Default::default()
            },
            Action {
                name: "Cavar no fundo".to_string(),
                emoji: Some(items::tool::SHOVEL.emoji),
                conditions: vec![Condition::HasItem(items::tool::SHOVEL, 1)],
                consequences: vec![Consequence {
                    kind: ConsequenceKind::Rewards {
                        message: "você encontrou alguns itens no fundo do poço!".to_string(),
                        iterations: 2,
                        items: vec![
                            (Probability::new(70), items::material::STONE, (1, 3)),
                            (Probability::new(50), items::consumable::WATER, (1, 2)),
                            (Probability::new(30), items::material::TOOL_HANDLE, (1, 1)),
                            (Probability::new(5), items::ore::TIN_ORE, (1, 1)),
                        ],
                        orbs: (5, 20),
                        xp: XpReward {
                            health: (0, 5),
                            intelligence: (5, 10),
                            strength: (5, 15),
                            knowledge: (5, 10)
                        }
                    },
                    ..Default::default()
                }],
                extra_consequences: vec![Consequence {
                    kind: ConsequenceKind::RemoveItemDurability(items::tool::SHOVEL, 1),
                    ..Default::default()
                }],
                ..Default::default()
            },
        ]
    }
);
