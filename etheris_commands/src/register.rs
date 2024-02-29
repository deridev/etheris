use std::time::Duration;

use etheris_data::personality::Personality;
use etheris_discord::twilight_model::channel::message::component::ButtonStyle;
use etheris_framework::{util::make_multiple_rows, watcher::WatcherOptions};
use tokio_stream::StreamExt;

use crate::prelude::*;

#[command("Registre seu personagem em Etheris!")]
#[name("registrar")]
pub async fn register(
    mut ctx: CommandContext,
    #[rename("nome")]
    #[description("O nome do seu personagem")]
    #[min_max_length(2, 32)]
    name: String,
) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    if ctx.db().characters().is_user_registered(author.id).await? {
        ctx.reply(
            Response::new_user_reply(
                &author,
                "você já possui um personagem registrado! Use **/perfil** para ver ele.",
            )
            .add_emoji_prefix(emojis::ERROR),
        )
        .await?;
        return Ok(());
    }

    let Some(personalities) = select_personalities(&mut ctx, &name).await? else {
        return Ok(());
    };

    let mut skills = personalities
        .iter()
        .flat_map(|p| p.initial_learnable_skills())
        .collect::<Vec<_>>();
    skills.sort_unstable();
    skills.dedup();

    if ctx
        .db()
        .characters()
        .get_by_user(&author.id.to_string())
        .await?
        .is_some()
    {
        ctx.reply(
            Response::new_user_reply(
                &author,
                "você já possui um personagem registrado! Use **/perfil** para ver ele.",
            )
            .add_emoji_prefix(emojis::ERROR),
        )
        .await?;
        return Ok(());
    }

    ctx.db()
        .characters()
        .register_character(author.id, name.clone(), personalities, skills)
        .await?;

    ctx.reply(Response::new_user_reply(
        &author,
        format!("você registrou o personagem **{}** com sucesso!\nUse **/perfil** para ver os atributos e aparência do seu personagem.\nUse **/aprender** para escolher as suas primeiras habilidades!\n\n**❓ Sua primeira vez?**\nUse **/tutorial** para aprender o básico de Etheris!", name),
    )
    .add_emoji_prefix(emojis::SUCCESS)).await?;

    Ok(())
}

async fn select_personalities(
    ctx: &mut CommandContext,
    character_name: &String,
) -> anyhow::Result<Option<Vec<Personality>>> {
    const MAX_PERSONALITIES: usize = 3;

    let author = ctx.author().await?;
    let mut personalities: [Option<Personality>; MAX_PERSONALITIES] = Default::default();
    let mut index = 0;

    let mut buttons = Personality::LIST
        .iter()
        .map(|p| {
            ButtonBuilder::new()
                .set_label(p.to_string())
                .set_custom_id(p.to_string())
        })
        .collect::<Vec<_>>();
    let rows = make_multiple_rows(buttons.clone());

    let message = ctx
        .send(
            Response::new_user_reply(
                &author,
                format!(
                    "escolha {MAX_PERSONALITIES} traços de personalidade para o seu personagem **{character_name}**:"
                ),
            )
            .set_components(rows),
        )
        .await?;

    let stream = ctx.watcher.create_component_stream(
        message.id,
        move |int| int.author_id() == Some(author.id),
        WatcherOptions {
            timeout: Duration::from_secs(256),
        },
    );

    tokio::pin!(stream);

    while let Some(Ok(input)) = stream.next().await {
        let data = input.parse_message_component_data()?;

        let personality = *Personality::LIST
            .iter()
            .find(|p| p.to_string() == data.custom_id)
            .context("Finding a personality in the list should never fail")?;
        personalities[index] = Some(personality);
        index += 1;

        let exclusive_personalities = personalities
            .iter()
            .filter_map(|p| p.map(|p| p.exclusive_personality()))
            .flatten()
            .copied()
            .collect::<Vec<_>>();

        buttons = buttons
            .iter()
            .map(|b| {
                let data = b.data.clone();
                let button_selected = personalities
                    .iter()
                    .any(|p| p.is_some_and(|p| Some(p.to_string()) == data.custom_id));
                let exclusive_ed = exclusive_personalities
                    .iter()
                    .any(|p| Some(p.to_string()) == data.custom_id);

                b.clone()
                    .set_disabled(exclusive_ed || button_selected || index == MAX_PERSONALITIES)
                    .set_style(if button_selected {
                        ButtonStyle::Success
                    } else {
                        ButtonStyle::Secondary
                    })
            })
            .collect();

        let mut ctx = CommandContext::from_with_interaction(ctx, Box::new(input));
        ctx.update_message(Response::default().set_components(make_multiple_rows(buttons.clone())))
            .await?;

        if index == MAX_PERSONALITIES {
            break;
        }
    }

    let mut valid_personalities = Vec::with_capacity(MAX_PERSONALITIES);

    for p in personalities {
        if let Some(personality) = p {
            valid_personalities.push(personality);
        } else {
            return Ok(None);
        }
    }

    Ok(Some(valid_personalities))
}
