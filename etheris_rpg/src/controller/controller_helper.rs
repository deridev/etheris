use std::ops::Mul;

use crate::*;
use common::*;
use etheris_common::Probability;
use etheris_data::emojis;
use etheris_discord::EmbedField;
use etheris_framework::Response;

fn melt_ice(melt_power: i32, api: &mut BattleApi<'_>) {
    if let Some(effect) = api.fighter().get_effect(EffectKind::Frozen) {
        api.fighter_mut().remove_effect(Effect::new(
            EffectKind::Frozen,
            melt_power,
            effect.culprit,
        ));
        api.fighter_mut()
            .apply_effect(Effect::new(EffectKind::Wet, melt_power, effect.culprit));
    }

    if let Some(effect) = api.fighter().get_effect(EffectKind::Ice) {
        api.fighter_mut().remove_effect(Effect::new(
            EffectKind::Ice,
            melt_power - 5,
            effect.culprit,
        ));
        api.fighter_mut().apply_effect(Effect::new(
            EffectKind::Wet,
            melt_power - 5,
            effect.culprit,
        ));
    }
}

pub async fn on_start(controller: &mut BattleController) -> anyhow::Result<()> {
    for fighter_index in controller.battle.alive_fighters.clone() {
        let fighter = controller.battle.get_fighter(fighter_index).clone();
        for skill in fighter.skills.clone() {
            let mut api = BattleApi::new(controller);
            api.fighter_index = fighter_index;
            api.target_index = fighter.target;

            let mut skill = skill.dynamic_skill.lock().await;
            skill.on_start(api).await?;
        }
    }
    Ok(())
}

pub async fn tick_every_modifier(
    fighters: &[FighterIndex],
    controller: &mut BattleController,
) -> anyhow::Result<()> {
    for fighter_index in fighters {
        let fighter = controller.battle.get_fighter_mut(*fighter_index);

        for modifier in fighter.modifiers.list.iter_mut() {
            if let Some(turns_remaining) = modifier.turns_remaining {
                modifier.turns_remaining = Some(turns_remaining.saturating_sub(1));
            }
        }

        fighter
            .modifiers
            .list
            .retain(|m| m.turns_remaining.is_none() || m.turns_remaining.is_some_and(|t| t > 0));
    }

    Ok(())
}

