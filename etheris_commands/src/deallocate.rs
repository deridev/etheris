use etheris_data::items::get_item;

use crate::prelude::*;

#[command("Desaloque itens do inventário de batalha!")]
#[name("desalocar")]
#[character_required(true)]
pub async fn deallocate(
    mut ctx: CommandContext,
    #[rename("item")]
    #[description("Item que você quer desalocar")]
    item_name: String,
) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let mut character = parse_user_character!(ctx, author);

    let Some(inventory_item) = character
        .get_battle_inventory_item_by_name(&item_name)
        .cloned()
    else {
        ctx.reply(
            Response::new_user_reply(
                &author,
                "esse item não existe, não está no seu inventário de batalha ou teve o nome escrito errado!",
            )
            .add_emoji_prefix(emojis::ERROR),
        )
        .await?;
        return Ok(());
    };

    let Some(item) = get_item(&inventory_item.identifier) else {
        return Ok(());
    };

    character.remove_battle_item(item, inventory_item.quantity);
    character.add_item(
        item,
        inventory_item.quantity,
        Some(inventory_item.values.clone()),
    );

    ctx.db().characters().save(character).await?;

    ctx.send(
        Response::new_user_reply(
            &author,
            format!(
                "você desalocou **{}x {}** do inventário de batalha para o seu inventário comum! Se quiser alocar o item, use **/alocar**.",
                inventory_item.quantity, item.display_name
            )
        )
        .add_emoji_prefix(item.emoji),
    )
    .await?;

    Ok(())
}
