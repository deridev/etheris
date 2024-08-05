use std::{
    collections::{HashMap, VecDeque},
    time::Duration,
};

use bitflags::bitflags;
use etheris_common::Color;
use etheris_data::{
    emojis,
    items::{get_item, Item, ItemTag},
    ItemValue,
};
use etheris_database::character_model::CharacterModel;
use etheris_discord::{
    twilight_model::{channel::message::component::ButtonStyle, user::User},
    *,
};
use etheris_framework::{util::make_multiple_rows, watcher::WatcherOptions, *};
use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};

use crate::{
    data::enemies::Enemy, encounter, shop::Shop, Battle, BattleController, BattleSettings,
    FighterData,
};

use self::list::{prelude::get_enemies_by_regions, EventBuildState, ALL_EVENTS};

use super::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ControllerAction {
    PickAEvent,
}

bitflags! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct ControllerFlag: u8 {
        const EXPLORING = 1 << 0;
    }
}

#[derive(Debug, Clone)]
pub struct EventController {
    pub user: User,
    pub ctx: CommandContext,
    pub event_queue: VecDeque<Event>,

    pub flags: ControllerFlag,
    pub ticks: usize,

    last_interaction: Option<Interaction>,
}

impl EventController {
    pub fn new(user: User, ctx: CommandContext, event_queue: Vec<Event>) -> Self {
        Self {
            user,
            ctx,
            event_queue: event_queue.into(),
            ticks: 0,
            last_interaction: None,
            flags: ControllerFlag::empty(),
        }
    }

