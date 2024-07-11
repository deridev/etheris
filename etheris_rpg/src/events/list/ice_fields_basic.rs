use etheris_data::{items, personality::Personality};
use weaklings_plus::frost_wolf;

use super::prelude::*;

make_event!(icefields_exploration, Event {
    identifier: "icefields_exploration",
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
                        iterations: 1,
                        items: vec![
                            (Probability::new(80), items::consumable::WATER, (1, 3)),
                            (Probability::new(50), items::material::STONE, (1, 3)),
                            (Probability::new(30), items::ore::COAL_ORE, (1, 2))
                        ],
                        orbs: (5, 20),
                        xp: XpReward::default()
                    },
                    ..Default::default()
                },
                Consequence {
                    probability: Probability::new(15),
                    kind: ConsequenceKind::Event(icefields_frozen_lake),
                    ..Default::default()
                },
                Consequence {
                    probability: Probability::new(10),
                    kind: ConsequenceKind::Event(icefields_snow_storm),
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    ]
});

make_event!(
    icefields_frozen_lake,
    Event {
        identifier: "icefields_frozen_lake",
        spawn: EventSpawn {
            weighted_regions: vec![(WorldRegion::Icefields, 1)],
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
                                (Probability::new(100), items::ore::GOLD_ORE, (1, 3)),
                                (Probability::new(40), items::ore::DIAMOND_ORE, (1, 1)),
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
                                (Probability::new(100), items::ore::GOLD_ORE, (2, 4)),
                                (Probability::new(30), items::special::INTELLIGENCE_CRYSTAL, (1, 1)),
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

make_event!(icefields_snow_storm, Event {
    identifier: "icefields_snow_storm",
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
