use crate::prelude::*;

const ITEMS_PER_PAGE: usize = 10;

#[command("Veja o inventário do seu personagem em Etheris")]
#[name("inventário")]
#[character_required(true)]
pub async fn inventory(mut ctx: CommandContext) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let character = parse_user_character!(ctx, author);

    let inventory = &character.inventory;
    if inventory.is_empty() {
        ctx.reply(
            Response::new_user_reply(
                &author,
                format!(
                    "o personagem **{}** não possui nenhum item!",
                    character.name
                ),
            )
            .set_ephemeral(),
        )
        .await?;
        return Ok(());
    }

    let mut pages = vec![];

    for i in (0..inventory.len()).step_by(ITEMS_PER_PAGE) {
        let mut embed = EmbedBuilder::new_common()
            .set_color(Color::LIGHT_GREEN)
            .set_author(EmbedAuthor {
                name: format!("Inventário do personagem {}", character.name),
                icon_url: Some(author.avatar_url()),
            });

        let mut description = String::new();

        for j in 0..ITEMS_PER_PAGE {
            let Some(inventory_item) = inventory.get(i + j) else {
                break;
            };

            let Some(item) = items::get_item(&inventory_item.identifier) else {
                println!(
                    "invalid item: {} | character id: {}",
                    inventory_item.identifier, character.id
                );
                continue;
            };

            description.push_str(&format!(
                "{} **{}** ({}x)",
                item.emoji, item.display_name, inventory_item.quantity
            ));

            let alt = inventory_item.values.alternative_names();
            if !alt.is_empty() {
                description.push_str(&format!("\n>  (`{}`)", alt.join("`), (`")))
            }

            description.push('\n');
        }

        embed = embed.set_description(description);
        pages.push(embed);
    }

    EmbedPagination::new(ctx, pages).send().await?;
    Ok(())
}
