use config::{BATTLE_INVENTORY_MAX_ALLOCATIONS, BATTLE_INVENTORY_MAX_ITEM_AMOUNT};
use etheris_data::items::get_item;
use items::ItemTag;

use crate::prelude::*;

#[command("Aloque itens para o inventário de batalha!")]
#[name("alocar")]
#[character_required(true)]
pub async fn allocate(
    mut ctx: CommandContext,
    #[rename("item")]
    #[description("Item que você quer alocar")]
    item_name: String,
    #[rename("quantia")]
    #[description("Quantia que você quer alocar")]
    quantity: Option<i64>,
) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let mut character = parse_user_character!(ctx, author);
    let quantity = quantity.unwrap_or(1).clamp(1, i32::MAX as i64) as usize;

    let Some(inventory_item) = character.get_inventory_item_by_name(&item_name).cloned() else {
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

    let mut quantity = quantity.clamp(1, inventory_item.quantity);
    let Some(item) = get_item(&inventory_item.identifier) else {
        return Ok(());
    };

    if character.battle_inventory.len() >= BATTLE_INVENTORY_MAX_ALLOCATIONS
        && !character
            .battle_inventory
            .iter()
            .any(|i| i.identifier == item.identifier)
    {
        ctx.send(
            Response::new_user_reply(
                &author,
                format!("você já tem mais de {BATTLE_INVENTORY_MAX_ALLOCATIONS} itens diferentes itens no inventário de batalha! Use **/desalocar** para remover itens do inventário de batalha."),	
            )
            .add_emoji_prefix(emojis::ERROR)
            .set_ephemeral(),
        ).await?;
        return Ok(());
    }

    if !item.stackable || item.tags.contains(&ItemTag::Crystal) {
        quantity = 1;
    }

    if item.tags.contains(&ItemTag::Crystal)
        && character
            .battle_inventory
            .iter()
            .any(|i| get_item(&i.identifier).is_some_and(|i| i.tags.contains(&ItemTag::Crystal)))
    {
        ctx.send(
            Response::new_user_reply(
                &author,
                "você só pode ter um cristal alocado no inventário de batalha! Use **/desalocar** para remover itens do inventário de batalha."
            )
            .add_emoji_prefix(emojis::ERROR)
            .set_ephemeral(),
        ).await?;
        return Ok(());
    }

    if let Some(battle_inventory_item) = character.get_battle_inventory_item(&item) {
        if !item.stackable {
            ctx.send(
                Response::new_user_reply(
                    &author,
                    "esse item já está no inventário de batalha e você não pode ter dois desse item! Use **/desalocar** para remover itens do inventário de batalha."
                )
                .add_emoji_prefix(emojis::ERROR)
                .set_ephemeral(),
            ).await?;
            return Ok(());
        }

        if battle_inventory_item.quantity + quantity > BATTLE_INVENTORY_MAX_ITEM_AMOUNT {
            ctx.reply(
                Response::new_user_reply(
                    &author,
                    format!("você só pode ter **{BATTLE_INVENTORY_MAX_ITEM_AMOUNT}x** do mesmo item no inventário de batalha!"),
                )
                .add_emoji_prefix(emojis::ERROR),
            )
            .await?;
            return Ok(());
        }
    }

    character.remove_item(item, quantity);
    character.add_battle_item(
        item,
        quantity.min(BATTLE_INVENTORY_MAX_ITEM_AMOUNT),
        Some(inventory_item.values.clone()),
    );

    ctx.db().characters().save(character).await?;

    ctx.send(
        Response::new_user_reply(
            &author,
            format!(
                "você alocou **{}x {}** para o inventário de batalha! Se quiser desalocar o item, use **/desalocar**.",
                quantity, item.display_name
            )
        )
        .add_emoji_prefix(item.emoji),
    )
    .await?;

    Ok(())
}
