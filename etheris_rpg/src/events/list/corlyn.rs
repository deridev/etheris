use etheris_data::items;

use super::prelude::*;

const EMOJI: Emoji = Emoji::from_emote(Some("corlyn"), 1267479446206545952);

const ANTI_TAG: &str = "ignored_by_corlyn";

pub fn corlyn_first_encounter(_: EventBuildState) -> Event {
    Event {
        identifier: "corlyn_first_encounter",
        spawn: EventSpawn {
            base_probability: Probability::new(60),
            weighted_regions: vec![(WorldRegion::Murkswamp, 1), (WorldRegion::Sunreach, 1)],
            conditions: vec![Condition::Not(Box::new(Condition::HasTag(
                "has_known_corlyn",
            )))],
        },
        emoji: EMOJI,
        message: EventMessage::Multiple(&[
            "você vê um jovem misterioso murmurando palavras no chão. O que você quer fazer?",
            "você vê um homem ajoelhado e sussurrando palavras. O que você quer fazer?",
        ]),
        actions: vec![
            common::ignore_action_with_extra_consequences(vec![Consequence {
                kind: ConsequenceKind::AddTag("has_known_corlyn".to_string()),
                ..Default::default()
            }]),
            Action {
                name: "\"Olá?\"".to_string(),
                emoji: None,
                consequences: vec![Consequence {
                    kind: ConsequenceKind::Event(corlyn_first_encounter_interaction),
                    ..Default::default()
                }],
                extra_consequences: vec![Consequence {
                    kind: ConsequenceKind::AddTag("has_known_corlyn".to_string()),
                    ..Default::default()
                }],
                ..Default::default()
            },
        ],
    }
}

fn corlyn_first_encounter_interaction(_: EventBuildState) -> Event {
    Event {
        identifier: "corlyn_first_encounter_interaction",
        spawn: EventSpawn::never(),
        emoji: EMOJI,
        message: EventMessage::Single(
            "o homem te encara. Ele diz: \"Você... Você é um avatar? O que você busca? O que você quer?\""
        ),
        actions: vec![
            Action {
                name: "Ir embora".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Message { message: "enquanto se afasta, o homem atrás de você permanece em silêncio.".to_string(), emoji: None },
                        ..Default::default()
                    },
                ],
                extra_consequences: vec![Consequence {
                    kind: ConsequenceKind::AddTag(ANTI_TAG.to_string()),
                    ..Default::default()
                }],
                ..Default::default()
            },
            Action {
                name: "Se apresentar".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Event(corlyn_first_interaction_question),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
        ]
    }
}

fn corlyn_first_interaction_question(_: EventBuildState) -> Event {
    Event {
        identifier: "corlyn_first_interaction_question",
        spawn: EventSpawn::never(),
        emoji: EMOJI,
        message: EventMessage::Single("o homem escuta com atenção e se apresenta. \"Nesta terra me chamam de Corlyn. Mas diga-me, [NAME], a ruína já te tocou ou ainda há de tocar?\""),
        actions: vec![
            Action {
                name: "\"Ruína?\"".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Event(corlyn_first_quest),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
            Action {
                name: "\"Você é louco.\"".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Message { message: "Corlyn te olha decepcionado. \"Por um instante pensei que você fosse o avatar que nós esperávamos... Vejo que me enganei.\". Depois de dizer isso, Corlyn dá as costas para você.".to_string(), emoji: Some(EMOJI) },
                        ..Default::default()
                    },
                ],
                extra_consequences: vec![Consequence {
                    kind: ConsequenceKind::AddTag(ANTI_TAG.to_string()),
                    ..Default::default()
                }],
                ..Default::default()
            }
        ]
    }
}

