use std::time::Duration;

use etheris_common::Color;
use etheris_discord::{
    twilight_model::channel::{message::component::ButtonStyle, Message},
    *,
};
use etheris_framework::{util::make_multiple_rows, CommandContext, Response};

use crate::*;

fn make_input_rows(
    controller: &mut BattleController,
    valid_inputs: &[BattleInputKind],
    selected_custom_id: Option<String>,
) -> Vec<ActionRowBuilder> {
    let buttons = valid_inputs
        .iter()
        .map(|b| (b.button(BattleApi::new(controller)), b.second_row()))
        .map(|(b, second_row)| {
            let data = b.data.custom_id.clone().unwrap_or_default();
            if let Some(selected) = &selected_custom_id {
                (
                    b.set_disabled(true).set_style(if &data == selected {
                        ButtonStyle::Success
                    } else {
                        ButtonStyle::Secondary
                    }),
                    second_row,
                )
            } else {
                (b, second_row)
            }
        })
        .collect::<Vec<_>>();

    let first_row = buttons
        .iter()
        .filter_map(|(b, second_row)| if !second_row { Some(b.clone()) } else { None })
        .collect::<Vec<_>>();
    let second_row = buttons
        .iter()
        .filter_map(|(b, second_row)| if *second_row { Some(b.clone()) } else { None })
        .collect::<Vec<_>>();

    let mut rows = vec![ActionRowBuilder::new().add_buttons(first_row)];

    if !second_row.is_empty() {
        rows.push(ActionRowBuilder::new().add_buttons(second_row));
    }

    rows
}

