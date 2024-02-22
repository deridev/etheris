use std::{
    ops::{Add, Mul, Sub},
    time::Duration,
};

use anyhow::bail;
use etheris_common::{Color, Probability};
use etheris_data::emojis;
use etheris_discord::{
    twilight_model::{
        channel::{message::component::ButtonStyle, *},
        id::{marker::UserMarker, Id},
    },
    *,
};
use etheris_framework::{watcher::WatcherOptions, CommandContext, EmbedPagination, Response};
use rand::{seq::SliceRandom, Rng};

use crate::{common::*, get_input, *};

use self::data::{weapons::execute_weapon_attack, Reward};

#[derive(Debug, Clone)]
pub struct BattleResult {
    pub reward: Reward,
    pub winners: Vec<Fighter>,
}

pub struct BattleController<'a> {
    pub battle: Battle,
    pub ctx: &'a mut CommandContext,
    pub last_message: Option<Message>,
    pub last_interaction: Option<Interaction>,
    pub current_turn_history: TurnHistory,

    should_reinput: bool,
}

impl<'a> BattleController<'a> {
    pub fn new(battle: Battle, ctx: &'a mut CommandContext) -> Self {
        Self {
            battle,
            ctx,
            last_message: None,
            last_interaction: None,
            current_turn_history: TurnHistory::default(),
            should_reinput: false,
        }
    }

    pub async fn send_full_history(
        &mut self,
        allowed_users: Vec<Id<UserMarker>>,
    ) -> anyhow::Result<()> {
        const TURNS_PER_PAGE: usize = 5;
        if self.battle.history.is_empty() {
            return Ok(());
        }

        let mut pages = vec![];

        for i in (0..self.battle.history.len()).step_by(TURNS_PER_PAGE) {
            let mut embed = EmbedBuilder::new_common()
                .set_color(Color::LIGHT_YELLOW)
                .set_description(
                    self.battle
                        .fighters
                        .iter()
                        .map(|f| bold(&f.name))
                        .collect::<Vec<_>>()
                        .join(" vs "),
                )
                .set_author(EmbedAuthor {
                    name: "Histórico da Luta".to_string(),
                    icon_url: None,
                });

            for j in 0..TURNS_PER_PAGE {
                let Some(turn) = self.battle.history.get(i + j) else {
                    break;
                };

                let turn_fighter = self.battle.get_fighter(turn.fighter);

                embed = embed.add_not_inlined_field(
                    format!("• [{}] Turno de {}", i + j + 1, turn_fighter.name),
                    turn.messages.join("\n"),
                );
            }

            pages.push(embed);
        }

        EmbedPagination::new(self.ctx.clone(), pages)
            .set_allowed_users(allowed_users)
            .set_timeout(Duration::from_secs(1000))
            .send()
            .await
    }

    async fn prepare(&mut self) -> anyhow::Result<()> {
        for (user, fighter) in self
            .battle
            .fighters
            .iter()
            .filter_map(|f| f.user.as_ref().map(|u| (u, f)))
        {
            self.ctx.client.mark_user_as_fighter(user.id).await;

            if self.battle.settings.has_consequences {
                let Some(mut character) = self
                    .ctx
                    .db()
                    .characters()
                    .get_by_user(&user.id.to_string())
                    .await?
                else {
                    continue;
                };

                character.remove_orbs(fighter.drop.orbs.into());
                for item in &fighter.drop.items {
                    character.remove_item(item.item, item.amount as usize);
                }

                self.ctx.db().characters().save(character).await?;
            }
        }

        self.battle.reallocate_all_targets();

        Ok(())
    }

