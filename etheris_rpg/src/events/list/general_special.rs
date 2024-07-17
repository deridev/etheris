use etheris_data::items;

use self::common::all_regions;

use super::prelude::*;

pub fn special_track_miniorbs(_: EventBuildState) -> Event {
    Event {
        identifier: "special_track_miniorbs",
        spawn: EventSpawn {
            base_probability: Probability::new(40),
            weighted_regions: all_regions(1),
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("🔍"),
        message: EventMessage::Conditional(vec![
            (Condition::SimilarPowerTo(enemies::special::miniorbs()), "você viu um Miniorbs da Sorte! Ele está prestes a fugir, e você sente que seu poder é parecido com o dele. Quer persegui-lo?".to_string()),
            (Condition::StrongerThan(enemies::special::miniorbs()), "você viu um Miniorbs da Sorte! Ele está prestes a fugir, e você sente que seu poder é superior! Quer persegui-lo?".to_string()),
            (Condition::WeakerThan(enemies::special::miniorbs()), "você viu um Miniorbs da Sorte! Ele está prestes a fugir, mas você nota que o poder do Miniorbs é uma AMEAÇA para você. Quer persegui-lo mesmo sendo perigoso irritá-lo?".to_string()),
        ]),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Perseguir".to_string(),
                emoji: None,
                conditions: vec![],
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Event(special_track_miniorbs_tracking_part),
                        ..Default::default()
                    }
                ],
                extra_consequences: vec![],
                ..Default::default()
            }
        ]
    }
}

make_event!(
    didnt_found_miniorbs,
    Event {
        identifier: "didnt_found_miniorbs",
        spawn: EventSpawn::never(),
        emoji: Emoji::from_unicode("☹️"),
        message: EventMessage::Single("você não encontrou o Miniorbs! Ele provavelmente fugiu sem que você pudesse o encontrar."),
        actions: vec![],
    }
);

fn special_track_miniorbs_tracking_part(_: EventBuildState) -> Event {
    let consequences = vec![
        Consequence {
            kind: ConsequenceKind::Event(didnt_found_miniorbs),
            ..Default::default()
        },
        Consequence {
            kind: ConsequenceKind::InstantBattle(enemies::special::miniorbs()),
            ..Default::default()
        },
    ];

    Event {
        identifier: "special_track_miniorbs_tracking_part",
        spawn: EventSpawn::never(),
        emoji: Emoji::from_unicode("🔍"),
        message: EventMessage::Single(
            "o Miniorbs se escondeu! Você notou um árvore suspeita em que ele pode ter subido, e também viu um buraco que cabe um Miniorbs. Qual você quer olhar?"
        ),
        actions: vec![
            Action {
                name: "Olhar Árvore".to_string(),
                emoji: Some(Emoji::from_unicode("🌳")),
                conditions: vec![],
                consequences: consequences.clone(),
                extra_consequences: vec![],
                ..Default::default()
            },
            Action {
                name: "Olhar Buraco".to_string(),
                emoji: Some(Emoji::from_unicode("🔍")),
                conditions: vec![],
                consequences: consequences.clone(),
                extra_consequences: vec![],
                ..Default::default()
            }
        ]
    }
}

