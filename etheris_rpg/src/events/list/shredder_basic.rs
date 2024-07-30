use etheris_data::{items, personality::Personality, weapon::WeaponKind, BrainKind, SkillKind};
use weaklings::weak_shredder;

use crate::BodyImmunities;

const TAG: &str = "knows_about_shredders";

use super::prelude::*;
pub fn basic_shredder_first_encounter(_: EventBuildState) -> Event {
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
            (Condition::SimilarPowerTo(weaklings::weak_shredder()), format!("uma pessoa mascarada com poder semelhante ao seu rapidamente colocou uma faca no seu pescoço e disse: \"{DIALOG_1}\"")),
            (Condition::StrongerThan(weaklings::weak_shredder()), format!("uma pessoa fraca mascarada colocou uma faca no seu pescoço e disse: \"{DIALOG_1}\"")),
            (Condition::WeakerThan(weaklings::weak_shredder()), format!("uma pessoa mascarada se aproximou com um poder surreal, colocou uma faca no seu pescoço e disse: \"{DIALOG_1}\"")),
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
                        kind: ConsequenceKind::Message { message: "o Retalhador olhou para você furioso e respondeu: \"Certo. Saiba que você acabou de decretar o seu fim, os Retalhadores nunca vão te deixar em paz.\", e então sumiu entre as árvores.".into(), emoji: None },
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
            conditions: vec![Condition::HasTag(TAG), Condition::HasTag("shredders_toy")],
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
            conditions: vec![Condition::HasTag(TAG), Condition::HasTag("hated_by_shredders")]
        },
        emoji: Emoji::from_unicode("🔪"),
        message: EventMessage::Single(
            "Você é emboscado por um grupo de retalhadores! O líder diz: \"Você escolheu o caminho difícil. Agora vai pagar por isso!\""
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
            conditions: vec![Condition::HasTag(TAG), Condition::HasTag("shredders_toy")]
        },
        emoji: Emoji::from_unicode("🔪"),
        message: EventMessage::Single(
            "Você é emboscado por um grupo de retalhadores. O líder, rindo, diz: \"Hora de pagar a taxa de proteção dos Retalhadores se quiser continuar vivendo!\"" 
        ),
        actions: vec![
            Action {
                name: "Pagar Taxa".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Prejudice {
                            message: "Você paga a taxa de proteção aos retalhadores.".to_string(),
                            items_amount: (1, 3),
                            max_item_valuability: 200,
                            fixed_orbs: (100, 200),
                            orbs_percentage: 0.1,
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
        base_probability: Probability::new(25),
        weighted_regions: vec![(WorldRegion::Gloomwood, 2), (WorldRegion::Mudland, 3)],
        conditions: vec![
            Condition::HasTag(TAG),
            Condition::Or(
                Box::new(Condition::HasTag("hated_by_shredders")), 
                Box::new(Condition::HasTag("shredders_toy"))
            )
        ]
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
                            knowledge: (10, 40),
                            intelligence: (10, 40),
                            ..Default::default()
                        }
                    },
                    extra_consequences: vec![
                        Consequence {
                            probability: Probability::new(50),
                            kind: ConsequenceKind::AddActionPoint(3),
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
                        on_win_kill_event: Some(shredder_training_win),
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

make_event!(
    shredder_recruitment,
    Event {
        identifier: "shredder_recruitment",
        spawn: EventSpawn {
            base_probability: Probability::new(5),
            weighted_regions: vec![(WorldRegion::Gloomwood, 2), (WorldRegion::Mudland, 3)],
            conditions: vec![
                Condition::HasTag(TAG),
                Condition::Not(Box::new(Condition::HasTag("hated_by_shredders"))),
                Condition::Not(Box::new(Condition::HasTag("shredders_toy"))),
                Condition::Not(Box::new(Condition::HasTag("shredder_member"))),
                Condition::StrongerThan(weak_shredder()),
            ]
        },
        emoji: Emoji::from_unicode("🎭"),
        message: EventMessage::Single(
            "você encontra um grupo de Retalhadores recrutando novos membros. O líder do grupo olha para você com interesse e diz: \"Ei, você parece forte. Já pensou em se juntar aos Retalhadores? Poderíamos usar alguém como você.\""
        ),
        actions: vec![
            Action {
                name: "Aceitar a oferta".to_string(),
                emoji: Some(Emoji::from_unicode("🤝")),
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Rewards {
                            message: "você concorda em se juntar aos Retalhadores. O líder sorri, satisfeito: \"Bem-vindo à família. Aqui está seu primeiro pagamento. Não nos desaponte.\"".to_string(),
                            iterations: 0,
                            items: vec![],
                            orbs: (200, 300),
                            xp: XpReward::default()
                        },
                        extra_consequences: vec![
                            Consequence {
                                kind: ConsequenceKind::AddTag("shredder_member".to_string()),
                                ..Default::default()
                            },
                            Consequence {
                                kind: ConsequenceKind::RemoveKarma(5),
                                ..Default::default()
                            },
                        ],
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Recusar educadamente".to_string(),
                emoji: Some(Emoji::from_unicode("🙅")),
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Message {
                            message: "Você recusa a oferta educadamente. O líder parece desapontado, mas acena com a cabeça: \"Tudo bem, mas pense nisso. A oferta continua de pé... por enquanto.\"".to_string(), 
                            emoji: None
                        },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Recusar agressivamente".to_string(),
                emoji: Some(Emoji::from_unicode("😠")),
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Message {
                            message: "Você recusa agressivamente, insultando os Retalhadores. O líder fica furioso: \"Você vai se arrepender disso. Marquem esse aí, pessoal. Ele é um alvo agora.\"".to_string(), 
                            emoji: None
                        },
                        extra_consequences: vec![
                            Consequence {
                                kind: ConsequenceKind::AddTag("hated_by_shredders".to_string()),
                                ..Default::default()
                            },
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

make_enemy!(
    weak_guard,
    Enemy {
        identifier: "weak_guard",
        name: "Guarda Despreparado",
        brain: BrainKind::Simple,
        boss: None,
        regions: &[],
        base_probability: Probability::ALWAYS,
        personalities: &[Personality::Courage],
        potential: EnemyPotential::Medium,
        immunities: BodyImmunities::new(),
        resistance: 230,
        vitality: 70,
        strength: 16,
        intelligence: 9,
        ether: 30,
        allies: None,
        weapon: Some(WeaponKind::Bat),
        skills: vec![
            SkillKind::TornadoKick,
            SkillKind::ImbuedPunch,
            SkillKind::Suplex,
            SkillKind::Charge,
        ],
        drop: EnemyReward {
            orbs: (10, 15),
            xp: (20, 50),
            items: vec![EnemyRewardItem {
                item: items::tool::BAT,
                amount: (1, 1),
                probability: Probability::new(100),
            }],
        },
    }
);

make_event!(
    shredder_heist,
    Event {
        identifier: "shredder_heist",
        spawn: EventSpawn {
            base_probability: Probability::new(20),
            weighted_regions: vec![(WorldRegion::Gloomwood, 1), (WorldRegion::Mudland, 3)],
            conditions: vec![
                Condition::HasTag(TAG),
                Condition::HasTag("shredder_member")
            ]
        },
        emoji: Emoji::from_unicode("💰"),
        message: EventMessage::Single(
            "um grupo de Retalhadores te chama para participar de um grande assalto a uma caravana que passará pela região. O líder da operação diz: \"Retalharemos todos e pegaremos o dinheiro! Ha, ha, ha!\""
        ),
        actions: vec![
            Action {
                name: "Participar do assalto".to_string(),
                emoji: Some(Emoji::from_unicode("🦹")),
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Battle(BattleConsequence {
                            enemies: vec![
                                weak_guard(),
                                weak_guard(),
                            ],
                            allies: vec![weak_shredder(), weak_shredder()],
                            prompt: false,
                            on_win_knockout_event: Some(shredder_heist_success),
                            on_lose_knockout_event: Some(shredder_heist_failure),
                            ..Default::default()
                        }),
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Recusar participação".to_string(),
                emoji: Some(Emoji::from_unicode("🚫")),
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Message {
                            message: "Você decide não participar do assalto. O líder da operação parece decepcionado: \"Que pena. Esperava mais de você. Talvez da próxima vez...\"".to_string(), 
                            emoji: None
                        },
                        extra_consequences: vec![
                            Consequence {
                                probability: Probability::new(40),
                                kind: ConsequenceKind::AddKarma(1),
                                ..Default::default()
                            }
                        ],
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Alertar as autoridades".to_string(),
                emoji: Some(Emoji::from_unicode("🚨")),
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Message {
                            message: "você decide alertar as autoridades sobre o assalto planejado. Os Retalhadores são pegos de surpresa e presos. Você sente que agora é odiado pelos Retalhadores.".to_string(),
                            emoji: None
                        },
                        extra_consequences: vec![
                            Consequence {
                                kind: ConsequenceKind::RemoveTag("shredder_member".to_string()),
                                ..Default::default()
                            },
                            Consequence {
                                kind: ConsequenceKind::AddTag("hated_by_shredders".to_string()),
                                ..Default::default()
                            },
                            Consequence {
                                kind: ConsequenceKind::AddKarma(3),
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

fn shredder_heist_success(_: EventBuildState) -> Event {
    Event {
        identifier: "shredder_heist_success",
        spawn: EventSpawn::never(),
        emoji: Emoji::from_unicode("🎉"),
        message: EventMessage::Single("o assalto é um sucesso! Vocês conseguem um grande butim e escapam ilesos. O líder da operação está impressionado com seu desempenho."),
        actions: vec![
            Action {
                name: "Receber sua parte".to_string(),
                emoji: Some(Emoji::from_unicode("💎")),
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Rewards {
                            message: "você recebe sua parte do butim.".to_string(),
                            iterations: 1,
                            items: vec![
                                (Probability::new(100), items::special::INVIGORATING_CRYSTAL, (1, 1)),
                                (Probability::new(100), items::special::GIFT, (1, 1)),
                                (Probability::new(30), items::special::INTELLIGENCE_CRYSTAL, (1, 1)),
                            ],
                            orbs: (100, 400),
                            xp: XpReward::default()
                        },
                        extra_consequences: vec![
                            Consequence {
                                kind: ConsequenceKind::RemoveKarma(3),
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
}

fn shredder_heist_failure(_: EventBuildState) -> Event {
    Event {
        identifier: "shredder_heist_failure",
        spawn: EventSpawn::never(),
        emoji: Emoji::from_unicode("😓"),
        message: EventMessage::Single("o assalto fracassa! Vocês são derrotados e mal conseguem escapar. O líder da operação está furioso com o fracasso."),
        actions: vec![
            Action {
                name: "Aceitar as consequências".to_string(),
                emoji: Some(Emoji::from_unicode("🙇")),
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Message {
                            message: "O líder da operação repreende vocês duramente: \"Isso foi patético! Vão ter que trabalhar dobrado para compensar esse prejuízo!\"".to_string(), 
                            emoji: None
                        },
                        extra_consequences: vec![
                            Consequence {
                                kind: ConsequenceKind::AddTag("shredder_in_debt".to_string()),
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
}