    pub async fn run(&mut self) -> anyhow::Result<BattleResult> {
        self.prepare().await?;

        let mut turn_errors = 0;

        while self.battle.state == BattleState::Running {
            match self.turn().await {
                Ok(()) => (),
                Err(e) => {
                    // Error tolerancy of 5
                    if turn_errors < 5 {
                        turn_errors += 1;
                        continue;
                    }

                    for user in self.battle.fighters.iter().filter_map(|f| f.user.as_ref()) {
                        self.ctx.client.remove_user_fighting_mark(user.id).await;
                    }

                    return Err(e);
                }
            }

            if self.battle.state != BattleState::Running {
                self.save_user_characters().await.ok();
            }

            tokio::time::sleep(Duration::from_millis(300)).await;

            if self.battle.turn_counter % 10 == 0 {
                self.save_user_characters().await.ok();
            }
        }

        for user in self.battle.fighters.iter().filter_map(|f| f.user.as_ref()) {
            self.ctx.client.remove_user_fighting_mark(user.id).await;
        }

        let winners: Vec<Fighter> = match &self.battle.state {
            BattleState::Running => {
                unreachable!()
            }
            BattleState::Ended { winners, .. } => winners
                .iter()
                .map(|w| self.battle.get_fighter(*w).to_owned())
                .collect(),
        };

        if winners.is_empty() {
            bail!("A battle can't draw");
        }

        let rewards = self
            .battle
            .defeated_fighters
            .iter()
            .map(|f| self.battle.get_fighter(*f).drop.clone())
            .fold(Reward::default(), |acc, r| acc.add(r));

        self.finish_battle(rewards.clone(), &winners).await?;

        Ok(BattleResult {
            reward: rewards,
            winners,
        })
    }

