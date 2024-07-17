use etheris_data::{items, world::regions::RegionKind};

use super::prelude::*;

const EMOJI: Emoji = Emoji::from_emote(Some("vinizi"), 1215290819133837364);
const TAG_FIRST_STAGE: &str = "vinizi_first_stage";

const REGIONS: &[(WorldRegion, i32)] = &[
    (WorldRegion::Emerelis, 1),
    (WorldRegion::Gloomwood, 1),
    (WorldRegion::Murkswamp, 1),
    (WorldRegion::Midgrass, 1),
    (WorldRegion::Sunreach, 1),
    (WorldRegion::Tenypt, 1),
    (WorldRegion::Ethergrove, 1),
    (WorldRegion::Starbreeze, 1),
    (WorldRegion::Ethergrove, 2),
    (WorldRegion::Midgrass, 1),
    (WorldRegion::Wornpeaks, 1),
];

pub fn vinizi_first_encounter(_: EventBuildState) -> Event {
    Event {
        identifier: "vinizi_first_encounter",
        spawn: EventSpawn {
            base_probability: Probability::new(99),
            weighted_regions: REGIONS.to_vec(),
            conditions: vec![Condition::Not(Box::new(Condition::HasTag("has_known_vinizi")))],
        },
        emoji: EMOJI,
        message: EventMessage::Multiple(&[
            "do nada um homem jovem loiro caiu do céu no chão perto de você e pareceu ferido depois de cair. O que você quer fazer?",
            "você se assusta com um homem que cai do céu perto de você. Ele parece ferido e cansado. O que você quer fazer?"
        ]),
        actions: vec![
            common::ignore_action_with_extra_consequences(vec![
                Consequence {
                    kind: ConsequenceKind::AddTag("has_known_vinizi".to_string()),
                    ..Default::default()
                }
            ]),
            Action {
                name: "\"Você está bem?\"".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Event(vinizi_first_encounter_interaction),
                        ..Default::default()
                    }
                ],
                extra_consequences: vec![Consequence {
                    kind: ConsequenceKind::AddTag("has_known_vinizi".to_string()),
                    ..Default::default()
                }],
                ..Default::default()
            }
        ]
    }
}

fn vinizi_first_encounter_interaction(_: EventBuildState) -> Event {
    Event {
        identifier: "vinizi_first_encounter_interaction",
        spawn: EventSpawn::never(),
        emoji: EMOJI,
        message: EventMessage::Single(
            "o homem parece feliz ao te ver. De forma desesperada ele se aproxima e diz: `\"Oi! Oi! Oi! Meu nome é Vinizi! Qual o seu nome? Qual o seu nome?!\"`"
        ),
        actions: vec![
            Action {
                name: "Ir embora".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Message { message: "enquanto se afasta, você escuta Vinizi te chamando repetidamente, você escuta `\"Por favor! Nós nunca mais nos verem-\"` até que a voz dele é cortada de forma abruta.".to_string(), emoji: None },
                        ..Default::default()
                    },
                ],
                ..Default::default()
            },
            Action {
                name: "Se apresentar".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Event(vinizi_first_interaction_stage_one),
                        ..Default::default()
                    },
                ],
                extra_consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::AddTag(TAG_FIRST_STAGE.to_string()),
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
        ]
    }
}

