use super::super::prelude::*;
use etheris_data::{items, BossKind};

make_event!(vastorrant_first_encounter, Event {
    identifier: "vastorrant_first_encounter",
    spawn: EventSpawn {
        base_probability: Probability::new(5),
        weighted_regions: vec![(WorldRegion::Midgrass, 1)],
        conditions: vec![
            Condition::Not(Box::new(Condition::HasTag("fighting_vastorrant"))),
            Condition::Not(Box::new(Condition::DefeatedBoss(BossKind::MacrolordVastorrant))),
            Condition::HasItem(items::special::EMBLEM_OF_THE_TWO_LORDS, 1)
        ]
    },
    message: EventMessage::Single("o seu emblema te guia até um reino abandonado. Um castelo no meio dele atrai o emblema. O que você quer fazer?"),
    emoji: Emoji::from_unicode("🏰"),
    actions: vec![
        common::ignore_action(),
        Action {
            name: "Entrar".to_string(),
            emoji: Some(Emoji::from_unicode("🚶‍♂️")),
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Event(vastorrant_castle),
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
    ],
    ..Default::default()
});

make_event!(vastorrant_castle, Event {
    identifier: "vastorrant_castle",
    spawn: EventSpawn::never(),
    message: EventMessage::Single("o trono vazio do castelo arrepia até sua espinha quando um coral ao fundo te cumprimenta. \"Forasteiro que invade o refúgio do Macrolorde...\", uma voz profunda de multidão ressoa. \"Entregue o emblema que não te pertence e saia.\""),
    emoji: Emoji::from_unicode("📣"),
    actions: vec![
        Action {
            name: "Entregar o Emblema".to_string(),
            emoji: Some(items::special::EMBLEM_OF_THE_TWO_LORDS.emoji),
            conditions: vec![Condition::HasItem(items::special::EMBLEM_OF_THE_TWO_LORDS, 1)],
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Rewards {
                        message: "você entrega o emblema e cai algumas recompensas do teto. Você sai do reino.".to_string(),
                        iterations: 1,
                        items: vec![
                            (Probability::new(100), items::special::INVIGORATING_CRYSTAL, (1, 1))
                        ],
                        orbs: (0, 0),
                        xp: XpReward::default()
                    },
                    ..Default::default()
                }
            ],
            extra_consequences: vec![
                Consequence {
                    kind: ConsequenceKind::RemoveItem(items::special::EMBLEM_OF_THE_TWO_LORDS, 1),
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
        Action {
            name: "Ficar com o Emblema".to_string(),
            emoji: None,
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Event(microlord_presentation),
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
    ],
    ..Default::default()
});

make_event!(microlord_presentation, Event {
    identifier: "microlord_presentation",
    spawn: EventSpawn::never(),
    message: EventMessage::Single("uma figura fraca se revela de uma porta. \"Não permitirei que você invada o reino do Macrolorde de forma tão desrespeitosa, invasor!\", sua voz calma, quase infantil, trêmula diz. \"Eu, o Microlorde, vou te impedir aqui!\""),
    emoji: Emoji::from_unicode("👁️"),
    actions: vec![
        Action {
            name: "Fugir".to_string(),
            emoji: None,
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Message { message: "você fugiu da porta e saiu do reino em desespero.".to_string(), emoji: None },
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
        Action {
            name: "\"Pode vir pra cima!\"".to_string(),
            emoji: None,
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Event(macrolord_presentation),
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
    ],
    ..Default::default()
});

make_event!(macrolord_presentation, Event {
    identifier: "macrolord_presentation",
    spawn: EventSpawn::never(),
    message: EventMessage::Single("você sente um poder avassalador sobre você e uma voz profunda e grossa diz, de trás do Microlord: \"Ora, irmão. Já não te falei que devemos lutar juntos?\" - e então, as tochas do castelo apagam e acendem novamente, revelando o trono não mais vazio. A figura imponente te encara. \"És tu aquele que profana o Vastorránt, o Macrolorde deste vasto reino?\""),
    emoji: Emoji::from_unicode("🏴‍☠️"),
    actions: vec![
        Action {
            name: "Fugir".to_string(),
            emoji: None,
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Message { message: "você fugiu da porta e saiu do reino em desespero.".to_string(), emoji: None },
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
        Action {
            name: "\"Eu mesmo.\"".to_string(),
            emoji: None,
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Event(vastorrant_boss_battle),
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
    ],
    ..Default::default()
});

make_event!(
    vastorrant_rematch,
    Event {
        identifier: "vastorrant_rematch",
        spawn: EventSpawn {
            base_probability: Probability::new(70),
            weighted_regions: vec![(WorldRegion::Midgrass, 1)],
            conditions: vec![
                Condition::HasTag("fighting_vastorrant"),
                Condition::Not(Box::new(Condition::DefeatedBoss(
                    BossKind::MacrolordVastorrant
                )))
            ]
        },
        message: EventMessage::Single(
            "você avista novamente o reino dos dois lordes. O que você quer fazer?"
        ),
        emoji: Emoji::from_unicode("🏰"),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Ir".to_string(),
                emoji: Some(Emoji::from_unicode("🚶‍♂️")),
                consequences: vec![Consequence {
                    kind: ConsequenceKind::Event(vastorrant_boss_battle),
                    ..Default::default()
                }],
                ..Default::default()
            },
        ],
        ..Default::default()
    }
);

make_event!(vastorrant_boss_battle, Event {
    identifier: "vastorrant_boss_battle",
    spawn: EventSpawn::never(),
    message: EventMessage::Single("Vastorránt te encara furioso, e seu irmão fica em sua retaguarda. \"Você sucumbirá perante ao poder superior, invasor.\""),
    emoji: Emoji::from_unicode("🏴‍☠️"),
    actions: vec![
        Action {
            name: "Enfrentar".to_string(),
            emoji: None,
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Battle(BattleConsequence {
                        allies: vec![],
                        enemies: vec![bosses::macrolord_vastorrant(), bosses::microlord_bedialus()],
                        prompt: true,
                        on_win_kill_event: Some(vastorrant_defeated),
                        on_win_knockout_event: Some(vastorrant_defeated),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            ],
            extra_consequences: vec![
                Consequence {
                    kind: ConsequenceKind::AddTag("fighting_vastorrant".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
    ],
    ..Default::default()
});

fn vastorrant_defeated(_: EventBuildState) -> Event {
    Event {
        identifier: "vastorrant_defeated",
        spawn: EventSpawn::never(),
        message: EventMessage::Single("com a derrota dos dois lordes, o reino ao seu redor começa a ruir e quebrar. Em meio a escombros, você corre e consegue escapar, mas nota um baú no meio dos escombros."),
        actions: vec![
            Action {
                name: "Abrir Baú".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Rewards {
                            message: "você abre o baú e encontra itens dos dois lordes.".to_string(), 
                            iterations: 6,
                            items: vec![
                                (Probability::ALWAYS, items::special::INVIGORATING_CRYSTAL, (1, 1)),
                                (Probability::ALWAYS, items::special::INTELLIGENCE_CRYSTAL, (1, 1)),
                                (Probability::ALWAYS, items::special::GIFT, (1, 1)),
                                (Probability::ALWAYS, items::special::TRAP, (1, 1)),
                                (Probability::ALWAYS, items::tool::TRANSLATOR, (1, 2)),
                                (Probability::ALWAYS, items::special::INTERNAL_KEY, (3, 3)),
                            ],
                            orbs: (800, 1000),
                            xp: XpReward::default()
                        },
                        ..Default::default()
                    }
                ],
                extra_consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::RemoveTag("fighting_vastorrant".to_string()),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
        ],
        ..Default::default()
    }
}
