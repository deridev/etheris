use etheris_rpg::shop::Shop;

use crate::prelude::*;

#[command("Em uma cidade, visite uma loja para comprar itens!")]
#[name("loja")]
#[character_required(true)]
pub async fn shop(mut ctx: CommandContext) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let character = parse_user_character!(ctx, author);
    let region = character.region;

    let Some(city) = character.region.city() else {
        ctx.reply(
            Response::new_user_reply(
                &author,
                "você precisa estar em uma cidade para poder ir em uma loja! Use **/viajar** para trocar de região e buscar uma região do tipo cidade."
            ).add_emoji_prefix(emojis::ERROR)
            .set_ephemeral(),
        )
        .await?;
        return Ok(());
    };

    let items: Vec<ShopItem> = city.shop_items;
    let shop = Shop::new(format!("{} Shop", region), None, items);

    shop.prompt(author, &mut ctx).await?;

    Ok(())
}
