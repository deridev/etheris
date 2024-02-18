use crate::prelude::*;

#[command("Envie orbs para outro usuário!")]
#[name("enviar orbs")]
#[character_required(true)]
pub async fn send_orbs(
    mut ctx: CommandContext,
    #[rename("usuário")]
    #[description("O usuário que você quer ver enviar orbs")]
    user: User,
    #[rename("orbs")]
    #[description("Quantia de orbs")]
    orbs: i64,
) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let orbs = orbs.clamp(1, i64::MAX);
    let author_character = parse_user_character!(ctx, author);
    let user_character = parse_user_character!(ctx, user);

    if author_character.region != user_character.region {
        ctx.reply(
            Response::new_user_reply(
                &author,
                "você precisa estar na mesma região para enviar orbs!",
            )
            .add_emoji_prefix(emojis::ERROR),
        )
        .await?;
        return Ok(());
    }

    let confirmation = ctx
        .helper()
        .create_confirmation(
            author.id,
            false,
            Response::new_user_reply(
                &author,
                format!(
                    "você tem certeza que quer enviar **{} ◎** para o personagem **{}**?",
                    orbs, user_character.name
                ),
            )
            .add_emoji_prefix("❓"),
        )
        .await?;
    if !confirmation {
        return Ok(());
    }

    let mut author_character = parse_user_character!(ctx, author);
    let mut user_character = parse_user_character!(ctx, user);

    if author_character.orbs < orbs {
        ctx.send(
            Response::new_user_reply(&author, "você não tem essa quantia de orbs!")
                .add_emoji_prefix(emojis::ERROR),
        )
        .await?;
        return Ok(());
    }

    author_character.remove_orbs(orbs);
    user_character.add_orbs(orbs);

    ctx.send(
        Response::new_user_reply(
            &author,
            format!("você enviou **{orbs} ◎** para **{}**!", user_character.name),
        )
        .add_emoji_prefix(emojis::ORB),
    )
    .await?;

    ctx.db().characters().save(author_character).await?;
    ctx.db().characters().save(user_character).await?;

    Ok(())
}
