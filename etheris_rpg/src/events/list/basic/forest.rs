use etheris_data::{items, personality::Personality, ShopItem};
use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};
use weaklings::dangerous_bear;

use super::prelude::*;

pub fn basic_forest_exploration(_state: EventBuildState) -> Event {
    Event {
        identifier: "basic_forest_exploration",
        spawn: EventSpawn {
            weighted_regions: vec![(WorldRegion::Mudland, 5), (WorldRegion::Gloomwood, 5), (WorldRegion::Ethergrove, 5), (WorldRegion::Starbreeze, 5), (WorldRegion::Murkswamp, 5)],
            ..Default::default()
        },
        emoji: Emoji::from_unicode("🗺️"),
        message: EventMessage::Multiple(&[
            "você está no meio de uma densa floresta! Como você vai explorar?",
            "a floresta em sua frente se extende até onde seus olhos conseguem enxergar. Como você quer explorar a floresta?",
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
                        kind: ConsequenceKind::Rewards { message: "você achou algumas coisas pelo caminho".to_string(), iterations: 1, items: vec![], orbs: (8, 16), xp: XpReward::default() },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(5),
                        kind: ConsequenceKind::FindARegionEnemy,
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(10),
                        kind: ConsequenceKind::Event(basic_forest_beginner_nomad_merchant),
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Cortar Árvore".to_string(),
                emoji: Some(items::tool::AXE.emoji),
                conditions: vec![Condition::HasItem(items::tool::AXE, 1)],
                consequences: vec![Consequence {
                    kind: ConsequenceKind::Rewards {
                        message: "o árvore foi cortada e você coletou madeira!".to_string(),
                        iterations: 1,
                        items: vec![(Probability::new(100), items::material::RAW_TRUNK, (1, 3))],
                        orbs: (0, 0),
                        xp: XpReward {
                            health: (0, 8),
                            intelligence: (0, 6),
                            strength: (0, 12),
                            knowledge: (0, 7)
                        }
                    },
                    extra_consequences: vec![Consequence {
                        kind: ConsequenceKind::RemoveItemDurability(items::tool::AXE, 1),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }
        ]
    }
}

make_event!(
    basic_forest_digging,
    Event {
        identifier: "basic_forest_digging",
        spawn: EventSpawn {
            weighted_regions: vec![
                (WorldRegion::Murkswamp, 2),
                (WorldRegion::Gloomwood, 2),
                (WorldRegion::Mudland, 2)
            ],
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
                    common::consequence_didnt_find_anything(Probability::new(5)),
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
    basic_forest_feet_stuck_in_vines,
    Event {
        identifier: "basic_forest_feet_stuck_in_vines",
        spawn: EventSpawn {
            base_probability: Probability::new(50),
            weighted_regions: vec![
                (WorldRegion::Gloomwood, 1),
                (WorldRegion::Murkswamp, 3),
                (WorldRegion::Ethergrove, 1),
                (WorldRegion::Starbreeze, 1),
                (WorldRegion::Mudland, 1),
                (WorldRegion::Sunreach, 1)
            ],
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("☘️"),
        message: EventMessage::Single("você prendeu seu pé em vinhas. O que você faz?"),
        actions: vec![
            Action {
                name: "Cortar".to_string(),
                emoji: Some(items::material::KNIFE.emoji),
                conditions: vec![Condition::HasItem(items::material::KNIFE, 1)],
                consequences: vec![Consequence {
                    kind: ConsequenceKind::Rewards {
                        message: "você cortou as vinhas!".to_string(),
                        iterations: 0,
                        items: vec![],
                        orbs: (0, 0),
                        xp: XpReward {
                            health: (0, 0),
                            intelligence: (0, 10),
                            strength: (0, 10),
                            knowledge: (0, 0)
                        }
                    },
                    extra_consequences: vec![Consequence {
                        kind: ConsequenceKind::RemoveItem(items::material::KNIFE, 1),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            },
            Action {
                name: "Quebrar Vinhas Com as Mãos".to_string(),
                emoji: None,
                conditions: vec![],
                consequences: vec![Consequence {
                    kind: ConsequenceKind::Prejudice {
                        message: "você se feriu cortando as vinhas com as mãos!".to_string(),
                        items_amount: (0, 0),
                        max_item_valuability: 0,
                        fixed_orbs: (0, 0),
                        orbs_percentage: 0.0,
                        specific_items: vec![],
                        damage_percentage: 0.1,
                        damage_limit: 80
                    },
                    extra_consequences: vec![],
                    ..Default::default()
                },],
                ..Default::default()
            }
        ]
    }
);

pub fn basic_forest_knowledge_books_pedestal(_: EventBuildState) -> Event {
    const COLORS: [(&str, Emoji); 3] = [
        ("Vermelho", Emoji::from_unicode("📕")),
        ("Verde", Emoji::from_unicode("📗")),
        ("Azul", Emoji::from_unicode("📘")),
    ];
    const BASE_MESSAGE: &str = "você achou um pedestal com três livros. Na base dele, está escrito: \"Os fortes estão destinados ao sucesso, os inteligentes ao topo e os covardes precisam de ajuda para melhorar. Um dos três livros é sábio, os outros são só papel.\"";

    let correct_color = *COLORS.choose(&mut StdRng::from_entropy()).unwrap();

    let actions = COLORS.iter().map(|(color, emoji)| {
        Action {
            name: format!("Ler Livro {}", color),
            emoji: Some(*emoji),
            consequences: vec![if color == &correct_color.0 {
                Consequence {
                    kind: ConsequenceKind::Rewards { message: "você pegou o livro correto! Ao ler cada página seu cérebro expandiu.".to_string(), iterations: 0, items: vec![], orbs: (0, 0), xp: XpReward { strength: (0, 0), health: (0, 0), intelligence: (80, 150), knowledge: (80, 150) } },
                    ..Default::default()
                }
            } else {
                Consequence {
                    kind: ConsequenceKind::Message { message: "você abriu o livro e só viu páginas em branco. Antes que você pudesse agir, os outros livros haviam desaparecido. Só restou um sentimento de vazio em você.".to_string(), emoji: Some(Emoji::from_unicode("😟")) },
                    ..Default::default()
                }
            }],
            ..Default::default()
        }
    }).collect::<Vec<_>>();

    Event {
        identifier: "basic_forest_knowledge_books_pedestal",
        spawn: EventSpawn {
            base_probability: Probability::new(20),
            weighted_regions: vec![(WorldRegion::Gloomwood, 1), (WorldRegion::Murkswamp, 1), (WorldRegion::Mudland, 1), (WorldRegion::Ethergrove, 2)],
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("📖"),
        message: EventMessage::Conditional(vec![
            (Condition::HasPersonality(Personality::Cowardice), format!("{BASE_MESSAGE}\nVocê sentiu a impressão de que o livro correto é o {}. Qual você vai ler?", correct_color.0.to_lowercase())),
            (Condition::None, format!("{BASE_MESSAGE}\nQual livro você vai ler?"))
        ]),
        actions,
    }
}

make_event!(basic_forest_dangerous_button, Event {
    identifier: "basic_forest_dangerous_button",
    spawn: EventSpawn {
        base_probability: Probability::new(5),
        weighted_regions: vec![(WorldRegion::Murkswamp, 1), (WorldRegion::Mudland, 1), (WorldRegion::Murkswamp, 1)],
        conditions: vec![]
    },
    emoji: Emoji::from_unicode("🔴"),
    message: EventMessage::Conditional(vec![
        (Condition::HasPersonality(Personality::Cowardice), "você encontrou um botão vermelho em uma árvore. TODOS OS SEUS INTINTOS GRITAM: PERIGO! O que você quer fazer?".to_string()),
        (Condition::StrongerThan(dangerous_bear()), "você encontrou um botão vermelho em uma árvore. Ele parece perigoso, mas você sente que está em segurança. O que você quer fazer?".to_string()),
        (Condition::None, "você encontrou um botão vermelho em uma árvore. Você sente que se apertar o botão você pode morrer. O que você quer fazer?".to_string())
    ]),
    actions: vec![
        common::ignore_action(),
        Action {
            name: "Apertar".to_string(),
            emoji: None,
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::InstantBattle(dangerous_bear()),
                    ..Default::default()
                }
            ],
            extra_consequences: vec![],
            ..Default::default()
        }
    ]
});

make_event!(
    basic_forest_beginner_nomad_merchant,
    Event {
        identifier: "basic_forest_beginner_nomad_merchant",
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
                            name: "Vendedor Nômade da Floresta".to_string(),
                            items: vec![
                                ShopItem::new_item(13, items::consumable::WATER, 1.1),
                                ShopItem::new_item(15, items::consumable::WHEAT, 1.2),
                                ShopItem::new_item(10, items::consumable::MILK, 1.1),
                                ShopItem::new_item(6, items::consumable::APPLE, 1.1),
                                ShopItem::new_item(8, items::consumable::CHEESE, 1.2),
                                ShopItem::new_item(2, items::consumable::CHOCOLATE, 1.2),
                                ShopItem::new_item(3, items::consumable::SALT, 1.2),
                                ShopItem::new_item(3, items::consumable::SUGAR, 1.1),
                                ShopItem::new_item(1, items::tool::SHOVEL, 0.7),
                                ShopItem::new_item(1, items::tool::PICKAXE, 1.3),
                                ShopItem::new_item(1, items::tool::HAMMER, 1.1),
                                ShopItem::new_item(1, items::tool::AXE, 1.1).with_description("Será seu melhor amigo em uma densa floresta"),
                                ShopItem::new_sellable_item(23, items::material::STONE, 1.2, 0.6),
                                ShopItem::new_sellable_item(15, items::material::STICK, 1.2, 0.7),
                                ShopItem::new_sellable_item(15, items::material::PAPER, 1.2, 0.7),
                                ShopItem::new_sellable_item(1, items::material::KNIFE, 1.3, 0.7),
                                ShopItem::new_sellable_item(5, items::ore::COAL_ORE, 1.4, 0.7),
                                ShopItem::new_sellable_item(0, items::ore::IRON_ORE, 1.4, 0.7),
                                ShopItem::new_sellable_item(0, items::ore::COPPER_ORE, 1.4, 0.7),

                                if Probability::new(5).generate_random_bool() {
                                    ShopItem::new_item(1, items::special::GIFT, 0.7).with_description("Não sei o que tem dentro, mas parece útil!")
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
    basic_forest_strange_shrine,
    Event {
        identifier: "basic_forest_strange_shrine",
        spawn: EventSpawn {
            base_probability: Probability::new(3),
            weighted_regions: vec![(WorldRegion::Mudland, 1), (WorldRegion::Murkswamp, 1), (WorldRegion::Starbreeze, 2), (WorldRegion::Gloomwood, 1)],
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("⛩️"),
        message: EventMessage::Single(
            "Você encontra um estranho santuário em ruínas. Parece haver uma inscrição antiga gravada na pedra."
        ),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Examinar Inscrição".to_string(),
                emoji: Some(Emoji::from_unicode("🔍")),
                conditions: vec![Condition::Or(Box::new(Condition::HasPersonality(Personality::Intelligence)), Box::new(Condition::Probability(Probability::new(50))))],
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Rewards {
                            message: "Você decifra a inscrição antiga e ganha conhecimento!".to_string(),
                            iterations: 1,
                            items: vec![],
                            orbs: (0, 0),
                            xp: XpReward {
                                health: (0, 0),
                                intelligence: (20, 40),
                                strength: (0, 0),
                                knowledge: (50, 100)
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
    basic_forest_suspicious_tree,
    Event {
        identifier: "basic_forest_suspicious_tree",
        spawn: EventSpawn {
            base_probability: Probability::new(10),
            weighted_regions: vec![
                (WorldRegion::Gloomwood, 1),
                (WorldRegion::Mudland, 1),
                (WorldRegion::Ethergrove, 1),
                (WorldRegion::Starbreeze, 1)
            ],
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("🌳"),
        message: EventMessage::Single(
            "você encontrou uma árvore de aparência suspeita na floresta. O que deseja fazer?"
        ),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Examinar Árvore".to_string(),
                emoji: Some(Emoji::from_unicode("🔍")),
                consequences: vec![
                    common::consequence_didnt_find_anything(Probability::new(80)),
                    Consequence {
                        probability: Probability::new(20),
                        kind: ConsequenceKind::Rewards {
                            message: "você encontrou um buraco na árvore contendo alguns itens!"
                                .to_string(),
                            iterations: 2,
                            items: vec![
                                (Probability::new(80), items::consumable::APPLE, (2, 5)),
                                (Probability::new(60), items::consumable::WATER, (1, 3)),
                                (Probability::new(40), items::material::RAW_TRUNK, (1, 2)),
                                (Probability::new(20), items::material::STICK, (3, 6)),
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
    basic_forest_animal_tracks,
    Event {
        identifier: "basic_forest_animal_tracks",
        spawn: EventSpawn {
            weighted_regions: vec![
                (WorldRegion::Gloomwood, 1),
                (WorldRegion::Mudland, 1),
                (WorldRegion::Murkswamp, 1)
            ],
            ..Default::default()
        },
        emoji: Emoji::from_unicode("🐾"),
        message: EventMessage::Single(
            "você encontrou pegadas de um animal na floresta. O que deseja fazer?"
        ),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Seguir Pegadas".to_string(),
                emoji: Some(Emoji::from_unicode("🚶")),
                consequences: vec![
                    common::consequence_didnt_find_anything(Probability::new(40)),
                    Consequence {
                        probability: Probability::new(40),
                        kind: ConsequenceKind::FindARegionEnemy,
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(20),
                        kind: ConsequenceKind::Rewards {
                            message: "você encontrou o animal e conseguiu alguns recursos!"
                                .to_string(),
                            iterations: 2,
                            items: vec![
                                (Probability::new(80), items::consumable::BEEF, (2, 5)),
                                (Probability::new(40), items::material::BONE, (1, 2)),
                            ],
                            orbs: (10, 30),
                            xp: XpReward {
                                strength: (10, 20),
                                ..Default::default()
                            }
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
    basic_swamp_murky_waters,
    Event {
        identifier: "basic_swamp_murky_waters",
        spawn: EventSpawn {
            weighted_regions: vec![(WorldRegion::Murkswamp, 3)],
            ..Default::default()
        },
        emoji: Emoji::from_unicode("🥾"),
        message: EventMessage::Single(
            "você se depara com águas turvas e lamacentas. O que deseja fazer?"
        ),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Atravessar".to_string(),
                emoji: Some(Emoji::from_unicode("🏊")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(60),
                        kind: ConsequenceKind::Rewards {
                            message: "você atravessou com sucesso e encontrou algo interessante!"
                                .to_string(),
                            iterations: 1,
                            items: vec![
                                (Probability::new(80), items::material::STONE, (2, 5)),
                                (Probability::new(50), items::consumable::WATER, (1, 3)),
                                (Probability::new(30), items::ore::IRON_ORE, (1, 2)),
                            ],
                            orbs: (15, 30),
                            xp: XpReward {
                                health: (10, 20),
                                strength: (5, 15),
                                ..Default::default()
                            }
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(40),
                        kind: ConsequenceKind::Prejudice {
                            message: "você ficou preso na lama e se feriu!".to_string(),
                            items_amount: (0, 0),
                            max_item_valuability: 0,
                            fixed_orbs: (0, 0),
                            orbs_percentage: 0.0,
                            specific_items: vec![],
                            damage_percentage: 0.15,
                            damage_limit: 75
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
    basic_swamp_quicksand,
    Event {
        identifier: "basic_swamp_quicksand",
        spawn: EventSpawn {
            base_probability: Probability::new(30),
            weighted_regions: vec![(WorldRegion::Murkswamp, 1)],
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("🕳️"),
        message: EventMessage::Single("você se depara com uma área de areia movediça no pântano. O terreno parece instável e perigoso. O que você faz?"),
        actions: vec![
            Action {
                name: "Tentar Atravessar Cuidadosamente".to_string(),
                emoji: Some(Emoji::from_unicode("🚶")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(60),
                        kind: ConsequenceKind::Message {
                            message: "com passos cautelosos, você consegue atravessar a área de areia movediça sem incidentes.".to_string(),
                            emoji: Some(Emoji::from_unicode("😌"))
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(40),
                        kind: ConsequenceKind::Prejudice {
                            message: "você escorrega e fica preso na areia movediça! Consegue se libertar, mas perde alguns itens no processo.".to_string(),
                            items_amount: (1, 3),
                            max_item_valuability: 100,
                            fixed_orbs: (10, 30),
                            orbs_percentage: 0.05,
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
                name: "Usar Ether para Flutuar".to_string(),
                emoji: Some(Emoji::from_unicode("🌀")),
                conditions: vec![Condition::HasEther(15)],
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Rewards {
                            message: "você usa seu ether para flutuar sobre a areia movediça, descobrindo um tesouro escondido no processo!".to_string(),
                            iterations: 1,
                            items: vec![
                                (Probability::new(80), items::special::GIFT, (1, 1)),
                                (Probability::new(60), items::ore::IRON_ORE, (1, 3)),
                            ],
                            orbs: (20, 50),
                            xp: XpReward {
                                intelligence: (10, 20),
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
                    }
                ],
                ..Default::default()
            },
            common::ignore_action()
        ]
    }
);

make_event!(
    basic_forest_ancient_tree_library,
    Event {
        identifier: "basic_forest_ancient_tree_library",
        spawn: EventSpawn {
            base_probability: Probability::new(10),
            weighted_regions: vec![
                (WorldRegion::Ethergrove, 3),
                (WorldRegion::Starbreeze, 2),
                (WorldRegion::Gloomwood, 1),
                (WorldRegion::Sunreach, 1),
                (WorldRegion::Mudland, 1),
            ],
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("📚"),
        message: EventMessage::Single(
            "você encontra uma árvore antiga com prateleiras naturais cheias de livros e pergaminhos. Parece ser uma biblioteca secreta da floresta. O que você faz?"
        ),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Ler um Livro Aleatório".to_string(),
                emoji: Some(Emoji::from_unicode("📖")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(70),
                        kind: ConsequenceKind::Rewards {
                            message: "você lê um livro fascinante e ganha conhecimento!".to_string(),
                            iterations: 1,
                            items: vec![],
                            orbs: (0, 0),
                            xp: XpReward {
                                intelligence: (20, 40),
                                knowledge: (40, 80),
                                ..Default::default()
                            }
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(30),
                        kind: ConsequenceKind::Message {
                            message: "o livro que você escolheu está em uma língua que você não entende.".to_string(),
                            emoji: Some(Emoji::from_unicode("❓"))
                        },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Procurar por Itens Escondidos".to_string(),
                emoji: Some(Emoji::from_unicode("🔍")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(50),
                        kind: ConsequenceKind::Rewards {
                            message: "você encontra alguns itens interessantes escondidos entre os livros!".to_string(),
                            iterations: 2,
                            items: vec![
                                (Probability::new(80), items::material::PAPER, (3, 7)),
                                (Probability::new(50), items::consumable::SALT, (1, 3)),
                                (Probability::new(30), items::consumable::SUGAR, (1, 3)),
                                (Probability::new(20), items::special::GIFT, (1, 1)),
                            ],
                            orbs: (10, 30),
                            xp: XpReward::default()
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(30),
                        kind: ConsequenceKind::Message {
                            message: "você não encontra nada de especial, mas a busca foi uma boa experiência.".to_string(),
                            emoji: Some(Emoji::from_unicode("🤷"))
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(20),
                        kind: ConsequenceKind::Prejudice {
                            message: "você acidentalmente derruba uma pilha de livros, causando uma pequena avalanche!".to_string(),
                            items_amount: (0, 0),
                            max_item_valuability: 0,
                            fixed_orbs: (0, 0),
                            orbs_percentage: 0.0,
                            specific_items: vec![],
                            damage_percentage: 0.05,
                            damage_limit: 20
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
    basic_forest_apple_tree,
    Event {
        identifier: "basic_forest_apple_tree",
        spawn: EventSpawn {
            base_probability: Probability::new(40),
            weighted_regions: vec![
                (WorldRegion::Gloomwood, 2),
                (WorldRegion::Mudland, 2),
                (WorldRegion::Ethergrove, 1),
                (WorldRegion::Starbreeze, 1),
            ],
            conditions: vec![]
        },
        emoji: items::consumable::APPLE.emoji,
        message: EventMessage::Single(
            "você encontra uma árvore carregada de frutas. Elas parecem suculentas."
        ),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Coletar Frutas".to_string(),
                emoji: Some(Emoji::from_unicode("🧺")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(90),
                        kind: ConsequenceKind::Rewards {
                            message: "você coleta um punhado de frutas frescas.".to_string(),
                            iterations: 1,
                            items: vec![
                                (Probability::new(100), items::consumable::APPLE, (2, 5)),
                                (Probability::new(100), items::consumable::GREEN_APPLE, (1, 3)),
                                (Probability::new(100), items::consumable::ORANGE, (1, 3)),
                            ],
                            orbs: (5, 10),
                            xp: XpReward::default()
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(10),
                        kind: ConsequenceKind::Message {
                            message: "ao se aproximar do arbusto, você espanta um pequeno animal que estava se alimentando das frutas. As frutas já estão mordidas!".to_string(),
                            emoji: Some(Emoji::from_unicode("🐿️"))
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
    basic_forest_abandoned_picnic,
    Event {
        identifier: "basic_forest_abandoned_picnic",
        spawn: EventSpawn {
            weighted_regions: vec![(WorldRegion::Gloomwood, 2), (WorldRegion::Mudland, 1),],
            ..Default::default()
        },
        emoji: Emoji::from_unicode("🧺"),
        message: EventMessage::Single(
            "você encontrou um piquenique abandonado às pressas. O que deseja fazer?"
        ),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Investigar".to_string(),
                emoji: Some(Emoji::from_unicode("🕵️")),
                consequences: vec![
                    common::consequence_didnt_find_anything(Probability::new(10)),
                    Consequence {
                        probability: Probability::new(80),
                        kind: ConsequenceKind::Rewards {
                            message: "você encontrou alguns alimentos e itens deixados para trás!"
                                .to_string(),
                            iterations: 2,
                            items: vec![
                                (Probability::new(90), items::consumable::APPLE, (1, 2)),
                                (Probability::new(90), items::consumable::MILK, (1, 2)),
                                (Probability::new(90), items::consumable::GREEN_APPLE, (1, 2)),
                                (Probability::new(80), items::consumable::WATER, (1, 2)),
                                (Probability::new(70), items::consumable::FRIED_EGG, (1, 2)),
                                (Probability::new(70), items::consumable::CHOCOLATE, (1, 1)),
                                (Probability::new(50), items::material::KNIFE, (1, 1)),
                                (Probability::new(30), items::material::PAPER, (1, 3)),
                            ],
                            orbs: (5, 15),
                            xp: XpReward {
                                intelligence: (5, 10),
                                ..Default::default()
                            }
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(25),
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
    basic_forest_fallen_tree,
    Event {
        identifier: "basic_forest_fallen_tree",
        spawn: EventSpawn {
            base_probability: Probability::new(35),
            weighted_regions: vec![
                (WorldRegion::Gloomwood, 2),
                (WorldRegion::Mudland, 2),
                (WorldRegion::Murkswamp, 1),
                (WorldRegion::Midgrass, 1),
            ],
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("🌳"),
        message: EventMessage::Single(
            "você se depara com uma grande árvore caída, bloqueando parcialmente o caminho."
        ),
        actions: vec![
            Action {
                name: "Pular Por Cima".to_string(),
                emoji: Some(Emoji::from_unicode("🦘")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(80),
                        kind: ConsequenceKind::Message {
                            message:
                                "você pula habilmente sobre o tronco caído e continua seu caminho."
                                    .to_string(),
                            emoji: Some(Emoji::from_unicode("👍"))
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(20),
                        kind: ConsequenceKind::Prejudice {
                            message: "você tropeça ao pular e se arranha levemente.".to_string(),
                            items_amount: (0, 0),
                            max_item_valuability: 0,
                            fixed_orbs: (0, 0),
                            orbs_percentage: 0.0,
                            specific_items: vec![],
                            damage_percentage: 0.05,
                            damage_limit: 30
                        },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Procurar Algo Útil".to_string(),
                emoji: Some(Emoji::from_unicode("🔍")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(70),
                        kind: ConsequenceKind::Rewards {
                            message: "você encontra alguns gravetos úteis perto da árvore caída."
                                .to_string(),
                            iterations: 1,
                            items: vec![(Probability::new(100), items::material::STICK, (2, 5)),],
                            orbs: (0, 2),
                            xp: XpReward::default()
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(30),
                        kind: ConsequenceKind::Message {
                            message: "você não encontra nada de útil.".to_string(),
                            emoji: Some(Emoji::from_unicode("🤷"))
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
    basic_forest_unusual_rock,
    Event {
        identifier: "basic_forest_unusual_rock",
        spawn: EventSpawn {
            base_probability: Probability::new(25),
            weighted_regions: vec![
                (WorldRegion::Mudland, 2),
                (WorldRegion::Gloomwood, 1),
                (WorldRegion::Ethergrove, 1),
                (WorldRegion::Starbreeze, 1),
                (WorldRegion::Sunreach, 1),
                (WorldRegion::Mudland, 1),
            ],
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("🪨"),
        message: EventMessage::Single(
            "você nota uma rocha com uma forma incomum. Parece um pouco fora de lugar."
        ),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Examinar a Rocha".to_string(),
                emoji: Some(Emoji::from_unicode("🔍")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(60),
                        kind: ConsequenceKind::Rewards {
                            message: "ao examinar de perto, você encontra rochas menores por perto".to_string(),
                            iterations: 1,
                            items: vec![
                                (Probability::new(70), items::material::STONE, (1, 4)),
                            ],
                            orbs: (1, 15),
                            xp: XpReward::default()
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(40),
                        kind: ConsequenceKind::Message {
                            message: "após um exame cuidadoso, você conclui que é apenas uma rocha comum, mas interessante.".to_string(),
                            emoji: Some(Emoji::from_unicode("🤔"))
                        },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Quebrar".to_string(),
                emoji: Some(items::tool::PICKAXE.emoji),
                conditions: vec![Condition::HasItem(items::tool::PICKAXE, 1)],
                consequences: vec![
                    Consequence {
                        probability: Probability::new(60),
                        kind: ConsequenceKind::Rewards {
                            message: "você quebra a rocha e encontra alguns minerais!".to_string(),
                            iterations: 3,
                            items: vec![
                                (Probability::new(100), items::material::STONE, (1, 3)),
                                (Probability::new(100), items::ore::COAL_ORE, (1, 3)),
                                (Probability::new(70), items::ore::COPPER_ORE, (1, 3)),
                                (Probability::new(50), items::ore::IRON_ORE, (1, 3)),
                                (Probability::new(50), items::ore::TIN_ORE, (1, 3)),
                                (Probability::new(50), items::ore::LEAD_ORE, (1, 3)),
                            ],
                            orbs: (0, 0),
                            xp: XpReward {
                                health: (0, 0),
                                intelligence: (0, 0),
                                strength: (10, 20),
                                knowledge: (0, 0)
                            }
                        },
                        ..Default::default()
                    },
                ],
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
    basic_forest_house,
    Event {
        identifier: "basic_forest_house",
        spawn: EventSpawn {
            base_probability: Probability::new(10),
            weighted_regions: vec![
                (WorldRegion::Gloomwood, 1),
                (WorldRegion::Mudland, 1),
                (WorldRegion::Sunreach, 1),
                (WorldRegion::Ethergrove, 1),
                (WorldRegion::Starbreeze, 1),
            ],
            ..Default::default()
        },
        emoji: Emoji::from_unicode("🏚️"),
        message: EventMessage::Multiple(&[
            "há uma casa abandonada em meio à floresta. O que você quer fazer?",
            "enquanto caminha, você vê uma mansão abandonada.",
        ]),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Entrar".to_string(),
                emoji: None,
                consequences: vec![Consequence {
                    kind: ConsequenceKind::Event(inside_forest_house),
                    ..Default::default()
                }],
                ..Default::default()
            },
        ],
    }
);

make_event!(
    inside_forest_house,
    Event {
        identifier: "inside_forest_house",
        spawn: EventSpawn::never(),
        emoji: Emoji::from_unicode("🏚️"),
        message: EventMessage::Single("você está dentro da casa abandonada. Que cômodo deseja explorar?"),
        actions: vec![
            Action {
                name: "Cozinha".to_string(),
                emoji: Some(Emoji::from_unicode("🍳")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(95),
                        kind: ConsequenceKind::Rewards {
                            message: "você encontrou alguns itens na cozinha!".to_string(),
                            iterations: 3,
                            items: vec![
                                (Probability::new(60), items::material::KNIFE, (1, 1)),
                                (Probability::new(80), items::consumable::MILK, (1, 2)),
                                (Probability::new(60), items::consumable::CHOCOLATE, (1, 1)),
                                (Probability::new(80), items::consumable::APPLE, (1, 2)),
                                (Probability::new(80), items::consumable::GREEN_APPLE, (1, 1)),
                                (Probability::new(40), items::consumable::WATER, (1, 2)),
                            ],
                            orbs: (0, 0),
                            xp: XpReward::default(),
                        },
                        ..Default::default()
                    },
                    common::consequence_didnt_find_anything(Probability::new(5)),
                ],
                ..Default::default()
            },
            Action {
                name: "Porão".to_string(),
                emoji: Some(Emoji::Unicode("🕳️")),
                probability: Probability::new(3),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(80),
                        kind: ConsequenceKind::Rewards {
                            message: "você encontrou alguns itens no porão!".to_string(),
                            iterations: 1,
                            items: vec![
                                (Probability::ALWAYS, items::lore::OLD_ABANDONED_BASEMENT_DIARY, (1, 1)),
                                (Probability::new(5), items::special::INTELLIGENCE_CRYSTAL, (1, 1)),
                            ],
                            orbs: (0, 0),
                            xp: XpReward::default(),
                        },
                        ..Default::default()
                    },
                    common::consequence_didnt_find_anything(Probability::new(20)),
                ],
                ..Default::default()
            },
            Action {
                name: "Sótão".to_string(),
                emoji: Some(Emoji::Unicode("🧳")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(80),
                        kind: ConsequenceKind::Rewards {
                            message: "você encontrou alguns itens empoeirados no sótão!".to_string(),
                            iterations: 2,
                            items: vec![
                                (Probability::new(70), items::material::STONE, (1, 1)),
                                (Probability::new(50), items::cosmetic::STRAWHAT, (1, 1)),
                                (Probability::new(50), items::cosmetic::EYE_BANDANA, (1, 1)),
                                (Probability::new(40), items::cosmetic::GLASSES, (1, 1)),
                                (Probability::new(30), items::special::GIFT, (1, 1)),
                                (Probability::new(30), items::special::TRAP, (1, 1)),
                                (Probability::new(20), items::lore::GOLDEN_ROBOT_POEM, (1, 1)),
                                (Probability::new(40), items::lore::METROLIS_LAW_338, (1, 1)),
                            ],
                            orbs: (5, 15),
                            xp: XpReward {
                                intelligence: (5, 10),
                                ..Default::default()
                            },
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(20),
                        kind: ConsequenceKind::Message {
                            message: "o sótão está vazio, exceto por algumas teias de aranha.".to_string(),
                            emoji: Some(Emoji::Unicode("🕸️")),
                        },
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
            Action {
                name: "Biblioteca".to_string(),
                emoji: Some(Emoji::Unicode("📚")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(90),
                        kind: ConsequenceKind::Rewards {
                            message: "você encontrou alguns livros interessantes na biblioteca!".to_string(),
                            iterations: 1,
                            items: vec![
                                (Probability::new(80), items::special::RECIPE_BOOK, (1, 1)),
                                (Probability::new(3), items::lore::ENTITY_039_REPORT, (1, 1)),
                                (Probability::new(3), items::lore::ENTITY_104_REPORT, (1, 1)),
                                (Probability::new(3), items::lore::HAKIKO_LEGEND, (1, 1)),
                                (Probability::new(1), items::special::INTELLIGENCE_CRYSTAL, (1, 1)),
                            ],
                            orbs: (10, 20),
                            xp: XpReward {
                                intelligence: (10, 20),
                                knowledge: (15, 25),
                                ..Default::default()
                            },
                        },
                        ..Default::default()
                    },
                    common::consequence_didnt_find_anything(Probability::new(10)),
                ],
                ..Default::default()
            },
            Action {
                name: "Quarto Principal".to_string(),
                emoji: Some(Emoji::Unicode("🛏️")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(85),
                        kind: ConsequenceKind::Rewards {
                            message: "você vasculhou o quarto principal e encontrou alguns itens!".to_string(),
                            iterations: 3,
                            items: vec![
                                (Probability::new(70), items::material::TOOL_HANDLE, (1, 2)),
                                (Probability::new(60), items::consumable::SALT, (1, 3)),
                                (Probability::new(50), items::tool::HAMMER, (1, 1)),
                                (Probability::new(40), items::tool::AXE, (1, 1)),
                                (Probability::new(30), items::tool::UMBRELLA, (1, 1)),
                                (Probability::new(20), items::special::TRAP, (1, 1)),
                            ],
                            orbs: (15, 30),
                            xp: XpReward {
                                strength: (5, 10),
                                health: (5, 10),
                                ..Default::default()
                            },
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(15),
                        kind: ConsequenceKind::Message {
                            message: "o quarto principal parece ter sido saqueado. Não há nada de valor aqui.".to_string(),
                            emoji: Some(Emoji::Unicode("😞")),
                        },
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
        ],
    }
);

make_event!(
    basic_gloomwood_mysterious_fog,
    Event {
        identifier: "basic_gloomwood_mysterious_fog",
        spawn: EventSpawn {
            base_probability: Probability::new(30),
            weighted_regions: vec![(WorldRegion::Gloomwood, 1)],
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("🌫️"),
        message: EventMessage::Single(
            "uma densa névoa misteriosa surge ao seu redor. O que você faz?"
        ),
        actions: vec![
            Action {
                name: "Recuar".to_string(),
                emoji: None,
                consequences: vec![Consequence {
                    kind: ConsequenceKind::Message {
                        message: "você decide não arriscar e recua para uma área mais segura."
                            .to_string(),
                        emoji: None
                    },
                    ..Default::default()
                }],
                ..Default::default()
            },
            Action {
                name: "Explorar a névoa".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Rewards {
                            message: "ao explorar a névoa, você encontra itens misteriosos!"
                                .to_string(),
                            iterations: 2,
                            items: vec![
                                (Probability::new(70), items::consumable::SALT, (1, 3)),
                                (Probability::new(50), items::consumable::SUGAR, (1, 2)),
                                (Probability::new(30), items::material::PAPER, (1, 2)),
                            ],
                            orbs: (15, 40),
                            xp: XpReward {
                                health: (0, 10),
                                intelligence: (10, 20),
                                strength: (0, 10),
                                knowledge: (10, 20)
                            }
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(40),
                        kind: ConsequenceKind::Prejudice {
                            message: "você não vê seu caminho na névoa e tropeça!".to_string(),
                            fixed_orbs: (0, 0),
                            items_amount: (0, 0),
                            max_item_valuability: 0,
                            orbs_percentage: 0.0,
                            specific_items: vec![],
                            damage_percentage: 0.05,
                            damage_limit: 45,
                        },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        ]
    }
);
