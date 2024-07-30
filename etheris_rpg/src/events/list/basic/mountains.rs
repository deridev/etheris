use etheris_data::{emojis, items, personality::Personality};
use weaklings_plus::trained_thief;

use super::prelude::*;
pub fn basic_mountain_exploration(_state: EventBuildState) -> Event {
    Event {
        identifier: "basic_mountain_exploration",
        spawn: EventSpawn {
            weighted_regions: vec![(WorldRegion::Wornpeaks, 5)],
            ..Default::default()
        },
        emoji: Emoji::from_unicode("🏔️"),
        message: EventMessage::Multiple(&[
            "você está no meio de uma região montanhosa imponente! Como você vai explorar?",
            "picos rochosos se erguem ao seu redor, desafiando sua coragem. Como você quer explorar essas montanhas?",
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
                name: "Escalar".to_string(),
                emoji: Some(Emoji::from_unicode("🧗")),
                consequences: vec![
                    common::consequence_didnt_find_anything(Probability::new(5)),
                    Consequence {
                        probability: Probability::new(50),
                        kind: ConsequenceKind::Rewards {
                            message: "você encontrou alguns recursos enquanto escalava!".to_string(),
                            iterations: 1,
                            items: vec![
                                (Probability::new(80), items::material::STONE, (2, 5)),
                                (Probability::new(40), items::ore::IRON_ORE, (1, 3)),
                                (Probability::new(20), items::ore::GOLD_ORE, (0, 2)),
                            ],
                            orbs: (15, 30),
                            xp: XpReward {
                                strength: (10, 20),
                                health: (5, 15),
                                ..Default::default()
                            }
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(10),
                        kind: ConsequenceKind::FindARegionEnemy,
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Minerar".to_string(),
                emoji: Some(items::tool::PICKAXE.emoji),
                conditions: vec![Condition::HasItem(items::tool::PICKAXE, 1)],
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Rewards {
                            message: "você encontrou alguns minérios valiosos!".to_string(),
                            iterations: 1,
                            items: vec![
                                (Probability::new(100), items::material::STONE, (3, 7)),
                                (Probability::new(70), items::ore::IRON_ORE, (1, 4)),
                                (Probability::new(40), items::ore::COPPER_ORE, (1, 3)),
                                (Probability::new(20), items::ore::GOLD_ORE, (0, 2)),
                            ],
                            orbs: (10, 25),
                            xp: XpReward {
                                strength: (5, 15),
                                intelligence: (0, 10),
                                knowledge: (0, 10),
                                ..Default::default()
                            }
                        },
                        extra_consequences: vec![
                            Consequence {
                                kind: ConsequenceKind::RemoveItemDurability(items::tool::PICKAXE, 1),
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

make_event!(
    basic_mountain_abandoned_campsite,
    Event {
        identifier: "basic_mountain_abandoned_campsite",
        spawn: EventSpawn {
            weighted_regions: vec![(WorldRegion::Wornpeaks, 2),],
            ..Default::default()
        },
        emoji: Emoji::from_unicode("🏕️"),
        message: EventMessage::Single(
            "você encontrou um acampamento abandonado. O que deseja fazer?"
        ),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Vasculhar".to_string(),
                emoji: Some(Emoji::from_unicode("🔍")),
                consequences: vec![
                    common::consequence_didnt_find_anything(Probability::new(20)),
                    Consequence {
                        probability: Probability::new(70),
                        kind: ConsequenceKind::Rewards {
                            message: "você encontrou algumas coisas úteis no acampamento!"
                                .to_string(),
                            iterations: 3,
                            items: vec![
                                (Probability::new(80), items::consumable::WATER, (1, 3)),
                                (Probability::new(60), items::consumable::FRIED_EGG, (1, 2)),
                                (Probability::new(40), items::consumable::APPLE, (1, 2)),
                                (Probability::new(20), items::material::STICK, (2, 5)),
                                (Probability::new(10), items::tool::SHOVEL, (1, 1)),
                            ],
                            orbs: (5, 20),
                            xp: XpReward::default()
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(10),
                        kind: ConsequenceKind::FindARegionEnemy,
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        ]
    }
);

pub fn basic_mountain_person_in_danger(_: EventBuildState) -> Event {
    let is_bad = Probability::new(40).generate_random_bool();

    let kind = if is_bad {
        ConsequenceKind::Event(basic_mountain_person_in_danger_bad)
    } else {
        ConsequenceKind::Rewards {
            message: "você ajudou uma pessoa que estava ferida e ela te recompensou.".to_string(),
            iterations: 3,
            items: vec![
                (Probability::new(100), items::consumable::WATER, (1, 2)),
                (Probability::new(100), items::consumable::APPLE, (1, 2)),
                (Probability::new(100), items::consumable::FRIED_EGG, (1, 2)),
                (Probability::new(100), items::material::STICK, (1, 2)),
                (Probability::new(60), items::material::KNIFE, (1, 1)),
                (Probability::new(20), items::material::TOOL_HANDLE, (1, 1)),
                (Probability::new(40), items::tool::PICKAXE, (1, 1)),
            ],
            orbs: (10, 30),
            xp: XpReward::default(),
        }
    };

    Event {
        identifier: "basic_mountain_person_in_danger",
        spawn: EventSpawn {
            weighted_regions: vec![
                (WorldRegion::Wornpeaks, 1),
            ],
            ..Default::default()
        },
        emoji: Emoji::from_unicode("💥"),
        message: EventMessage::Conditional(vec![
            (
                Condition::HasPersonality(Personality::Cowardice),
                if is_bad {
                    "você escuta uma pessoa gritando em perigo na borda de um penhasco, mas tem uma sensação ruim de perigo. Você quer ajudar?".to_string()
                } else {
                    "você escuta uma pessoa gritando em perigo na borda de um penhasco. Você sente que ela pode estar passando risco de vida. Você quer ajudar?".to_string()
                },
            ),
            (
                Condition::None,
                "você escuta uma pessoa gritando em perigo por perto, vindo da borda de um penhasco. Você quer ajudar?"
                    .to_string(),
            ),
        ]),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Ajudar".to_string(),
                emoji: None,
                conditions: vec![],
                consequences: vec![Consequence {
                    kind,
                    ..Default::default()
                }],
                extra_consequences: vec![],
                ..Default::default()
            },
        ],
    }
}

fn basic_mountain_person_in_danger_bad(_: EventBuildState) -> Event {
    let prejudice = ConsequenceKind::Prejudice {
        message: "você teve orbs roubados!".to_string(),
        fixed_orbs: (0, 0),
        items_amount: (0, 0),
        max_item_valuability: (0),
        orbs_percentage: 0.25,
        specific_items: vec![],
        damage_percentage: 0.0,
        damage_limit: 0,
    };

    Event {
        identifier: "basic_plains_person_in_danger_bad",
        spawn: EventSpawn::never(),
        emoji: Emoji::from_unicode("😠"),
        message: EventMessage::Conditional(vec![
            (Condition::SimilarPowerTo(trained_thief()), "você foi emboscado! Um ladrão te ameaçou com uma faca e pediu orbs! A força dele é semelhante à sua. Como você quer reagir?".to_string()),
            (Condition::StrongerThan(trained_thief()), "era uma emboscada! Um criminoso fraco se revela e fala para você entregar seus orbs. Ele não aparenta ameaça alguma para sua força. Como você quer reagir?".to_string()),
            (Condition::WeakerThan(trained_thief()), "não era uma pessoa pedindo ajuda... É uma emboscada! Um ladrão poderoso te ameaça com uma faca e pede orbs. Você sentiu uma poderosa pressão de ether vindo dele, é um inimigo perigoso. Como você quer reagir?".to_string()),
        ]),
        actions: vec![
            Action {
                name: "Aceitar Assalto".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: prejudice,
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
                        kind: ConsequenceKind::InstantBattle(trained_thief()),
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        ]
    }
}

make_event!(
    basic_montain_unstable_path,
    Event {
        identifier: "basic_montain_unstable_path",
        spawn: EventSpawn {
            weighted_regions: vec![(WorldRegion::Wornpeaks, 4)],
            ..Default::default()
        },
        emoji: Emoji::from_unicode("🏔️"),
        message: EventMessage::Single("você se depara com um caminho instável na montanha. Como você vai atravessá-lo?"),
        actions: vec![
            Action {
                name: "Pular".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        probability: Probability::new(60),
                        kind: ConsequenceKind::Message {
                            message: "você conseguiu pular com sucesso e atravessar o caminho instável!".to_string(),
                            emoji: None
                        },
                        extra_consequences: vec![
                            Consequence {
                                kind: ConsequenceKind::Rewards {
                                    message: "você encontrou alguns itens do outro lado!".to_string(),
                                    iterations: 1,
                                    items: vec![
                                        (Probability::new(80), items::ore::TIN_ORE, (1, 3)),
                                        (Probability::new(20), items::special::INVIGORATING_CRYSTAL, (1, 1)),
                                    ],
                                    orbs: (5, 35),
                                    xp: XpReward {
                                        health: (5, 15),
                                        intelligence: (0, 5),
                                        strength: (10, 20),
                                        knowledge: (0, 5)
                                    }
                                },
                                ..Default::default()
                            }
                        ],
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(40),
                        kind: ConsequenceKind::Prejudice {
                            message: "você escorregou e se machucou ao tentar pular!".to_string(),
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
                name: "Usar Cabo de Ferramenta".to_string(),
                emoji: None,
                conditions: vec![Condition::HasItem(items::material::TOOL_HANDLE, 1)],
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Rewards {
                            message: "você usou uma corda improvisada com o cabo de ferramenta para atravessar o caminho com segurança!".to_string(),
                            iterations: 1,
                            items: vec![
                                (Probability::new(100), items::ore::IRON_ORE, (1, 3)),
                                (Probability::new(30), items::special::INVIGORATING_CRYSTAL, (1, 2)),
                                (Probability::new(40), items::tool::PICKAXE, (1, 1)),
                            ],
                            orbs: (10, 60),
                            xp: XpReward {
                                health: (5, 15),
                                intelligence: (5, 15),
                                strength: (5, 15),
                                knowledge: (5, 15)
                            }
                        },
                        ..Default::default()
                    }
                ],
                extra_consequences: vec![Consequence {
                    kind: ConsequenceKind::RemoveItem(items::material::TOOL_HANDLE, 1),
                    ..Default::default()
                }],
                ..Default::default()
            }
        ]
    }
);

make_event!(
    basic_mountain_mysterious_cave,
    Event {
        identifier: "basic_mountain_mysterious_cave",
        spawn: EventSpawn {
            weighted_regions: vec![(WorldRegion::Wornpeaks, 2)],
            ..Default::default()
        },
        emoji: Emoji::from_unicode("🕳️"),
        message: EventMessage::Single("você encontra a entrada de uma caverna misteriosa na encosta da montanha. O que você faz?"),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Explorar a caverna".to_string(),
                emoji: Some(Emoji::from_unicode("🔦")),
                consequences: vec![
                    common::consequence_didnt_find_anything(Probability::new(20)),
                    Consequence {
                        probability: Probability::new(60),
                        kind: ConsequenceKind::Rewards {
                            message: "você encontrou alguns minérios valiosos dentro da caverna!".to_string(),
                            iterations: 2,
                            items: vec![
                                (Probability::new(80), items::ore::COAL_ORE, (1, 3)),
                                (Probability::new(50), items::ore::TIN_ORE, (1, 2)),
                                (Probability::new(50), items::ore::LEAD_ORE, (1, 2)),
                                (Probability::new(30), items::ore::COPPER_ORE, (1, 2)),
                                (Probability::new(30), items::ore::IRON_ORE, (1, 2)),
                                (Probability::new(10), items::ore::GOLD_ORE, (1, 1)),
                            ],
                            orbs: (15, 30),
                            xp: XpReward {
                                strength: (5, 15),
                                intelligence: (5, 15),
                                ..Default::default()
                            }
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(20),
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
    wornpeaks_old_mineshaft,
    Event {
        identifier: "wornpeaks_old_mineshaft",
        spawn: EventSpawn {
            weighted_regions: vec![(WorldRegion::Wornpeaks, 1)],
            ..Default::default()
        },
        emoji: Emoji::from_unicode("⛏️"),
        message: EventMessage::Single("você descobre a entrada de um antigo túnel de mineração. Parece instável, mas pode conter recursos valiosos. O que você faz?"),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Entrar com cuidado".to_string(),
                emoji: Some(Emoji::from_unicode("🚶")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(70),
                        kind: ConsequenceKind::Rewards {
                            message: "você encontrou alguns minérios abandonados!".to_string(),
                            iterations: 2,
                            items: vec![
                                (Probability::new(90), items::ore::COAL_ORE, (1, 4)),
                                (Probability::new(60), items::ore::COPPER_ORE, (1, 2)),
                                (Probability::new(60), items::ore::TIN_ORE, (1, 3)),
                                (Probability::new(20), items::ore::IRON_ORE, (1, 3)),
                                (Probability::new(5), items::ore::GOLD_ORE, (1, 2)),
                                (Probability::new(2), items::ore::DIAMOND_ORE, (1, 1)),
                            ],
                            orbs: (20, 50),
                            xp: XpReward {
                                strength: (10, 20),
                                intelligence: (5, 15),
                                ..Default::default()
                            }
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(30),
                        kind: ConsequenceKind::Prejudice {
                            message: "o túnel desmoronou e você foi atingido por alguns destroços!".to_string(),
                            items_amount: (0, 0),
                            max_item_valuability: 0,
                            fixed_orbs: (0, 0),
                            orbs_percentage: 0.0,
                            specific_items: vec![],
                            damage_percentage: 0.3,
                            damage_limit: 300
                        },

                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        ]
    }
);

pub fn basic_mountain_avalanche_event(state: EventBuildState) -> Event {
    let has_pickaxe = state
        .character
        .inventory
        .iter()
        .any(|i| i.identifier == items::tool::PICKAXE.identifier);
    let has_strength = state.character.stats.strength_level > 60;
    let has_intelligence = state.character.stats.intelligence_level > 60;

    Event {
        identifier: "basic_mountain_avalanche_event",
        spawn: EventSpawn {
            base_probability: Probability::new(15),
            weighted_regions: vec![(WorldRegion::Wornpeaks, 2)],
            ..Default::default()
        },
        emoji: Emoji::from_unicode("❄️"),
        message: EventMessage::Single("você ouve um estrondo ensurdecedor acima de você. Uma avalanche está prestes a cair! O que você faz?"),
        actions: vec![
            Action {
                name: "Procurar abrigo".to_string(),
                emoji: Some(Emoji::from_unicode("🏔️")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(70),
                        kind: ConsequenceKind::Message {
                            message: "você encontra uma saliência rochosa e se abriga sob ela. A avalanche passa por cima, deixando você ileso.".to_string(),
                            emoji: Some(Emoji::from_unicode("😌"))
                        },
                        extra_consequences: vec![
                            Consequence {
                                kind: ConsequenceKind::Rewards {
                                    message: "após a avalanche passar, você encontra alguns itens que foram arrastados.".to_string(),
                                    iterations: 2,
                                    items: vec![
                                        (Probability::new(80), items::ore::IRON_ORE, (1, 3)),
                                        (Probability::new(50), items::ore::COPPER_ORE, (1, 2)),
                                        (Probability::new(20), items::special::GIFT, (1, 1)),
                                        (Probability::new(10), items::special::TRAP, (1, 1)),
                                        (Probability::new(1), items::special::INTELLIGENCE_CRYSTAL, (1, 1)),
                                    ],
                                    orbs: (10, 30),
                                    xp: XpReward {
                                        intelligence: (10, 20),
                                        ..Default::default()
                                    }
                                },
                                ..Default::default()
                            }
                        ],
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(30),
                        kind: ConsequenceKind::Prejudice {
                            message: "Você não consegue encontrar um abrigo a tempo e é atingido pela avalanche!".to_string(),
                            items_amount: (0, 1),
                            max_item_valuability: 50,
                            fixed_orbs: (5, 15),
                            orbs_percentage: 0.1,
                            specific_items: vec![],
                            damage_percentage: 0.3,
                            damage_limit: 100
                        },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Tentar desviar a avalanche".to_string(),
                emoji: Some(Emoji::from_unicode("💪")),
                consequences: vec![
                    Consequence {
                        probability: if has_pickaxe && has_strength { Probability::new(80) } else { Probability::new(40) },
                        kind: ConsequenceKind::Message {
                            message: if has_pickaxe {
                                "com sua picareta, você rapidamente cria um desvio na encosta da montanha. A avalanche muda de curso, evitando você!"
                            } else {
                                "usando pedras e seus próprios braços e ether, você consegue criar um pequeno desvio. A maior parte da avalanche passa ao seu lado!"
                            }.to_string(),
                            emoji: Some(Emoji::from_unicode("🦸"))
                        },
                        extra_consequences: vec![
                            Consequence {
                                kind: ConsequenceKind::Rewards {
                                    message: "seu ato heroico não passa despercebido. Um viajante impressionado te recompensa!".to_string(),
                                    iterations: 1,
                                    items: vec![
                                        (Probability::new(100), items::special::GIFT, (1, 1)),
                                        (Probability::new(50), items::special::INVIGORATING_CRYSTAL, (1, 1)),
                                    ],
                                    orbs: (20, 600),
                                    xp: XpReward {
                                        strength: (20, 40),
                                        intelligence: (10, 20),
                                        ..Default::default()
                                    }
                                },
                                ..Default::default()
                            },
                            Consequence {
                                kind: if has_pickaxe {
                                    ConsequenceKind::RemoveItemDurability(items::tool::PICKAXE, 2)
                                } else {
                                    ConsequenceKind::RemoveEther(20)
                                },
                                ..Default::default()
                            }
                        ],
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(40),
                        kind: ConsequenceKind::Prejudice {
                            message: "apesar de seus esforços, você não consegue desviar a avalanche completamente. Você é parcialmente atingido!".to_string(),
                            items_amount: (0, 1),
                            max_item_valuability: 100,
                            fixed_orbs: (10, 30),
                            orbs_percentage: 0.15,
                            specific_items: vec![],
                            damage_percentage: 0.2,
                            damage_limit: 140
                        },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Usar ether para se proteger".to_string(),
                emoji: Some(emojis::ETHER),
                conditions: vec![Condition::HasEther(20)],
                consequences: vec![
                    Consequence {
                        probability: if has_intelligence { Probability::new(90) } else { Probability::new(60) },
                        kind: ConsequenceKind::Message {
                            message: "você rapidamente conjura um escudo de ether ao seu redor. A avalanche passa por você sem causar danos!".to_string(),
                            emoji: Some(Emoji::from_unicode("✨"))
                        },
                        extra_consequences: vec![
                            Consequence {
                                kind: ConsequenceKind::Rewards {
                                    message: "ao dissipar seu escudo, você percebe que alguns itens ficaram presos nele.".to_string(),
                                    iterations: 1,
                                    items: vec![
                                        (Probability::new(100), items::consumable::GREEN_APPLE, (1, 4)),
                                        (Probability::new(10), items::special::TRAP, (1, 1)),
                                    ],
                                    orbs: (10, 40),
                                    xp: XpReward {
                                        intelligence: (30, 50),
                                        knowledge: (10, 20),
                                        ..Default::default()
                                    }
                                },
                                ..Default::default()
                            },
                            Consequence {
                                kind: ConsequenceKind::RemoveEther(20),
                                ..Default::default()
                            }
                        ],
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(10),
                        kind: ConsequenceKind::Message {
                            message: "seu escudo de ether não é forte o suficiente para suportar toda a avalanche. Ele se quebra, mas absorve a maior parte do impacto.".to_string(),
                            emoji: Some(Emoji::from_unicode("💔"))
                        },
                        extra_consequences: vec![
                            Consequence {
                                kind: ConsequenceKind::Prejudice {
                                    message: "Você sofre alguns ferimentos leves.".to_string(),
                                    items_amount: (0, 0),
                                    max_item_valuability: 0,
                                    fixed_orbs: (0, 0),
                                    orbs_percentage: 0.0,
                                    specific_items: vec![],
                                    damage_percentage: 0.1,
                                    damage_limit: 50
                                },
                                ..Default::default()
                            },
                            Consequence {
                                kind: ConsequenceKind::RemoveEther(20),
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

make_event!(
    basic_mountain_climber,
    Event {
        identifier: "basic_mountain_climber",
        spawn: EventSpawn {
            base_probability: Probability::new(60),
            weighted_regions: vec![(WorldRegion::Wornpeaks, 1)],
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("🧗"),
        message: EventMessage::Single("você encontra um escalador ferido na encosta da montanha. O que você quer fazer?"),
        actions: vec![
            common::ignore_action_with_extra_consequences(vec![
                Consequence {
                    kind: ConsequenceKind::RemoveKarma(1),
                    ..Default::default()
                }
            ]),
            Action {
                name: "Ajudar o Escalador".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Rewards {
                            message: "o escalador agradece sua ajuda e compartilha alguns suprimentos com você.".to_string(),
                            iterations: 1,
                            items: vec![
                                (Probability::new(100), items::consumable::WATER, (1, 3)),
                                (Probability::new(80), items::consumable::APPLE, (1, 2)),
                                (Probability::new(50), items::cosmetic::STRAWHAT, (1, 1)),
                            ],
                            orbs: (10, 25),
                            xp: XpReward {
                                health: (5, 10),
                                intelligence: (0, 5),
                                strength: (5, 10),
                                knowledge: (0, 5)
                            }
                        },
                        ..Default::default()
                    },
                    Consequence {
                        kind: ConsequenceKind::AddKarma(1),
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        ]
    }
);

make_event!(
    basic_mountain_abandoned_cabin,
    Event {
        identifier: "basic_mountain_abandoned_cabin",
        spawn: EventSpawn {
            weighted_regions: vec![(WorldRegion::Wornpeaks, 3)],
            ..Default::default()
        },
        emoji: Emoji::from_unicode("🏚️"),
        message: EventMessage::Single(
            "você encontra uma cabana abandonada na encosta da montanha. O que você quer fazer?"
        ),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Explorar a cabana".to_string(),
                emoji: Some(Emoji::from_unicode("🔍")),
                consequences: vec![
                    common::consequence_didnt_find_anything(Probability::new(20)),
                    Consequence {
                        probability: Probability::new(70),
                        kind: ConsequenceKind::Rewards {
                            message: "você encontra alguns itens úteis deixados para trás!"
                                .to_string(),
                            iterations: 4,
                            items: vec![
                                (Probability::new(80), items::consumable::WATER, (1, 2)),
                                (Probability::new(60), items::consumable::FRIED_EGG, (1, 2)),
                                (Probability::new(60), items::consumable::CHOCOLATE, (1, 2)),
                                (Probability::new(40), items::consumable::ORANGE, (1, 2)),
                                (Probability::new(40), items::consumable::TOMATO, (1, 2)),
                                (Probability::new(40), items::tool::AXE, (1, 1)),
                                (
                                    Probability::new(2),
                                    items::special::INVIGORATING_CRYSTAL,
                                    (1, 1)
                                ),
                            ],
                            orbs: (10, 30),
                            xp: XpReward {
                                intelligence: (5, 10),
                                knowledge: (5, 10),
                                ..Default::default()
                            }
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(10),
                        kind: ConsequenceKind::FindARegionEnemy,
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        ]
    }
);