pub async fn tick_every_effect(
    fighters: &[FighterIndex],
    controller: &mut BattleController,
) -> anyhow::Result<()> {
    for fighter_index in fighters {
        let fighter = controller.battle.get_fighter_mut(*fighter_index);
        let immunities = fighter.body_immunities.clone();
        let fighter_name = fighter.name.clone();

        for effect in fighter.effects.clone() {
            let mut api = BattleApi::new(controller);
            api.fighter_index = *fighter_index;
            api.target_index = *fighter_index;

            let immunity_dmg_multiplier = if let Some(immunity) = effect.kind.affected_immunity() {
                immunities.dmg_multiplier_from_immunity(immunity)
            } else {
                1.0
            };

            match effect.kind {
                EffectKind::Flaming => {
                    api.fighter_mut()
                        .remove_effect(Effect::new(effect.kind, 5, effect.culprit));
                    melt_ice(10, &mut api);

                    let dmg = 2 + (api.fighter().health().max as f32 * 0.01) as i32;

                    let dmg = api
                        .apply_damage(
                            api.fighter_index,
                            DamageSpecifier {
                                kind: DamageKind::Fire,
                                amount: (dmg as f64 * immunity_dmg_multiplier) as i32,
                                balance_effectiveness: 1,
                                accuracy: 255,
                                ..Default::default()
                            },
                        )
                        .await;

                    api.emit_message(format!("**{}** queimou e recebeu **{dmg}**!", fighter_name));
                }
                EffectKind::Burning => {
                    api.fighter_mut()
                        .remove_effect(Effect::new(effect.kind, 5, effect.culprit));

                    melt_ice(20, &mut api);

                    let dmg = 8 + (api.fighter().health().max as f32 * 0.03) as i32;

                    let dmg = api
                        .apply_damage(
                            api.fighter_index,
                            DamageSpecifier {
                                kind: DamageKind::Fire,
                                amount: (dmg as f64 * immunity_dmg_multiplier) as i32,
                                balance_effectiveness: 4,
                                accuracy: 255,
                                ..Default::default()
                            },
                        )
                        .await;

                    api.emit_message(format!(
                        "**{}** está em combustão e recebeu **{dmg}**!",
                        fighter_name
                    ));
                }
                EffectKind::Shocked => {
                    api.fighter_mut()
                        .remove_effect(Effect::new(effect.kind, 10, effect.culprit));
                    // Handled by the controller
                }
                EffectKind::Paralyzed => {}
                EffectKind::Curse => {
                    let safe = api.fighter_mut().remove_effect(Effect::new(
                        effect.kind,
                        10,
                        effect.culprit,
                    ));
                    if safe {
                        api.emit_message(format!(
                            "**{}** não está mais com uma maldição!",
                            fighter_name
                        ));
                    }
                }
                EffectKind::LowProtection => {
                    let unprotected = api.fighter_mut().remove_effect(Effect::new(
                        effect.kind,
                        1,
                        effect.culprit,
                    ));

                    if unprotected {
                        api.emit_message(format!(
                            "**{}** perdeu a proteção leve extra!*",
                            api.fighter().name
                        ));
                    }
                }
                EffectKind::Ice => {
                    api.fighter_mut()
                        .remove_effect(Effect::new(effect.kind, 5, effect.culprit));
                }
                EffectKind::Wet => {
                    api.fighter_mut()
                        .remove_effect(Effect::new(effect.kind, 15, effect.culprit));
                    api.fighter_mut().remove_effect(Effect::new(
                        EffectKind::Flaming,
                        60,
                        effect.culprit,
                    ));
                    api.fighter_mut().remove_effect(Effect::new(
                        EffectKind::Burning,
                        20,
                        effect.culprit,
                    ));

                    if let Some(shock) = api.fighter().get_effect(EffectKind::Shocked) {
                        if shock.amount < 10 || effect.amount < 10 {
                            return Ok(());
                        }

                        let dmg = match shock.amount {
                            0..=30 => 0.05,
                            31..=60 => 0.08,
                            61..=90 => 0.1,
                            _ => 0.2,
                        };

                        let dmg = ((api.fighter().health().max as f32) * dmg).floor() as i32;

                        api.fighter_mut().remove_effect(shock);
                        let dmg = api
                            .apply_damage(
                                api.fighter_index,
                                DamageSpecifier {
                                    culprit: effect.culprit,
                                    kind: DamageKind::Special,
                                    amount: dmg,
                                    balance_effectiveness: 10,
                                    accuracy: 100,
                                    effect: None,
                                },
                            )
                            .await;

                        api.emit_message(format!(
                            "**{}** eletrocutou devido a água e recebeu **{dmg}**!",
                            api.fighter().name
                        ));

                        if Probability::new(20).generate_random_bool() {
                            api.apply_effect(
                                api.fighter().index,
                                Effect::new(EffectKind::Paralyzed, 1, effect.culprit),
                            )
                            .await;
                        }
                    }
                }
                EffectKind::Frozen => {
                    let melted = api.fighter_mut().remove_effect(Effect::new(
                        effect.kind,
                        5,
                        effect.culprit,
                    ));
                    api.fighter_mut().remove_effect(Effect::new(
                        EffectKind::Flaming,
                        20,
                        effect.culprit,
                    ));
                    api.fighter_mut().remove_effect(Effect::new(
                        EffectKind::Burning,
                        10,
                        effect.culprit,
                    ));

                    if melted {
                        api.emit_message(format!("**{}** descongelou*", api.fighter().name));
                    }
                }
                EffectKind::Bleeding => {
                    if effect.amount >= 100 {
                        api.fighter_mut().remove_effect(Effect::new(
                            effect.kind,
                            effect.amount,
                            Default::default(),
                        ));

                        let dmg = 20 + (api.fighter().health().max as f32).mul(0.15) as i32;
                        api.fighter_mut().take_damage(
                            effect.culprit,
                            DamageSpecifier {
                                culprit: effect.culprit,
                                kind: DamageKind::Special,
                                amount: (dmg as f64 * immunity_dmg_multiplier) as i32,
                                balance_effectiveness: 0,
                                accuracy: 255,
                                effect: None,
                            },
                        );
                        api.defer_message(format!(
                            "**{fighter_name}** teve uma hemorragia que causou **{dmg} dano**!"
                        ));
                    } else {
                        api.fighter_mut().remove_effect(Effect::new(
                            effect.kind,
                            5,
                            Default::default(),
                        ));
                    }
                }
                EffectKind::Poisoned => {
                    api.fighter_mut().remove_effect(Effect::new(
                        effect.kind,
                        5,
                        Default::default(),
                    ));

                    let dmg = 6 + (api.fighter().health().max as f32 * 0.022) as i32;
                    let dmg = api
                        .apply_damage(
                            api.fighter_index,
                            DamageSpecifier {
                                kind: DamageKind::Poisonous,
                                amount: (dmg as f64 * immunity_dmg_multiplier) as i32,
                                balance_effectiveness: 2,
                                accuracy: 255,
                                ..Default::default()
                            },
                        )
                        .await;

                    api.defer_message(format!("**{}** recebeu **{dmg}** do veneno!", fighter_name));
                }
                EffectKind::Exhausted => {
                    api.fighter_mut().remove_effect(Effect::new(
                        effect.kind,
                        1,
                        Default::default(),
                    ));
                }
            }
        }
    }

    Ok(())
}

