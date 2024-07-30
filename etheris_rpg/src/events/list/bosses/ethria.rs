use super::super::prelude::*;
use etheris_data::{items, BossKind};

make_event!(ethria_first_encounter, Event {
    identifier: "ethria_first_encounter",
    spawn: EventSpawn {
        base_probability: Probability::new(15),
        weighted_regions: vec![(WorldRegion::Tenypt, 1), (WorldRegion::Sandywater, 2)],
        conditions: vec![
            Condition::Not(Box::new(Condition::HasTag("fighting_ethria"))),
            Condition::Not(Box::new(Condition::DefeatedBoss(BossKind::Ethria)))
        ]
    },
    message: EventMessage::Single("você avista um templo roxo e escuro no meio do nada. Toda a areia ao redor é roxa e escura. O que você quer fazer?"),
    actions: vec![
        common::ignore_action(),
        Action {
            name: "Entrar".to_string(),
            emoji: Some(Emoji::from_unicode("🚶‍♂️")),
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Event(ethria_first_meeting),
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
    ],
    ..Default::default()
});

make_event!(ethria_first_meeting, Event {
    identifier: "ethria_first_meeting",
    spawn: EventSpawn::never(),
    message: EventMessage::Single("no meio do templo há uma mulher de mantos negros e máscara nos olhos. Seus cabelos roxos contrastam com meus lábios brancos. Ela te encara e fala com uma voz fria. \"Não é muito cortês invadir a casa de uma dama...\""), 
    actions: vec![
        Action {
            name: "Continuar ali".to_string(),
            emoji: None,
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Event(ethria_rage),
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
        Action {
            name: "Sair correndo".to_string(),
            emoji: Some(Emoji::from_unicode("🚶‍♂️")),
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Message { message: "você sai correndo do perigo iminente.".to_string(), emoji: None },
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    ],
    ..Default::default()
});

make_event!(ethria_rage, Event {
    identifier: "ethria_rage",
    spawn: EventSpawn::never(),
    message: EventMessage::Single("a mulher te encara com um olhar decepcionado. \"Entendo... Então deixe que Ethria te ensine o respeito.\" - a mulher denominada Ethria pega uma katana do altar."), 
    actions: vec![
        Action {
            name: "Se preparar".to_string(),
            emoji: None,
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Event(ethria_boss_battle),
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
    ],
    ..Default::default()
});

make_event!(
    ethria_rematch,
    Event {
        identifier: "ethria_rematch",
        spawn: EventSpawn {
            base_probability: Probability::new(80),
            weighted_regions: vec![(WorldRegion::Tenypt, 1), (WorldRegion::Sandywater, 2)],
            conditions: vec![
                Condition::HasTag("fighting_ethria"),
                Condition::Not(Box::new(Condition::DefeatedBoss(BossKind::Ethria)))
            ]
        },
        message: EventMessage::Single(
            "você avista o templo roxo de Ethria. O que você quer fazer?"
        ),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Ir".to_string(),
                emoji: Some(Emoji::from_unicode("🚶‍♂️")),
                consequences: vec![Consequence {
                    kind: ConsequenceKind::Event(ethria_boss_battle),
                    ..Default::default()
                }],
                ..Default::default()
            },
        ],
        ..Default::default()
    }
);

make_event!(ethria_boss_battle, Event {
    identifier: "ethria_boss_battle",
    spawn: EventSpawn::never(),
    message: EventMessage::Single("Ethria aponta a katana para você. \"Te farei entender, ser simplório, o motivo pelo qual leis existem.\""),
    actions: vec![
        Action {
            name: "Enfrentar".to_string(),
            emoji: None,
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Battle(BattleConsequence {
                        allies: vec![],
                        enemies: vec![bosses::ethria()],
                        prompt: true,
                        on_win_kill_event: Some(ethria_defeated),
                        on_win_knockout_event: Some(ethria_defeated),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            ],
            extra_consequences: vec![
                Consequence {
                    kind: ConsequenceKind::AddTag("fighting_ethria".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
    ],
    ..Default::default()
});

fn ethria_defeated(_: EventBuildState) -> Event {
    Event {
        identifier: "ethria_defeated",
        spawn: EventSpawn::never(),
        message: EventMessage::Single("Ethria cai no chão, derrubando sua katana. Com a perda de sua consciência, o templo perde sua cor roxa escura e vira uma construção normal."),
        actions: vec![
            Action {
                name: "Pegar sua katana".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Rewards {
                            message: "você recolhe a katana de Ethria.".to_string(),
                            iterations: 1,
                            items: vec![
                                (Probability::ALWAYS, items::tool::ETHRIA_KATANA, (1, 1))
                            ],
                            orbs: (0, 0),
                            xp: XpReward::default()
                        },
                        ..Default::default()
                    }
                ],
                extra_consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::RemoveTag("fighting_ethria".to_string()),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
        ],
        ..Default::default()
    }
}
