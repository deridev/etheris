use etheris_data::items::{get_item, Item};
use etheris_util::math;

use crate::prelude::*;

#[command("Consuma itens para recuperar fome, sede ou energia do seu personagem!")]
#[name("consumir")]
#[character_required(true)]
pub async fn consume(
    mut ctx: CommandContext,
    #[rename("item")]
    #[description("Nome do item que você quer consumir")]
    item_name: String,
    #[rename("quantidade")]
    #[description("Quantidade de itens que você quer consumir")]
    quantity: Option<i64>,
) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let mut character = parse_user_character!(ctx, author);
    let quantity = quantity.unwrap_or(1).clamp(1, i32::MAX as i64) as i32;

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

    let Some(item) = get_item(&inventory_item.identifier) else {
        return Ok(());
    };

    if inventory_item.quantity < quantity as usize {
        ctx.reply(
            Response::new_user_reply(
                &author,
                format!("você não possui **{quantity}x {}**!", item.display_name),
            )
            .add_emoji_prefix(emojis::ERROR),
        )
        .await?;
        return Ok(());
    }

    if item.has_consumption_function {
        special_consumption_function(ctx, character, item).await?;
        return Ok(());
    }

    let Some(consumption_properties) = item.consumption_properties else {
        ctx.reply(
            Response::new_user_reply(&author, "esse item não pode ser consumido!")
                .add_emoji_prefix(emojis::ERROR),
        )
        .await?;
        return Ok(());
    };

    let health_regeneration = math::calculate_health_regeneration(
        consumption_properties,
        quantity,
        character.stats.vitality.max + character.stats.resistance.max,
    );
    let ether_regeneration = math::calculate_ether_regeneration(
        consumption_properties,
        quantity,
        character.stats.ether.max,
    );

    let mut outputs = vec![];
    if health_regeneration != 0 {
        outputs.push(format!("{} `{}`", emojis::HEALTH, health_regeneration));
    }

    if ether_regeneration != 0 {
        outputs.push(format!("{} `{}`", emojis::ETHER, ether_regeneration));
    }

    character.heal(health_regeneration);
    character.add_ether(ether_regeneration);
    character.remove_item(item, quantity as usize);

    ctx.db().characters().save(character).await?;

    ctx.send(
        Response::new_user_reply(
            &author,
            format!(
                "você consumiu **{}x {}**! {}",
                quantity,
                item.display_name,
                outputs.join(", ")
            ),
        )
        .add_emoji_prefix(format!("😋{}", item.emoji)),
    )
    .await?;

    Ok(())
}

async fn special_consumption_function(
    mut ctx: CommandContext,
    mut character: CharacterModel,
    item: Item,
) -> anyhow::Result<()> {
    let author = ctx.author().await?;

    match item.identifier {
        "intelligence_crystal" => {
            character.intelligence_xp += 500;
            ctx.reply(
                Response::new_user_reply(&author, "você consumiu um cristal da inteligência e ganhou **500 XP**! Estude uma vez e sinta o conhecimento expandir.")
                .add_emoji_prefix(item.emoji),
            ).await?;
        }
        "invigorating_crystal" => {
            if character.action_points < character.max_action_points {
                character.action_points = character.max_action_points;
            }

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
