use anyhow::bail;

use crate::prelude::*;

#[command("Use itens para o seu personagem!")]
#[name("usar")]
#[character_required(true)]
pub async fn usecmd(
    mut ctx: CommandContext,
    #[rename("item")]
    #[description("Nome do item que você quer ler")]
    item: String,
) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let mut character = parse_user_character!(ctx, author);
    let Some(inventory_item) = character.get_inventory_item_by_name(&item) else {
        ctx.reply(
            Response::new_user_reply(&author, "você não possui esse item ou esse item não existe! Utilize **/inventário** para ver os seus itens.")
            .add_emoji_prefix(emojis::ERROR),
        ).await?;
        return Ok(());
    };

    let Some(item) = items::get_item(&inventory_item.identifier) else {
        bail!("Item not found: {}", inventory_item.identifier);
    };

    match item.identifier {
        "intelligence_crystal" => {
            character.intelligence_xp += 500;
            ctx.reply(
                Response::new_user_reply(&author, "você consumiu um cristal da inteligência e ganhou **500 XP**! Estude uma vez e sinta o conhecimento expandir.")
                .add_emoji_prefix(item.emoji),
            ).await?;
        }
        "invigorating_crystal" => {
            character.action_points = character.max_action_points;
            character.stats.resistance.value = character.stats.resistance.max;
            character.stats.vitality.value = character.stats.vitality.max;
            character.stats.ether.value = character.stats.ether.max;
            ctx.reply(
                Response::new_user_reply(
                    &author,
                    "você consumiu um cristal revigorante e entrou no seu potencial máximo!",
                )
                .add_emoji_prefix(item.emoji),
            )
            .await?;
        }
        _ => {
            ctx.reply(
                Response::new_user_reply(&author, "esse item não pode ser utilizado!.")
                    .add_emoji_prefix(emojis::ERROR),
            )
            .await?;
            return Ok(());
        }
    }

    character.remove_item(item, 1);
    ctx.db().characters().save(character).await?;

    Ok(())
}
