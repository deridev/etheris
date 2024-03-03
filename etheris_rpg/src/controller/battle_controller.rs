use std::{
    ops::{Add, Sub},
    time::Duration,
};

use anyhow::bail;
use etheris_common::{Color, Probability};
use etheris_data::{emojis, items::get_item_by_weapon, ItemValues};
use etheris_database::{
    character_model::{DeathCause, DeathInfo},
    common::{DatabaseDateTime, InventoryItem},
};
use etheris_discord::{
    twilight_model::{
        channel::{message::component::ButtonStyle, *},
        id::{marker::UserMarker, Id},
    },
    *,
};
use etheris_framework::{watcher::WatcherOptions, CommandContext, EmbedPagination, Response};
use etheris_util::math;
use rand::{seq::SliceRandom, Rng};

use crate::{common::*, get_input, *};

use self::data::{weapons::execute_weapon_attack, Reward};

#[derive(Debug, Clone)]
pub struct BattleResult {
    pub reward: Reward,
    pub winners: Vec<Fighter>,
}

pub struct BattleController {
    pub battle: Battle,
    pub ctx: CommandContext,
    pub last_message: Option<Message>,
    pub last_interaction: Option<Interaction>,
    pub current_turn_history: TurnHistory,

    should_reinput: bool,
}

impl BattleController {
    pub fn new(battle: Battle, ctx: CommandContext) -> Self {
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
                    eprintln!("battle_controller::run Error: {}", e);
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
                    character.knowledge_xp += (xp + 2) / 5;
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

        {
            let allowed = allowed_users.clone();
            let ctx = self.ctx.clone();
            let battle = self.battle.clone();
            tokio::spawn(async move {
                let controller_ctx = ctx.clone();
                let mut controller = Self::new(battle, controller_ctx);

                let Ok(Some(component)) = controller
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
                    return Ok::<(), anyhow::Error>(());
                };

                let mut temp_ctx =
                    CommandContext::from_with_interaction(&controller.ctx, Box::new(component));
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

                controller.send_full_history(allowed_users).await?;
                Ok(())
            });
        }

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

            // Kill the character
            if character.alive && fighter.killed_by.is_some() {
                let killer = fighter.killed_by.unwrap();
                let killer_name = self.battle.get_fighter(killer).name.clone();
                character.alive = false;
                character.death_info = Some(DeathInfo {
                    cause: DeathCause::KilledBy(killer_name),
                    date: DatabaseDateTime::now(),
                });
            }

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
                        && Probability::new(30).generate_random_bool()
                    {
                        character.aknowledge_skill(kind);
                    }
                }
            }

            let mut skills = Vec::with_capacity(fighter.skills.capacity());
            for skill in fighter.skills.clone() {
                skills.push(skill.dynamic_skill.lock().await.save_kind());
            }

            character.weapon = fighter.weapon.map(|w| w.kind);
            character.stats.resistance = fighter.resistance.into();
            character.stats.vitality = fighter.vitality.into();
            character.stats.ether = fighter.ether.into();
            character.battle_inventory = fighter
                .inventory
                .iter()
                .map(|i| InventoryItem {
                    identifier: i.item.identifier.to_string(),
                    quantity: i.quantity,
                    values: i.values.clone(),
                })
                .collect();
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
        let alive_fighters = self.battle.alive_fighters.clone();

        controller_helper::tick_every_effect(&alive_fighters, self).await?;
        controller_helper::passives(self).await?;

        // Ask every fighter to check if it should risk life
        controller_helper::should_risk_life(&alive_fighters, self).await?;

        // Move to next turn
        let before_message_len = self.current_turn_history.messages.len();
        self.battle.next_turn(&mut self.current_turn_history);

        // Update the turn message
        if before_message_len != self.current_turn_history.messages.len() {
            tokio::time::sleep(Duration::from_secs(1)).await;
            self.update_turn_history_message().await?;
            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        let before_message_len = self.current_turn_history.messages.len();
        if self.battle.fighters_queue.is_empty() {
            self.battle.cycle_counter += 1;
            controller_helper::tick_cycle(&self.battle.alive_fighters.clone(), self).await?;
        }

        // Update the turn message - cycle tick
        if before_message_len != self.current_turn_history.messages.len() {
            self.update_turn_history_message().await?;
            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        self.battle.next_fighter();

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

                if !is_kick && api.fighter().composure != Composure::OnGround {
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
                if finisher.fail_probability().generate_random_bool() {
                    self.emit_turn_message(format!(
                        "**{}** tentou executar uma finalização mas não conseguiu!",
                        fighter.name
                    ));
                    return Ok(());
                }

                finisher.execute_finisher(BattleApi::new(self)).await?;
                tokio::time::sleep(Duration::from_millis(300)).await;
                self.update_turn_history_message().await?;
                tokio::time::sleep(Duration::from_secs(1)).await;
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
            BattleInput::UseItem(item) => {
                if !fighter
                    .inventory
                    .iter()
                    .any(|i| i.item.identifier == item.identifier)
                {
                    self.emit_turn_message(format!(
                        "**{}** não possui o item {}!",
                        fighter.name, item.display_name
                    ));
                    return Ok(());
                }

                if let Some(weapon) = item.weapon {
                    {
                        let fighter = self.battle.get_fighter_mut(fighter.index);
                        if let Some(weapon) = fighter.weapon {
                            let weapon_item = get_item_by_weapon(weapon.kind);
                            fighter.inventory.push(BattleItem {
                                item: weapon_item,
                                quantity: 1,
                                values: ItemValues::empty(),
                            });
                        }

                        fighter.weapon = Some(FighterWeapon { kind: weapon });
                    }

                    let weapon_item = get_item_by_weapon(weapon);
                    self.battle
                        .get_fighter_mut(fighter.index)
                        .remove_item(weapon_item, 1);

                    self.emit_turn_message(format!(
                        "**{}** equipou a arma **{}**",
                        fighter.name, weapon_item.display_name
                    ));
                    return Ok(());
                }

                let Some(consumption_properties) = item.consumption_properties else {
                    self.emit_turn_message(format!(
                        "**{}** tentou usar um item que não pode ser usado em batalha!",
                        fighter.name
                    ));
                    return Ok(());
                };

                let health_regeneration = math::calculate_health_regeneration(
                    consumption_properties,
                    1,
                    fighter.health().max,
                );
                let ether_regeneration = math::calculate_ether_regeneration(
                    consumption_properties,
                    1,
                    fighter.ether.max,
                );

                let mut messages = vec![];
                if health_regeneration > 0 {
                    messages.push(format!("**{health_regeneration} vida**"));
                }

                if ether_regeneration > 0 {
                    messages.push(format!("**{ether_regeneration} ether**"));
                }

                {
                    let fighter = self.battle.get_fighter_mut(fighter.index);
                    fighter.heal(fighter.index, health_regeneration);
                    fighter.ether.add(ether_regeneration);
                    fighter.remove_item(item, 1);
                }

                self.emit_turn_message(format!(
                    "**{}** usou o item **{}** e regenerou {}!",
                    fighter.name,
                    item.display_name,
                    messages.join(" e ")
                ));

                self.update_turn_history_message().await?;
                tokio::time::sleep(Duration::from_secs(1)).await;

                self.should_reinput = true;
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
                Composure::OnAir(n) => displays.push(format!("**{n}m No Ar**")),
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
                    EffectKind::Curse => {
                        displays.push(format!("⚫ **Maldição**: {}%", effect.amount))
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