make_event!(
    creative_general_mysterious_portal,
    Event {
        identifier: "creative_general_mysterious_portal",
        spawn: EventSpawn {
            base_probability: Probability::new(8),
            weighted_regions: all_regions(1),
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("🌀"),
        message: EventMessage::Single("você se depara com um portal misterioso flutuando no ar. Ele emite uma luz pulsante em diferentes cores. O que você quer fazer?"),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Entrar no Portal Vermelho".to_string(),
                emoji: Some(Emoji::from_unicode("🔴")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(70),
                        kind: ConsequenceKind::Rewards {
                            message: "você é envolvido por uma energia ardente que fortalece seu corpo!".to_string(),
                            iterations: 0,
                            items: vec![],
                            orbs: (0, 0),
                            xp: XpReward {
                                strength: (30, 50),
                                health: (20, 40),
                                ..Default::default()
                            }
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(30),
                        kind: ConsequenceKind::Prejudice {
                            message: "o portal se fecha repentinamente, causando dano no seu corpo!".to_string(),
                            items_amount: (0, 0),
                            max_item_valuability: 0,
                            fixed_orbs: (0, 0),
                            orbs_percentage: 0.0,
                            specific_items: vec![],
                            damage_percentage: 0.2,
                            damage_limit: 100
                        },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Entrar no Portal Azul".to_string(),
                emoji: Some(Emoji::from_unicode("🔵")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(70),
                        kind: ConsequenceKind::Rewards {
                            message: "uma onda de conhecimento inunda sua mente!".to_string(),
                            iterations: 0,
                            items: vec![],
                            orbs: (0, 0),
                            xp: XpReward {
                                intelligence: (30, 50),
                                knowledge: (30, 50),
                                ..Default::default()
                            }
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(30),
                        kind: ConsequenceKind::Prejudice {
                            message: "o portal colapsa, roubando parte de sua fortuna!".to_string(),
                            items_amount: (0, 0),
                            max_item_valuability: 0,
                            fixed_orbs: (20, 50),
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
                name: "Entrar no Portal Verde".to_string(),
                emoji: Some(Emoji::from_unicode("🟢")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(70),
                        kind: ConsequenceKind::Rewards {
                            message: "você é transportado para um local cheio de recursos!".to_string(),
                            iterations: 3,
                            items: vec![
                                (Probability::new(80), items::consumable::WATER, (1, 3)),
                                (Probability::new(70), items::consumable::APPLE, (1, 3)),
                                (Probability::new(60), items::material::STONE, (2, 5)),
                                (Probability::new(50), items::material::STICK, (2, 4)),
                                (Probability::new(40), items::ore::COAL_ORE, (1, 3)),
                                (Probability::new(30), items::ore::IRON_ORE, (1, 2)),
                                (Probability::new(30), items::ore::LEAD_ORE, (1, 2)),
                                (Probability::new(30), items::ore::TIN_ORE, (1, 2)),
                                (Probability::new(10), items::special::GIFT, (1, 1)),
                            ],
                            orbs: (10, 50),
                            xp: XpReward::default()
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(30),
                        kind: ConsequenceKind::Message {
                            message: "o portal se fecha antes que você possa entrar. Nada acontece.".to_string(),
                            emoji: Some(Emoji::from_unicode("😕"))
                        },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        ]
    }
);

pub fn special_ether_fountain(state: EventBuildState) -> Event {
    let low_recovery_amount = (state.character.stats.ether.max as f32 * 0.20) as i32;
    let medium_recovery_amount = (state.character.stats.ether.max as f32 * 0.50) as i32;
    let high_recovery_amount = (state.character.stats.ether.max as f32 * 1.00) as i32;

    Event {
        identifier: "special_ether_fountain",
        spawn: EventSpawn {
            base_probability: Probability::new(20),
            weighted_regions: all_regions(1),
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("✨"),
        message: EventMessage::Single(
            "você encontra uma fonte brilhante de ether. A energia pura emana dela, prometendo restaurar seu poder, mas também apresentando riscos."
        ),
        actions: vec![
            Action {
                name: "Recuperação Pequena".to_string(),
                emoji: Some(Emoji::from_unicode("🍵")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(100),
                        kind: ConsequenceKind::Message {
                            message: "você se aproxima com cautela e absorve uma pequena quantidade de ether. Você recupera 20% do seu ether máximo sem nenhum risco.".to_string(),
                            emoji: Some(Emoji::from_unicode("✨"))
                        },
                        extra_consequences: vec![
                            Consequence {
                                kind: ConsequenceKind::AddEther(low_recovery_amount),
                                ..Default::default()
                            }
                        ],
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Recuperação Média".to_string(),
                emoji: Some(Emoji::from_unicode("🥤")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(70),
                        kind: ConsequenceKind::Message {
                            message: "você se aproxima mais da fonte e absorve uma quantidade moderada de ether. Você recupera 50% do seu ether máximo.".to_string(),
                            emoji: Some(Emoji::from_unicode("✨"))
                        },
                        extra_consequences: vec![
                            Consequence {
                                kind: ConsequenceKind::AddEther(medium_recovery_amount),
                                ..Default::default()
                            }
                        ],
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(30),
                        kind: ConsequenceKind::Message {
                            message: "ao tentar absorver uma quantidade maior de ether, você perde o controle e a energia se dispersa. Você perde todo o seu ether atual.".to_string(),
                            emoji: Some(Emoji::from_unicode("💨"))
                        },
                        extra_consequences: vec![
                            Consequence {
                                kind: ConsequenceKind::RemoveEther(state.character.stats.ether.value),
                                ..Default::default()
                            }
                        ],
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Recuperação Total".to_string(),
                emoji: Some(Emoji::from_unicode("🍶")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(40),
                        kind: ConsequenceKind::Message {
                            message: "você mergulha completamente na fonte de ether, arriscando tudo. Milagrosamente, você consegue absorver todo o poder e recupera 100% do seu ether máximo!".to_string(),
                            emoji: Some(Emoji::from_unicode("✨"))
                        },
                        extra_consequences: vec![
                            Consequence {
                                kind: ConsequenceKind::AddEther(high_recovery_amount),
                                ..Default::default()
                            }
                        ],
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(60),
                        kind: ConsequenceKind::Message {
                            message: "você tenta absorver todo o poder da fonte, mas a energia é demais para controlar. Uma explosão de ether ocorre, e você perde todo o seu ether atual.".to_string(),
                            emoji: Some(Emoji::from_unicode("💥"))
                        },
                        extra_consequences: vec![
                            Consequence {
                                kind: ConsequenceKind::RemoveEther(state.character.stats.ether.value),
                                ..Default::default()
                            }
                        ],
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
                            message: "você decide que os riscos são muito altos e passa pela fonte de ether sem interagir com ela.".to_string(),
                            emoji: None
                        },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        ]
    }
}
