use etheris_data::{items, personality::Personality};

use super::prelude::*;
pub fn basic_mountain_exploration(_state: EventBuildState) -> Event {
    Event {
        identifier: "basic_mountain_exploration",
        spawn: EventSpawn {
            weighted_regions: vec![(WorldRegion::Wornpeaks, 10)],
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
        ConsequenceKind::Event(basic_mountain_person_in_danger)
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
                (WorldRegion::Greenagis, 3),
                (WorldRegion::Emerelis, 3),
                (WorldRegion::Midgrass, 1),
            ],
            ..Default::default()
        },
        emoji: Emoji::from_unicode("💥"),
        message: EventMessage::Conditional(vec![
            (
                Condition::HasPersonality(Personality::Arrogance),
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