    pub async fn execute(&mut self) -> anyhow::Result<()> {
        while let Some(event) = self.event_queue.pop_front() {
            self.ticks += 1;
            self.execute_single_event(event).await?;
            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        Ok(())
    }

    pub async fn execute_action(&mut self, action: ControllerAction) -> anyhow::Result<()> {
        match action {
            ControllerAction::PickAEvent => {
                let Some(character) = self
                    .ctx
                    .db()
                    .characters()
                    .get_by_user(&self.user.id.to_string())
                    .await?
                else {
                    return Ok(());
                };

                let Some(event) = self.pick_event(character) else {
                    return Ok(());
                };

                self.event_queue.push_back(event);
            }
        }

        Ok(())
    }

    pub fn pick_event(&self, character: CharacterModel) -> Option<Event> {
        let mut rng = StdRng::from_entropy();
        let mut events = vec![];
        for event in ALL_EVENTS.iter() {
            let event = event(EventBuildState::new(character.clone()));

            if !event.spawn.base_probability.generate_random_bool() {
                continue;
            }

            if event
                .spawn
                .conditions
                .iter()
                .any(|c| !self.compute_condition(&character, c))
            {
                continue;
            }

            events.push(event.clone());
        }

        events.shuffle(&mut rng);

        let events: Vec<(Event, i32)> = events
            .into_iter()
            .filter_map(|e| {
                Some((
                    e.clone(),
                    e.spawn
                        .weighted_regions
                        .iter()
                        .find(|r| r.0 == character.region)?
                        .1,
                ))
            })
            .collect();

        println!(
            "\n======================================\n{}\n",
            events
                .iter()
                .map(|(e, w)| format!("- {w} | {}", e.identifier))
                .collect::<Vec<_>>()
                .join("\n")
        );

        if events.is_empty() {
            return None;
        }

        let events = events.choose_weighted(&mut rng, |(_, prob)| *prob).unwrap();

        Some(events.0.clone())
    }

    fn compute_condition(&self, character: &CharacterModel, condition: &Condition) -> bool {
        let pl_range = (character.pl as f64 * 0.9)..(character.pl as f64 * 1.1);

        match condition {
            Condition::None => true,
            Condition::Not(condition) => !self.compute_condition(character, condition),
            Condition::Or(a, b) => {
                self.compute_condition(character, a) || self.compute_condition(character, b)
            }
            Condition::HasOrbs(orbs) => character.orbs >= *orbs,
            Condition::HasItem(item, amount) => character.has_item(item, *amount),
            Condition::HasTag(tag) => character.has_tag(tag),
            Condition::HasPersonality(personality) => character.personalities.contains(personality),
            Condition::HasEther(ether) => character.stats.ether.value >= *ether,
            Condition::HasKarma(karma) => character.karma >= *karma,
            Condition::DefeatedBoss(boss_kind) => character.defeated_bosses.contains(boss_kind),
            Condition::SimilarPowerTo(enemy) => pl_range.contains(&(enemy.power_level() as f64)),
            Condition::StrongerThan(enemy) => character.pl > enemy.power_level(),
            Condition::WeakerThan(enemy) => character.pl < enemy.power_level(),
            Condition::IsFlagSet(flag) => self.flags.contains(*flag),
            Condition::Probability(probability) => probability.generate_random_bool(),
        }
    }

    async fn execute_single_event(&mut self, event: Event) -> anyhow::Result<()> {
        let Some(character) = self
            .ctx
            .db()
            .characters()
            .get_by_user(&self.user.id.to_string())
            .await?
        else {
            return Ok(());
        };

        let message = match event.message {
            EventMessage::Single(message) => message.to_string(),
            EventMessage::SingleString(message) => message,
            EventMessage::Multiple(messages) => messages
                .choose(&mut StdRng::from_entropy())
                .copied()
                .unwrap_or("TEXTO INVÁLIDO")
                .to_string(),
            EventMessage::MultipleString(messages) => messages
                .choose(&mut StdRng::from_entropy())
                .cloned()
                .unwrap_or(String::from("TEXTO INVÁLIDO")),
            EventMessage::Conditional(messages) => messages
                .iter()
                .filter(|m| self.compute_condition(&character, &m.0))
                .map(|m| m.1.clone())
                .collect::<Vec<_>>()
                .first()
                .cloned()
                .unwrap_or("TEXTO INVÁLIDO OU NENHUMA CONDIÇÃO TEXTUAL CUMPRIDA.".to_string()),
        };

        let message = message.replace("[REGION]", &format!("{}", character.region));
        let message = message.replace("[NAME]", &character.name);

        let response = Response::new_user_reply(&self.user, message).add_emoji_prefix(event.emoji);

        if event.actions.is_empty() {
            self.ctx.send(response).await?;
            return Ok(());
        }

        let Some(character) = self
            .ctx
            .db()
            .characters()
            .get_by_user(&self.user.id.to_string())
            .await?
        else {
            return Ok(());
        };

        let mut buttons = vec![];
        for action in event.actions.iter() {
            if !action.probability.generate_random_bool() {
                continue;
            }

            let mut button = ButtonBuilder::new()
                .set_custom_id(&action.name)
                .set_label(&action.name);

            if action
                .conditions
                .iter()
                .any(|c| !self.compute_condition(&character, c))
            {
                button = button.set_disabled(true);
            }

            if let Some(emoji) = action.emoji {
                button = button.set_emoji(emoji);
            }

            buttons.push(button);
        }

        let message = self
            .ctx
            .send(response.set_components(make_multiple_rows(buttons.clone())))
            .await?;

        let user_id = self.user.id;
        let Ok(Some(interaction)) = self
            .ctx
            .watcher
            .await_single_component(
                message.id,
                move |interaction| interaction.author_id() == Some(user_id),
                WatcherOptions {
                    timeout: Duration::from_secs(120),
                },
            )
            .await
        else {
            return Ok(());
        };

        let data = interaction.parse_message_component_data()?;
        self.last_interaction = Some(interaction.clone());

        let mut ctx = CommandContext::from_with_interaction(&self.ctx, Box::new(interaction));
        let buttons = buttons
            .into_iter()
            .map(|b| {
                let custom_id = b.data.custom_id.clone().unwrap_or_default();
                b.set_disabled(true)
                    .set_style(if custom_id == data.custom_id {
                        ButtonStyle::Success
                    } else {
                        ButtonStyle::Secondary
                    })
            })
            .collect::<Vec<_>>();

        ctx.update_message(Response::default().set_components(make_multiple_rows(buttons)))
            .await?;

        let action = event
            .actions
            .iter()
            .find(|a| a.name == data.custom_id)
            .expect("Action not found");

        self.execute_one_of_multiple_consequences(&character, action.consequences.clone())
            .await?;

        for extra in action.extra_consequences.clone() {
            if extra.probability.generate_random_bool() {
                self.execute_single_consequence(extra).await?;
            }
        }

        Ok(())
    }

    async fn execute_one_of_multiple_consequences(
        &mut self,
        character: &CharacterModel,
        consequences: Vec<Consequence>,
    ) -> anyhow::Result<()> {
        let valid_consequences = consequences
            .iter()
            .filter(|c| {
                c.conditions
                    .iter()
                    .all(|c| self.compute_condition(character, c))
            })
            .cloned()
            .collect::<Vec<_>>();

        if valid_consequences.is_empty() {
            return Ok(());
        }

        let consequence = valid_consequences
            .choose_weighted(&mut StdRng::from_entropy(), |c| c.probability.value())
            .ok();

        let consequence = match consequence {
            Some(consequence) => consequence,
            None => valid_consequences
                .iter()
                .fold(&valid_consequences[0], |acc, c| {
                    if c.probability.value() > acc.probability.value() {
                        c
                    } else {
                        acc
                    }
                }),
        };

        self.execute_single_consequence(consequence.clone()).await?;

        for extra in consequence.extra_consequences.clone() {
            if extra.probability.generate_random_bool() {
                self.execute_single_consequence(extra).await?;
            }
        }

        Ok(())
    }

    #[async_recursion::async_recursion]
    async fn execute_single_consequence(
        &mut self,
        mut consequence: Consequence,
    ) -> anyhow::Result<()> {
        let Some(mut character) = self
            .ctx
            .db()
            .characters()
            .get_by_user(&self.user.id.to_string())
            .await?
        else {
            return Ok(());
        };

        if consequence.kind == ConsequenceKind::FindARegionEnemy {
            consequence.kind =
                ConsequenceKind::MultiplePossibleEncounters(get_enemies_by_regions(&[
                    character.region
                ]));
        }

        match consequence.kind {
            ConsequenceKind::FindARegionEnemy => unreachable!(),
            ConsequenceKind::Message { message, emoji } => {
                let mut response = Response::new_user_reply(&self.user, message);
                if let Some(emoji) = emoji {
                    response = response.add_emoji_prefix(emoji);
                }

                self.ctx.send(response).await?;
            }
            ConsequenceKind::ConditionalConsequence {
                condition,
                consequence,
                else_consequence,
            } => {
                if self.compute_condition(&character, &condition) {
                    let consequence = Consequence {
                        kind: *(consequence.clone()),
                        ..Default::default()
                    };

                    self.execute_single_consequence(consequence).await?;
                } else if let Some(else_consequence) = else_consequence {
                    let consequence = Consequence {
                        kind: *(else_consequence.clone()),
                        ..Default::default()
                    };

                    self.execute_single_consequence(consequence).await?;
                }
            }
            ConsequenceKind::Event(event) => {
                self.event_queue
                    .push_front(event(EventBuildState::new(character)));
            }
            ConsequenceKind::Action(action) => {
                self.execute_action(action).await?;
            }
            ConsequenceKind::Battle(battle) => {
                let main_enemy = battle.enemies.first().unwrap().clone();

                let player_pl = character.pl;

                let mut rng = StdRng::from_entropy();
                let enemies = battle.enemies;
                let enemies_fighter_data = enemies
                    .iter()
                    .map(|e| {
                        FighterData::new_from_enemy(
                            1,
                            e.drop.to_reward(
                                &mut rng,
                                player_pl,
                                e.power_level(),
                                e.boss.is_some(),
                            ),
                            e.clone(),
                        )
                    })
                    .collect::<Vec<_>>();

                let allies_fighter_data = battle
                    .allies
                    .iter()
                    .map(|e| {
                        FighterData::new_from_enemy(
                            0,
                            e.drop.to_reward(
                                &mut rng,
                                player_pl,
                                e.power_level(),
                                e.boss.is_some(),
                            ),
                            e.clone(),
                        )
                    })
                    .collect::<Vec<_>>();

                let result = if battle.prompt {
                    encounter::prompt_encounter(
                        &mut self.ctx,
                        self.user.clone(),
                        enemies_fighter_data,
                        allies_fighter_data,
                    )
                    .await?
                } else {
                    let mut fighters = vec![FighterData::new_from_character(
                        0,
                        &character,
                        self.user.clone(),
                        Default::default(),
                    )];
                    fighters.extend_from_slice(&enemies_fighter_data);
                    fighters.extend_from_slice(&allies_fighter_data);

                    let battle = Battle::new(
                        character.region,
                        BattleSettings {
                            casual: false,
                            has_consequences: true,
                            is_risking_life_allowed: true,
                            max_intruders: 2,
                        },
                        fighters,
                    )?;

                    let mut controller = BattleController::new(battle, self.ctx.clone());
                    Some(controller.run().await?)
                };

                if result.is_none() {
                    return Ok(());
                }

                let result = result.unwrap_or_default();

                let all_fighters = result
                    .winners
                    .iter()
                    .chain(result.losers.iter())
                    .cloned()
                    .collect::<Vec<_>>();
                let user_fighter_index = all_fighters
                    .iter()
                    .find(|f| f.user.as_ref().map(|u| u.id) == Some(self.user.id))
                    .unwrap()
                    .index;

                let won = result.winners.iter().any(|w| w.index == user_fighter_index);
                let is_alive = all_fighters
                    .iter()
                    .find(|f| f.index == user_fighter_index)
                    .unwrap()
                    .killed_by
                    .is_none();

                let killed = result
                    .losers
                    .iter()
                    .any(|w| w.name == main_enemy.name && w.killed_by == Some(user_fighter_index));

                if !won {
                    if !is_alive {
                        if let Some(on_lose_kill_event) = battle.on_lose_die_event {
                            self.event_queue
                                .push_front(on_lose_kill_event(EventBuildState::new(character)));
                        }
                    } else if let Some(on_lose_knockout_event) = battle.on_lose_knockout_event {
                        self.event_queue
                            .push_front(on_lose_knockout_event(EventBuildState::new(character)));
                    }
                } else if killed {
                    if let Some(on_win_kill_event) = battle.on_win_kill_event {
                        self.event_queue
                            .push_front(on_win_kill_event(EventBuildState::new(character)));
                    }
                } else if let Some(on_win_knockout_event) = battle.on_win_knockout_event {
                    self.event_queue
                        .push_front(on_win_knockout_event(EventBuildState::new(character)));
                }
            }
            ConsequenceKind::Encounter(enemy) => {
                self.execute_single_encounter(enemy, false).await?
            }
            ConsequenceKind::InstantBattle(enemy) => {
                self.execute_single_encounter(enemy, true).await?
            }
            ConsequenceKind::MultiplePossibleEncounters(encounters) => {
                let enemies = encounters
                    .iter()
                    .filter_map(|e| {
                        if !e.base_probability.generate_random_bool() {
                            return None;
                        }

                        let regions = e.regions.iter().find(|r| r.0 == character.region)?;

                        Some((e.clone(), *regions))
                    })
                    .collect::<Vec<_>>();

                let Ok((enemy, ..)) =
                    enemies.choose_weighted(&mut StdRng::from_entropy(), |(_, (_, prob))| *prob)
                else {
                    self.ctx
                        .send(
                            Response::new_user_reply(
                                &self.user,
                                "você não encontrou nenhum inimigo! Aproveite a paz.",
                            )
                            .add_emoji_prefix(emojis::ERROR)
                            .set_ephemeral(),
                        )
                        .await?;
                    return Ok(());
                };

                self.execute_single_encounter(enemy.clone(), false).await?;
            }
            ConsequenceKind::Rewards {
                message,
                iterations,
                items,
                orbs,
                xp,
            } => {
                let rng = &mut StdRng::from_entropy();

                let mut rewarded_items = HashMap::new();
                for _ in 0..iterations {
                    let items = items
                        .iter()
                        .filter(|(p, ..)| p.generate_random_bool())
                        .collect::<Vec<_>>();
                    let Some((_, item, item_amount)) = items.choose(rng).cloned() else {
                        continue;
                    };

                    let item_amount = rng.gen_range(item_amount.0..=item_amount.1);
                    if item_amount == 0 {
                        continue;
                    }

                    let amount = rewarded_items.entry(item).or_insert(0);
                    *amount += item_amount;
                }

                let orbs = rng.gen_range(orbs.0..=orbs.1);

                let is_xp_zero = xp.is_empty();
                let strength_xp = rng.gen_range(xp.strength.0..=xp.strength.1);
                let health_xp = rng.gen_range(xp.health.0..=xp.health.1);
                let intelligence_xp = rng.gen_range(xp.intelligence.0..=xp.intelligence.1);
                let knowledge_xp = rng.gen_range(xp.knowledge.0..=xp.knowledge.1);

                let items = rewarded_items.into_iter().collect::<Vec<_>>();

                let mut embed = EmbedBuilder::new_common()
                    .set_color(Color::LIGHT_CYAN)
                    .set_author(EmbedAuthor {
                        name: "💰 Recompensas".into(),
                        icon_url: Some(self.user.avatar_url()),
                    });

                if orbs > 0 {
                    embed = embed.add_field_with_emoji(
                        emojis::ORB,
                        EmbedField {
                            name: "Orbs".into(),
                            value: format!("{} ◎", orbs),
                            inline: true,
                        },
                    );
                }

                if !is_xp_zero {
                    let mut xp_texts = vec![];
                    if strength_xp > 0 {
                        xp_texts.push(format!("**Força**: {} XP", strength_xp));
                    }
                    if health_xp > 0 {
                        xp_texts.push(format!("**Vida**: {} XP", health_xp));
                    }
                    if intelligence_xp > 0 {
                        xp_texts.push(format!("**Inteligência**: {} XP", intelligence_xp));
                    }
                    if knowledge_xp > 0 {
                        xp_texts.push(format!("**Conhecimento**: {} XP", knowledge_xp));
                    }

                    embed = embed.add_field_with_emoji(
                        emojis::XP,
                        EmbedField {
                            name: "XP".into(),
                            value: xp_texts.join("\n"),
                            inline: true,
                        },
                    );
                }

                if !items.is_empty() {
                    embed = embed.add_field_with_emoji(
                        items[0].0.emoji,
                        EmbedField {
                            name: "Itens".into(),
                            value: items
                                .iter()
                                .map(|(item, amount)| {
                                    format!("{} {amount}x {}", item.emoji, item.display_name)
                                })
                                .collect::<Vec<_>>()
                                .join("\n"),
                            inline: false,
                        },
                    );
                }

                character.add_orbs(orbs);
                character.strength_xp += strength_xp as u32;
                character.health_xp += health_xp as u32;
                character.intelligence_xp += intelligence_xp as u32;
                character.knowledge_xp += knowledge_xp as u32;

                for (item, amount) in items {
                    character.add_item(*item, amount as usize, None);
                }

                self.ctx.db().characters().save(character).await?;

                self.ctx
                    .send(Response::new_user_reply(&self.user, message).add_embed(embed))
                    .await?;
            }
            ConsequenceKind::Prejudice {
                message,
                items_amount,
                max_item_valuability,
                fixed_orbs,
                orbs_percentage,
                specific_items,
                damage_percentage,
                damage_limit,
            } => {
                let rng = &mut StdRng::from_entropy();
                let steal_amount = rng.gen_range(items_amount.0..=items_amount.1);

                let mut stealed_items: HashMap<Item, usize> = HashMap::new();
                let mut total_valiability = 0;
                let mut stealed_amount = 0;

                for (item, amount) in specific_items {
                    stealed_items.insert(item, amount);
                    total_valiability += item.purchase_properties.base_price * amount as i64;
                    stealed_amount += amount;
                }

                let health = character.stats.resistance.max + character.stats.vitality.max;
                let damage = (health as f64 * damage_percentage) as i32;
                let damage = damage.min(damage_limit);

                const MAX_ITEM_ITERATIONS: usize = 30;
                for _ in 0..MAX_ITEM_ITERATIONS {
                    let mut last_valuable_steal = false;
                    if stealed_amount >= steal_amount {
                        break;
                    }

                    if character.inventory.is_empty()
                        || stealed_items.len() >= character.inventory.len()
                    {
                        break;
                    }

                    let Some(inventory_item) = character.inventory.choose(rng) else {
                        break;
                    };

                    let item = get_item(&inventory_item.identifier).unwrap();
                    if item.tags.contains(&ItemTag::Specific)
                        && item.tags.contains(&ItemTag::Special)
                    {
                        continue;
                    }

                    let amount = rng.gen_range(items_amount.0..=items_amount.1);
                    let amount = amount.clamp(1, inventory_item.quantity);

                    let valuability = item.purchase_properties.base_price * amount as i64;
                    if valuability + total_valiability > max_item_valuability as i64 {
                        if rng.gen_bool(0.2) {
                            last_valuable_steal = true;
                        } else {
                            continue;
                        }
                    }

                    if amount > inventory_item.quantity {
                        continue;
                    }

                    total_valiability += valuability;

                    let entry_amount = stealed_items.entry(item).or_insert(0);
                    stealed_amount += amount;
                    *entry_amount += amount;

                    if !item.stackable {
                        *entry_amount = 1;
                    }

                    if last_valuable_steal {
                        break;
                    }
                }

                let stealed_items = stealed_items.into_iter().collect::<Vec<_>>();
                let mut stealed_orbs = rng.gen_range(fixed_orbs.0..=fixed_orbs.1);
                stealed_orbs += (character.orbs as f64 * orbs_percentage) as i64;

                character.remove_orbs(stealed_orbs.clamp(0, character.orbs));
                character.take_damage(damage);
                for (item, amount) in &stealed_items {
                    character.remove_item(*item, *amount);
                }

                self.ctx.db().characters().save(character).await?;

                let mut embed = EmbedBuilder::new_common()
                    .set_color(Color::LIGHT_RED)
                    .set_author(EmbedAuthor {
                        name: "☹️ Prejuízo".into(),
                        icon_url: Some(self.user.avatar_url()),
                    });

                if stealed_orbs > 0 {
                    embed = embed.add_field_with_emoji(
                        emojis::ORB,
                        EmbedField {
                            name: "Orbs Perdidos".into(),
                            value: format!("{} ◎", stealed_orbs),
                            inline: true,
                        },
                    );
                }

                if damage > 0 {
                    embed = embed.add_field_with_emoji(
                        emojis::HEALTH,
                        EmbedField {
                            name: "Dano".into(),
                            value: format!("**{} dano**", damage),
                            inline: true,
                        },
                    );
                }

                if !stealed_items.is_empty() {
                    embed = embed.add_field_with_emoji(
                        stealed_items[0].0.emoji,
                        EmbedField {
                            name: "Itens Perdidos".into(),
                            value: stealed_items
                                .iter()
                                .map(|(item, amount)| {
                                    format!("{} {amount}x {}", item.emoji, item.display_name)
                                })
                                .collect::<Vec<_>>()
                                .join("\n"),
                            inline: false,
                        },
                    );
                }

                self.ctx
                    .send(Response::new_user_reply(&self.user, message).add_embed(embed))
                    .await?;
            }
            ConsequenceKind::Shop { name, items } => {
                let shop = Shop::new(name, None, items);
                shop.prompt(self.user.clone(), &mut self.ctx).await?;
            }
            ConsequenceKind::RemoveItemDurability(item, amount) => {
                let Some(inventory_item) = character.get_inventory_item_mut(&item) else {
                    return Ok(());
                };

                let mut broke = false;
                for value in inventory_item.values.values.iter_mut() {
                    if let ItemValue::Durability(durability) = value {
                        *durability = durability.saturating_sub(amount);
                        if *durability == 0 {
                            broke = true;
                            break;
                        }
                    }
                }

                if broke {
                    character.remove_item(item, amount as usize);
                    self.ctx
                        .send_in_channel(
                            Response::new_user_reply(
                                &self.user,
                                format!("seu item **{}** quebrou!", item.display_name),
                            )
                            .add_emoji_prefix(item.emoji),
                        )
                        .await?;
                }

                self.ctx.db().characters().save(character).await?;
            }
            ConsequenceKind::RemoveItem(item, amount) => {
                character.remove_item(item, amount);
                self.ctx.db().characters().save(character).await?;
            }
            ConsequenceKind::AddActionPoint(amount) => {
                character.action_points =
                    (character.action_points + amount).min(character.max_action_points);
                self.ctx.db().characters().save(character).await?;
            }
            ConsequenceKind::AddEther(ether) => {
                character.stats.ether.value =
                    (character.stats.ether.value + ether).min(character.stats.ether.max);
                self.ctx.db().characters().save(character).await?;
            }
            ConsequenceKind::RemoveEther(ether) => {
                character.stats.ether.value = character.stats.ether.value.saturating_sub(ether);
                self.ctx.db().characters().save(character).await?;
            }
            ConsequenceKind::AddTag(tag) => {
                character.insert_tag(tag);
                self.ctx.db().characters().save(character).await?;
            }
            ConsequenceKind::RemoveTag(tag) => {
                character.remove_tag(&tag);
                self.ctx.db().characters().save(character).await?;
            }
            ConsequenceKind::AddKarma(amount) => {
                character.add_karma(amount);
                self.ctx.db().characters().save(character).await?;
            }
            ConsequenceKind::RemoveKarma(amount) => {
                character.remove_karma(amount);
                self.ctx.db().characters().save(character).await?;
            }
            ConsequenceKind::RemoveOrbs(amount) => {
                character.remove_orbs(amount);
                self.ctx.db().characters().save(character).await?;
            }
        }

        Ok(())
    }

    async fn execute_single_encounter(
        &mut self,
        enemy: Enemy,
        instant: bool,
    ) -> anyhow::Result<()> {
        let Some(character) = self
            .ctx
            .db()
            .characters()
            .get_by_user(&self.user.id.to_string())
            .await?
        else {
            return Ok(());
        };

        let mut enemies = vec![enemy.clone()];

        let allies = enemy.allies.unwrap_or_default();
        for ally in allies
            .into_iter()
            .filter(|e| e.0.generate_random_bool())
            .map(|e| e.1)
        {
            enemies.push((*ally).clone());
        }

        let enemies = enemies
            .into_iter()
            .map(|e| {
                let reward = e.drop.to_reward(
                    &mut StdRng::from_entropy(),
                    character.pl,
                    e.power_level(),
                    e.boss.is_some(),
                );
                FighterData::new_from_enemy(1, reward, e)
            })
            .collect::<Vec<_>>();

        if !instant {
            encounter::prompt_encounter(&mut self.ctx, self.user.clone(), enemies, vec![]).await?;
        } else {
            let Some(character) = self
                .ctx
                .db()
                .characters()
                .get_by_user(&self.user.id.to_string())
                .await?
            else {
                return Ok(());
            };

            let mut fighters = enemies;
            fighters.insert(
                0,
                FighterData::new_from_character(
                    0,
                    &character,
                    self.user.clone(),
                    Default::default(),
                ),
            );

            let battle = Battle::new(
                character.region,
                BattleSettings {
                    casual: false,
                    has_consequences: true,
                    is_risking_life_allowed: true,
                    max_intruders: 2,
                },
                fighters,
            )?;

            let mut controller = BattleController::new(battle, self.ctx.clone());
            controller.run().await?;

            self.ctx = controller.ctx;
        }

        Ok(())
    }
}
