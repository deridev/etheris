use std::time::Duration;

use anyhow::bail;
use etheris_discord::{
    twilight_model::id::{marker::MessageMarker, Id},
    Interaction,
};
use etheris_framework::{watcher::WatcherOptions, *};
use tokio_stream::StreamExt;

use crate::{BattleController, FighterData};

pub async fn await_component_allowing_intruders(
    message_id: Id<MessageMarker>,
    ctx: &mut CommandContext,
    controller: &mut BattleController,
) -> anyhow::Result<Interaction> {
    let current_fighter_user_id = match &controller.battle.get_current_fighter().user {
        Some(u) => u.id,
        None => bail!("User expected as current fighter"),
    };

    let stream = ctx.watcher.create_component_stream(
        message_id,
        |_| true,
        WatcherOptions {
            timeout: Duration::from_secs(60),
        },
    );

    tokio::pin!(stream);

    while let Some(Ok(collected)) = stream.next().await {
        // Return the interaction if it's not a intruder
        if collected.author_id() == Some(current_fighter_user_id) {
            return Ok(collected);
        }

        let is_another_fighter_input = controller.battle.fighters.iter().any(|f| {
            f.user
                .as_ref()
                .is_some_and(|u| Some(u.id) == collected.author_id())
        });

        if is_another_fighter_input {
            continue;
        }

        let mut ctx =
            CommandContext::from_with_interaction(&controller.ctx, Box::new(collected.clone()));

        let Some(author_id) = collected.author_id() else {
            continue;
        };
        let author = ctx.author().await?;

        if controller.battle.intruder_count >= controller.battle.settings.max_intruders as usize {
            ctx.reply(
                Response::new_user_reply(&author, "essa batalha não suporta mais invasores!")
                    .set_ephemeral(),
            )
            .await?;
            continue;
        }

        if controller.ctx.client.is_user_fighting(author.id).await {
            ctx.reply(
                Response::new_user_reply(
                    &author,
                    "você já está em batalha e não pode invadir essa luta!",
                )
                .set_ephemeral(),
            )
            .await?;
            continue;
        }

        if let Some(character) = ctx
            .db()
            .characters()
            .get_by_user(&author_id.to_string())
            .await?
        {
            if character.region != controller.battle.region {
                ctx.reply_interaction(
                    Response::new_user_reply(
                        &author,
                        format!("essa batalha está localizada na região **{}**! Você precisa estar nessa região para invadir a batalha.",
                         controller.battle.region)
                    ).set_ephemeral(),
                )
                .await?;
                continue;
            }

            let new_team = controller
                .battle
                .teams()
                .keys()
                .fold(0, |acc, x| if *x > acc { *x } else { acc })
                .saturating_add(1);
            let fighter = FighterData::new_from_character(
                new_team,
                &character,
                author.clone(),
                Default::default(),
            );
            controller.emit_turn_message(format!("**{}** entrou na luta!", fighter.name));
            controller.battle.join_intruder(fighter);
            controller.ctx.client.mark_user_as_fighter(author.id).await;

            ctx.reply_interaction(
                Response::new_user_reply(&author, "você invadiu a batalha!").set_ephemeral(),
            )
            .await?;
        } else {
            ctx.reply(
                Response::new_user_reply(
                    &author,
                    "crie um personagem com **/registrar** para poder usar Etheris!",
                )
                .set_ephemeral(),
            )
            .await?;
        }
    }

    bail!("No component found in time")
}
