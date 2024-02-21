use etheris_data::{items::ALL_ITEMS, world::regions::WorldRegion};
use etheris_rpg::shop::Shop;

use crate::prelude::*;

#[command("Em uma cidade, visite uma loja para comprar itens!")]
#[name("loja")]
#[character_required(true)]
pub async fn shop(mut ctx: CommandContext) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let character = parse_user_character!(ctx, author);
    let region = character.region;

    if character.region.city().is_none() {
        ctx.reply(
            Response::new_user_reply(
                &author,
                "você precisa estar em uma cidade para poder ir em uma loja! Use **/viajar** para trocar de região e buscar uma região do tipo cidade."
            ).add_emoji_prefix(emojis::ERROR)
            .set_ephemeral(),
        )
        .await?;
        return Ok(());
    }

    let mut items: Vec<ShopItem> = ALL_ITEMS
        .iter()
        .filter(|i| i.purchase_properties.default_shop_sells)
        .map(|i| ShopItem {
            identifier: i.identifier.to_string(),
            price: i.purchase_properties.base_price,
            quantity: match i.purchase_properties.base_price {
                i64::MIN..=100 => 100,
                101..=400 => 50,
                401..=1000 => 25,
                _ => 5,
            },
        })
        .collect();

    if character.region == WorldRegion::SwordTown {
        items.push(ShopItem {
            identifier: "katana".into(),
            price: 150,
            quantity: 500
        });
        items.push(ShopItem {
            identifier: "intelligence_crystal".into(),
            price: 2500,
            quantity: 1
        });
    }

    let shop = Shop::new(
        format!("{} Shop", region),
        None,
        items,
    );

    shop.prompt(author, &mut ctx).await?;

    Ok(())
}