pub async fn should_risk_life(
    fighters: &[FighterIndex],
    controller: &mut BattleController,
) -> anyhow::Result<()> {
    let settings = controller.battle.settings.clone();
    for fighter_index in fighters {
        let fighter = controller.battle.get_fighter_mut(*fighter_index);
        let fighter_name = fighter.name.clone();

        if fighter.resistance.value > 0
            || fighter.flags.contains(FighterFlags::ASKED_TO_RISK_LIFE)
            || fighter.flags.contains(FighterFlags::GAVE_UP)
        {
            continue;
        }

        if fighter.health().value == 0 {
            continue;
        }

        let fighter = fighter.clone();
        let confirmation = if !settings.is_risking_life_allowed {
            false
        } else if let Some(brain) = fighter.brain.clone() {
            let mut api = BattleApi::new(controller);
            api.fighter_index = fighter.index;
            api.target_index = fighter.target;

            brain.dynamic_brain.lock().await.should_risk_life(api).await
        } else if let Some(user) = &fighter.user {
            controller.ctx.helper()
                .create_confirmation(user.id, true, Response::new_user_reply(
                    user,
                    if settings.casual {
                        "sua resistência chegou a zero! Quer apostar sua vida na batalha? (Essa é uma batalha CASUAL. Seu personagem não vai morrer de verdade)"
                    } else {
                        "sua resistência chegou a zero! Isso significa que seu personagem está perto do nocaute. Você deseja continuar lutando e apostar sua vida nessa batalha?\nSe não aceitar, você perderá por nocaute. Se aceitar, você continuará lutando mas perderá **vitalidade** ao invés de resistência. Se a vitalidade zerar seu personagem morre pra sempre."
                    },
                ))
                .await?
        } else {
            false
        };

        let fighter = controller.battle.get_fighter_mut(*fighter_index);
        fighter.flags.insert(FighterFlags::ASKED_TO_RISK_LIFE);

        if confirmation {
            fighter.flags.intersects(FighterFlags::RISKING_LIFE);
            fighter.defeated_by = None;
            fighter.is_defeated = false;
            controller.defer_message(format!(
                "**{}** está arriscando sua vida agora!",
                fighter_name
            ));
        } else {
            fighter.is_defeated = true;
            controller.defer_message(format!(
                "**{}** não teve motivação para arriscar sua vida e perdeu a consciência!",
                fighter_name
            ));
        }
    }

    Ok(())
}

