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
use rand::{
    rngs::StdRng,
    seq::{IteratorRandom, SliceRandom},
    Rng, SeedableRng,
};

use crate::{
    data::enemies::Enemy, encounter, shop::Shop, Battle, BattleController, BattleSettings,
    FighterData,
};

use self::list::{EventBuildState, ALL_EVENTS};

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

        if events.is_empty() {
            return None;
        }

        let mut rng = StdRng::from_entropy();
        let events = events.choose_weighted(&mut rng, |(_, prob)| *prob).unwrap();

        Some(events.0.clone())
    }

    fn compute_condition(&self, character: &CharacterModel, condition: &Condition) -> bool {
        let pl_range = (character.pl as f64 * 0.9)..(character.pl as f64 * 1.1);

        match condition {
            Condition::Not(condition) => !self.compute_condition(character, condition),
            Condition::HasItem(item, amount) => character.has_item(item, *amount),
            Condition::SimilarPowerTo(enemy) => pl_range.contains(&(enemy.power_level() as f64)),
            Condition::StrongerThan(enemy) => character.pl > enemy.power_level(),
            Condition::WeakerThan(enemy) => character.pl < enemy.power_level(),
            Condition::IsFlagSet(flag) => self.flags.contains(*flag),
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
            EventMessage::Multiple(messages) => messages
                .choose(&mut StdRng::from_entropy())
                .copied()
                .unwrap_or("TEXTO INVÁLIDO")
                .to_string(),
            EventMessage::Conditional(messages) => messages
                .iter()
                .filter(|m| self.compute_condition(&character, &m.0))
                .map(|m| m.1.clone())
                .collect::<Vec<_>>()
                .first()
                .cloned()
                .unwrap_or("TEXTO INVÁLIDO OU NENHUMA CONDIÇÃO TEXTUAL CUMPRIDA.".to_string()),
        };
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
                .set_custom_id(action.name)
                .set_label(action.name);

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
        self.execute_one_of_multiple_consequences(&character, action.extra_consequences.clone())
            .await?;

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
            .iter()
            .filter(|c| c.probability.generate_random_bool())
            .choose(&mut StdRng::from_entropy());

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

        self.execute_single_consequence(consequence.clone()).await
    }

    async fn execute_single_consequence(&mut self, consequence: Consequence) -> anyhow::Result<()> {
        match consequence.kind {
            ConsequenceKind::Event(event) => {
                let Some(character) = self
                    .ctx
                    .db()
                    .characters()
                    .get_by_user(&self.user.id.to_string())
                    .await?
                else {
                    return Ok(());
                };

                self.event_queue
                    .push_front(event(EventBuildState::new(character)));
            }
            ConsequenceKind::Action(action) => {
                self.execute_action(action).await?;
            }
            ConsequenceKind::Encounter(enemy) => {
                self.execute_single_encounter(enemy, false).await?
            }
            ConsequenceKind::InstantBattle(enemy) => {
                self.execute_single_encounter(enemy, true).await?
            }
            ConsequenceKind::MultiplePossibleEncounters(encounters) => {
                let Some(character) = self
                    .ctx
                    .db()
                    .characters()
                    .get_by_user(&self.user.id.to_string())
                    .await?
                else {
                    return Ok(());
                };

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

                let Some(mut character) = self
                    .ctx
                    .db()
                    .characters()
                    .get_by_user(&self.user.id.to_string())
                    .await?
                else {
                    return Ok(());
                };

                character.add_orbs(orbs);
                character.strength_xp += strength_xp as u32;
                character.health_xp += health_xp as u32;
                character.intelligence_xp += intelligence_xp as u32;
                character.knowledge_xp += knowledge_xp as u32;

                for (item, amount) in items {
                    character.add_item(*item, amount as usize, None);
                }

                self.ctx.db().characters().save(character).await?;

                self.ctx.send(embed).await?;
            }
            ConsequenceKind::Prejudice {
                items_amount,
                max_item_valuability,
                fixed_orbs,
                orbs_percentage,
                specific_items,
            } => {
                let rng = &mut StdRng::from_entropy();
                let steal_amount = rng.gen_range(items_amount.0..=items_amount.1);

                let Some(mut character) = self
                    .ctx
                    .db()
                    .characters()
                    .get_by_user(&self.user.id.to_string())
                    .await?
                else {
                    return Ok(());
                };

                let mut stealed_items: HashMap<Item, usize> = HashMap::new();
                let mut total_valiability = 0;
                let mut stealed_amount = 0;

                for (item, amount) in specific_items {
                    stealed_items.insert(item, amount);
                    total_valiability += item.purchase_properties.base_price * amount as i64;
                    stealed_amount += amount;
                }

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

                self.ctx.send(embed).await?;
            }
            ConsequenceKind::Shop { name, items } => {
                let shop = Shop::new(name, None, items);
                shop.prompt(self.user.clone(), &mut self.ctx).await?;
            }
            ConsequenceKind::RemoveItemDurability(item, amount) => {
                let Some(mut character) = self
                    .ctx
                    .db()
                    .characters()
                    .get_by_user(&self.user.id.to_string())
                    .await?
                else {
                    return Ok(());
                };

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
            ConsequenceKind::AddActionPoint(amount) => {
                let Some(mut character) = self
                    .ctx
                    .db()
                    .characters()
                    .get_by_user(&self.user.id.to_string())
                    .await?
                else {
                    return Ok(());
                };

                character.action_points =
                    (character.action_points + amount).min(character.max_action_points);
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

        let mut rng = StdRng::from_entropy();
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
                let reward = e.drop.to_reward(&mut rng, character.pl, e.power_level());
                FighterData::new_from_enemy(1, reward, e)
            })
            .collect::<Vec<_>>();

        if !instant {
            encounter::prompt_encounter(&mut self.ctx, self.user.clone(), enemies).await?;
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