pub async fn get_input(controller: &mut BattleController) -> anyhow::Result<BattleInput> {
    let current_fighter_user_id = match &controller.battle.get_current_fighter().user {
        Some(user) => user.id,
        None => {
            let Some(brain) = controller.battle.get_current_fighter().brain.clone() else {
                panic!("No brain for the current fighter that also has no user attached to it!");
            };

            return Ok(brain
                .dynamic_brain
                .lock()
                .await
                .select_input(BattleApi::new(controller))
                .await);
        }
    };

    let mut valid_inputs = vec![];

    for input in BattleInputKind::LIST {
        if input.can_use(BattleApi::new(controller)) {
            valid_inputs.push(input);
        }
    }

    let rows = make_input_rows(controller, &valid_inputs, None);
    let mut response = Response::from(controller.create_battle_embed())
        .add_string_content(format!("<@{}>", current_fighter_user_id))
        .set_components(rows);

    if let Some(last_message) = &controller.last_message {
        if controller.battle.turn_counter % 6 == 0 {
            controller
                .ctx
                .client
                .http
                .delete_message(last_message.channel_id, last_message.id)
                .await
                .ok();
            controller.last_message = None;
        }
    }

    let message = match &controller.last_message {
        Some(message) => {
            controller
                .ctx
                .update_specific_message(message, response.clone())
                .await?;
            message.clone()
        }
        None => {
            let message = controller.ctx.send(response.clone()).await?;
            controller.last_message = Some(message.clone());
            message
        }
    };

    let mut ctx = controller.ctx.clone();
    let Ok(collected) =
        input_util::await_component_allowing_intruders(message.id, &mut ctx, controller).await
    else {
        return Ok(BattleInput::Nothing);
    };

    let data = collected.parse_message_component_data()?;
    let input_kind = BattleInputKind::LIST.iter().find(|input| input.id() == data.custom_id)
        .expect("BattleController::get_input should never fail at retrieving input kind from the collected component data custom id.");

    controller.last_interaction = Some(collected.clone());

    let mut ctx =
        CommandContext::from_with_interaction(&controller.ctx, Box::new(collected.clone()));

    let rows = make_input_rows(controller, &valid_inputs, Some(data.custom_id.clone()));
    response = response.set_components(rows);

    if !matches!(
        *input_kind,
        BattleInputKind::UseSkill
            | BattleInputKind::Finish
            | BattleInputKind::ChangeTarget
            | BattleInputKind::ChangeTeam
            | BattleInputKind::UseItem
    ) {
        ctx.update_message(response).await?;
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    Ok(match input_kind {
        BattleInputKind::ChangeTarget => {
            get_change_target_input(controller, collected, message).await?
        }
        BattleInputKind::ChangeTeam => {
            get_change_team_input(controller, collected, message).await?
        }
        BattleInputKind::Attack => BattleInput::Attack,
        BattleInputKind::Defend => BattleInput::Defend,
        BattleInputKind::UseSkill => get_skill_input(controller, collected, message).await?,
        BattleInputKind::Finish => get_finisher_input(controller, collected, message).await?,
        BattleInputKind::GetUp => BattleInput::GetUp,
        BattleInputKind::Upkick => BattleInput::Upkick,
        BattleInputKind::UseItem => get_item_input(controller, collected, message).await?,
    })
}

pub async fn get_skill_input(
    controller: &mut BattleController,
    interaction: Interaction,
    message: Message,
) -> anyhow::Result<BattleInput> {
    let skills = controller.battle.get_current_fighter().skills.clone();

    let mut skill_displays = vec![];
    for skill in skills.iter() {
        let display = skill.dynamic_skill.lock().await.display();
        skill_displays.push(display);
    }

    let embed = EmbedBuilder::new_common()
        .set_color(Color::LIGHT_RED)
        .set_author(EmbedAuthor {
            name: format!(
                "Habilidades de {}",
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
            skill_displays
                .iter()
                .map(|display| {
                    format!(
                        "## {}\n{}\n{}",
                        display.header, display.sub_header, display.body
                    )
                })
                .collect::<Vec<_>>()
                .join("\n"),
        );

    let mut buttons = vec![];

    for skill in skills.iter() {
        let dynamic_skill = skill.dynamic_skill.lock().await;
        let button = ButtonBuilder::new()
            .set_custom_id(skill.identifier)
            .set_label(dynamic_skill.data().name)
            .set_style(ButtonStyle::Primary)
            .set_disabled(!dynamic_skill.can_use(BattleApi::new(controller)));
        buttons.push(button);
    }

    buttons.insert(
        0,
        ButtonBuilder::new()
            .set_custom_id("return")
            .set_label("Voltar"),
    );

    let mut ctx =
        CommandContext::from_with_interaction(&controller.ctx, Box::new(interaction.clone()));

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

    let Some(skill) = skills.iter().find(|s| s.identifier == data.custom_id) else {
        return Ok(BattleInput::Reinput);
    };

    Ok(BattleInput::UseSkill(skill.clone()))
}

pub async fn get_finisher_input(
    controller: &mut BattleController,
    interaction: Interaction,
    message: Message,
) -> anyhow::Result<BattleInput> {
    let finishers = controller.battle.get_current_fighter().finishers.clone();

    let embed = EmbedBuilder::new_common()
        .set_color(Color::LIGHT_RED)
        .set_author(EmbedAuthor {
            name: format!(
                "Finalizações de {}",
                controller.battle.get_current_fighter().name
            ),
            icon_url: controller
                .battle
                .get_current_fighter()
                .user
                .as_ref()
                .map(|u| u.avatar_url()),
        })
        .set_description(format!(
            "## **Você irá finalizar:** {}\nEscolha um método de finalização.\n\n{}",
            controller.battle.get_target_fighter().name,
            finishers
                .iter()
                .map(|f| format!(
                    "- **{}**\n{}\nChance de falhar: `{}%`",
                    f.name(),
                    if f.is_fatal() { "Fatal" } else { "Não-fatal" },
                    f.fail_probability().value()
                ))
                .collect::<Vec<_>>()
                .join("\n")
        ));

    let mut buttons = vec![];

    for finisher in finishers.iter() {
        let button = ButtonBuilder::new()
            .set_custom_id(finisher.name())
            .set_label(finisher.name())
            .set_style(ButtonStyle::Primary);
        buttons.push(button);
    }

    buttons.insert(
        0,
        ButtonBuilder::new()
            .set_custom_id("return")
            .set_label("Voltar"),
    );

    let mut ctx =
        CommandContext::from_with_interaction(&controller.ctx, Box::new(interaction.clone()));

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

    let Some(finisher) = finishers.iter().find(|s| s.name() == data.custom_id) else {
        return Ok(BattleInput::Reinput);
    };

    Ok(BattleInput::Finish(*finisher))
}

pub async fn get_item_input(
    controller: &mut BattleController,
    interaction: Interaction,
    message: Message,
) -> anyhow::Result<BattleInput> {
    let inventory = controller.battle.get_current_fighter().inventory.clone();

    let embed = EmbedBuilder::new_common()
        .set_color(Color::LIGHT_ORANGE)
        .set_author(EmbedAuthor {
            name: format!(
                "Inventário de {}",
                controller.battle.get_current_fighter().name
            ),
            icon_url: controller
                .battle
                .get_current_fighter()
                .user
                .as_ref()
                .map(|u| u.avatar_url()),
        })
        .set_title("Escolha um item para usar:")
        .set_description(
            inventory
                .iter()
                .map(|i| format!("{} {}x {}", i.item.emoji, i.quantity, i.item.display_name))
                .collect::<Vec<_>>()
                .join("\n"),
        );

    let mut buttons = vec![];

    for item in inventory.iter() {
        let button = ButtonBuilder::new()
            .set_custom_id(item.item.identifier)
            .set_label(item.item.display_name)
            .set_emoji(item.item.emoji)
            .set_style(ButtonStyle::Secondary);
        buttons.push(button);
    }

    buttons.insert(
        0,
        ButtonBuilder::new()
            .set_custom_id("return")
            .set_label("Voltar"),
    );

    let mut ctx =
        CommandContext::from_with_interaction(&controller.ctx, Box::new(interaction.clone()));

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

    let Some(item) = inventory
        .iter()
        .find(|s| s.item.identifier == data.custom_id)
    else {
        return Ok(BattleInput::Reinput);
    };

    Ok(BattleInput::UseItem(item.item))
}