pub async fn passives(controller: &mut BattleController) -> anyhow::Result<()> {
    // Passives: on_damage
    for (damage, fighter) in controller.battle.turn_end_queues.damages.clone() {
        let fighter_index = fighter;
        let target_index = damage.culprit;

        let skills = controller.battle.get_fighter(fighter).skills.clone();
        for skill in skills {
            let mut api = BattleApi::new(controller);
            api.fighter_index = fighter_index;
            api.target_index = target_index;

            skill
                .dynamic_skill
                .lock()
                .await
                .passive_on_damage(api, damage)
                .await
                .ok();
        }
    }

    // Passives: on_damage (pact)
    for (damage, fighter) in controller.battle.turn_end_queues.damages.clone() {
        let fighter_index = fighter;
        let target_index = damage.culprit;

        for pact in controller.battle.get_fighter(fighter).pacts.clone() {
            let mut api = BattleApi::new(controller);
            api.fighter_index = fighter_index;
            api.target_index = target_index;

            pact.dynamic_pact
                .lock()
                .await
                .on_damage(api, damage)
                .await
                .ok();
        }
    }

    // Passives: on_damage_miss
    for (who_missed, damage, fighter) in controller.battle.turn_end_queues.damage_misses.clone() {
        let fighter_index = fighter;

        let skills = controller.battle.get_fighter(fighter).skills.clone();
        for skill in skills {
            let mut api = BattleApi::new(controller);
            api.fighter_index = fighter_index;
            api.target_index = who_missed;

            skill
                .dynamic_skill
                .lock()
                .await
                .passive_on_damage_miss(api, damage)
                .await
                .ok();
        }
    }

    // Passives: tick fighter skills
    for fighter in controller.battle.alive_fighters.clone() {
        let fighter = controller.battle.get_fighter_mut(fighter);
        let fighter_index = fighter.index;
        let target_index = fighter.target;

        for skill in fighter.skills.clone() {
            let mut api = BattleApi::new(controller);
            api.fighter_index = fighter_index;
            api.target_index = target_index;

            skill
                .dynamic_skill
                .lock()
                .await
                .passive_fighter_tick(api)
                .await
                .ok();
        }
    }

    // Passives: tick fighter pacts
    for fighter_index in controller.battle.alive_fighters.clone() {
        let fighter = controller.battle.get_fighter_mut(fighter_index);
        let fighter_index = fighter.index;
        let target_index = fighter.target;

        for pact in fighter.pacts.clone() {
            let mut api = BattleApi::new(controller);
            api.fighter_index = fighter_index;
            api.target_index = target_index;

            pact.dynamic_pact.lock().await.fighter_tick(api).await.ok();
        }
    }

    // Passives: on_kill
    for fighter_index in controller.battle.alive_fighters.clone() {
        let fighter = controller.battle.get_fighter(fighter_index).clone();
        let Some(killer) = fighter.killed_by else {
            continue;
        };

        let killer_skills = controller.battle.get_fighter(killer).skills.clone();

        for skill in killer_skills {
            let mut api = BattleApi::new(controller);
            api.fighter_index = killer;
            api.target_index = fighter.index;
            skill
                .dynamic_skill
                .lock()
                .await
                .passive_on_kill(api, fighter.index)
                .await
                .ok();
        }
    }

    Ok(())
}

pub async fn tick_cycle(
    fighters: &[FighterIndex],
    controller: &mut BattleController,
) -> anyhow::Result<()> {
    for fighter_index in fighters {
        let fighter = controller.battle.get_fighter(*fighter_index).clone();
        let fighter_name = fighter.name.clone();

        if let Composure::OnAir(meters) = fighter.composure {
            if meters <= 1 {
                controller.battle.get_fighter_mut(*fighter_index).composure = Composure::Standing;
                controller.emit_turn_message(format!("**{}** pousou no chão!", fighter_name));
            } else {
                controller.battle.get_fighter_mut(*fighter_index).composure =
                    Composure::OnAir(meters - 1);
            }
        }

        // Passives: on_cycle
        for fighter_index in controller.battle.alive_fighters.clone() {
            let fighter = controller.battle.get_fighter(fighter_index).clone();
            let skills = fighter.skills.clone();

            for skill in skills {
                let mut api = BattleApi::new(controller);
                api.fighter_index = fighter_index;
                api.target_index = fighter.target;
                skill
                    .dynamic_skill
                    .lock()
                    .await
                    .passive_on_cycle(api)
                    .await
                    .ok();
            }
        }
    }

    Ok(())
}