    async fn finish_battle(&mut self, rewards: Reward, winners: &[Fighter]) -> anyhow::Result<()> {
        self.save_user_characters_stats().await?;
        let winner_team = winners[0].team;
        let mut teams = self.battle.full_teams().into_iter().collect::<Vec<_>>();
        teams.sort_unstable_by_key(|(x, _)| *x);

        let mut embed = EmbedBuilder::new_common()
            .set_color(Color::GREEN)
            .set_title("Fim da batalha! ⚔️")
            .set_description(
                teams
                    .into_iter()
                    .map(|(n, fighters)| {
                        format!(
                            "## {}**Time [{n}]**\n{}",
                            if n == winner_team { "🌟 " } else { "" },
                            fighters
                                .iter()
                                .map(|f| f.name.to_owned())
                                .collect::<Vec<_>>()
                                .join(", ")
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n"),
            );

        if let Some(human_winner) = winners.iter().find_map(|w| w.user.clone()) {
            if !rewards.is_empty() {
                if let Some(mut character) = self
                    .ctx
                    .db()
                    .characters()
                    .get_by_user(&human_winner.id.to_string())
                    .await?
                {
                    let orbs = rewards.orbs as i64;
                    let xp = rewards.xp as u32;
                    let items = rewards.items;

                    character.add_orbs(orbs);
                    character.health_xp += (xp + 2) / 3;
                    character.intelligence_xp += (xp + 2) / 3;
                    character.strength_xp += (xp + 2) / 3;

                    for item in items.iter() {
                        character.add_item(item.item, item.amount as usize, None);
                    }

                    self.ctx.db().characters().save(character).await?;

                    embed = embed.add_field_with_emoji(
                        "💰",
                        EmbedField {
                            name: "Recompensas".into(),
                            value: format!(
                                "-> {}\n**Orbs**: {}\n**XP**: {}\n{}",
                                human_winner.mention(),
                                orbs,
                                xp,
                                items
                                    .iter()
                                    .map(|i| format!(
                                        "**{} {}x {}**",
                                        i.item.emoji, i.amount, i.item.display_name
                                    ))
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            ),
                            inline: true,
                        },
                    );
                }
            }
        }
        let history_button = ButtonBuilder::new()
            .set_label("Ver Histórico")
            .set_custom_id("show_history");
        let row = ActionRowBuilder::new().add_button(history_button.clone());
        let message = self
            .ctx
            .send(Response::from(embed).set_components(vec![row]))
            .await?;

        let allowed_users = self
            .battle
            .fighters
            .iter()
            .filter_map(|u| u.user.as_ref().map(|u| u.id))
            .collect::<Vec<_>>();

        let allowed = allowed_users.clone();
        let Ok(Some(component)) = self
            .ctx
            .watcher
            .await_single_component(
                message.id,
                move |interaction| {
                    interaction
                        .author_id()
                        .is_some_and(|id| allowed.contains(&id))
                },
                WatcherOptions {
                    timeout: Duration::from_secs(120),
                },
            )
            .await
        else {
            return Ok(());
        };

        let mut temp_ctx = CommandContext::from_with_interaction(self.ctx, Box::new(component));
        temp_ctx
            .update_message(Response::default().set_components(
                vec![ActionRowBuilder::new().add_button(
                    history_button
                        .clone()
                        .set_disabled(true)
                        .set_style(ButtonStyle::Success),
                )],
            ))
            .await?;

        self.send_full_history(allowed_users).await?;

        Ok(())
    }

    pub async fn turn(&mut self) -> anyhow::Result<()> {
        let fighter = self.battle.get_current_fighter().clone();

        self.current_turn_history = TurnHistory {
            round: self.battle.history.len() + 1,
            fighter: fighter.index,
            target: fighter.target,
            messages: vec![],
        };

        if fighter.has_effect(EffectKind::Frozen) {
            self.emit_turn_message(format!(
                "**{}** está congelado e não pode lutar.",
                fighter.name
            ));
            self.should_reinput = false;
        } else if let Some(paralyzed) = fighter.get_effect(EffectKind::Paralyzed) {
            self.battle
                .get_current_fighter_mut()
                .remove_effect(Effect::new(EffectKind::Paralyzed, 1, paralyzed.culprit));
            self.emit_turn_message(format!(
                "**{}** está paralisado e não pode lutar. Turnos de paralisia restantes: `{}`",
                fighter.name,
                paralyzed.amount - 1
            ));
            self.should_reinput = false;
        } else {
            let input = self.get_input().await?;
            self.process_input(input).await?;
        }

        if !self.should_reinput {
            self.update_turn_history_message().await?;
            self.next_turn().await?;
            self.battle.next_fighter();
        }

        self.should_reinput = false;

        Ok(())
    }

    pub async fn update_turn_history_message(&mut self) -> anyhow::Result<()> {
        let response = Response::from(self.create_turn_embed()).remove_all_components();

        if let Some(message) = &self.last_message {
            self.ctx.update_specific_message(message, response).await?;
            tokio::time::sleep(Duration::from_secs(
                2 + (self.current_turn_history.messages.len() as u64).saturating_sub(2),
            ))
            .await;
        } else {
            let message = self.ctx.send(response).await?;
            self.last_message = Some(message);
        }

        Ok(())
    }

    pub async fn save_user_characters(&mut self) -> anyhow::Result<()> {
        if !self.battle.settings.has_consequences {
            return Ok(());
        }

        for (user, fighter) in self
            .battle
            .fighters
            .iter()
            .filter_map(|f| f.user.as_ref().map(|user| (user.clone(), f.clone())))
        {
            let Some(mut character) = self
                .ctx
                .db()
                .characters()
                .get_by_user(&user.id.to_string())
                .await?
            else {
                continue;
            };

            for fighter in self.battle.fighters.clone() {
                if fighter.is_defeated {
                    continue;
                }

                for skill in fighter.skills {
                    let skill = skill.dynamic_skill.lock().await;
                    let kind = skill.save_kind();
                    if skill
                        .data()
                        .complexity
                        .prob_of_aknowleding()
                        .generate_random_bool()
                        && Probability::new(50).generate_random_bool()
                    {
                        character.aknowledge_skill(kind);
                    }
                }
            }

            let mut skills = Vec::with_capacity(fighter.skills.capacity());
            for skill in fighter.skills.clone() {
                skills.push(skill.dynamic_skill.lock().await.save_kind());
            }

            character.stats.resistance = fighter.resistance.into();
            character.stats.vitality = fighter.vitality.into();
            character.stats.ether = fighter.ether.into();
            character.skills = skills;
            self.ctx.db().characters().save(character).await?;
        }

        Ok(())
    }

    pub async fn save_user_characters_stats(&mut self) -> anyhow::Result<()> {
        if !self.battle.settings.has_consequences {
            return Ok(());
        }

        for (user, fighter) in self
            .battle
            .fighters
            .iter()
            .filter_map(|f| f.user.as_ref().map(|user| (user.clone(), f.clone())))
        {
            let fighter_index = fighter.index;
            let Some(mut character) = self
                .ctx
                .db()
                .characters()
                .get_by_user(&user.id.to_string())
                .await?
            else {
                continue;
            };

            let is_pvp = self
                .battle
                .fighters
                .iter()
                .filter_map(|f| f.user.clone())
                .filter(|u| u.id != user.id)
                .count()
                > 0;
            let is_pve = self
                .battle
                .fighters
                .iter()
                .filter(|f| f.user.is_none())
                .count()
                > 0;

            // Update stats
            for fighter in self.battle.fighters.clone() {
                if fighter.index == fighter_index {
                    continue;
                }

                let is_ai = fighter.user.is_none();

                if fighter.killed_by == Some(fighter_index) {
                    if is_ai {
                        character.stats.pve.kills += 1;
                    } else {
                        character.stats.pvp.kills += 1;
                    }
                } else if fighter.defeated_by == Some(fighter_index) {
                    if is_ai {
                        character.stats.pve.knockouts += 1;
                    } else {
                        character.stats.pvp.knockouts += 1;
                    }
                }
            }

            if let BattleState::Ended { winners, .. } = &self.battle.state {
                if winners.contains(&fighter.index) {
                    if is_pvp {
                        character.stats.pvp.wins += 1;
                    } else if is_pve {
                        character.stats.pve.wins += 1;
                    }
                } else if is_pvp {
                    character.stats.pvp.losses += 1;
                } else {
                    character.stats.pve.losses += 1;
                }
            }

            if fighter.flags.contains(FighterFlags::RISKING_LIFE) {
                if is_pvp {
                    character.stats.pvp.life_risks += 1;
                } else if is_pve {
                    character.stats.pve.life_risks += 1;
                }
            } else if fighter.is_defeated {
                if is_pvp {
                    character.stats.pvp.withdrawals += 1;
                } else if is_pve {
                    character.stats.pve.withdrawals += 1;
                }
            }

            self.ctx.db().characters().save(character).await?;
        }

        Ok(())
    }

    pub async fn next_turn(&mut self) -> anyhow::Result<()> {
        let fighter = self.battle.get_current_fighter();

        if fighter.vitality.value <= 0 {
            let fighter = self.battle.get_current_fighter_mut();
            fighter.is_defeated = true;
        }

        let before_message_len = self.current_turn_history.messages.len();

        // Effects
        let fighters = self.battle.alive_fighters.clone();
        for fighter in fighters {
            let fighter = self.battle.get_fighter_mut(fighter);
            let fighter_name = fighter.name.clone();
            let fighter_index = fighter.index;

            let effects = fighter.effects.clone();
            for effect in effects {
                let mut api = BattleApi::new(self);
                api.target_index = fighter_index;

                match effect.kind {
                    EffectKind::Flaming => {
                        api.fighter_mut().remove_effect(Effect::new(
                            effect.kind,
                            5,
                            Default::default(),
                        ));
                        api.fighter_mut().remove_effect(Effect::new(
                            EffectKind::Frozen,
                            10,
                            Default::default(),
                        ));

                        if api.fighter().has_effect(EffectKind::Ice) {
                            api.apply_effect(
                                api.fighter_index,
                                Effect::new(EffectKind::Wet, 5, api.fighter_index),
                            )
                            .await;
                        }
                        api.fighter_mut().remove_effect(Effect::new(
                            EffectKind::Ice,
                            5,
                            Default::default(),
                        ));

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

                        api.defer_message(format!(
                            "***{}** queimou e recebeu **{dmg}***",
                            fighter_name
                        ));
                    }
                    EffectKind::Burning => {
                        api.fighter_mut().remove_effect(Effect::new(
                            effect.kind,
                            5,
                            Default::default(),
                        ));
                        api.fighter_mut().remove_effect(Effect::new(
                            EffectKind::Frozen,
                            20,
                            Default::default(),
                        ));
                        api.fighter_mut().remove_effect(Effect::new(
                            EffectKind::Ice,
                            10,
                            Default::default(),
                        ));

                        let dmg = api
                            .apply_damage(
                                api.fighter_index,
                                DamageSpecifier {
                                    kind: DamageKind::Fire,
                                    amount: (api.fighter().health().max as f32 * 0.04) as i32,
                                    balance_effectiveness: 3,
                                    accuracy: 100,
                                    ..Default::default()
                                },
                            )
                            .await;

                        api.defer_message(format!(
                            "***{}** está em combustão e recebeu **{dmg}***",
                            fighter_name
                        ));
                    }
                    EffectKind::Shocked => {
                        api.fighter_mut().remove_effect(Effect::new(
                            effect.kind,
                            10,
                            Default::default(),
                        ));
                    }
                    EffectKind::Paralyzed => {}
                    EffectKind::LowProtection => {
                        let unprotected = api.fighter_mut().remove_effect(Effect::new(
                            effect.kind,
                            1,
                            Default::default(),
                        ));

                        if unprotected {
                            api.emit_message(format!(
                                "***{}** perdeu a proteção leve extra!*",
                                api.fighter().name
                            ));
                        }
                    }
                    EffectKind::Ice => {
                        api.fighter_mut().remove_effect(Effect::new(
                            effect.kind,
                            5,
                            Default::default(),
                        ));
                        api.fighter_mut().remove_effect(Effect::new(
                            EffectKind::Flaming,
                            10,
                            Default::default(),
                        ));
                        api.fighter_mut().remove_effect(Effect::new(
                            EffectKind::Burning,
                            5,
                            Default::default(),
                        ));
                    }
                    EffectKind::Wet => {
                        api.fighter_mut().remove_effect(Effect::new(
                            effect.kind,
                            15,
                            Default::default(),
                        ));
                        api.fighter_mut().remove_effect(Effect::new(
                            EffectKind::Flaming,
                            60,
                            Default::default(),
                        ));
                        api.fighter_mut().remove_effect(Effect::new(
                            EffectKind::Burning,
                            40,
                            Default::default(),
                        ));

                        api.fighter_mut().remove_effect(Effect::new(
                            EffectKind::Frozen,
                            3,
                            Default::default(),
                        ));
                        api.fighter_mut().remove_effect(Effect::new(
                            EffectKind::Ice,
                            1,
                            Default::default(),
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
                            Default::default(),
                        ));
                        api.fighter_mut().remove_effect(Effect::new(
                            EffectKind::Flaming,
                            20,
                            Default::default(),
                        ));
                        api.fighter_mut().remove_effect(Effect::new(
                            EffectKind::Burning,
                            10,
                            Default::default(),
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

                            let dmg = (api.fighter().health().max as f32).mul(0.1) as i32;
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

        // Passives: on_damage
        for (damage, fighter) in self.battle.turn_end_queues.damages.clone() {
            let fighter_index = fighter;
            let target_index = damage.culprit;

            let skills = self.battle.get_fighter(fighter).skills.clone();
            for skill in skills {
                let mut api = BattleApi::new(self);
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
        for (damage, fighter) in self.battle.turn_end_queues.damage_misses.clone() {
            let fighter_index = fighter;
            let target_index = damage.culprit;

            let skills = self.battle.get_fighter(fighter).skills.clone();
            for skill in skills {
                let mut api = BattleApi::new(self);
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
        for fighter in self.battle.alive_fighters.clone() {
            let fighter = self.battle.get_fighter_mut(fighter);
            let fighter_index = fighter.index;
            let target_index = fighter.target;

            for skill in fighter.skills.clone() {
                let mut api = BattleApi::new(self);
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
        for fighter_index in self.battle.alive_fighters.clone() {
            let fighter = self.battle.get_fighter(fighter_index).clone();
            let Some(killer) = fighter.killed_by else {
                continue;
            };

            let killer_skills = self.battle.get_fighter(killer).skills.clone();

            for skill in killer_skills {
                let mut api = BattleApi::new(self);
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

        if before_message_len != self.current_turn_history.messages.len() {
            tokio::time::sleep(Duration::from_secs(1)).await;
            self.update_turn_history_message().await?;
            tokio::time::sleep(Duration::from_secs(2)).await;
        }

        for fighter_index in self.battle.alive_fighters.clone() {
            let fighter = self.battle.get_fighter(fighter_index).clone();
            if fighter.is_defeated {
                continue;
            }

            if fighter.resistance.value > 0
                || fighter.flags.contains(FighterFlags::ASKED_TO_RISK_LIFE)
            {
                continue;
            }

            let confirmation = if !self.battle.settings.is_risking_life_allowed {
                false
            } else if let Some(user) = &fighter.user {
                self.ctx.helper()
                .create_confirmation(user.id, true,
                    Response::new_user_reply(user,
                        if self.battle.settings.casual {
                            "sua resistência chegou a zero! Quer apostar sua vida na batalha? (Essa é uma batalha CASUAL. Seu personagem não vai morrer de verdade)"
                        } else {
                        "sua resistência chegou a zero! Isso significa que seu personagem está perto do nocaute. Você deseja continuar lutando e apostar sua vida nessa batalha?\nSe não aceitar, você perderá por nocaute. Se aceitar, você continuará lutando mas perderá **vitalidade** ao invés de resistência. Se a vitalidade zerar seu personagem morre pra sempre."
                        }))
                        .await?
            } else {
                let mut api = BattleApi::new(self);
                api.fighter_index = fighter_index;
                api.target_index = fighter.target;
                ai::should_risk_life(api).await
            };

            let fighter = self.battle.get_fighter_mut(fighter_index);
            let fighter_name = fighter.name.clone();
            fighter.flags.insert(FighterFlags::ASKED_TO_RISK_LIFE);

            if confirmation {
                fighter.flags.insert(FighterFlags::RISKING_LIFE);
                fighter.defeated_by = None;
                self.defer_message(format!(
                    "**{}** agora está arriscando sua vida.",
                    fighter_name
                ));
            } else {
                fighter.is_defeated = true;
                self.defer_message(format!(
                    "**{}** não teve motivação o suficiente e perdeu a consciência.",
                    fighter_name
                ));
            }
        }

        // Next turn
        let before_message_len = self.current_turn_history.messages.len();
        self.battle.next_turn(&mut self.current_turn_history);

        // Update turn message
        if before_message_len != self.current_turn_history.messages.len() {
            tokio::time::sleep(Duration::from_secs(1)).await;
            self.update_turn_history_message().await?;
            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        Ok(())
    }

    pub async fn process_input(&mut self, input: BattleInput) -> anyhow::Result<()> {
        let fighter = self.battle.get_current_fighter().clone();
        let fighter_name = fighter.name.clone();

        match input {
            BattleInput::Nothing => {
                self.emit_turn_message(format!("**{}** não fez nada!", fighter.name));
            }
            BattleInput::Reinput => {
                self.should_reinput = true;
                return Ok(());
            }
            BattleInput::ChangeTarget(target) => {
                self.should_reinput = true;
                if target != fighter.index {
                    self.battle.get_current_fighter_mut().target = target;
                }

                return Ok(());
            }
            BattleInput::ChangeTeam(team) => {
                self.should_reinput = true;
                self.battle.get_current_fighter_mut().team = team;
                self.battle.reallocate_all_targets();

                return Ok(());
            }
            BattleInput::Attack | BattleInput::Upkick => {
                let is_kick = input == BattleInput::Upkick;
                let mut api = BattleApi::new(self);

                if !is_kick && api.fighter().composure == Composure::Standing {
                    if let Some(weapon) = api.fighter().weapon {
                        execute_weapon_attack(api, weapon).await?;
                        return Ok(());
                    }
                }

                // Default attack
                let mut damage = api.rng().gen_range(4..=7);
                if is_kick {
                    damage += api.rng().gen_range(3..=7);
                }

                let damage = (damage as f32 * api.fighter().strength_multiplier() * 1.1) as i32;

                let damage = api
                    .apply_damage(
                        api.target_index,
                        DamageSpecifier {
                            culprit: fighter.index,
                            amount: damage,
                            kind: DamageKind::Physical,
                            balance_effectiveness: 5 + if is_kick { 10 } else { 0 },
                            accuracy: if is_kick { 60 } else { 90 },
                            effect: None,
                        },
                    )
                    .await;

                let target_name = api.target().name.to_owned();
                api.emit_message(format!(
                    "**{}** {} **{}** e causou **{damage}**",
                    fighter_name,
                    if is_kick { "chutou" } else { "atacou" },
                    target_name
                ));
            }
            BattleInput::Defend => {
                if fighter.composure == Composure::OnGround {
                    self.emit_random_turn_message(&[
                        format!("**{}** tentou se defender mas está no chão!", fighter_name),
                        format!("**{}** não conseguiu se defender no chão!", fighter_name),
                    ]);
                } else {
                    let mut message = format!("**{}** está defendendo!", fighter_name);
                    if fighter.balance < 90 {
                        message.push_str(" Um pouco de sua postura foi restaurada.");
                    }

                    self.emit_turn_message(message);

                    let fighter = self.battle.get_current_fighter_mut();
                    fighter.defense += 2; // One will be removed at the end of the turn, so we add an extra defense turn
                    fighter.balance = fighter.balance.saturating_add(10).min(100);
                    fighter.ether.add((fighter.ether.max as f32 * 0.05) as i32);
                }
            }
            BattleInput::UseSkill(skill) => {
                let fighter = self.battle.get_current_fighter_mut();
                let mut dynamic_skill = skill.dynamic_skill.lock().await;
                fighter.ether.value = fighter
                    .ether
                    .value
                    .sub(dynamic_skill.data().use_cost.ether)
                    .max(0);

                dynamic_skill.on_use(BattleApi::new(self)).await?;
            }
            BattleInput::Finish(finisher) => {
                finisher.execute_finisher(BattleApi::new(self)).await?;
                tokio::time::sleep(Duration::from_millis(300)).await;
                self.update_turn_history_message().await?;
                tokio::time::sleep(Duration::from_secs(1)).await;
                return Ok(());
            }
            BattleInput::GetUp => {
                let fighter = self.battle.get_current_fighter_mut();
                let extreme_low_balance = fighter.balance < 40;
                let prob_to_fail = Probability::new(if extreme_low_balance { 30 } else { 15 });

                if prob_to_fail.generate_random_bool() {
                    fighter.balance = fighter.balance.add(10).min(100);

                    self.emit_random_turn_message(&[
                        format!("**{}** tentou levantar e caiu novamente!", fighter_name),
                        format!(
                            "**{}** tentou ficar em pé e perdeu o equilíbrio!",
                            fighter_name
                        ),
                        format!("**{}** falhou em levantar!", fighter_name),
                    ]);
                } else {
                    fighter.composure = Composure::Standing;
                    fighter.balance = fighter.balance.add(30).min(100);

                    self.emit_random_turn_message(&[
                        format!("**{}** recuperou sua postura e ficou em pé!", fighter_name),
                        format!("**{}** levantou do chão!", fighter_name),
                    ]);
                }
            }
        }

        Ok(())
    }

    pub async fn get_input(&mut self) -> anyhow::Result<BattleInput> {
        get_input(self).await
    }

    pub fn emit_turn_message(&mut self, message: impl Into<String>) {
        self.current_turn_history.messages.push(message.into());
    }

    pub fn defer_message(&mut self, message: impl Into<String>) {
        self.battle.deferred_turn_messages.push(message.into());
    }

    pub fn emit_random_turn_message(&mut self, messages: &[impl ToString]) {
        let messages = messages.iter().map(|m| m.to_string()).collect::<Vec<_>>();
        let message = messages
            .choose(&mut self.battle.rng)
            .cloned()
            .unwrap_or_default();
        self.current_turn_history.messages.push(message);
    }

    pub fn create_turn_embed(&self) -> EmbedBuilder {
        let current_fighter = self.battle.get_current_fighter();
        let target_fighter = self.battle.get_target_fighter();
        let turn_num = self.current_turn_history.round;

        EmbedBuilder::new_common()
            .set_color(if turn_num % 2 == 0 {
                Color::ORANGE
            } else {
                Color::DARK_ORANGE
            })
            .set_author(EmbedAuthor {
                name: format!("Rodada N° {turn_num} - {}", current_fighter.name),
                icon_url: current_fighter.user.as_ref().map(|u| u.avatar_url()),
            })
            .set_title(format!(
                "{} x {}",
                current_fighter.name, target_fighter.name
            ))
            .set_description(self.current_turn_history.messages.join("\n"))
    }

    pub fn create_battle_embed(&mut self) -> EmbedBuilder {
        let current_fighter = self.battle.get_current_fighter().clone();
        let mut embed = EmbedBuilder::new_common().set_author(EmbedAuthor {
            name: format!("Rodada de {}", current_fighter.name),
            icon_url: current_fighter.user.as_ref().map(|u| u.avatar_url()),
        });

        let mut alive_fighters = self
            .battle
            .alive_fighters
            .clone()
            .into_iter()
            .map(|f| self.battle.get_fighter(f).clone())
            .collect::<Vec<_>>();
        alive_fighters.sort_unstable_by_key(|f| {
            if f.index == current_fighter.index {
                0
            } else if f.index == current_fighter.target {
                1
            } else {
                2 + f.index.0
            }
        });

        for fighter in alive_fighters {
            let fighter = self.battle.get_fighter_mut(fighter.index);
            let index = fighter.index;

            let is_target = index == current_fighter.target;
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

            match fighter.composure {
                Composure::Standing => (),
                Composure::OnGround => displays.push("**No Chão**".to_string()),
            }

            fighter.effects.sort_by_key(|k| k.kind);

            for effect in fighter.effects.iter() {
                match effect.kind {
                    EffectKind::Flaming => {
                        displays.push(format!("♨️ **Queimando**: {}%", effect.amount))
                    }
                    EffectKind::Burning => {
                        displays.push(format!("🔥 **Combustão**: {}%", effect.amount))
                    }
                    EffectKind::Shocked => {
                        displays.push(format!("⚡ **Choque**: {}%", effect.amount))
                    }
                    EffectKind::Paralyzed => {
                        displays.push(format!("😵‍💫 **Paralisado**: {} turnos", effect.amount))
                    }
                    EffectKind::Wet => displays.push(format!("💧 **Molhado**: {}%", effect.amount)),
                    EffectKind::Ice => {
                        displays.push(format!("❄️ **Congelando**: {}%", effect.amount))
                    }
                    EffectKind::Frozen => {
                        displays.push(format!("🧊 **Congelado**: {}%", effect.amount))
                    }
                    EffectKind::Bleeding => {
                        displays.push(format!("🩸 **Sangramento**: {}%", effect.amount))
                    }

                    EffectKind::LowProtection => {
                        displays.push(format!("🛡️ **Proteção Leve**: {} turnos", effect.amount))
                    }
                }
            }

            embed = embed.add_field(EmbedField {
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
            });
        }

        embed
    }
}
