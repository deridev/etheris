use std::ops::Mul;

use crate::*;
use common::*;
use etheris_common::Probability;
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

pub async fn tick_every_effect(
    fighters: &[FighterIndex],
    controller: &mut BattleController,
) -> anyhow::Result<()> {
    for fighter_index in fighters {
        let fighter = controller.battle.get_fighter_mut(*fighter_index);
        let fighter_name = fighter.name.clone();

        for effect in fighter.effects.clone() {
            let mut api = BattleApi::new(controller);
            api.fighter_index = *fighter_index;
            api.target_index = *fighter_index;

            match effect.kind {
                EffectKind::Flaming => {
                    api.fighter_mut()
                        .remove_effect(Effect::new(effect.kind, 5, effect.culprit));
                    melt_ice(10, &mut api);

                    let dmg = api
                        .apply_damage(
                            api.fighter_index,
                            DamageSpecifier {
                                kind: DamageKind::Fire,
                                amount: (api.fighter().health().max as f32 * 0.01) as i32,
                                balance_effectiveness: 1,
                                accuracy: 100,
                                ..Default::default()
                            },
                        )
                        .await;

                    api.emit_message(format!(
                        "***{}** queimou e recebeu **{dmg}**!",
                        fighter_name
                    ));
                }
                EffectKind::Burning => {
                    api.fighter_mut()
                        .remove_effect(Effect::new(effect.kind, 5, effect.culprit));

                    melt_ice(20, &mut api);

                    let dmg = api
                        .apply_damage(
                            api.fighter_index,
                            DamageSpecifier {
                                kind: DamageKind::Fire,
                                amount: (api.fighter().health().max as f32 * 0.01) as i32,
                                balance_effectiveness: 1,
                                accuracy: 100,
                                ..Default::default()
                            },
                        )
                        .await;

                    api.emit_message(format!(
                        "***{}** está em combustão e recebeu **{dmg}**!",
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
                            "***{}** não está mais com uma maldição!",
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
                            "***{}** perdeu a proteção leve extra!*",
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
                        api.emit_message(format!("***{}** descongelou*", api.fighter().name));
                    }
                }
                EffectKind::Bleeding => {
                    if effect.amount >= 100 {
                        api.fighter_mut().remove_effect(Effect::new(
                            effect.kind,
                            effect.amount,
                            Default::default(),
                        ));

                        let dmg = (api.fighter().health().max as f32).mul(0.2) as i32;
                        api.fighter_mut().take_damage(
                            effect.culprit,
                            DamageSpecifier {
                                culprit: effect.culprit,
                                kind: DamageKind::Special,
                                amount: dmg,
                                balance_effectiveness: 0,
                                accuracy: 100,
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

        if fighter.resistance.value > 0 || fighter.flags.contains(FighterFlags::ASKED_TO_RISK_LIFE)
        {
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

    // Passives: on_damage_miss
    for (damage, fighter) in controller.battle.turn_end_queues.damage_misses.clone() {
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
                controller.emit_turn_message(format!("***{}** pousou no chão!*", fighter_name));
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
