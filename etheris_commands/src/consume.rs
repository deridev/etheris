// use anyhow::bail;
use etheris_data::items::Item;
// use rand::{Rng, SeedableRng};

use crate::prelude::*;

#[command("Consuma itens para recuperar fome, sede ou energia do seu personagem!")]
#[name("consumir")]
#[character_required(true)]
pub async fn consume(
    mut ctx: CommandContext,
    #[rename("item")]
    #[description("Nome do item que você quer consumir")]
    _item: String,
    #[rename("quantidade")]
    #[description("Quantidade de itens que você quer consumir")]
    _quantity: Option<i64>,
) -> anyhow::Result<()> {
    /*
    let quantity = quantity.unwrap_or(1).clamp(1, i32::MAX as i64) as i32;
    let author = ctx.author().await?;
    let Some(item) = items::get_item_by_name(&item) else {
        ctx.reply(Response::new_user_reply(
            &author,
            format!("nenhum item chamado **{}** foi encontrado! Utilize **\/inventário** para ver os seus itens.", item)
        ).add_emoji_prefix(emojis::ERROR)
        ).await?;

        return Ok(());
    };

    let character = parse_user_character!(ctx, author);
    if !character.has_item(&item, quantity as usize) {
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
        special_consumption(ctx, item, author, character).await?;
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

    let hunger = consumption_properties.hunger_regeneration * quantity;
    let thirst = consumption_properties.thirst_regeneration * quantity;
    let stamina = consumption_properties.stamina_regeneration * quantity;
    let calories = consumption_properties.calories * quantity as f32;
    let _sugar_level = consumption_properties.sugar_level * quantity as f32;
    let _salt_level = consumption_properties.salt_level * quantity as f32;

    let weight = calories * 0.006;

    let mut character = parse_user_character!(ctx, author);
    character.remove_item(item, quantity as usize);
    character.stats.hunger = (hunger + character.stats.hunger).min(character.stats.resistance);
    character.stats.thirst = (thirst + character.stats.thirst).min(character.stats.resistance);
    character.stats.stamina = (stamina + character.stats.stamina).min(character.stats.resistance);

    character.stats.weight += weight;
    ctx.db().characters().save(character).await?;

    let mut output_effects = Vec::new();
    if hunger != 0 {
        output_effects.push(format!("{} `{}`", emojis::HUNGER, hunger));
    }

    if thirst != 0 {
        output_effects.push(format!("{} `{}`", emojis::THIRST, thirst));
    }

    if stamina != 0 {
        output_effects.push(format!("{} `{}`", emojis::STAMINA, stamina));
    }

    ctx.send(
        Response::new_user_reply(
            &author,
            format!(
                "você consumiu **{}x {}**! {}",
                quantity,
                item.display_name,
                output_effects.join(", ")
            ),
        )
        .add_emoji_prefix(format!("😋{}", item.emoji)),
    )
    .await?;
    */

    Ok(())
}

#[allow(unused)]
async fn special_consumption(
    mut ctx: CommandContext,
    item: Item,
    user: User,
    mut character: CharacterModel,
) -> anyhow::Result<()> {
    /*
    character.remove_item(item, 1);

    let mut output = vec![];

    match item.identifier {
        "intelligence_crystal" => {
            let iq_gain = rand::rngs::StdRng::from_entropy().gen_range(3.0..=5.0);
            output.push(format!(
                "{} Seu QI aumentou em **{iq_gain:.2}**!",
                emojis::IQ
            ));
            character.add_iq(iq_gain as f32);
        }
        "invigorating_crystal" => {
            character.stats.hunger = character.stats.resistance;
            character.stats.thirst = character.stats.resistance;
            character.stats.stamina = character.stats.resistance;
            output.push("Você se sente completamente revigorado(a)!".into());
        }
        _ => bail!(
            "Item does not have a consumption function: {}",
            item.identifier
        ),
    };

    ctx.db().characters().save(character).await?;

    ctx.send(
        Response::new_user_reply(
            &user,
            format!(
                "você consumiu o item especial **{}**!\n{}",
                item.display_name,
                output.join("\n")
            ),
        )
        .add_emoji_prefix(format!("😋{}", item.emoji)),
    )
    .await?;
    */
    Ok(())
}