pub fn create_fighter_embed_fields(
    fighter: &Fighter,
    target_index: Option<FighterIndex>,
) -> EmbedField {
    let mut fighter = fighter.clone();
    let index = fighter.index;

    let is_target = Some(index) == target_index;
    let target_string = if is_target {
        "🎯 ".to_string()
    } else {
        String::new()
    };

    let mut displays = vec![];

    if fighter.resistance.value > 0 {
        displays.push(format!(
            "{} {}",
            emojis::RESISTANCE,
            fighter.resistance.value
        ))
    }

    if fighter.vitality.value != fighter.vitality.max || fighter.resistance.value <= 0 {
        displays.push(format!("{} {}", emojis::VITALITY, fighter.vitality.value));
    }

    displays.push(format!("{} {}", emojis::ETHER, fighter.ether.value));

    if fighter.overload > 5.0 {
        displays.push(format!("🧨 **Sobrecarga**: {}%", fighter.overload as i64));
    }

    match fighter.balance {
        0..=15 => displays.push("Equilíbrio Zero".to_string()),
        16..=40 => displays.push("Nenhum Equilíbrio".to_string()),
        41..=60 => displays.push("Equilíbrio Muito Baixo".to_string()),
        61..=80 => displays.push("Equilíbrio Baixo".to_string()),
        _ => (),
    }

    match &fighter.composure {
        Composure::Standing => (),
        Composure::OnGround => displays.push("**No Chão**".to_string()),
        Composure::OnAir(n) => displays.push(format!("**{n}m No Ar**")),
    }

    fighter.effects.sort_by_key(|k| k.kind);

    for effect in fighter.effects.iter() {
        match effect.kind {
            EffectKind::Flaming => displays.push(format!("♨️ **Queimando**: {}%", effect.amount)),
            EffectKind::Burning => displays.push(format!("🔥 **Combustão**: {}%", effect.amount)),
            EffectKind::Shocked => displays.push(format!("⚡ **Choque**: {}%", effect.amount)),
            EffectKind::Paralyzed => {
                displays.push(format!("😵‍💫 **Paralisado**: {} turnos", effect.amount))
            }
            EffectKind::Wet => displays.push(format!("💧 **Molhado**: {}%", effect.amount)),
            EffectKind::Ice => displays.push(format!("❄️ **Congelando**: {}%", effect.amount)),
            EffectKind::Frozen => {
                displays.push(format!("🧊 **Congelado**: {} turnos", effect.amount))
            }
            EffectKind::Bleeding => {
                displays.push(format!("🩸 **Sangramento**: {}%", effect.amount))
            }
            EffectKind::Poisoned => displays.push(format!(
                "<:poison:1260219639624110090> **Veneno**: {}%",
                effect.amount
            )),
            EffectKind::Curse => displays.push(format!("⚫ **Maldição**: {}%", effect.amount)),
            EffectKind::Exhausted => {
                displays.push(format!("😞 **Exausto**: {} turnos", effect.amount))
            }

            EffectKind::LowProtection => {
                displays.push(format!("🛡️ **Proteção Leve**: {} turnos", effect.amount))
            }
        }
    }

    EmbedField {
        name: format!(
            "{target_string}{}{}",
            if is_target {
                format!("__{}__", fighter.name)
            } else {
                fighter.name.to_owned()
            },
            if fighter.defense == 0 {
                String::new()
            } else {
                format!(" ({})", emojis::SHIELD)
            }
        ),
        value: displays.join("\n"),
        inline: true,
    }
}
