use etheris_data::{
    items::{self, get_item, ItemTag},
    personality::Personality,
    BrainKind, SkillKind,
};

use super::prelude::*;

pub fn basic_plains_exploration(state: EventBuildState) -> Event {
    Event {
        identifier: "basic_plains_exploration",
        spawn: EventSpawn {
            weighted_regions: vec![(WorldRegion::Greenagis, 6), (WorldRegion::Emerelis, 6)],
            ..Default::default()
        },
        emoji: Emoji::from_unicode("🗺️"),
        message: EventMessage::Multiple(&[
            "você avista uma paisagem verde e pacífica à sua frente! Como você vai explorar?",
            "do seu redor até o horizonte só se pode ver natureza e paisagens verdes. Como você quer explorar a planície?",
        ]),
        actions: vec![
            Action {
                name: "Procurar Ameaças",
                emoji: Some(Emoji::from_unicode("⚔️")),
                consequences: vec![
                    common::consequence_didnt_find_anything(Probability::new(20)),
                    Consequence {
                        kind: ConsequenceKind::MultiplePossibleEncounters(get_enemies_by_regions(&[state.character.region])),
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Caminhar",
                emoji: Some(Emoji::from_unicode("🚶")),
                consequences: vec![
                    common::consequence_didnt_find_anything(Probability::new(5)),
                    Consequence {
                        probability: Probability::new(60),
                        kind: ConsequenceKind::Rewards { iterations: 1, items: vec![], orbs: (8, 16), xp: XpReward::default() },
                        ..Default::default()
                    },
                    Consequence {
                        probability: Probability::new(20),
                        kind: ConsequenceKind::Event(basic_plains_weak_thief),
                        ..Default::default()
                    }
                ],
                ..Default::default()
            }
        ]
    }
}

static WEAK_THIEF: Lazy<Enemy> = Lazy::new(|| Enemy {
    identifier: "weak_thief",
    name: "Ladrão Comum",
    brain: BrainKind::Simple,
    base_probability: Probability::ALWAYS,
    regions: &[],
    personalities: &[Personality::Cowardice],
    allies: None,
    weapon: None,
    resistance: 110,
    vitality: 200,
    intelligence: 3,
    strength: 6,
    ether: 15,
    skills: vec![
        SkillKind::TornadoKick,
        SkillKind::MirrorDamage,
        SkillKind::ImbuedPunch,
    ],
    drop: EnemyReward {
        orbs: (10, 15),
        xp: (20, 50),
        items: vec![EnemyRewardItem {
            item: items::tool::SHOVEL,
            amount: (1, 1),
            probability: Probability::new(30),
        }],
    },
});

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
        fixed_orbs: (0, 10),
        items_amount: (1, 5),
        max_item_valuability: 100,
        orbs_percentage: 0.05,
        specific_items: if let Some(item) = most_valuable_item {
            vec![(item, 1)]
        } else {
            vec![]
        },
    };

    Event {
        identifier: "basic_plains_weak_thief",
        spawn: EventSpawn::never(),
        emoji: Emoji::from_unicode("🔫"),
        message: EventMessage::Conditional(vec![
            (Condition::SimilarPowerTo(WEAK_THIEF.to_owned()), format!("um ladrão te ameaçou e {asking_text}. A força dele é semelhante à sua. Como você quer reagir?")),
            (Condition::StrongerThan(WEAK_THIEF.to_owned()), format!("um ladrão te ameaçou e {asking_text}. Ele não aparenta ameaça alguma para sua força. Como você quer reagir?")),
            (Condition::WeakerThan(WEAK_THIEF.to_owned()), format!("um ladrão te ameaçou e {asking_text}. Você sentiu uma poderosa pressão de ether vindo dele, é um inimigo perigoso. Como você quer reagir?")),
        ]),
        actions: vec![
            Action {
                name: "Aceitar Assalto",
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
                name: "Resistir",
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::InstantBattle(WEAK_THIEF.to_owned()),
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
                name: "Cavar",
                emoji: Some(items::tool::SHOVEL.emoji),
                conditions: vec![Condition::HasItem(items::tool::SHOVEL, 1)],
                consequences: vec![
                    common::consequence_didnt_find_anything(Probability::new(5)),
                    Consequence {
                        kind: ConsequenceKind::Rewards {
                            iterations: 5,
                            items: vec![
                                (Probability::new(60), items::material::STONE, (1, 2)),
                                (Probability::new(40), items::material::RAW_TRUNK, (1, 1)),
                                (Probability::new(20), items::consumable::APPLE, (0, 1)),
                                (Probability::new(40), items::consumable::WATER, (0, 1)),
                                (Probability::new(60), items::consumable::SALT, (0, 1)),
                                (Probability::new(60), items::consumable::SUGAR, (0, 1)),
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

make_event!(
    basic_plains_water_well,
    Event {
        identifier: "basic_plains_water_well",
        spawn: EventSpawn {
            weighted_regions: vec![
                (WorldRegion::Greenagis, 1),
                (WorldRegion::Mudland, 1),
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
                name: "Cavar",
                emoji: Some(items::tool::SHOVEL.emoji),
                conditions: vec![Condition::HasItem(items::tool::SHOVEL, 1)],
                consequences: vec![
                    common::consequence_didnt_find_anything(Probability::new(5)),
                    Consequence {
                        kind: ConsequenceKind::Rewards {
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
                name: "Destruir",
                emoji: Some(items::tool::HAMMER.emoji),
                conditions: vec![Condition::HasItem(items::tool::HAMMER, 1)],
                consequences: vec![Consequence {
                    kind: ConsequenceKind::Rewards {
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
