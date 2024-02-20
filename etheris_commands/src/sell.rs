use etheris_data::items::get_item;

use crate::prelude::*;

#[command("Em uma cidade, venda itens para conseguir orbs!")]
#[name("vender")]
#[character_required(true)]
pub async fn sell(
    mut ctx: CommandContext,
    #[rename("item")]
    #[description("Item que você quer enviar")]
    item_name: String,
    #[rename("quantia")]
    #[description("Quantia de itens")]
    amount: Option<i64>,
) -> anyhow::Result<()> {
    let amount = amount.unwrap_or(1).clamp(1, i32::MAX as i64) as i32;
    let author = ctx.author().await?;
    let character = parse_user_character!(ctx, author);

    let Some(city) = character.region.city() else {
        ctx.reply(
            Response::new_user_reply(
                &author,
                "você precisa estar em uma cidade para poder vender itens! Use **/viajar** para trocar de região e buscar uma região do tipo cidade."
            ).add_emoji_prefix(emojis::ERROR)
            .set_ephemeral(),
        )
        .await?;
        return Ok(());
    };

    let Some(inventory_item) = character.get_inventory_item_by_name(&item_name) else {
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

    if !item.purchase_properties.is_sellable || item.purchase_properties.base_sell_price < 1 {
        ctx.reply(
            Response::new_user_reply(
                &author,
                "você não encontrou ninguém que queira comprar este item!",
            )
            .add_emoji_prefix(emojis::ERROR),
        )
        .await?;
        return Ok(());
    }

    let price = ((item.purchase_properties.base_sell_price as f32 * city.sell_multiplier).round()
        as i32)
        .max(1);
    let profit = amount * price;

    let confirmation = ctx
        .helper()
        .create_confirmation(
            author.id,
            false,
            Response::new_user_reply(
                &author,
                format!(
                    "você tem certeza que quer vender **{}x {}** por **{} ◎** {}? (Valor por unidade: {} ◎)",
                    amount, item.display_name, profit, emojis::ORB, price
                ),
            )
            .add_emoji_prefix(item.emoji),
        )
        .await?;
    if !confirmation {
        return Ok(());
    }

    let mut character = parse_user_character!(ctx, author);

    if !character.has_item_by_name(&item_name, amount as usize) {
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

    character.remove_item(item, amount as usize);
    character.add_orbs(profit as i64);

    ctx.db().characters().save(character).await?;

    ctx.send(
        Response::new_user_reply(
            &author,
            format!(
                "você vendeu **{}x {}** e ganhou **{profit} ◎**!",
                amount, item.display_name
            ),
        )
        .add_emoji_prefix(item.emoji),
    )
    .await?;

    Ok(())
}
