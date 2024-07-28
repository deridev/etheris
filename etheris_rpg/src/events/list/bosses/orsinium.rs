use super::super::prelude::*;
use etheris_data::{items, BossKind};

make_event!(orsinium_first_encounter, Event {
    identifier: "orsinium_first_encounter",
    spawn: EventSpawn {
        base_probability: Probability::new(5),
        weighted_regions: vec![(WorldRegion::Wornpeaks, 1)],
        conditions: vec![
            Condition::Not(Box::new(Condition::HasTag("fighting_orsinium"))),
            Condition::Not(Box::new(Condition::DefeatedBoss(BossKind::Orsinium)))
        ]
    },
    message: EventMessage::Single("você encontra uma caverna com construções metálicas e uma porta gigante de ferro. O que você quer fazer?"),
    emoji: Emoji::from_unicode("🕳️"),
    actions: vec![
        common::ignore_action(),
        Action {
            name: "Entrar".to_string(),
            emoji: Some(Emoji::from_unicode("🚶‍♂️")),
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Event(orsinium_body_encounter),
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
    ],
    ..Default::default()
});

make_event!(orsinium_body_encounter, Event {
    identifier: "orsinium_body_encounter",
    spawn: EventSpawn::never(),
    message: EventMessage::Single("você se depara com uma construção de metal e madeira humanoide de 3 metros no chão, deitada. Em seus punhos, a escritura \"Orsinium\" está encravada com metais diversos. O que você quer fazer?"), 
    actions: vec![
        common::ignore_action(),
        Action {
            name: "Tocar".to_string(),
            emoji: None,
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Event(orsinium_boss_battle),
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
    ],
    ..Default::default()
});

make_event!(orsinium_rematch, Event {
    identifier: "orsinium_rematch",
    spawn: EventSpawn {
        base_probability: Probability::new(80),
        weighted_regions: vec![(WorldRegion::Wornpeaks, 1)],
        conditions: vec![
            Condition::HasTag("fighting_orsinium"),
            Condition::Not(Box::new(Condition::DefeatedBoss(BossKind::Orsinium)))
        ]
    },
    message: EventMessage::Single("você avista a mesma caverna em que outrora enfrentou o Colosso Forjado. O que você quer fazer?"),
    actions: vec![
        common::ignore_action(),
        Action {
            name: "Ir".to_string(),
            emoji: Some(Emoji::from_unicode("🚶‍♂️")),
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Event(orsinium_boss_battle),
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
    ],
    ..Default::default()
});

make_event!(orsinium_boss_battle, Event {
    identifier: "orsinium_boss_battle",
    spawn: EventSpawn::never(),
    message: EventMessage::Single("Orsinium te encara com os olhos brilhando em fogo. `\"Ameaça prestes a ser neutralizada. Glória à Soberana.\"`"),
    actions: vec![
        Action {
            name: "Enfrentar".to_string(),
            emoji: None,
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Battle(BattleConsequence {
                        allies: vec![],
                        enemies: vec![bosses::orsinium()],
                        prompt: true,
                        on_win_kill_event: Some(orsinium_defeated),
                        on_win_knockout_event: Some(orsinium_defeated),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            ],
            extra_consequences: vec![
                Consequence {
                    kind: ConsequenceKind::AddTag("fighting_orsinium".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
    ],
    ..Default::default()
});

fn orsinium_defeated(_: EventBuildState) -> Event {
    Event {
        identifier: "orsinium_defeated",
        spawn: EventSpawn::never(),
        message: EventMessage::Single("o corpo metálico de Orsinium cai no chão, causando um estrondo pela caverna. Sua boca abre e emite um som. `\"Ó, não será hoje nosso crepúsculo. Amelia, me diga, fui um bom colosso?\"` seguido de suspiros, até que seus olhos apagam."),
        actions: vec![
            Action {
                name: "Tocar no núcleo".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Rewards {
                            message: "você toca no núcleo de Orsinium e recolhe um poderoso item.".to_string(),
                            iterations: 1,
                            items: vec![
                                (Probability::ALWAYS, items::special::INVIGORATING_CRYSTAL, (1, 1))
                            ],
                            orbs: (0, 0),
                            xp: XpReward::default()
                        },
                        ..Default::default()
                    }
                ],
                extra_consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::RemoveTag("fighting_orsinium".to_string()),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
        ],
        ..Default::default()
    }
}
