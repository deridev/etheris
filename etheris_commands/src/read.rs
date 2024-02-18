use anyhow::bail;
use etheris_data::{
    items::{get_item, recipes::ALL_RECIPES},
    util::translate,
};

use crate::prelude::*;

#[command("Leia algum item em seu inventário!")]
#[name("ler")]
#[character_required(true)]
pub async fn read(
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
        )
        .await?;
        return Ok(());
    };

    let Some(item) = items::get_item(&inventory_item.identifier) else {
        bail!("Item not found: {}", inventory_item.identifier);
    };

    let recipes = inventory_item
        .values
        .recipes()
        .into_iter()
        .filter_map(|name| ALL_RECIPES.iter().find(|r| r.output == name))
        .collect::<Vec<_>>();
    if !recipes.is_empty() {
        const RECIPES_PER_PAGE: usize = 5;
        let mut pages = vec![];

        for i in (0..recipes.len()).step_by(RECIPES_PER_PAGE) {
            let embed = EmbedBuilder::new_common()
                .set_color(Color::BLURPLE)
                .set_author(EmbedAuthor {
                    name: format!("Leitura por {}", author.display_name()),
                    icon_url: Some(author.avatar_url()),
                });

            let mut description = format!("{} **Livro**: {}\n", item.emoji, item.display_name);

            for j in 0..RECIPES_PER_PAGE {
                let Some(recipe) = recipes.get(i + j) else {
                    break;
                };

                let Some(output_item) = get_item(recipe.output) else {
                    break;
                };

                let ingredients = recipe
                    .ingredients
                    .iter()
                    .filter_map(|ingredient| {
                        let item = get_item(ingredient.item)?;
                        Some(format!("{}x {}", ingredient.quantity, item.display_name))
                    })
                    .collect::<Vec<_>>();

                description.push_str(&format!(
                    "{} **{}**: `{}`",
                    output_item.emoji,
                    output_item.display_name,
                    ingredients.join(" + ")
                ));
            }

            pages.push(embed.set_description(description));
        }

        EmbedPagination::new(ctx, pages).send().await?;
        return Ok(());
    }

    let mut used_translator = false;

    if item.pages.is_empty() {
        ctx.reply(
            Response::new_user_reply(&author, "esse item não pode ser lido ou está vazio!")
                .add_emoji_prefix(emojis::ERROR),
        )
        .await?;

        return Ok(());
    }

    if character.has_item(&items::tool::TRANSLATOR, 1) && item.pages[0].translate {
        let confirmation = ctx.helper().create_confirmation(
            author.id, false,
            Response::new_user_reply(&author, "você possui o item **Tradutor** e pode usar para ler. Você quer usar? (Você perderá o tradutor depois de usar!)")
            .add_emoji_prefix("❓")
        ).await?;
        used_translator = confirmation;

        if confirmation {
            character.remove_item(items::tool::TRANSLATOR, 1);
            ctx.db().characters().save(character).await?;
        }
    }

    let mut pages = vec![];

    for page in item.pages {
        let embed = EmbedBuilder::new_common()
            .set_color(Color::BLURPLE)
            .set_author(EmbedAuthor {
                name: format!("Leitura por {}", author.display_name()),
                icon_url: Some(author.avatar_url()),
            });

        let mut description = format!("{} **Livro**: {}\n", item.emoji, item.display_name);
        if page.translate && !used_translator {
            description.push_str(&format!(
                "## {}\n{}",
                translate(page.title),
                translate(page.content)
            ));
        } else {
            description.push_str(&format!("## {}\n{}", page.title, page.content));
        }

        pages.push(embed.set_description(description));
    }

    EmbedPagination::new(ctx, pages).send().await?;

    Ok(())
}