fn vinizi_first_interaction_stage_one(_: EventBuildState) -> Event {
    Event {
        identifier: "vinizi_first_interaction_stage_one",
        spawn: EventSpawn::never(),
        emoji: EMOJI,
        message: EventMessage::Single("`\"fico feliz em te conhecer, [NAME]! É ótimo conhecer novas pessoas! Desculpe parecer tão desesperado, eu tenho uma doença que me teletransporta para um lugar aleatório a cada alguns minutos, e eu não tenho como controlar. Ha ha! É difícil viver teleportando de região em região... Ei, [NAME], posso te colocar minha marca de ether? Rápido! Meu tempo nessa região tá acabando!\"`"),
        actions: vec![
            Action {
                name: "\"O que? Não!\"".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Message { message: "`\"POR FAVOR! A MINHA MARCA É SEGURA, PARA QUE MINHA DOENÇA DO TELEPORTE FAÇA EU PODER TE VER OUTRA VEZ, AMIGO! EU NÃO QUERO CONTINUAR SOZIN-\"`, antes que pudesse terminar a frase, Vinizi teleportou.".to_string(), emoji: None },
                        ..Default::default()
                    },
                ],
                extra_consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::RemoveTag(TAG_FIRST_STAGE.to_string()),
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            Action {
                name: "\"Marca?\"".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Message { message: "`\"Sim! Minha marca de ether faz com que minha doença me teleporte até você às vezes! Assim nós poderemos nos ver. Aqui vai ela!\"`. Depois de dizer isso, Vinizi toca na sua testa e aplica um pouco de ether. Você sente uma sensação confortável e fecha seus olhos. Quando abre os olhos, Vinizi não está mais ali.".to_string(), emoji: Some(EMOJI) },
                        ..Default::default()
                    },
                ],
                ..Default::default()
            }
        ]
    }
}

pub fn vinizi_first_stage(_: EventBuildState) -> Event {
    Event {
        identifier: "vinizi_first_stage",
        spawn: EventSpawn {
            base_probability: Probability::new(5),
            weighted_regions: REGIONS.to_vec(),
            conditions: vec![Condition::HasTag(TAG_FIRST_STAGE)],
        },
        emoji: EMOJI,
        message: EventMessage::Multiple(&[
            "Vinizi te chama: `\"Ei! [NAME], lembra de mim? Vinizi! Não tô com muito tempo, mas vamos conversar!\"`",
            "você vê Vinizi aparecer do nada e se assusta, Vinizi nota seu susto e diz: `\"Ha ha! Desculpa te assustar, [NAME]. Eu só estava em mais uma das minhas infinitas viagens. Ei, quer conversar antes que eu suma?\"`",
            "um homem loiro aparece do nada do seu lado: é Vinizi. Ele olha para você, e feliz, diz: `\"Oi [NAME]! A marca nos uniu novamente! Quer conversar?\"`",
        ]),
        actions: vec![
            Action {
                name: "\"Sim, eu quero conversar!\"".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Event(vinizi_first_stage_interaction),
                        ..Default::default()
                    }
                ],
                extra_consequences: vec![],
                ..Default::default()
            },
            Action {
                name: "\"Estou sem tempo...\"".to_string(),
                emoji: None,
                consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::Message { message: "`Vinizi responde: \"Ah, tudo bem. A marca vai nos unir de novo algum dia! Eu espero.\"` e então desaparece.".to_string(), emoji: None },
                        ..Default::default()
                    }
                ],
                extra_consequences: vec![
                    Consequence {
                        kind: ConsequenceKind::RemoveTag("vinizi_first_stage".to_string()),
                        probability: Probability::new(5),
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
        ]
    }
}

