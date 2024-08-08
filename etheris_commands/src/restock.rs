use config::BATTLE_INVENTORY_MAX_ITEM_AMOUNT;
use items::ItemTag;

use crate::prelude::*;

#[command("Reponha seus itens do inventário de batalha!")]
#[name("repor")]
#[character_required(true)]
pub async fn restock(mut ctx: CommandContext) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let mut character = parse_user_character!(ctx, author);

    let mut items_migrated = vec![];
    for battle_item in character.battle_inventory.clone() {
        let Some(item) = items::get_item(&battle_item.identifier) else {
            continue;
        };

        if battle_item.quantity >= BATTLE_INVENTORY_MAX_ITEM_AMOUNT
            || item.tags.contains(&ItemTag::Crystal)
            || !item.stackable
        {
            continue;
        }

        let Some(inventory_item) = character.get_inventory_item(&item).cloned() else {
            continue;
        };

        let items_to_restock = BATTLE_INVENTORY_MAX_ITEM_AMOUNT - battle_item.quantity;
        let items_to_restock = items_to_restock.min(inventory_item.quantity);
        if items_to_restock == 0 {
            continue;
        }

        character.remove_item(item, items_to_restock);
        character.add_battle_item(item, items_to_restock, Some(inventory_item.values.clone()));
        items_migrated.push((items_to_restock, item.clone()));
    }

    if items_migrated.is_empty() {
        ctx.send(
            Response::new_user_reply(&author, "você não possui itens para repor!")
                .add_emoji_prefix(emojis::ERROR),
        )
        .await?;
        return Ok(());
    }

    ctx.db().characters().save(character).await?;

    ctx.send(
        Response::new_user_reply(
            &author,
            format!(
                "você repôs os itens do seu inventário de batalha!\n{}",
                items_migrated
                    .iter()
                    .map(|(amount, item)| format!(
                        "- {} **{}x {}**",
                        item.emoji, amount, item.display_name
                    ))
                    .collect::<Vec<_>>()
                    .join("\n")
            ),
        )
        .add_emoji_prefix(emojis::SUCCESS),
    )
    .await?;

    Ok(())
}