fn corlyn_first_quest(_: EventBuildState) -> Event {
    Event {
        identifier: "corlyn_first_quest",
        spawn: EventSpawn::never(),
        emoji: EMOJI,
        message: EventMessage::Single("Corlyn olha para você e começa a falar. \"Ó, avatar nomeado [NAME], perdoe minha falta de explicações, mas eu garanto que você será recompensado na chegada da era da abnegação! Sim, sim... Eu só peço uma coisa, caro avatar. Traga-me uma katana vermelha e profanada, e eu te recompensarei.\""),
        actions: vec![
            Action {
                name: "Recusar tarefa".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Message { message: "Corlyn te olha decepcionado. \"Envergonho-me por ter chamado você de avatar. Perdoe-me rainha de Yiuricat, nosso avatar prometido ainda não chegou...\" - Corlyn te dá as costas.".to_string(), emoji: Some(EMOJI) }, 
                        ..Default::default()
                    },
                ],
                extra_consequences: vec![Consequence {
                    kind: ConsequenceKind::AddTag(ANTI_TAG.to_string()),
                    ..Default::default()
                }],
                ..Default::default()
            },
            Action {
                name: "Aceitar tarefa".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Message { message: "Corlyn te encara feliz. \"Sim! Traga a katana profanada até Icefields, avatar. Será a prova de sua lealdade a Yiuricat.\"".to_string(), emoji: Some(EMOJI) }, 
                        ..Default::default()
                    },
                ],
                extra_consequences: vec![Consequence {
                    kind: ConsequenceKind::AddTag("corlyn_quest".to_string()),
                    ..Default::default()
                }],
                ..Default::default()
            },
        ]
    }
}

pub fn corlyn_quest_icefields(_: EventBuildState) -> Event {
    Event {
        identifier: "corlyn_quest_icefields",
        spawn: EventSpawn {
            base_probability: Probability::new(50),
            weighted_regions: vec![(WorldRegion::Icefields, 1)],
            conditions: vec![
                Condition::HasTag("corlyn_quest"), 
                Condition::Not(Box::new(Condition::HasTag(ANTI_TAG)))
            ],
        },
        emoji: EMOJI,
        message: EventMessage::Single("você reencontra Corlyn, que te vê e se aproxima. \"Avatar [NAME]! Já conseguiu a katana profanada?\""),
        actions: vec![
            Action {
                name: "Entregar Katana Profanada".to_string(),
                emoji: Some(items::tool::ETHRIA_KATANA.emoji),
                conditions: vec![Condition::HasItem(items::tool::ETHRIA_KATANA, 1)],
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Message {
                            emoji: Some(EMOJI),
                            message: "Corlyn pega a Katana de Ethria. \"Entendo. Então Ethria realmente abandonou Yiuricat. Uma pena mesmo. Obrigado, avatar. Aguardaremos a Era da Abnegação juntos!\". Depois de dizer isso, Corlyn te entrega uma bolsa com vários itens e dá as costas em meio à neve.".to_string(), 
                        },
                        ..Default::default()
                    },
                ],
                extra_consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::RemoveTag("corlyn_quest".to_string()),
                        ..Default::default()
                    },
                    Consequence {
                        kind: ConsequenceKind::RemoveItem(items::tool::ETHRIA_KATANA, 1),
                        ..Default::default()
                    },
                    Consequence {
                        kind: ConsequenceKind::Rewards {
                            message: "você recebeu uma bolsa com vários itens!".to_string(),
                            iterations: 3,
                            items: vec![
                                (Probability::new(100), items::special::INVIGORATING_CRYSTAL, (1, 1)),
                                (Probability::new(100), items::special::INVIGORATING_CRYSTAL, (1, 1)),
                                (Probability::new(100), items::special::GIFT, (1, 1)),
                                (Probability::new(100), items::special::GIFT, (1, 1)),
                                (Probability::new(100), items::special::TRAP, (1, 1)),
                                (Probability::new(100), items::special::INTELLIGENCE_CRYSTAL, (1, 1)),
                            ],
                            orbs: (0, 0),
                            xp: XpReward {
                                health: (0, 0),
                                intelligence: (0, 0),
                                strength: (0, 0),
                                knowledge: (0, 0)
                            }
                        },
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
            Action {
                name: "\"Ainda não encontrei a katana\"".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Message {
                            emoji: Some(EMOJI),
                            message: "\"Entendo. Tome seu tempo, avatar. A era da abnegação não tem pressa.\" - Corlyn te dá as costas.".to_string(), 
                        },
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }
        ],
    }
}
