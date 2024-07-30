use etheris_data::{
    emojis,
    items::{self, get_item, Item, ItemTag},
    personality::Personality,
    weapon::WeaponKind,
    BrainKind, ShopItem, SkillKind,
};
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use weaklings::{giant_rat, weak_thief};

use crate::BodyImmunities;

use super::prelude::*;

make_event!(basic_plains_exploration, Event {
    identifier: "basic_plains_exploration",
    spawn: EventSpawn {
        weighted_regions: vec![(WorldRegion::Greenagis, 5), (WorldRegion::Emerelis, 5), (WorldRegion::Midgrass, 5)],
        ..Default::default()
    },
    emoji: Emoji::from_unicode("🗺️"),
    message: EventMessage::Multiple(&[
        "você avista uma paisagem verde e pacífica à sua frente! Como você vai explorar?",
        "do seu redor até o horizonte só se pode ver natureza e paisagens verdes. Como você quer explorar a planície?",
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
                    probability: Probability::new(70),
                    kind: ConsequenceKind::Rewards { message: "você achou algumas coisas pelo caminho".to_string(), iterations: 1, items: vec![], orbs: (3, 16), xp: XpReward::default() },
                    ..Default::default()
                },
                Consequence {
                    probability: Probability::new(2),
                    kind: ConsequenceKind::Event(basic_plains_weak_thief),
                    ..Default::default()
                },
                Consequence {
                    probability: Probability::new(15),
                    kind: ConsequenceKind::Event(basic_plains_beginner_nomad_merchant),
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    ]
});

make_event!(
    basic_plains_old_man_help,
    Event {
        identifier: "basic_plains_old_man_help",
        spawn: EventSpawn {
            base_probability: Probability::new(15),
            weighted_regions: vec![(WorldRegion::Greenagis, 2), (WorldRegion::Emerelis, 2), (WorldRegion::Midgrass, 1),(WorldRegion::Starbreeze, 1), (WorldRegion::Gloomwood, 1), (WorldRegion::Mudland, 1), (WorldRegion::Sunreach, 1)],
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("👴"),
        message: EventMessage::Single(
            "você encontra um velho homem caído no chão, aparentemente ferido. Ele parece estar precisando de ajuda."
        ),
        actions: vec![
            common::ignore_action_with_extra_consequences(vec![
                Consequence {
                    kind: ConsequenceKind::RemoveKarma(1),
                    ..Default::default()
                }
            ]),
            Action {
                name: "Ajudar o Velho".to_string(),
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Rewards {
                            message: "O velho homem agradece sua ajuda e lhe entrega uma recompensa.".to_string(),
                            iterations: 1,
                            items: vec![
                                (Probability::new(100), items::consumable::CHEESE, (1, 3)),
                                (Probability::new(80), items::tool::SHOVEL, (1, 1)),
                                (Probability::new(50), items::material::KNIFE, (1, 1)),
                            ],
                            orbs: (10, 30),
                            xp: XpReward {
                                health: (0, 5),
                                intelligence: (0, 5),
                                strength: (0, 5),
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
    basic_plains_beginner_nomad_merchant,
    Event {
        identifier: "basic_plains_beginner_nomad_merchant",
        spawn: EventSpawn::never(),
        emoji: Emoji::from_unicode("💸"),
        message: EventMessage::Multiple(&[
            "um vendedor nômade te parou e perguntou se você tem interesse em comprar alguns itens.",
            "você ouviu uma voz te chamando, e quando olhou era um vendedor nômade. Quer dar uma olhada em seus itens à venda?",
        ]),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Ver Loja".to_string(),
                emoji: Some(Emoji::from_unicode("🏪")),
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Shop {
                            name: "Vendedor Nômade da Planície".to_string(),
                            items: vec![
                                ShopItem::new_item(6, items::consumable::WATER, 1.1),
                                ShopItem::new_item(3, items::consumable::APPLE, 1.1),
                                ShopItem::new_item(17, items::consumable::EGG, 1.1),
                                ShopItem::new_item(5, items::consumable::SALT, 1.2),
                                ShopItem::new_item(1, items::tool::SHOVEL, 0.9),
                                ShopItem::new_item(1, items::tool::PICKAXE, 1.2).with_description("Hi hi, essa belezinha deu trabalho para conseguir!"),
                                ShopItem::new_item(1, items::tool::HAMMER, 1.2),
                                ShopItem::new_item(1, items::tool::AXE, 1.4).with_description("Bem, não há muitas árvores aqui perto. Não sei o que você faria com isso."),
                                ShopItem::new_sellable_item(23, items::material::STONE, 1.2, 0.6),
                                ShopItem::new_sellable_item(15, items::material::STICK, 1.2, 0.7),
                                ShopItem::new_sellable_item(1, items::material::KNIFE, 1.4, 0.7),
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
    basic_plains_lost_traveler,
    Event {
        identifier: "basic_plains_lost_traveler",
        spawn: EventSpawn {
            base_probability: Probability::new(30),
            weighted_regions: vec![(WorldRegion::Greenagis, 1), (WorldRegion::Emerelis, 1), (WorldRegion::Midgrass, 2)],
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("🧭"),
        message: EventMessage::Single("você encontrou um viajante perdido. Ele parece confuso e pede sua ajuda. O que você quer fazer?"),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Ajudar".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Rewards {
                            message: "o viajante ficou muito agradecido pela sua ajuda e ofereceu alguns itens como recompensa!".to_string(),
                            iterations: 2,
                            items: vec![
                                (Probability::new(70), items::consumable::WATER, (1, 2)),
                                (Probability::new(50), items::consumable::BREAD, (1, 1)),
                                (Probability::new(30), items::material::PAPER, (1, 1)),
                            ],
                            orbs: (10, 30),
                            xp: XpReward {
                                health: (0, 5),
                                intelligence: (0, 10),
                                strength: (0, 5),
                                knowledge: (0, 10)
                            }
                        },
                        extra_consequences: vec![Consequence {
                            kind: ConsequenceKind::AddKarma(1),
                            ..Default::default()
                        }],
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Roubar".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Rewards {
                            message: "você aproveitou a confusão do viajante e roubou alguns de seus pertences!".to_string(),
                            iterations: 1,
                            items: vec![
                                (Probability::new(80), items::consumable::WATER, (1, 3)),
                                (Probability::new(60), items::consumable::BREAD, (1, 2)),
                                (Probability::new(40), items::material::PAPER, (1, 1)),
                                (Probability::new(20), items::ore::COAL_ORE, (1, 3)),
                            ],
                            orbs: (20, 50),
                            xp: XpReward {
                                health: (0, 0),
                                intelligence: (0, 5),
                                strength: (0, 10),
                                knowledge: (0, 0)
                            }
                        },
                        extra_consequences: vec![Consequence {
                            kind: ConsequenceKind::RemoveKarma(2),
                            ..Default::default()
                        }],
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        ]
    }
);

pub fn basic_plains_weak_thief(state: EventBuildState) -> Event {
    let inventory = state
        .character
        .inventory
        .iter()
        .filter_map(|i| get_item(&i.identifier))
        .filter(|i| !i.tags.contains(&ItemTag::Specific) && !i.tags.contains(&ItemTag::Special))
        .collect::<Vec<_>>();
    let most_valuable_item = inventory
        .iter()
        .max_by_key(|item| {
            // Avoid a weak thief asking for a VERY expensive item
            if item.purchase_properties.base_price > 250 {
                0
            } else {
                item.purchase_properties.base_price
            }
        })
        .copied();

    let is_thief_asking_most_valuable_item =
        most_valuable_item.is_some() && Probability::new(30).generate_random_bool();

    let asking_text = if is_thief_asking_most_valuable_item {
        format!(
            "pediu seus pertences e seu item mais valioso: **{}**",
            most_valuable_item.unwrap().display_name
        )
    } else {
        "pediu seus pertences".to_string()
    };

    let prejudice = ConsequenceKind::Prejudice {
        message: "você foi assaltado por um inimigo!".to_string(),
        fixed_orbs: (0, 10),
        items_amount: (1, 5),
        max_item_valuability: 100,
        orbs_percentage: 0.05,
        specific_items: if let Some(item) = most_valuable_item {
            vec![(item, 1)]
        } else {
            vec![]
        },
        damage_percentage: 0.0,
        damage_limit: 0,
    };

    Event {
        identifier: "basic_plains_weak_thief",
        spawn: EventSpawn::never(),
        emoji: Emoji::from_unicode("🔫"),
        message: EventMessage::Conditional(vec![
            (Condition::SimilarPowerTo(weak_thief()), format!("um ladrão te ameaçou e {asking_text}. A força dele é semelhante à sua. Como você quer reagir?")),
            (Condition::StrongerThan(weak_thief()), format!("um ladrão te ameaçou e {asking_text}. Ele não aparenta ameaça alguma para sua força. Como você quer reagir?")),
            (Condition::WeakerThan(weak_thief()), format!("um ladrão te ameaçou e {asking_text}. Você sentiu uma poderosa pressão de ether vindo dele, é um inimigo perigoso. Como você quer reagir?")),
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
                        kind: ConsequenceKind::InstantBattle(weak_thief()),
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        ]
    }
}

make_event!(
    basic_plains_digging,
    Event {
        identifier: "basic_plains_digging",
        spawn: EventSpawn {
            weighted_regions: vec![(WorldRegion::Greenagis, 4), (WorldRegion::Emerelis, 4)],
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
                                (Probability::new(60), items::material::STONE, (1, 2)),
                                (Probability::new(40), items::material::RAW_TRUNK, (1, 1)),
                                (Probability::new(40), items::material::PAPER, (1, 1)),
                                (Probability::new(20), items::material::TOOL_HANDLE, (1, 1)),
                                (Probability::new(20), items::consumable::APPLE, (0, 1)),
                                (Probability::new(40), items::consumable::WATER, (0, 1)),
                                (Probability::new(60), items::consumable::SALT, (0, 1)),
                                (Probability::new(60), items::consumable::SUGAR, (0, 1)),
                                (Probability::new(60), items::consumable::TOMATO, (0, 2)),
                            ],
                            orbs: (0, 20),
                            xp: XpReward {
                                health: (0, 10),
                                intelligence: (0, 10),
                                strength: (0, 15),
                                knowledge: (0, 5)
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

make_enemy!(
    begger,
    Enemy {
        identifier: "begger",
        name: "Mendigo",
        base_probability: Probability::NEVER,
        brain: BrainKind::Simple,
        boss: None,
        regions: &[],
        personalities: &[Personality::Courage],
        potential: EnemyPotential::Low,
        immunities: BodyImmunities::new(),
        strength: 8,
        intelligence: 3,
        resistance: 130,
        vitality: 60,
        ether: 20,
        weapon: Some(WeaponKind::Stick),
        allies: None,
        skills: vec![
            SkillKind::DefensiveJump,
            SkillKind::CyclonePush,
            SkillKind::TornadoKick,
        ],
        drop: EnemyReward {
            orbs: (40, 60),
            xp: (20, 40),
            items: vec![
                EnemyRewardItem {
                    amount: (1, 4),
                    item: items::consumable::WATER,
                    probability: Probability::new(50),
                },
                EnemyRewardItem {
                    amount: (1, 2),
                    item: items::material::STICK,
                    probability: Probability::ALWAYS,
                }
            ],
        },
    }
);

make_event!(
    basic_plains_begger,
    Event {
        identifier: "basic_plains_begger",
        spawn: EventSpawn {
            base_probability: Probability::new(15),
            weighted_regions: vec![(WorldRegion::Greenagis, 2), (WorldRegion::Emerelis, 1)],
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("🍔"),
        message: EventMessage::Single("você viu um mendigo pedindo alguns orbs. Você quer ajudar?"),
        actions: vec![
            common::ignore_action_with_extra_consequences(vec![Consequence {
                kind: ConsequenceKind::RemoveKarma(1),
                ..Default::default()
            }]),
            Action {
                name: "Ajudar".to_string(),
                emoji: None,
                conditions: vec![Condition::HasOrbs(20)],
                consequences: vec![Consequence {
                    kind: ConsequenceKind::Prejudice {
                        message: "você deu alguns orbs para o mendigo! Seu karma melhorou."
                            .to_string(),
                        items_amount: (0, 0),
                        max_item_valuability: 0,
                        fixed_orbs: (20, 40),
                        orbs_percentage: 0.01,
                        specific_items: vec![],
                        damage_percentage: 0.0,
                        damage_limit: 0
                    },
                    extra_consequences: vec![Consequence {
                        kind: ConsequenceKind::AddKarma(1),
                        ..Default::default()
                    }],
                    ..Default::default()
                },],
                ..Default::default()
            },
            Action {
                name: "Roubar".to_string(),
                emoji: None,
                conditions: vec![],
                consequences: vec![Consequence {
                    kind: ConsequenceKind::Battle(BattleConsequence {
                        enemies: vec![begger(), giant_rat()],
                        prompt: false,
                        ..Default::default()
                    }),
                    extra_consequences: vec![Consequence {
                        kind: ConsequenceKind::RemoveKarma(2),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }
        ]
    }
);

make_event!(
    basic_plains_water_well,
    Event {
        identifier: "basic_plains_water_well",
        spawn: EventSpawn {
            weighted_regions: vec![
                (WorldRegion::Greenagis, 1),
                (WorldRegion::Mudland, 1),
                (WorldRegion::Midgrass, 1),
                (WorldRegion::Emerelis, 1),
                (WorldRegion::Gloomwood, 1),
                (WorldRegion::Sunreach, 1)
            ],
            ..Default::default()
        },
        emoji: Emoji::from_unicode("💧"),
        message: EventMessage::Multiple(&[
            "você encontrou um pequeno poço. O que deseja fazer?",
            "você achou um poço de água. Deseja usá-lo?",
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
                            message: "o poço tinha água e algumas outras coisas!".to_string(),
                            iterations: 1,
                            items: vec![(Probability::new(100), items::consumable::WATER, (1, 4)),],
                            orbs: (0, 5),
                            xp: XpReward {
                                health: (0, 10),
                                intelligence: (0, 5),
                                strength: (0, 5),
                                knowledge: (0, 5)
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
            Action {
                name: "Destruir".to_string(),
                emoji: Some(items::tool::HAMMER.emoji),
                conditions: vec![Condition::HasItem(items::tool::HAMMER, 1)],
                consequences: vec![Consequence {
                    kind: ConsequenceKind::Rewards {
                        message: "o poço foi destruído e você pegou alguns materiais e itens!"
                            .to_string(),
                        iterations: 3,
                        items: vec![
                            (Probability::new(100), items::consumable::WATER, (1, 4)),
                            (Probability::new(100), items::material::STONE, (2, 6)),
                            (Probability::new(70), items::material::RAW_TRUNK, (1, 1)),
                        ],
                        orbs: (5, 20),
                        xp: XpReward {
                            health: (0, 15),
                            intelligence: (0, 5),
                            strength: (0, 15),
                            knowledge: (0, 5)
                        }
                    },
                    ..Default::default()
                }],
                extra_consequences: vec![Consequence {
                    kind: ConsequenceKind::RemoveItemDurability(items::tool::HAMMER, 1),
                    ..Default::default()
                }],
                ..Default::default()
            }
        ]
    }
);

pub fn basic_plains_person_wanting_materials(state: EventBuildState) -> Event {
    const VALID_ITEMS: &[((i32, i32), Item)] = &[
        ((2, 15), items::material::STONE),
        ((1, 2), items::material::RAW_TRUNK),
        ((1, 3), items::material::KNIFE),
        ((1, 6), items::material::STICK),
    ];

    let mut rng = StdRng::from_entropy();

    let mut items = VALID_ITEMS.iter().collect::<Vec<_>>();
    items.shuffle(&mut rng);

    let items = items
        .into_iter()
        .map(|(amount, item)| (rng.gen_range(amount.0..=amount.1), *item))
        .take(2)
        .collect::<Vec<_>>();

    let orbs_reward = rng.gen_range(if state.character.pl > 150 {
        10..=40
    } else {
        50..=90
    });

    Event {
        identifier: "basic_plains_person_wanting_materials",
        spawn: EventSpawn {
            base_probability: Probability::new(50),
            weighted_regions: vec![(WorldRegion::Greenagis, 1), (WorldRegion::Emerelis, 1)],
            conditions: vec![]
        },
        emoji: items::material::PLANK.emoji,
        message: EventMessage::SingleString(
            format!(
                "uma pessoa se aproximou se você e ofereceu **{} {orbs_reward} ◎** para você em troca de alguns itens. Os itens são: {}.", 
                emojis::ORB, items.iter().map(|(amount, item)| format!("**{}x {}**", amount, item.display_name)).collect::<Vec<_>>().join(", ")
            )
        ),
        actions: vec![
            Action {
                name: "\"Eu não tenho esse itens\"".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Message { message: "a pessoa respondeu: \"Ah, entendo. Uma pena! Eu precisava mesmo desses materiais para criar algumas coisas...\"".to_string(), emoji: None },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Dar Itens".to_string(),
                conditions: items.iter().map(|(amount, item)| Condition::HasItem(*item, *amount as usize)).collect(),
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Rewards {
                            message: "a pessoa respondeu: \"Muito obrigado! Aqui estão seus orbs. Ah! Finalmente minha criação vai se concluir!\"".to_string(),
                            iterations: 0,
                            items: vec![],
                            orbs: (orbs_reward, orbs_reward),
                            xp: XpReward {
                                health: (0, 0),
                                intelligence: (0, 0),
                                strength: (0, 0),
                                knowledge: (0, 0)
                            }
                        },
                        extra_consequences: items.iter().map(|(amount, item)| Consequence {
                            kind: ConsequenceKind::RemoveItem(*item, *amount as usize),
                            ..Default::default()
                        }).collect(),
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        ]
    }
}

pub fn basic_plains_person_in_danger(_: EventBuildState) -> Event {
    let is_bad = Probability::new(40).generate_random_bool();

    let kind = if is_bad {
        ConsequenceKind::Event(basic_plains_person_in_danger_bad)
    } else {
        ConsequenceKind::Rewards {
            message: "você ajudou uma pessoa que estava ferida e ela te recompensou.".to_string(),
            iterations: 3,
            items: vec![
                (Probability::new(100), items::consumable::WATER, (1, 2)),
                (Probability::new(100), items::consumable::APPLE, (1, 2)),
                (Probability::new(100), items::consumable::FRIED_EGG, (1, 2)),
                (Probability::new(100), items::consumable::MILK, (1, 2)),
                (Probability::new(100), items::material::STICK, (1, 2)),
                (Probability::new(60), items::material::KNIFE, (1, 1)),
                (Probability::new(30), items::material::TOOL_HANDLE, (1, 1)),
                (Probability::new(20), items::tool::SHOVEL, (1, 1)),
                (Probability::new(2), items::special::TRAP, (1, 1)),
            ],
            orbs: (10, 30),
            xp: XpReward::default(),
        }
    };

    Event {
        identifier: "basic_plains_person_in_danger",
        spawn: EventSpawn {
            weighted_regions: vec![
                (WorldRegion::Greenagis, 1),
                (WorldRegion::Emerelis, 1),
                (WorldRegion::Midgrass, 1),
            ],
            ..Default::default()
        },
        emoji: Emoji::from_unicode("💥"),
        message: EventMessage::Conditional(vec![
            (
                Condition::HasPersonality(Personality::Intelligence),
                if is_bad {
                    "você escuta uma pessoa gritando em perigo mas tem uma sensação ruim de perigo. Você quer ajudar?".to_string()
                } else {
                    "você escuta uma pessoa gritando em perigo. Você sente que ela pode estar passando risco de vida. Você quer ajudar?".to_string()
                },
            ),
            (
                Condition::None,
                "você escuta uma pessoa gritando em perigo por perto. Você quer ajudar?"
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

fn basic_plains_person_in_danger_bad(_: EventBuildState) -> Event {
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
            (Condition::SimilarPowerTo(weak_thief()), "você foi emboscado! Um ladrão te ameaçou com uma faca e pediu orbs! A força dele é semelhante à sua. Como você quer reagir?".to_string()),
            (Condition::StrongerThan(weak_thief()), "era uma emboscada! Um criminoso fraco se revela e fala para você entregar seus orbs. Ele não aparenta ameaça alguma para sua força. Como você quer reagir?".to_string()),
            (Condition::WeakerThan(weak_thief()), "não era uma pessoa pedindo ajuda... É uma emboscada! Um ladrão poderoso te ameaça com uma faca e pede orbs. Você sentiu uma poderosa pressão de ether vindo dele, é um inimigo perigoso. Como você quer reagir?".to_string()),
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
                        kind: ConsequenceKind::InstantBattle(weak_thief()),
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        ]
    }
}

make_event!(basic_plains_trapped, Event {
    identifier: "basic_plains_trapped",
    spawn: EventSpawn {
        base_probability: Probability::new(25),
        weighted_regions: vec![(WorldRegion::Greenagis, 1), (WorldRegion::Midgrass, 2), (WorldRegion::Emerelis, 1)],
        conditions: vec![]
    },
    emoji: Emoji::from_unicode("🩸"),
    message: EventMessage::Single("você caiu em uma armadilha! Um chão falso se revelou quando você pisou nele e uma armadilha para urso prendeu seu pé. O que você quer fazer?"),
    actions: vec![
        Action {
            name: "Quebrar".to_string(),
            emoji: Some(items::tool::HAMMER.emoji),
            conditions: vec![Condition::HasItem(items::tool::HAMMER, 1)],
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Rewards {
                        message: "você quebrou a armadilha antes que pudesse se ferir!".to_string(),
                        iterations: 1,
                        items: vec![(Probability::new(100), items::material::STONE, (1, 3)), (Probability::new(50), items::ore::IRON_ORE, (1, 3))],
                        orbs: (0, 0),
                        xp: XpReward {
                            health: (10, 30),
                            intelligence: (0, 10),
                            strength: (0, 0),
                            knowledge: (0, 0)
                        }
                    },
                    extra_consequences: vec![Consequence {
                        kind: ConsequenceKind::RemoveItemDurability(items::tool::HAMMER, 1),
                        ..Default::default()
                    }],
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
        Action {
            name: "Abrir Armadilha Com a Mão".to_string(),
            emoji: None,
            conditions: vec![],
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Prejudice {
                        message: "você conseguiu tirar a armadilha, mas se feriu gravemente!".to_string(),
                        items_amount: (0, 0),
                        max_item_valuability: 0,
                        fixed_orbs: (0, 0), orbs_percentage: 0.0, specific_items: vec![], damage_percentage: 0.3, damage_limit: 300
                    },
                    extra_consequences: vec![],
                    ..Default::default()
                },
            ],
            ..Default::default()
        }
    ]
});

make_event!(
    basic_plains_abandoned_campsite,
    Event {
        identifier: "basic_plains_abandoned_campsite",
        spawn: EventSpawn {
            weighted_regions: vec![
                (WorldRegion::Greenagis, 2),
                (WorldRegion::Emerelis, 2),
                (WorldRegion::Midgrass, 1)
            ],
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

make_event!(
    basic_plains_wounded_traveler,
    Event {
        identifier: "basic_plains_wounded_traveler",
        spawn: EventSpawn {
            base_probability: Probability::new(10),
            weighted_regions: vec![
                (WorldRegion::Greenagis, 1),
                (WorldRegion::Emerelis, 1),
                (WorldRegion::Midgrass, 1)
            ],
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("🩹"),
        message: EventMessage::Single(
            "você encontrou um viajante ferido e caído no chão. O que deseja fazer?"
        ),
        actions: vec![
            common::ignore_action_with_extra_consequences(vec![Consequence {
                kind: ConsequenceKind::RemoveKarma(1),
                ..Default::default()
            }]),
            Action {
                name: "Ajudar o Viajante".to_string(),
                emoji: None,
                consequences: vec![Consequence {
                    kind: ConsequenceKind::Rewards {
                        message:
                            "você ajudou o viajante ferido e ele te recompensou com alguns itens."
                                .to_string(),
                        iterations: 1,
                        items: vec![
                            (Probability::new(100), items::consumable::WATER, (2, 5)),
                            (Probability::new(70), items::consumable::FRIED_EGG, (1, 3)),
                            (Probability::new(50), items::material::STICK, (2, 4)),
                            (Probability::new(5), items::ore::GOLD_ORE, (0, 1)),
                        ],
                        orbs: (10, 30),
                        xp: XpReward {
                            health: (5, 15),
                            ..Default::default()
                        }
                    },
                    extra_consequences: vec![Consequence {
                        kind: ConsequenceKind::AddKarma(1),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }
        ]
    }
);

make_event!(
    basic_plains_fork_in_the_road,
    Event {
        identifier: "basic_plains_fork_in_the_road",
        spawn: EventSpawn {
            base_probability: Probability::new(40),
            weighted_regions: vec![
                (WorldRegion::Greenagis, 3),
                (WorldRegion::Emerelis, 3),
                (WorldRegion::Midgrass, 2)
            ],
            ..Default::default()
        },
        emoji: Emoji::from_unicode("⛓️"),
        message: EventMessage::Single(
            "você chegou a uma bifurcação no caminho. Qual caminho deseja seguir?"
        ),
        actions: vec![
            Action {
                name: "Caminho da Esquerda".to_string(),
                emoji: Some(Emoji::from_unicode("⬅️")),
                consequences: vec![
                    common::consequence_didnt_find_anything(Probability::new(70)),
                    Consequence {
                        probability: Probability::new(20),
                        kind: ConsequenceKind::FindARegionEnemy,
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(10),
                        kind: ConsequenceKind::Event(basic_plains_digging),
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Caminho da Direita".to_string(),
                emoji: Some(Emoji::from_unicode("➡️")),
                consequences: vec![
                    common::consequence_didnt_find_anything(Probability::new(60)),
                    Consequence {
                        probability: Probability::new(30),
                        kind: ConsequenceKind::Event(basic_plains_beginner_nomad_merchant),
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(10),
                        kind: ConsequenceKind::Event(basic_plains_water_well),
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        ]
    }
);

make_event!(
    basic_plains_abandoned_picnic,
    Event {
        identifier: "basic_plains_abandoned_picnic",
        spawn: EventSpawn {
            weighted_regions: vec![
                (WorldRegion::Greenagis, 1),
                (WorldRegion::Emerelis, 3),
                (WorldRegion::Midgrass, 1)
            ],
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
                                (Probability::new(90), items::consumable::APPLE, (1, 3)),
                                (Probability::new(80), items::consumable::WATER, (1, 2)),
                                (Probability::new(70), items::consumable::FRIED_EGG, (1, 2)),
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

make_event!(
    basic_plains_fallen_tree,
    Event {
        identifier: "basic_plains_fallen_tree",
        spawn: EventSpawn {
            weighted_regions: vec![
                (WorldRegion::Greenagis, 1),
                (WorldRegion::Emerelis, 1),
                (WorldRegion::Gloomwood, 2),
                (WorldRegion::Midgrass, 2),
            ],
            ..Default::default()
        },
        emoji: Emoji::from_unicode("🌳"),
        message: EventMessage::Single(
            "você se depara com uma grande árvore caída bloqueando o caminho. O que deseja fazer?"
        ),
        actions: vec![
            Action {
                name: "Escalar".to_string(),
                emoji: Some(Emoji::from_unicode("🧗")),
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Rewards {
                            message: "você escalou a árvore com sucesso e encontrou alguns itens!"
                                .to_string(),
                            iterations: 1,
                            items: vec![
                                (Probability::new(80), items::material::STICK, (1, 3)),
                                (Probability::new(40), items::consumable::APPLE, (1, 2)),
                            ],
                            orbs: (5, 15),
                            xp: XpReward {
                                strength: (10, 20),
                                ..Default::default()
                            }
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(20),
                        kind: ConsequenceKind::Prejudice {
                            message: "você se feriu ao escalar!".to_string(),
                            items_amount: (0, 0),
                            max_item_valuability: 0,
                            fixed_orbs: (0, 0),
                            orbs_percentage: 0.0,
                            specific_items: vec![],
                            damage_percentage: 0.1,
                            damage_limit: 100
                        },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Cortar".to_string(),
                emoji: Some(items::tool::AXE.emoji),
                conditions: vec![Condition::HasItem(items::tool::AXE, 1)],
                consequences: vec![Consequence {
                    kind: ConsequenceKind::Rewards {
                        message: "você cortou a árvore e liberou o caminho, coletando materiais!"
                            .to_string(),
                        iterations: 2,
                        items: vec![
                            (Probability::new(100), items::material::RAW_TRUNK, (1, 1)),
                            (Probability::new(80), items::material::STICK, (2, 3)),
                        ],
                        orbs: (10, 25),
                        xp: XpReward {
                            strength: (15, 30),
                            ..Default::default()
                        }
                    },
                    ..Default::default()
                }],
                extra_consequences: vec![Consequence {
                    kind: ConsequenceKind::RemoveItemDurability(items::tool::AXE, 1),
                    ..Default::default()
                }],
                ..Default::default()
            }
        ]
    }
);

make_event!(
    basic_plains_small_stream,
    Event {
        identifier: "basic_plains_small_stream",
        spawn: EventSpawn {
            base_probability: Probability::new(60),
            weighted_regions: vec![
                (WorldRegion::Greenagis, 1),
                (WorldRegion::Emerelis, 1),
                (WorldRegion::Mudland, 1),
                (WorldRegion::Midgrass, 1),
                (WorldRegion::Sunreach, 1),
            ],
            ..Default::default()
        },
        emoji: Emoji::from_unicode("🏞️"),
        message: EventMessage::Single(
            "você encontrou um pequeno riacho cortando as planícies. O que deseja fazer?"
        ),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Beber Água".to_string(),
                emoji: Some(Emoji::from_unicode("🥤")),
                consequences: vec![Consequence {
                    kind: ConsequenceKind::Rewards {
                        message: "você bebeu a água fresca do riacho e se sentiu revigorado!"
                            .to_string(),
                        iterations: 1,
                        items: vec![],
                        orbs: (0, 0),
                        xp: XpReward {
                            health: (5, 15),
                            ..Default::default()
                        }
                    },
                    ..Default::default()
                }],
                ..Default::default()
            },
            Action {
                name: "Coletar Peixes".to_string(),
                emoji: Some(items::tool::SHOVEL.emoji),
                conditions: vec![Condition::HasItem(items::tool::SHOVEL, 1)],
                consequences: vec![
                    common::consequence_didnt_find_anything(Probability::new(30)),
                    Consequence {
                        probability: Probability::new(70),
                        kind: ConsequenceKind::Rewards {
                            message: "você conseguiu pegar alguns peixes e outros itens do riacho!"
                                .to_string(),
                            iterations: 3,
                            items: vec![
                                (Probability::new(100), items::consumable::WATER, (1, 1)),
                                (
                                    Probability::new(100),
                                    items::consumable::COMMON_FISH,
                                    (1, 1)
                                ),
                                (
                                    Probability::new(100),
                                    items::consumable::TROPICAL_FISH,
                                    (1, 1)
                                ),
                                (Probability::new(30), items::material::STONE, (1, 2)),
                            ],
                            orbs: (5, 20),
                            xp: XpReward {
                                intelligence: (5, 15),
                                strength: (5, 15),
                                ..Default::default()
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
            }
        ]
    }
);

make_event!(
    basic_plains_long_path_choice,
    Event {
        identifier: "basic_plains_long_path_choice",
        spawn: EventSpawn {
            base_probability: Probability::new(60),
            weighted_regions: vec![
                (WorldRegion::Greenagis, 1),
                (WorldRegion::Emerelis, 1),
                (WorldRegion::Mudland, 1),
                (WorldRegion::Murkswamp, 2),
                (WorldRegion::Sunreach, 1),
                (WorldRegion::Midgrass, 1),
            ],
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("🛣️"),
        message: EventMessage::Single("o caminho à sua frente é longo. Você pode escolher entre ir por outro caminho ou passar nadando. A correnteza é forte, mas pode ser um bom treino físico."),
        actions: vec![
            Action {
                name: "Ir por Outro Caminho".to_string(),
                emoji: Some(Emoji::from_unicode("🚶")),
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Message {
                            message: "você optou pelo caminho mais seguro e permaneceu seguro.".to_string(),
                            emoji: Some(Emoji::from_unicode("🌳"))
                        },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Passar Nadando".to_string(),
                emoji: Some(Emoji::from_unicode("🏊")),
                consequences: vec![
                    Consequence {
                        probability: Probability::new(60),
                        kind: ConsequenceKind::Rewards {
                            message: "você conseguiu nadar contra a correnteza e atravessar o rio!".to_string(),
                            iterations: 1,
                            items: vec![],
                            orbs: (0, 0),
                            xp: XpReward {
                                health: (20, 40),
                                intelligence: (0, 0),
                                strength: (10, 30),
                                knowledge: (0, 0)
                            }
                        },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(40),
                        kind: ConsequenceKind::Prejudice {
                            message: "a correnteza era muito forte e você foi arrastado!".to_string(),
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
            }
        ]
    }
);

make_event!(
    basic_plains_training_person,
    Event {
        identifier: "basic_plains_training_person",
        spawn: EventSpawn {
            base_probability: Probability::new(20),
            weighted_regions: vec![
                (WorldRegion::Greenagis, 1),
                (WorldRegion::Emerelis, 1),
                (WorldRegion::Midgrass, 1),
                (WorldRegion::Mudland, 1),
                (WorldRegion::Sunreach, 1),
            ],
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("🏋️‍♂️"),
        message: EventMessage::Single("você encontrou alguém treinando em plena luz do dia. Você quer treinar com ela?"),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Treinar".to_string(),
                emoji: Some(Emoji::from_unicode("💪")),
                conditions: vec![],
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Rewards {
                            message: "você treinou junto com a pessoa e sentiu uma melhora na sua força e saúde!".to_string(),
                            iterations: 1,
                            items: vec![],
                            orbs: (0, 0),
                            xp: XpReward {
                                health: (5, 10),
                                intelligence: (0, 0),
                                strength: (10, 20),
                                knowledge: (0, 0)
                            }
                        },
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }
        ]
    }
);

make_event!(
    basic_plains_fruit_tree,
    Event {
        identifier: "basic_plains_fruit_tree",
        spawn: EventSpawn {
            base_probability: Probability::new(60),
            weighted_regions: vec![
                (WorldRegion::Greenagis, 1),
                (WorldRegion::Emerelis, 2),
                (WorldRegion::Midgrass, 2)
            ],
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("🌳"),
        message: EventMessage::Multiple(&[
            "você encontrou uma árvore frutífera. O que deseja fazer?",
            "você avistou uma árvore carregada de frutas. Deseja colhê-las?",
        ]),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Colher Frutas".to_string(),
                emoji: None,
                consequences: vec![
                    common::consequence_didnt_find_anything(Probability::new(10)),
                    Consequence {
                        kind: ConsequenceKind::Rewards {
                            message: "você colheu algumas frutas frescas!".to_string(),
                            iterations: 2,
                            items: vec![
                                (Probability::new(80), items::consumable::APPLE, (1, 2)),
                                (Probability::new(80), items::consumable::ORANGE, (1, 2)),
                                (Probability::new(80), items::consumable::LEMON, (1, 2)),
                                (Probability::new(50), items::consumable::GREEN_APPLE, (1, 2)),
                            ],
                            orbs: (0, 5),
                            xp: XpReward {
                                health: (5, 15),
                                intelligence: (0, 5),
                                strength: (0, 10),
                                knowledge: (0, 5)
                            }
                        },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Cortar Galhos".to_string(),
                emoji: Some(items::tool::AXE.emoji),
                conditions: vec![Condition::HasItem(items::tool::AXE, 1)],
                consequences: vec![Consequence {
                    kind: ConsequenceKind::Rewards {
                        message: "você cortou alguns galhos da árvore e obteve materiais!"
                            .to_string(),
                        iterations: 2,
                        items: vec![
                            (Probability::new(100), items::material::STICK, (2, 4)),
                            (Probability::new(70), items::material::RAW_TRUNK, (1, 2)),
                        ],
                        orbs: (5, 15),
                        xp: XpReward {
                            health: (0, 5),
                            intelligence: (0, 5),
                            strength: (5, 15),
                            knowledge: (0, 10)
                        }
                    },
                    ..Default::default()
                }],
                extra_consequences: vec![Consequence {
                    kind: ConsequenceKind::RemoveItemDurability(items::tool::AXE, 1),
                    ..Default::default()
                }],
                ..Default::default()
            }
        ]
    }
);
