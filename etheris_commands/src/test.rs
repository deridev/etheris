use etheris_rpg::shop::Shop;

use crate::prelude::*;

#[command("test")]
#[name("test")]
#[character_required(true)]
pub async fn test(mut ctx: CommandContext) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    if author.id.get() != 518830049949122571 {
        return Ok(());
    }

    let shop = Shop::new(
        "Teste".into(),
        None,
        vec![ShopItem {
            identifier: "stick".into(),
            price: 2,
            quantity: 5,
        }],
    );

    shop.prompt(author, &mut ctx).await?;

    Ok(())
}
