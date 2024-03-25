use self::common::all_regions;

use super::prelude::*;

pub fn special_track_miniorbs(_: EventBuildState) -> Event {
    Event {
        identifier: "special_track_miniorbs",
        spawn: EventSpawn {
            base_probability: Probability::new(40),
            weighted_regions: all_regions(1),
            conditions: vec![]
        },
        emoji: Emoji::from_unicode("🔍"),
        message: EventMessage::Conditional(vec![
            (Condition::SimilarPowerTo(enemies::special::miniorbs()), "você viu um Miniorbs da Sorte! Ele está prestes a fugir, e você sente que seu poder é parecido com o dele. Quer persegui-lo?".to_string()),
            (Condition::StrongerThan(enemies::special::miniorbs()), "você viu um Miniorbs da Sorte! Ele está prestes a fugir, e você sente que seu poder é superior! Quer persegui-lo?".to_string()),
            (Condition::WeakerThan(enemies::special::miniorbs()), "você viu um Miniorbs da Sorte! Ele está prestes a fugir, mas você nota que o poder do Miniorbs é uma AMEAÇA para você. Quer persegui-lo mesmo sendo perigoso irritá-lo?".to_string()),
        ]),
        actions: vec![
            common::ignore_action(),
            Action {
                name: "Perseguir".to_string(),
                emoji: None,
                conditions: vec![],
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Event(special_track_miniorbs_tracking_part),
                        ..Default::default()
                    }
                ],
                extra_consequences: vec![],
                ..Default::default()
            }
        ]
    }
}

make_event!(
    didnt_found_miniorbs,
    Event {
        identifier: "didnt_found_miniorbs",
        spawn: EventSpawn::never(),
        emoji: Emoji::from_unicode("☹️"),
        message: EventMessage::Single("você não encontrou o Miniorbs! Ele provavelmente fugiu sem que você pudesse o encontrar."),
        actions: vec![],
    }
);

fn special_track_miniorbs_tracking_part(_: EventBuildState) -> Event {
    let consequences = vec![
        Consequence {
            kind: ConsequenceKind::Event(didnt_found_miniorbs),
            ..Default::default()
        },
        Consequence {
            kind: ConsequenceKind::InstantBattle(enemies::special::miniorbs()),
            ..Default::default()
        },
    ];

    Event {
        identifier: "special_track_miniorbs_tracking_part",
        spawn: EventSpawn::never(),
        emoji: Emoji::from_unicode("🔍"),
        message: EventMessage::Single(
            "o Miniorbs se escondeu! Você notou um árvore suspeita em que ele pode ter subido, e também viu um buraco que cabe um Miniorbs. Qual você quer olhar?"
        ),
        actions: vec![
            Action {
                name: "Olhar Árvore".to_string(),
                emoji: Some(Emoji::from_unicode("🌳")),
                conditions: vec![],
                consequences: consequences.clone(),
                extra_consequences: vec![],
                ..Default::default()
            },
            Action {
                name: "Olhar Buraco".to_string(),
                emoji: Some(Emoji::from_unicode("🔍")),
                conditions: vec![],
                consequences: consequences.clone(),
                extra_consequences: vec![],
                ..Default::default()
            }
        ]
    }
}
