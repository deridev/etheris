use super::prelude::*;
pub fn basic_shredder_first_encounter(_: EventBuildState) -> Event {
    const TAG: &str = "already_visited_by_a_shredder";
    const DIALOG_1: &str = "Você... É novo por aqui, certo? Heh, heh, heh. Vou te falar como as coisas funcionam por aqui em [REGION]: Nós, os Retalhadores, mandamos em tudo. Você obedece e vive, ou desobedece e é retalhado. Entendeu? Se entendeu, me passe uns orbs e não perderá sua vida.";

    Event {
        identifier: "basic_shredder_first_encounter",
        spawn: EventSpawn {
            base_probability: Probability::new(80),
            weighted_regions: vec![(WorldRegion::Gloomwood, 2), (WorldRegion::Mudland, 3)],
            conditions: vec![Condition::Not(Box::new(Condition::HasTag(TAG)))],
        },
        emoji: Emoji::from_unicode("🔪"),
        message: EventMessage::Conditional(vec![
            (Condition::SimilarPowerTo(weaklings::weak_shredder()), format!("uma pessoa mascarada com poder semelhante ao seu rapidamente colocou uma faca no seu pescoço e disse: `\"{DIALOG_1}\"`")),
            (Condition::StrongerThan(weaklings::weak_shredder()), format!("uma pessoa fraca mascarada colocou uma faca no seu pescoço e disse: `\"{DIALOG_1}\"`")),
            (Condition::WeakerThan(weaklings::weak_shredder()), format!("uma pessoa mascarada se aproximou com um poder surreal, colocou uma faca no seu pescoço e disse: `\"{DIALOG_1}\"`")),
        ]),
        actions: vec![
            Action {
                name: "\"Entendi\"".to_string(),
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Event(basic_shredder_robbery),
                        extra_consequences: vec![Consequence {
                            kind: ConsequenceKind::AddTag(TAG.to_string()),
                            ..Default::default()
                        }, Consequence {
                            kind: ConsequenceKind::AddTag("shredders_toy".to_string()),
                            ..Default::default()
                        }],
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "\"Você não me assusta.\"".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Message { message: "o Retalhador olhou para você furioso e respondeu: `\"Certo. Saiba que você acabou de decretar o seu fim, os Retalhadores nunca vão te deixar em paz.\"`, e então sumiu entre as árvores.".into(), emoji: None },
                        extra_consequences: vec![Consequence {
                            kind: ConsequenceKind::AddTag("hated_by_shredders".to_string()),
                            ..Default::default()
                        }, Consequence {
                            kind: ConsequenceKind::AddTag(TAG.to_string()),
                            ..Default::default()
                        }],
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        ],
    }
}

pub fn basic_shredder_robbery(state: EventBuildState) -> Event {
    let amount = (state.character.orbs as f64 * 0.3) as i64;

    Event {
        identifier: "basic_shredder_robbery",
        spawn: EventSpawn {
            base_probability: Probability::new(30),
            weighted_regions: vec![(WorldRegion::Gloomwood, 1), (WorldRegion::Mudland, 2)],
            conditions: vec![Condition::HasTag("shredders_toy")],
        },
        emoji: Emoji::from_unicode("🔪"),
        message: EventMessage::Conditional(vec![
            (Condition::SimilarPowerTo(weaklings::weak_shredder()), format!("um Retalhador te ameaça com uma faca e pede **{amount} ◎**! Você sente que a força do retalhador é semelhante à sua.")),
            (Condition::WeakerThan(weaklings::weak_shredder()), format!("um Retalhador te ameaça com uma faca e pede **{amount} ◎**! Você sente que a força do retalhador é extremamente maior que a sua.")),
            (Condition::StrongerThan(weaklings::weak_shredder()), format!("um Retalhador te ameaça com uma faca e pede **{amount} ◎**! Você tentou sentir a força do retalhador, mas só sentiu uma pequena pressão de ether.")),
        ]),
        actions: vec![
            Action {
                name: "Aceitar Assalto".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Prejudice { message: "o Retalhador pegou seus orbs e foi embora.".to_string(), items_amount: (0, 0), max_item_valuability: 0, fixed_orbs: (amount, amount), orbs_percentage: 0.0, specific_items: vec![], damage_percentage: 0.0, damage_limit: 0 },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Resistir".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::InstantBattle(weaklings::weak_shredder()),
                        ..Default::default()
                    },
                    Consequence {
                        kind: ConsequenceKind::AddTag("hated_by_shredders".to_string()),
                        ..Default::default()
                    },
                    Consequence {
                        kind: ConsequenceKind::RemoveTag("shredders_toy".to_string()),
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        ]
    }
}

make_event!(
    shredder_ambush,
    Event {
        identifier: "shredder_ambush",
        spawn: EventSpawn {
            base_probability: Probability::new(8),
            weighted_regions: vec![(WorldRegion::Gloomwood, 2), (WorldRegion::Mudland, 3)],
            conditions: vec![Condition::HasTag("hated_by_shredders")]
        },
        emoji: Emoji::from_unicode("🔪"),
        message: EventMessage::Single(
            "Você é emboscado por um grupo de retalhadores! O líder diz: `\"Você escolheu o caminho difícil. Agora vai pagar por isso!\"`"
        ),
        actions: vec![
            Action {
                name: "Aceitar Assalto".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Prejudice { message: "os Retalhadores te cortaram, pegaram suas coisas e foram embora.".to_string(), items_amount: (1, 4), max_item_valuability: 500, fixed_orbs: (0, 50), orbs_percentage: 0.2, specific_items: vec![], damage_percentage: 0.3, damage_limit: 300 },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Lutar".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Battle(BattleConsequence {
                            enemies: vec![
                                weaklings::weak_shredder(),
                                weaklings::weak_shredder(),
                                weaklings::weak_shredder()
                            ],
                            prompt: false,
                            ..Default::default()
                        }),
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        ]
    }
);

make_event!(
    shredder_ambush_for_payment,
    Event {
        identifier: "shredder_ambush_for_payment",
        spawn: EventSpawn {
            base_probability: Probability::new(10),
            weighted_regions: vec![(WorldRegion::Gloomwood, 2), (WorldRegion::Mudland, 3)],
            conditions: vec![Condition::HasTag("shredders_toy")]
        },
        emoji: Emoji::from_unicode("🔪"),
        message: EventMessage::Single(
            "Você é emboscado por um grupo de retalhadores. O líder, rindo, diz: `\"Hora de pagar a taxa de proteção dos Retalhadores se quiser continuar vivendo!\"`" 
        ),
        actions: vec![
            Action {
                name: "Pagar Taxa".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Prejudice {
                            message: "Você paga a taxa de proteção aos retalhadores.".to_string(),
                            items_amount: (0, 0),
                            max_item_valuability: 0,
                            fixed_orbs: (100, 200),
                            orbs_percentage: 0.0,
                            specific_items: vec![],
                            damage_percentage: 0.0,
                            damage_limit: 0
                        },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Resistir".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Battle(BattleConsequence {
                            enemies: vec![
                                weaklings::weak_shredder(),
                                weaklings::weak_shredder(),
                                weaklings::weak_shredder(),
                            ],
                            prompt: false,
                            ..Default::default()
                        }),
                        ..Default::default()
                    },
                ],
                extra_consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::AddKarma(1),
                        ..Default::default()
                    },
                    Consequence {
                        kind: ConsequenceKind::AddTag("hated_by_shredders".to_string()),
                        ..Default::default()
                    },
                    Consequence {
                        kind: ConsequenceKind::RemoveTag("shredders_toy".to_string()),
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        ]
    }
);