fn vinizi_first_stage_interaction(state: EventBuildState) -> Event {
    let mut messages = vec![
        "`\"Ah! É tão bom poder conversar com você, [NAME]. Apesar de eu já estar sentindo alguns átomos do meu corpo teleportando. Ei, você sabia que é difícil conseguir comida pra mim? É sério! Eu nem mesmo tenho tempo de comprar algo. Se eu teleporto para uma loja, em alguns segundos já sou teleportado novamente para outro lugar, ha ha ha! É por isso qu-\"` Vinizi teleportou antes de terminar a frase.".to_string(),
        "`\"Ei [NAME], você alguma vez já viu um yeti? Alguns minutos atrás eu fui teleportado para perto de um. Foi assustador! Ele segurava um bastão de gelo, e quando estava prestes a me bater, eu teleportei! De vez em quando o meu poder me salva. Mas normalmente ele só-\"`. Vinizi teleportou antes de terminar a frase.".to_string(),
        "`\"[NAME], você gosta de política? Eu não! Mas dois dias atrás eu teleportei para a sala do prefeito de Metrolis e ouvi ele dizendo algo sobre iniciar uma guerra contra Garhyan. Já pensou?! Essa guerra só acabaria quando Hakiko descongelar, ha ha ha! Você acha que Metrolis teria chanc-\"`. Vinizi teleportou antes de terminar a frase.".to_string(),
        "`\"Ei [NAME], já te contei sobre o tempo que fui teleportado para um show de rock? Foi chocante! Eu apareci no meio do palco, bem na frente da banda. Eles ficaram muito assustados! Mas antes que pudesse desfrutar do show, já tinha sido teleportado novamente. Às vezes, ser eu é uma verdadeira montanha-russa! Ha ha ha! Teve um dia qu-\"`. Vinizi teleportou antes de terminar a frase.".to_string(),
        "`\"[NAME], você já imaginou ser teleportado para um lugar completamente diferente enquanto está no meio de uma conversa? Isso acontece comigo o tempo todo! Às vezes eu acabo em situações estranhas por causa disso. Eu já me acostumei, mas às vezes isso me irrit-\"`. Vinizi teleportou antes de terminar a frase.".to_string(),
        "`\"Aí, [NAME], você já ouviu falar do Festival de Fogos de Artifício de Astra? Eu fui teleportado para lá uma vez e acabei no meio de um show de luzes espetacular! Foi uma experiência incrível, mas durou apenas alguns segundos antes de eu ser teleportado novamente. Às vezes, desejo que pudesse ficar em um lugar por mais tempo, mas a vida de um teleportador é cheia de surpresas! Ah, estou sentindo, acho que vou ser telepor-\"`. Vinizi teleportou antes de terminar a frase.".to_string(),
        "`\"[NAME], você já se perguntou como seria ser um pássaro? Eu me teleportei para o topo de uma montanha uma vez e fiquei rodeado por pássaros voando. Foi uma sensação incrível. Sério! Por alguns instantes, senti como se pudesse voar também. Mas é claro que logo fui teleportado novamente para algum outro lugar. Ah, a vida emocionante que eu levo! Ha ha ha! Ei, voc-\"`. Vinizi teleportou antes de terminar a frase.".to_string(),
    ];

    if state.character.region.kind() == RegionKind::Desert {
        for _ in 0..=3 {
            messages.push("`\"Eu estava com saudade, [NAME]! Mas por que você tá nesse deserto? Eu não gosto muito de teleportar para regiões com temperaturas extremas, eu não tenho tempo nem para preparar minhas roupas! A minha sorte é que minha doença me teleporta para o meio do oceano às vezes, então eu consigo tomar banho. Ha ha h-\"`. Vinizi teleportou antes de terminar a frase.".to_string());
        }
    }

    if state.character.region.kind() == RegionKind::Mountains {
        for _ in 0..=3 {
            messages.push("`\"Me conta, [NAME], como você est... Espera... A gente está em uma montanha?! EI, EU TENHO MUITO MEDO DE ALTURA! O QUE VOCÊ ESTÁ FAZENDO AQUI, AMIGO? ALTURA É PERIGOSA, SABIA? VOCÊ DEVIA DESC-\"`. Vinizi teleportou antes de terminar a frase.".to_string());
        }
    }

    if state.character.has_item(&items::lore::HAKIKO_LEGEND, 1) {
        for _ in 0..=2 {
            messages.push("`\"Ah, espera! [NAME], onde você conseguiu essa item? A Lenda de Hakiko?! Eu sou um grande fã do General Hakiko! Eu gostaria de ter tempo para estudar o Hakikotenchou, ha ha ha! Se bem qu-\"`. Vinizi teleportou antes de terminar a frase.".to_string());
            messages.push("`\"Ei, [NAME]. Um dia desses eu conheci um descendente distante do Hakiko! Você consegue acreditar? Ele não é tão forte quando o General, mas possui uma determinação lendária. Será que um dia veremos uma nova lenda? Espero qu-\"`. Vinizi teleportou antes de terminar a frase.".to_string());
        }
    }

    Event {
        identifier: "vinizi_first_stage_interaction",
        spawn: EventSpawn::never(),
        emoji: EMOJI,
        message: EventMessage::MultipleString(messages),
        actions: vec![],
    }
}
