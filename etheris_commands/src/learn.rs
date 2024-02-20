use std::time::Duration;

use etheris_discord::twilight_model::channel::message::component::ButtonStyle;
use etheris_framework::{util::make_multiple_rows, watcher::WatcherOptions};
use etheris_rpg::list::get_boxed_skill_from_kind;

use crate::prelude::*;

#[command("Gaste o conhecimento que você adquiriu para aprender habilidades novas!")]
#[name("aprender")]
#[character_required(true)]
pub async fn learn(mut ctx: CommandContext) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let character = parse_user_character!(ctx, author);
    if character.action_points < 1 {
        ctx.reply(
            Response::new_user_reply(
                &author,
                "você precisa de 1 ponto de ação para aprender habilidades! Use **/perfil** para ver quando seus pontos de ações recarregam."
            )
            .set_ephemeral(),
        )
        .await?;
        return Ok(());
    }

    if character.learnable_skills.is_empty() {
        ctx.reply(
            Response::new_user_reply(
                &author,
                "você não tem nenhuma habilidade conhecida para aprender! Continue estudando (**/estudar**) e enfrentando oponentes para descobrir habilidades novas."
            )
            .set_ephemeral()
            .add_emoji_prefix(emojis::ERROR),
        )
        .await?;
        return Ok(());
    }

    let learnable = character
        .learnable_skills
        .iter()
        .map(|s| get_boxed_skill_from_kind(s.clone()))
        .rev()
        .take(10)
        .collect::<Vec<_>>();

    let mut buttons = vec![];
    for skill in learnable.iter() {
        let knowledge_cost = skill.kind().knowledge_cost();
        buttons.push(
            ButtonBuilder::new()
                .set_custom_id(skill.data().identifier)
                .set_label(skill.data().name.to_string())
                .set_disabled(knowledge_cost > character.knowledge_points),
        );
    }

    let embed = EmbedBuilder::new_common()
        .set_color(Color::CYAN_GREEN)
        .set_author(EmbedAuthor {
            name: format!("{} está aprendendo habilidades!", character.name),
            icon_url: Some(author.avatar_url()),
        })
        .set_description(format!(
            "## 🧠 Você tem `{}` pontos de conhecimento!\n{}",
            character.knowledge_points,
            learnable
                .iter()
                .map(|skill| {
                    format!(
                        "## {}\n**`{} conhecimento`**\n{} **{}**\n{}",
                        skill.data().name,
                        skill.kind().knowledge_cost(),
                        emojis::ETHER,
                        skill.data().use_cost.ether,
                        skill.data().description
                    )
                })
                .collect::<Vec<_>>()
                .join("\n")
        ))
        .add_footer_text("Qual habilidade você quer aprender?");

    let message = ctx
        .send(Response::from(embed).set_components(make_multiple_rows(buttons.clone())))
        .await?;

    let author_id = author.id;
    let Ok(Some(component)) = ctx
        .watcher
        .await_single_component(
            message.id,
            move |interaction| interaction.author_id() == Some(author_id),
            WatcherOptions {
                timeout: Duration::from_secs(60),
            },
        )
        .await
    else {
        return Ok(());
    };

    let data = component.parse_message_component_data()?;
    let Some(skill) = learnable
        .into_iter()
        .find(|skill| skill.data().identifier == data.custom_id)
    else {
        return Ok(());
    };

    let cost = skill.kind().knowledge_cost();

    let mut character = parse_user_character!(ctx, author);
    if character.action_points < 1 {
        return Ok(());
    }

    if character.knowledge_points < cost {
        return Ok(());
    }

    character.action_points -= 1;
    character.knowledge_points -= cost;

    character.learn_skill(skill.kind());

    let auto_equipped = character.learned_skills.len() < 5;
    if auto_equipped {
        character.equip_skill(skill.kind());
    }

    ctx.db().characters().save(character).await?;

    let mut ctx = CommandContext::from_with_interaction(&ctx, Box::new(component));

    let buttons = buttons
        .into_iter()
        .map(|b| {
            let button_data = b.data.clone();
            b.set_disabled(true).set_style(
                if Some(data.custom_id.clone()) == button_data.custom_id {
                    ButtonStyle::Success
                } else {
                    ButtonStyle::Secondary
                },
            )
        })
        .collect::<Vec<_>>();

    ctx.update_message(Response::default().set_components(make_multiple_rows(buttons)))
        .await?;

    tokio::time::sleep(Duration::from_secs(1)).await;

    if auto_equipped {
        ctx.send_in_channel(
            Response::new_user_reply(
                &author,
                format!("você aprendeu e equipou a habilidade **{}** com sucesso! Use **/habilidades** para ver as habilidades equipadas e aprendidas do seu personagem.", skill.data().name)
            ).add_emoji_prefix(emojis::SUCCESS)
        ).await?;
    } else {
        ctx.send_in_channel(
            Response::new_user_reply(
                &author,
                format!("você aprendeu a habilidade **{}** com sucesso! Para equipar essa habilidade, use **/habilidade equipar**. Ou use **/habilidades** para ver as habilidades do seu personagem.", skill.data().name)
            ).add_emoji_prefix(emojis::SUCCESS)
        ).await?;
    }

    Ok(())
}
