use etheris_data::items::recipes::{get_item_by_recipe, Ingredient};

use crate::prelude::*;

#[command("Crie itens misturando ingredientes!")]
#[name("criar")]
#[character_required(true)]
pub async fn craft(
    mut ctx: CommandContext,
    #[rename("receita")]
    #[description("Receita que você quer criar. Exemplo: ovo + sal")]
    recipe: String,
    #[rename("quantidade")]
    #[description("Quantidade que você quer criar. Aumenta o número de ingredientes usados.")]
    quantity: Option<i64>,
) -> anyhow::Result<()> {
    let quantity = quantity.unwrap_or(1).clamp(1, i32::MAX as i64) as usize;
    let author = ctx.author().await?;

    let recipe = recipe
        .split('+')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>();

    let mut ingredients = vec![];
    for mut ingredient_token in recipe {
        let mut explicit_ingredient_quantity = None;

        // Optional prefix of the item quantity in recipe
        // Ex: item + 2x item2
        let prefixed_part = ingredient_token
            .split_whitespace()
            .next()
            .unwrap_or_default();
        if let Some(quantity) = prefixed_part.strip_suffix('x') {
            let Ok(quantity) = quantity.parse::<usize>() else {
                ctx.reply(
                    Response::new_user_reply(&author, "ingrediente inválido! Se você quiser especificar a quantidade de um ingrediente, use o formato: `[quantia]x <ingrediente>`. Exemplo: `2x ovo`.")
                        .add_emoji_prefix(emojis::ERROR),
                )
                .await?;
                return Ok(());
            };

            explicit_ingredient_quantity = Some(quantity);
            ingredient_token = ingredient_token
                .split_whitespace()
                .nth(1)
                .unwrap_or_default()
                .to_string();
        }

        let quantity = explicit_ingredient_quantity.unwrap_or(quantity);

        let Some(item) = items::get_item_by_name(&ingredient_token) else {
            ctx.reply(Response::new_user_reply(
                &author,
                format!("nenhum item chamado **{}** foi encontrado! Utilize **/inventário** para ver os seus itens.", ingredient_token)
            ).add_emoji_prefix(emojis::ERROR)
            ).await?;

            return Ok(());
        };

        ingredients.push(Ingredient {
            item: item.identifier,
            quantity,
        });
    }

    let Some(output) = get_item_by_recipe(&ingredients) else {
        ctx.reply(
            Response::new_user_reply(
                &author,
                "nenhum item foi criado com essa combinação de ingredientes!",
            )
            .add_emoji_prefix(emojis::ERROR),
        )
        .await?;
        return Ok(());
    };

    let ingredients_string = output
        .2
        .iter()
        .map(|i| {
            let item = items::get_item(i.item);
            match item {
                None => format!("{}x ITEM_INVÁLIDO", i.quantity),
                Some(item) => format!("{}x {}", i.quantity, item.display_name),
            }
        })
        .collect::<Vec<_>>();
    let confirmation = ctx.helper().create_confirmation(
        author.id, false,
        Response::new_user_reply(
            &author,
            format!("você tem certeza que quer fazer essa criação? Você receberá **{}x {}** {}\nIngredientes: `{}`", output.1, output.0.display_name, output.0.emoji, ingredients_string.join("`, `"))
        ).add_emoji_prefix(format!("{}❓", output.0.emoji))
    ).await?;

    if !confirmation {
        return Ok(());
    }

    let mut character = parse_user_character!(ctx, author);

    for ingredient in output.2 {
        let item = items::get_item(ingredient.item).context("expected a valid ingredient item")?;
        if !character.has_item(&item, ingredient.quantity) {
            ctx.reply(
                Response::new_user_reply(
                    &author,
                    format!(
                        "você não tem **{}x {}**!",
                        ingredient.quantity, item.display_name
                    ),
                )
                .add_emoji_prefix(emojis::ERROR),
            )
            .await?;
            return Ok(());
        }

        character.remove_item(item, ingredient.quantity);
    }

    character.add_recipe(output.0.identifier.to_owned());
    character.add_item(output.0, output.1, None);
    ctx.db().characters().save(character).await?;

    ctx.send(
        Response::new_user_reply(
            &author,
            format!(
                "você criou **{}x {}** com sucesso!",
                output.1, output.0.display_name
            ),
        )
        .add_emoji_prefix(format!("{}✅", output.0.emoji)),
    )
    .await?;

    Ok(())
}
