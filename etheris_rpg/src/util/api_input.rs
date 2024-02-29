use std::fmt::Debug;

use etheris_common::Identifiable;
use etheris_data::emojis;
use etheris_discord::{twilight_model::channel::message::component::ButtonStyle, *};
use etheris_framework::{util::make_multiple_rows, *};
use rand::seq::SliceRandom;

use crate::{input_util, BattleApi, Fighter, FighterIndex};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ApiInput<T: Identifiable + Clone> {
    pub name: String,
    pub description: String,
    pub emoji: Option<Emoji<'static>>,
    pub active: bool,
    pub value: T,
}

pub async fn select_input<T: Identifiable + Clone>(
    api: &mut BattleApi<'_>,
    embed: Option<EmbedBuilder>,
    inputs: Vec<ApiInput<T>>,
) -> anyhow::Result<Option<ApiInput<T>>> {
    let fighter = api.fighter().clone();

    let Some(user_id) = fighter.user.map(|u| u.id) else {
        let input = inputs.choose(api.rng()).cloned();
        return Ok(input);
    };

    let mut buttons = vec![];
    let mut embed = if let Some(embed) = embed {
        embed
    } else {
        EmbedBuilder::new_common().set_author(EmbedAuthor {
            name: format!("{} está fazendo sua escolha!", fighter.name),
            icon_url: api
                .battle()
                .get_current_fighter()
                .user
                .as_ref()
                .map(|u| u.avatar_url()),
        })
    };

    for input in &inputs {
        let mut button = ButtonBuilder::new()
            .set_custom_id(&input.value.identifier())
            .set_label(&input.name)
            .set_disabled(!input.active);
        if let Some(emoji) = input.emoji {
            button = button.set_emoji(emoji);
        }

        buttons.push(button);

        embed = embed.add_inlined_field(
            format!(
                "{}{}",
                if let Some(emoji) = input.emoji {
                    format!("{emoji} ")
                } else {
                    String::new()
                },
                input.name
            ),
            &input.description,
        );
    }

    let response = Response::from(embed)
        .add_string_content(format!("<@{user_id}>"))
        .set_components(make_multiple_rows(buttons.clone()));

    let message = if let Some(message) = api.controller.last_message.clone() {
        api.ctx()
            .update_specific_message(&message, response)
            .await
            .unwrap();
        message
    } else {
        let message = api.ctx().send(response).await.unwrap();
        api.controller.last_message = Some(message.clone());
        message
    };

    let mut ctx = api.ctx().clone();
    let Ok(collected) =
        input_util::await_component_allowing_intruders(message.id, &mut ctx, api.controller).await
    else {
        return Ok(None);
    };

    let data = collected.parse_message_component_data()?;

    let mut ctx = CommandContext::from_with_interaction(&ctx, Box::new(collected));

    let buttons = buttons
        .into_iter()
        .map(|b| {
            let custom_id = b.data.custom_id.clone().unwrap_or_default();
            let style = b.data.style;
            b.set_disabled(true)
                .set_style(if custom_id == data.custom_id {
                    ButtonStyle::Success
                } else {
                    style
                })
        })
        .collect::<Vec<_>>();

    ctx.update_message(Response::default().set_components(make_multiple_rows(buttons)))
        .await?;

    let action = inputs
        .into_iter()
        .find(|input| input.value.identifier() == data.custom_id);
    Ok(action)
}

pub async fn select_ally(api: &mut BattleApi<'_>) -> anyhow::Result<Option<Fighter>> {
    let teams = api.battle().teams();
    let fighter = api.fighter().clone();

    let Some(ally_team) = teams.get(&fighter.team) else {
        return Ok(None);
    };

    let ally_team = ally_team
        .iter()
        .map(|index| api.battle().get_fighter(*index).clone())
        .collect::<Vec<_>>();
    if ally_team.len() < 2 {
        return Ok(Some(fighter));
    }

    let Some(user_id) = fighter.user.map(|u| u.id) else {
        let ally = ally_team.choose(api.rng()).cloned();
        return Ok(ally);
    };

    let mut buttons = vec![];
    let mut embed = EmbedBuilder::new_common().set_author(EmbedAuthor {
        name: format!("{} está selecionando um aliado!", fighter.name),
        icon_url: api
            .battle()
            .get_current_fighter()
            .user
            .as_ref()
            .map(|u| u.avatar_url()),
    });

    for ally in &ally_team {
        buttons.push(
            ButtonBuilder::new()
                .set_custom_id(ally.index.0.to_string())
                .set_label(ally.name.to_owned()),
        );

        embed = embed.add_inlined_field(
            &ally.name,
            format!(
                "{} **{}**/{}\n{} **{}**/{}",
                emojis::HEALTH,
                ally.health().value,
                ally.health().max,
                emojis::ETHER,
                ally.ether.value,
                ally.ether.max
            ),
        );
    }

    let response = Response::from(embed)
        .add_string_content(format!("<@{user_id}>"))
        .set_components(make_multiple_rows(buttons.clone()));

    let message = if let Some(message) = api.controller.last_message.clone() {
        api.ctx()
            .update_specific_message(&message, response)
            .await
            .unwrap();
        message
    } else {
        let message = api.ctx().send(response).await.unwrap();
        api.controller.last_message = Some(message.clone());
        message
    };

    let mut ctx = api.ctx().clone();
    let Ok(collected) =
        input_util::await_component_allowing_intruders(message.id, &mut ctx, api.controller).await
    else {
        return Ok(None);
    };

    let data = collected.parse_message_component_data()?;

    let mut ctx = CommandContext::from_with_interaction(&ctx, Box::new(collected));

    let buttons = buttons
        .into_iter()
        .map(|b| {
            let custom_id = b.data.custom_id.clone().unwrap_or_default();
            let style = b.data.style;
            b.set_disabled(true)
                .set_style(if custom_id == data.custom_id {
                    ButtonStyle::Success
                } else {
                    style
                })
        })
        .collect::<Vec<_>>();

    ctx.update_message(Response::default().set_components(make_multiple_rows(buttons)))
        .await?;

    let fighter_index = FighterIndex(data.custom_id.parse::<usize>().unwrap_or(fighter.index.0));
    Ok(Some(api.battle().get_fighter(fighter_index).clone()))
}