make_event!(shredder_training, Event {
    identifier: "shredder_training",
    spawn: EventSpawn {
        base_probability: Probability::new(10),
        weighted_regions: vec![(WorldRegion::Gloomwood, 2), (WorldRegion::Mudland, 3)],
        conditions: vec![Condition::Or(Box::new(Condition::HasTag("hated_by_shredders")), Box::new(Condition::HasTag("shredders_toy")))]
    },
    emoji: Emoji::from_unicode("🔪"),
    message: EventMessage::Single("você se depara com um campo de treinamento escondido dos retalhadores. Você vê os retalhadores praticando com facas e escuta a voz poderosa do líder. O que você quer fazer?"),
    actions: vec![
        Action {
            name: "Observar".to_string(),
            emoji: Some(Emoji::from_unicode("👀")),
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Rewards {
                        message: "você cuidadosamente observa o treinamento dos retalhadores, aprendendo algumas das suas técnicas.".to_string(),
                        iterations: 0,
                        items: vec![],
                        orbs: (0, 0),
                        xp: XpReward {
                            knowledge: (20, 50),
                            ..Default::default()
                        }
                    },
                    extra_consequences: vec![
                        Consequence {
                            probability: Probability::new(50),
                            kind: ConsequenceKind::AddActionPoint(1),
                            ..Default::default()
                        }
                    ],
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
        Action {
            name: "Atacar".to_string(),
            emoji: Some(Emoji::from_unicode("⚔️")),
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Battle(BattleConsequence {
                        enemies: vec![
                            weaklings::weak_shredder(),
                            weaklings::weak_shredder(),
                            weaklings::weak_shredder(),
                        ],
                        prompt: false,
                        on_win_knockout_event: Some(shredder_training_win),
                        ..Default::default()
                    }),
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
                    kind: ConsequenceKind::Message { message: "você silenciosamente foge do acampamento, evitando conflito.".to_string(), emoji: None },
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    ]
});

fn shredder_training_win(_: EventBuildState) -> Event {
    Event {
        identifier: "shredder_training_win",
        spawn: EventSpawn::never(),
        emoji: Emoji::from_unicode("🏆"),
        message: EventMessage::Single("o líder dos Retalhadores se surpreende com seu poder. Ele oferece uma aliança: os retalhadores te deixarão em paz, e você deixa os retalhadores em paz! O que você faz?"),
        actions: vec![
            Action {
                name: "Aceitar".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Message { message: "você está em paz com os retalhadores!".to_string(), emoji: None },
                        extra_consequences: vec![
                            Consequence {
                                kind: ConsequenceKind::RemoveTag("hated_by_shredders".to_string()),
                                ..Default::default()
                            },
                            Consequence {
                                kind: ConsequenceKind::RemoveTag("shredders_toy".to_string()),
                                ..Default::default()
                            },
                            Consequence {
                                kind: ConsequenceKind::RemoveKarma(3),
                                ..Default::default()
                            }
                        ],
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Recusar".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Message { message: "você recusa a oferta e vai embora.".to_string(), emoji: None },
                        ..Default::default()
                    }
                ],
                extra_consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::AddKarma(1),
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        ]
    }
}
