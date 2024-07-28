use etheris_data::{items, personality::Personality, BossKind};
use weaklings::weak_shredder;

use super::super::prelude::*;

make_event!(garhyan_shredder_first_invitation, Event {
    identifier: "garhyan_shredder_first_invitation",
    spawn: EventSpawn {
        base_probability: Probability::new(5),
        weighted_regions: vec![(WorldRegion::Gloomwood, 1), (WorldRegion::Mudland, 3)],
        conditions: vec![
            Condition::StrongerThan(weak_shredder()),
            Condition::HasTag("hated_by_shredders"),
            Condition::Not(Box::new(Condition::HasTag("fighting_garhyan"))),
            Condition::Not(Box::new(Condition::DefeatedBoss(BossKind::Garhyan)))
        ]
    },
    message: EventMessage::Single("um Retalhador se aproxima com desdém. `\"Você... Sempre nos dando problemas! O chefe te chamou pra uma conversinha. Você vai vir comigo, AGORA!\"`"),
    actions: vec![
        Action {
            name: "Ir".to_string(),
            emoji: Some(Emoji::from_unicode("🚶‍♂️")),
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Event(garhyan_going_to),
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
        Action {
            name: "Recusar chamado".to_string(),
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Battle(BattleConsequence {
                        allies: vec![],
                        enemies: vec![weak_shredder(), weak_shredder(), weak_shredder()],
                        prompt: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    ],
    ..Default::default()
});

make_event!(garhyan_going_to, Event {
    identifier: "garhyan_going_to",
    spawn: EventSpawn::never(),
    message: EventMessage::Single("o Retalhador começa a te escoltar até uma base subterrânea por alguns minutos. Nenhuma palavra é dita no caminho, até que, dentro de um bunker subterrâneo, você vê uma porta gigante de ferro. `\"Atrás dessa porta está nosso chefe, verme. Respeite-o!` - a porta se abre."),
    actions: vec![
        Action {
            name: "Entrar".to_string(),
            emoji: None,
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Event(garhyan_inside_bunker),
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
        Action {
            name: "Fugir".to_string(),
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Battle(BattleConsequence {
                        allies: vec![],
                        enemies: vec![weak_shredder(), weak_shredder(), weak_shredder()],
                        prompt: false,
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    ],
    ..Default::default()
});

make_event!(garhyan_inside_bunker, Event {
    identifier: "garhyan_inside_bunker",
    spawn: EventSpawn::never(),
    message: EventMessage::Single("você se depara com um homem em pé, olhando para um quadro na parede, uma pintura de destroços com uma figura humana no meio. O homem começa a falar: `\"Ora... Você sabe quanto tempo eu levei? Os Retalhadores, por mais fracos que sejam, são parte do meu império.\"` - homem se vira. `\"Seus pais nunca te ensinaram a temer Garhyan?`\""),
    actions: vec![
        Action {
            name: "\"Você não me assusta!\"".to_string(),
            emoji: None,
            conditions: vec![Condition::HasPersonality(Personality::Courage)],
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Event(garhyan_first_agression),
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
        Action {
            name: "\"Por favor poupe minha vida!\"".to_string(),
            emoji: None,
            conditions: vec![Condition::HasPersonality(Personality::Cowardice)],
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Event(garhyan_first_agression),
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
        Action {
            name: "Ficar em silêncio".to_string(),
            emoji: None,
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Event(garhyan_first_agression),
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
    ],
    ..Default::default()
});

make_event!(garhyan_first_agression, Event {
    identifier: "garhyan_first_agression",
    spawn: EventSpawn::never(),
    message: EventMessage::Single("Garhyan lhe oferece uma última chance. `\"Una-se a mim como Retalhador, e eu vou poupar a sua vida miserável.\"`"),
    actions: vec![
        Action {
            name: "Recusar".to_string(),
            emoji: None,
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Event(garhyan_boss_battle),
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
        Action {
            name: "Aceitar".to_string(),
            emoji: None,
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::AddTag("shredder_member".to_string()),
                    ..Default::default()
                },
                Consequence {
                    kind: ConsequenceKind::RemoveTag("hated_by_shredders".to_string()),
                    ..Default::default()
                },
                Consequence {
                    kind: ConsequenceKind::RemoveKarma(3),
                    ..Default::default()
                },
                Consequence {
                    kind: ConsequenceKind::Message {
                        emoji: None,
                        message: "`\"ótimo!\"` - Garhyan exclama. `\"Meu cemitério já tá cheio de corpos. Agora você é um Retalhador! Pode sair daqui com vida.\"`".to_string()
                    },
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
    ],
    ..Default::default()
});

make_event!(garhyan_rematch, Event {
    identifier: "garhyan_rematch",
    spawn: EventSpawn {
        base_probability: Probability::new(80),
        weighted_regions: vec![(WorldRegion::Gloomwood, 1), (WorldRegion::Mudland, 3)],
        conditions: vec![
            Condition::HasTag("fighting_garhyan"),
            Condition::Not(Box::new(Condition::DefeatedBoss(BossKind::Garhyan)))
        ]
    },
    message: EventMessage::Single("um Retalhador te avista de longe. `\"Ei! O Lorde Garhyan tá te caçando. Vou te levar pra ele agora mesmo!\"`"),
    actions: vec![
        common::ignore_action(),
        Action {
            name: "Ir".to_string(),
            emoji: Some(Emoji::from_unicode("🚶‍♂️")),
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Event(garhyan_boss_battle),
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
    ],
    ..Default::default()
});

make_event!(garhyan_boss_battle, Event {
    identifier: "garhyan_boss_battle",
    spawn: EventSpawn::never(),
    message: EventMessage::Single("Garhyan te encara com raiva. `\"Pode vir pra cima. Eu vou te retalhar por mexer com Garhyan, o Senhor dos Ratos.\"`"),
    actions: vec![
        Action {
            name: "Enfrentar".to_string(),
            emoji: None,
            consequences: vec![
                Consequence {
                    kind: ConsequenceKind::Battle(BattleConsequence {
                        allies: vec![],
                        enemies: vec![bosses::garhyan()],
                        prompt: true,
                        on_win_kill_event: Some(garhyan_defeated),
                        on_win_knockout_event: Some(garhyan_defeated),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            ],
            extra_consequences: vec![
                Consequence {
                    kind: ConsequenceKind::AddTag("fighting_garhyan".to_string()),
                    ..Default::default()
                },
            ],
            ..Default::default()
        },
    ],
    ..Default::default()
});

fn garhyan_defeated(_: EventBuildState) -> Event {
    Event {
        identifier: "garhyan_defeated",
        spawn: EventSpawn::never(),
        message: EventMessage::Single("alguns minutos depois da sua vitória, Retalhadores entram na sala e se chocam ao ver Garhyan derrotado. Eles imediatamente saem correndo, com medo de você. O senhor dos ratos, Garhyan, foi derrotado por você, **[NAME]**!"),
        actions: vec![
            Action {
                name: "Ir embora".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Message {
                            message: "você sai do bunker, sabendo que derrotou um poderoso inimigo.".to_string(), 
                            emoji: None,
                        },
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "Vascular o bunker de Garhyan".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Rewards {
                            message: "você achou alguns itens interessantes!".to_string(),
                            iterations: 1,
                            items: vec![
                                (Probability::ALWAYS, items::lore::RAT_LORDS_DIARY, (1, 1))
                            ],
                            orbs: (20, 40),
                            xp: XpReward::default()
                        },
                        ..Default::default()
                    }
                ],
                extra_consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::RemoveTag("fighting_garhyan".to_string()),
                        ..Default::default()
                    },
                    Consequence {
                        kind: ConsequenceKind::AddKarma(4),
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
        ],
        ..Default::default()
    }
}
