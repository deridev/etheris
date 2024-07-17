use etheris_data::{items, personality::Personality, ShopItem};
use weaklings_plus::frost_wolf;

use super::prelude::*;

make_event!(basic_icefields_exploration, Event {
    identifier: "basic_icefields_exploration",
    spawn: EventSpawn {
        weighted_regions: vec![(WorldRegion::Icefields, 10)],
        ..Default::default()
    },
    emoji: Emoji::from_unicode("❄️"),
    message: EventMessage::Multiple(&[
        "você se depara com uma vasta extensão de gelo e neve. Como você deseja explorar esta paisagem gelada?",
        "a sua frente se estende um horizonte branco e congelado. Como você quer aventurar-se neste terreno gelado?",
    ]),
    actions: vec![
        Action {
            name: "Procurar Ameaças".to_string(),
            emoji: Some(Emoji::from_unicode("🔍")),
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
            name: "Caminhar Cautelosamente".to_string(),
            emoji: Some(Emoji::from_unicode("🚶")),
            consequences: vec![
                common::consequence_didnt_find_anything(Probability::new(5)),
                Consequence {
                    probability: Probability::new(70),
                    kind: ConsequenceKind::Rewards {
                        message: "você encontrou alguns itens úteis durante sua caminhada".to_string(),
                        iterations: 2,
                        items: vec![
                            (Probability::new(80), items::consumable::WATER, (1, 3)),
                            (Probability::new(10), items::consumable::MILK, (1, 1)),
                            (Probability::new(50), items::material::STONE, (1, 3)),
                            (Probability::new(50), items::material::STICK, (1, 2)),
                            (Probability::new(30), items::ore::COAL_ORE, (1, 2))
                        ],
                        orbs: (5, 20),
                        xp: XpReward::default()
                    },
                    ..Default::default()
                },
                Consequence {
                    probability: Probability::new(15),
                    kind: ConsequenceKind::Event(basic_icefields_frozen_lake),
                    ..Default::default()
                },
                Consequence {
                    probability: Probability::new(10),
                    kind: ConsequenceKind::Event(basic_icefields_snow_storm),
                    ..Default::default()
                },
                Consequence {
                    probability: Probability::new(5),
                    kind: ConsequenceKind::Event(basic_icefields_nomad_merchant),
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    ]
});

make_event!(
    basic_icefields_nomad_merchant,
    Event {
        identifier: "basic_icefields_nomad_merchant",
        spawn: EventSpawn::never(),
        emoji: Emoji::from_unicode("💸"),
        message: EventMessage::Multiple(&[
            "um vendedor gritou de longe te chamando para comprar algo. O que você quer fazer?",
            "alguém tocou no seu ombro. Quando você olhou para trás, era um vendedor nômade, de mantos azuis. Quer dar uma olhada nos seus itens à venda?",
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
                                ShopItem::new_sellable_item(5, items::ore::TIN_ORE, 1.4, 0.9),
                                ShopItem::new_sellable_item(0, items::ore::LEAD_ORE, 1.4, 0.9),

                                if Probability::new(50).generate_random_bool() {
                                    ShopItem::new_item(1, items::special::GIFT, 1.2)
                                } else {
                                    ShopItem::new_item(1, items::special::TRAP, 1.2)
                                },

                                if Probability::new(50).generate_random_bool() {
                                    ShopItem::new_item(1, items::lore::ENTITY_039_REPORT, 1.0)
                                } else {
                                    ShopItem::new_item(1, items::lore::ENTITY_104_REPORT, 1.1)
                                }.with_description("Só... Não pergunte como eu consegui isso."),
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
    basic_icefields_frozen_lake,
    Event {
        identifier: "basic_icefields_frozen_lake",
        spawn: EventSpawn {
            weighted_regions: vec![(WorldRegion::Icefields, 1)],
            base_probability: Probability::new(50),
            ..Default::default()
        },
        emoji: Emoji::from_unicode("❄️"),
        message: EventMessage::Single(
            "você encontrou um enorme lago congelado. A superfície parece frágil, mas você vê algo brilhante sob o gelo. O que você deseja fazer?"
        ),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Quebrar o Gelo".to_string(),
                emoji: Some(items::tool::PICKAXE.emoji),
                conditions: vec![Condition::HasItem(items::tool::PICKAXE, 1)],
                consequences: vec![
                    Consequence {
                        probability: Probability::new(70),
                        kind: ConsequenceKind::Rewards {
                            message: "você conseguiu quebrar o gelo e recuperar o objeto brilhante!".to_string(),
                            iterations: 1,
                            items: vec![
                                (Probability::new(100), items::special::GIFT, (1, 1)),
                                (Probability::new(30), items::ore::GOLD_ORE, (1, 1)),
                                (Probability::new(5), items::ore::DIAMOND_ORE, (1, 1)),
                            ],
                            orbs: (50, 100),
                            xp: XpReward {
                                strength: (10, 20),
                                ..Default::default()
                            }
                        },
                        extra_consequences: vec![Consequence {
                            kind: ConsequenceKind::RemoveItemDurability(items::tool::PICKAXE, 2),
                            ..Default::default()
                        }],
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(30),
                        kind: ConsequenceKind::Prejudice {
                            message: "o gelo se quebrou sob seus pés e você caiu na água gelada!".to_string(),
                            items_amount: (0, 0),
                            max_item_valuability: 0,
                            fixed_orbs: (0, 0),
                            orbs_percentage: 0.0,
                            specific_items: vec![],
                            damage_percentage: 0.3,
                            damage_limit: 200
                        },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Tentar Andar no Gelo".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        probability: Probability::new(40),
                        kind: ConsequenceKind::Rewards {
                            message: "você conseguiu atravessar o gelo com cuidado e encontrou um tesouro escondido!".to_string(),
                            iterations: 1,
                            items: vec![
                                (Probability::new(100), items::ore::COPPER_ORE, (1, 1)),
                                (Probability::new(60), items::ore::GOLD_ORE, (1, 1)),
                                (Probability::new(2), items::special::INTELLIGENCE_CRYSTAL, (1, 1)),
                            ],
                            orbs: (30, 80),
                            xp: XpReward {
                                intelligence: (15, 25),
                                ..Default::default()
                            }
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(60),
                        kind: ConsequenceKind::Prejudice {
                            message: "o gelo se quebrou e você caiu na água gelada!".to_string(),
                            items_amount: (0, 0),
                            max_item_valuability: 0,
                            fixed_orbs: (0, 0),
                            orbs_percentage: 0.0,
                            specific_items: vec![],
                            damage_percentage: 0.3,
                            damage_limit: 250
                        },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        ]
    }
);

make_event!(basic_icefields_snow_storm, Event {
    identifier: "basic_icefields_snow_storm",
    spawn: EventSpawn {
        weighted_regions: vec![(WorldRegion::Icefields, 3)],
        ..Default::default()
    },
    emoji: Emoji::from_unicode("🌨️"),
    message: EventMessage::Single("uma tempestade de neve repentina se forma! Você precisa encontrar abrigo rapidamente. O que você faz?"),
    actions: vec![
        Action {
            name: "Procurar uma Caverna".to_string(),
            emoji: Some(Emoji::from_unicode("🕳️")),
            consequences: vec![
                Consequence {
                    probability: Probability::new(70),
                    kind: ConsequenceKind::Rewards {
                        message: "você encontrou uma caverna segura e esperou a tempestade passar".to_string(),
                        iterations: 1,
                        items: vec![
                            (Probability::new(50), items::ore::COAL_ORE, (1, 3)),
                            (Probability::new(30), items::ore::IRON_ORE, (1, 2)),
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
                    probability: Probability::new(30),
                    kind: ConsequenceKind::FindARegionEnemy,
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
        Action {
            name: "Construir um Iglu".to_string(),
            emoji: Some(Emoji::from_unicode("🧱")),
            consequences: vec![
                Consequence {
                    probability: Probability::new(60),
                    kind: ConsequenceKind::Rewards {
                        message: "você conseguiu construir um iglu e se proteger da tempestade".to_string(),
                        iterations: 1,
                        items: vec![],
                        orbs: (10, 20),
                        xp: XpReward {
                            intelligence: (10, 20),
                            strength: (5, 15),
                            ..Default::default()
                        }
                    },
                    ..Default::default()
                },
                Consequence {
                    probability: Probability::new(40),
                    kind: ConsequenceKind::Prejudice {
                        message: "você não conseguiu construir o iglu a tempo e sofreu com o frio intenso".to_string(),
                        items_amount: (0, 0),
                        max_item_valuability: 0,
                        fixed_orbs: (0, 0),
                        orbs_percentage: 0.0,
                        specific_items: vec![],
                        damage_percentage: 0.15,
                        damage_limit: 150
                    },
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    ]
});

make_event!(icefields_frost_wolf_pack, Event {
    identifier: "icefields_frost_wolf_pack",
    spawn: EventSpawn {
        base_probability: Probability::new(15),
        weighted_regions: vec![(WorldRegion::Icefields, 1)],
        conditions: vec![]
    },
    emoji: Emoji::from_unicode("🐺"),
    message: EventMessage::Single("você ouve uivos ao longe e percebe que está cercado por uma alcateia de lobos do gelo. O que você faz?"),
    actions: vec![
        Action {
            name: "Lutar".to_string(),
            emoji: Some(Emoji::from_unicode("⚔️")),
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Battle(BattleConsequence {
                        enemies: vec![frost_wolf(), frost_wolf()],
                        prompt: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
        Action {
            name: "Tentar Acalmar".to_string(),
            emoji: Some(Emoji::from_unicode("🤚")),
            conditions: vec![Condition::HasPersonality(Personality::Intelligence)],
            consequences: vec![
                Consequence {
                    probability: Probability::new(40),
                    kind: ConsequenceKind::Rewards {
                        message: "você conseguiu acalmar os lobos e eles se afastaram pacificamente".to_string(),
                        iterations: 1,
                        items: vec![],
                        orbs: (20, 40),
                        xp: XpReward {
                            intelligence: (20, 30),
                            ..Default::default()
                        }
                    },
                    ..Default::default()
                },
                Consequence {
                    probability: Probability::new(60),
                    kind: ConsequenceKind::Battle(BattleConsequence {
                        enemies: vec![frost_wolf()],
                        prompt: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    ]
});
