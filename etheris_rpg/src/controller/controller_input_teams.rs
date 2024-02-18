use std::time::Duration;

use etheris_common::Color;
use etheris_discord::{
    twilight_model::channel::{message::component::ButtonStyle, *},
    *,
};
use etheris_framework::{util::make_multiple_rows, *};

use crate::*;

pub async fn get_change_team_input(
    controller: &mut BattleController<'_>,
    interaction: Interaction,
    message: Message,
) -> anyhow::Result<BattleInput> {
    let teams = controller.battle.teams();
    let mut teams = teams.into_iter().collect::<Vec<_>>();
    teams.sort_by_key(|(t, ..)| *t);

    let is_fighter_team_empty = teams
        .iter()
        .find(|(t, ..)| *t == controller.battle.get_current_fighter().team)
        .map(|(.., f)| f.len() <= 1)
        .unwrap_or(true);

    let new_team = controller
        .battle
        .fighters
        .iter()
        .fold(0, |acc, fighter| acc.max(fighter.team))
        .saturating_add(1);

    let embed = EmbedBuilder::new_common()
        .set_color(Color::LIGHT_CYAN)
        .set_author(EmbedAuthor {
            name: format!(
                "{} está trocando de time!",
                controller.battle.get_current_fighter().name
            ),
            icon_url: controller
                .battle
                .get_current_fighter()
                .user
                .as_ref()
                .map(|u| u.avatar_url()),
        })
        .set_description(
            teams
                .iter()
                .map(|(team, fighters)| {
                    let fighters = fighters
                        .iter()
                        .map(|index| controller.battle.get_fighter(*index))
                        .collect::<Vec<_>>();
                    format!(
                        "### Time {}\n{}",
                        team,
                        fighters
                            .iter()
                            .map(|f| f.name.clone())
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                })
                .collect::<Vec<_>>()
                .join("\n"),
        )
        .add_footer_text(format!(
            "{} está no time {}",
            controller.battle.get_current_fighter().name,
            controller.battle.get_current_fighter().team
        ));

    let mut buttons = vec![];

    for (team, ..) in teams.iter() {
        if *team == controller.battle.get_current_fighter().team {
            continue;
        }

        buttons.push(
            ButtonBuilder::new()
                .set_custom_id(team.to_string())
                .set_label(format!("Time {team}"))
                .set_style(ButtonStyle::Primary),
        );
    }

    buttons.insert(
        0,
        ButtonBuilder::new()
            .set_custom_id("return")
            .set_label("Voltar"),
    );

    if !is_fighter_team_empty {
        buttons.push(
            ButtonBuilder::new()
                .set_custom_id("make_new")
                .set_style(ButtonStyle::Primary)
                .set_label(format!("Criar time {new_team}")),
        );
    }

    let mut ctx =
        CommandContext::from_with_interaction(controller.ctx, Box::new(interaction.clone()));

    ctx.update_message(
        Response::from(embed.clone()).set_components(make_multiple_rows(buttons.clone())),
    )
    .await?;

    let Ok(collected) =
        input_util::await_component_allowing_intruders(message.id, &mut ctx, controller).await
    else {
        return Ok(BattleInput::Nothing);
    };

    controller.last_interaction = Some(collected.clone());
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

    ctx.update_message(Response::from(embed.clone()).set_components(make_multiple_rows(buttons)))
        .await?;

    tokio::time::sleep(Duration::from_millis(100)).await;

    if data.custom_id == "return" {
        return Ok(BattleInput::Reinput);
    } else if data.custom_id == "make_new" {
        return Ok(BattleInput::ChangeTeam(new_team));
    }

    let Some((team, fighters)) = teams
        .into_iter()
        .find(|(team, ..)| team.to_string() == data.custom_id)
    else {
        return Ok(BattleInput::Nothing);
    };

    let current_fighter = controller.battle.get_current_fighter().clone();
    if current_fighter.team == team {
        return Ok(BattleInput::ChangeTeam(team));
    }

    // Get authorization of every fighter in this team
    let mut fighter_that_disallowed = None;
    for fighter in fighters {
        let fighter = controller.battle.get_fighter(fighter).clone();

        if let Some(user) = fighter.user.clone() {
            let confirmation = ctx.helper()
                .create_confirmation(
                    user.id, true,
                    Response::new_user_reply(
                        &user, format!("**{}**, do time **{}**, quer entrar no seu time. Você autoriza?\nMembros do mesmo time podem vencer a batalha unidos!", 
                        current_fighter.name, current_fighter.team))).await?;
            if !confirmation {
                fighter_that_disallowed = Some(fighter);
                break;
            }
        } else {
            let confirmation = ai::allow_fighter_to_enter_his_team(
                BattleApi::new(controller),
                current_fighter.index,
            )
            .await;
            if !confirmation {
                fighter_that_disallowed = Some(fighter);
                break;
            }
        }
    }

    if let Some(fighter) = fighter_that_disallowed {
        controller.emit_turn_message(format!(
            "**{}** não deu permissão para **{}** entrar em seu time!",
            fighter.name, current_fighter.name
        ));
        controller.update_turn_history_message().await?;
        tokio::time::sleep(Duration::from_secs(2)).await;
        return Ok(BattleInput::Reinput);
    }

    Ok(BattleInput::ChangeTeam(team))
}

pub async fn get_change_target_input(
    controller: &mut BattleController<'_>,
    interaction: Interaction,
    message: Message,
) -> anyhow::Result<BattleInput> {
    let mut alive_fighters = controller
        .battle
        .alive_fighters
        .iter()
        .filter(|index| **index != controller.battle.get_current_fighter().index)
        .map(|index| controller.battle.get_fighter(*index).clone())
        .collect::<Vec<_>>();

    alive_fighters.sort_by_key(|f| f.team);

    let embed = EmbedBuilder::new_common()
        .set_color(Color::LIGHT_YELLOW)
        .set_author(EmbedAuthor {
            name: format!(
                "{} está trocando de alvo!",
                controller.battle.get_current_fighter().name
            ),
            icon_url: controller
                .battle
                .get_current_fighter()
                .user
                .as_ref()
                .map(|u| u.avatar_url()),
        })
        .set_description(
            alive_fighters
                .iter()
                .map(|f| {
                    format!(
                        "- **{}** (time {}){}",
                        f.name,
                        f.team,
                        if f.team == controller.battle.get_current_fighter().team {
                            " (ALIADO)"
                        } else {
                            ""
                        }
                    )
                })
                .collect::<Vec<_>>()
                .join("\n"),
        )
        .add_footer_text(format!(
            "{} está no time {}",
            controller.battle.get_current_fighter().name,
            controller.battle.get_current_fighter().team
        ));

    let mut buttons = vec![];

    for fighter in alive_fighters.iter() {
        buttons.push(
            ButtonBuilder::new()
                .set_custom_id(format!("{}", fighter.index.0))
                .set_label(&fighter.name)
                .set_style(ButtonStyle::Primary),
        );
    }

    buttons.insert(
        0,
        ButtonBuilder::new()
            .set_custom_id("return")
            .set_label("Voltar"),
    );

    let mut ctx =
        CommandContext::from_with_interaction(controller.ctx, Box::new(interaction.clone()));

    ctx.update_message(
        Response::from(embed.clone()).set_components(make_multiple_rows(buttons.clone())),
    )
    .await?;

    let Ok(collected) =
        input_util::await_component_allowing_intruders(message.id, &mut ctx, controller).await
    else {
        return Ok(BattleInput::Nothing);
    };

    controller.last_interaction = Some(collected.clone());
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

    ctx.update_message(Response::from(embed.clone()).set_components(make_multiple_rows(buttons)))
        .await?;

    tokio::time::sleep(Duration::from_millis(100)).await;

    if data.custom_id == "return" {
        return Ok(BattleInput::Reinput);
    }

    let Some(fighter) = controller
        .battle
        .fighters
        .iter()
        .find(|f| f.index.0.to_string() == data.custom_id)
    else {
        return Ok(BattleInput::Nothing);
    };

    Ok(BattleInput::ChangeTarget(fighter.index))
}
