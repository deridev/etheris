use etheris_data::items::get_item;

use crate::prelude::*;

#[command("Envie orbs para outro usuário!")]
#[name("enviar item")]
#[character_required(true)]
pub async fn send_item(
    mut ctx: CommandContext,
    #[rename("usuário")]
    #[description("O usuário que você quer ver enviar orbs")]
    user: User,
    #[rename("item")]
    #[description("Item que você quer enviar")]
    item_name: String,
    #[rename("quantia")]
    #[description("Quantia de itens")]
    amount: Option<i64>,
) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let amount = amount.unwrap_or(1).clamp(1, i64::MAX);
    let author_character = parse_user_character!(ctx, author);
    let user_character = parse_user_character!(ctx, user);

    if author_character.region != user_character.region {
        ctx.reply(
            Response::new_user_reply(
                &author,
                "você precisa estar na mesma região para enviar itens!",
            )
            .add_emoji_prefix(emojis::ERROR),
        )
        .await?;
        return Ok(());
    }

    let Some(inventory_item) = user_character.get_inventory_item_by_name(&item_name) else {
        ctx.reply(
            Response::new_user_reply(
                &author,
                "esse item não existe, não está no seu inventário ou teve o nome escrito errado!",
            )
            .add_emoji_prefix(emojis::ERROR),
        )
        .await?;
        return Ok(());
    };

    let Some(item) = get_item(&inventory_item.identifier) else {
        return Ok(());
    };

    let confirmation = ctx
        .helper()
        .create_confirmation(
            author.id,
            false,
            Response::new_user_reply(
                &author,
                format!(
                    "você tem certeza que quer enviar **{}x {}** para o personagem **{}**?",
                    amount, item.display_name, user_character.name
                ),
            )
            .add_emoji_prefix(item.emoji),
        )
        .await?;
    if !confirmation {
        return Ok(());
    }

    let mut author_character = parse_user_character!(ctx, author);
    let mut user_character = parse_user_character!(ctx, user);

    if !author_character.has_item_by_name(&item_name, amount as usize) {
        ctx.send(
            Response::new_user_reply(
                &author,
                "você não tem essa quantia desse item no **/inventário**!",
            )
            .add_emoji_prefix(emojis::ERROR),
        )
        .await?;
        return Ok(());
    }

    author_character.remove_item(item, amount as usize);
    user_character.add_item(item, amount as usize, Some(inventory_item.values.clone()));

    ctx.send(
        Response::new_user_reply(
            &author,
            format!(
                "você enviou **{}x {}** para **{}**!",
                amount, item.display_name, user_character.name
            ),
        )
        .add_emoji_prefix(item.emoji),
    )
    .await?;

    ctx.db().characters().save(author_character).await?;
    ctx.db().characters().save(user_character).await?;

    Ok(())
}
