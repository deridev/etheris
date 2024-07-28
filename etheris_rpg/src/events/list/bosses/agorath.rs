use etheris_data::BossKind;

use super::super::prelude::*;

make_event!(agorath_first_encounter, Event {
    identifier: "agorath_first_encounter",
    emoji: Emoji::from_unicode("🤜"),
    spawn: EventSpawn {
        base_probability: Probability::new(40),
        weighted_regions: vec![(WorldRegion::Sunreach, 1)],
        conditions: vec![
            Condition::Not(Box::new(Condition::HasTag("fighting_agorath"))),
            Condition::Not(Box::new(Condition::DefeatedBoss(BossKind::Agorath)))
        ]
    },
    message: EventMessage::Single("um homem com cicatrizes no rosto e corpo musculoso se aproxima de você. `\"Ei... Você... Quer duelar? Você parece forte.\"`"),
    actions: vec![
        Action {
            name: "Duelar".to_string(),
            emoji: Some(Emoji::from_unicode("🚶‍♂️")),
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Event(agorath_battle),
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
        Action {
            name: "Recusar duelo".to_string(),
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Message {
                        message: "o homem olha pra você com um olhar visivelmente decepcionado. `\"Entendo... Você não é um duelista honrado. Adeus.\"`".to_string(),
                        emoji: None
                    },
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    ],
    ..Default::default()
});

make_event!(
    agorath_rematch,
    Event {
        identifier: "agorath_rematch",
        emoji: Emoji::from_unicode("🤜"),
        spawn: EventSpawn {
            base_probability: Probability::new(80),
            weighted_regions: vec![(WorldRegion::Sunreach, 1)],
            conditions: vec![
                Condition::HasTag("fighting_agorath"),
                Condition::Not(Box::new(Condition::DefeatedBoss(BossKind::Agorath)))
            ]
        },
        message: EventMessage::Single(
            "Agorath se aproxima de você. `\"Ei... Você... Quer duelar de novo?\"`"
        ),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Duelar".to_string(),
                emoji: None,
                consequences: vec![Consequence {
                    kind: ConsequenceKind::Event(agorath_battle),
                    ..Default::default()
                },],
                ..Default::default()
            },
        ],
    }
);

make_event!(
    agorath_battle,
    Event {
        identifier: "agorath_battle",
        spawn: EventSpawn::never(),
        emoji: Emoji::from_unicode("🤜"),
        message: EventMessage::Single(
            "a figura imponente te encara. `\"Sim! Isso! Finalmente... Um duelo de verdade!\"`"
        ),
        actions: vec![Action {
            name: "Duelar".to_string(),
            emoji: None,
            consequences: vec![Consequence {
                kind: ConsequenceKind::Battle(BattleConsequence {
                    allies: vec![],
                    enemies: vec![bosses::agorath()],
                    prompt: true,
                    on_win_kill_event: Some(agorath_battle_win),
                    on_win_knockout_event: Some(agorath_battle_win),
                    ..Default::default()
                }),
                ..Default::default()
            }],
            extra_consequences: vec![Consequence {
                kind: ConsequenceKind::AddTag("fighting_agorath".to_string()),
                ..Default::default()
            }],
            ..Default::default()
        },],
    }
);

fn agorath_battle_win(_: EventBuildState) -> Event {
    Event {
        identifier: "agorath_battle_win",
        emoji: Emoji::from_unicode("✊"),
        spawn: EventSpawn::never(),
        message: EventMessage::Single("você ouve sussurros de um duelista invencível que fora derrotado. `\"Você... Você é uma verdadeira força a ser respeitada. O primeiro a vencer um duelo... Contra o Duelista.\"` - as palavras cessam."),
        actions: vec![
            Action {
                name: "Continuar".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Message {
                            message: "você encara o corpo derrotado de Agorath, e continua sua jornada.".to_string(),
                            emoji: None
                        },
                        ..Default::default()
                    },
                ],
                extra_consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::RemoveTag("fighting_agorath".to_string()),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
        ],
        ..Default::default()
    }
}
