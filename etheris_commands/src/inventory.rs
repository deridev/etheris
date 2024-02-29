use std::time::Duration;

use etheris_database::common::InventoryItem;
use etheris_discord::twilight_model::channel::message::component::ButtonStyle;
use etheris_framework::watcher::WatcherOptions;
use tokio_stream::StreamExt;

use crate::prelude::*;

const ITEMS_PER_PAGE: usize = 10;

#[command("Veja o inventário do seu personagem em Etheris")]
#[name("inventário")]
#[character_required(true)]
pub async fn inventory(mut ctx: CommandContext) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let character = parse_user_character!(ctx, author);

    let mut inventory = &character.inventory;
    let mut is_battle_inventory = false;

    let mut pages = generate_inventory_pages(&author, inventory, is_battle_inventory, &character);
    let mut page = 0;

    let rows = generate_rows(is_battle_inventory, inventory.is_empty());
    let message = ctx
        .send(Response::from(pages[page].clone()).set_components(rows))
        .await?;

    let stream = ctx.watcher.create_component_stream(
        message.id,
        move |interaction| interaction.author_id() == Some(author.id),
        WatcherOptions {
            timeout: Duration::from_secs(500),
        },
    );

    tokio::pin!(stream);

    while let Some(Ok(interaction)) = stream.next().await {
        let data = interaction.parse_message_component_data()?;

        let mut ctx = CommandContext::from_with_interaction(&ctx, interaction.into());

        if data.custom_id == "change_inventory" {
            is_battle_inventory = !is_battle_inventory;
            inventory = if is_battle_inventory {
                &character.battle_inventory
            } else {
                &character.inventory
            };
            pages = generate_inventory_pages(&author, inventory, is_battle_inventory, &character);
            page = 0;

            ctx.update_message(
                Response::from(pages[page].clone())
                    .set_components(generate_rows(is_battle_inventory, inventory.is_empty())),
            )
            .await?;
            continue;
        }

        if data.custom_id == "previous" {
            page = if page == 0 { pages.len() - 1 } else { page - 1 };
        } else if data.custom_id == "next" {
            page = (page + 1) % pages.len();
        }

        ctx.update_message(
            Response::from(pages[page].clone())
                .set_components(generate_rows(is_battle_inventory, inventory.is_empty())),
        )
        .await?;
    }

    Ok(())
}

fn generate_rows(is_battle_inventory: bool, is_inventory_empty: bool) -> Vec<ActionRowBuilder> {
    let mut rows = vec![ActionRowBuilder::new().add_button(
        ButtonBuilder::new()
            .set_label(if is_battle_inventory {
                "Ver Inventário"
            } else {
                "Ver Inventário de Batalha"
            })
            .set_custom_id("change_inventory")
            .set_style(ButtonStyle::Primary),
    )];

    if !is_inventory_empty {
        rows.push(ActionRowBuilder::new().add_buttons(vec![
                ButtonBuilder::new()
                    .set_emoji(Emoji::from_unicode("⬅️"))
                    .set_custom_id("previous"),
                ButtonBuilder::new()
                    .set_emoji(Emoji::from_unicode("➡️"))
                    .set_custom_id("next"),
            ]));
    }

    rows
}

fn generate_inventory_pages(
    user: &User,
    inventory: &[InventoryItem],
    is_battle_inventory: bool,
    character: &CharacterModel,
) -> Vec<EmbedBuilder> {
    let mut pages = vec![];

    for i in (0..=inventory.len()).step_by(ITEMS_PER_PAGE) {
        let mut embed = EmbedBuilder::new_common()
            .set_color(if is_battle_inventory {
                Color::LIGHT_RED
            } else {
                Color::LIGHT_GREEN
            })
            .set_author(EmbedAuthor {
                name: format!(
                    "{} do personagem {}",
                    if is_battle_inventory {
                        "Inventário de Batalha"
                    } else {
                        "Inventário"
                    },
                    character.name
                ),
                icon_url: Some(user.avatar_url()),
            })
            .add_footer_text(format!(
                "Página {} de {}",
                i / ITEMS_PER_PAGE + 1,
                (inventory.len() / ITEMS_PER_PAGE) + 1
            ));

        let mut description = String::new();

        if is_battle_inventory {
            description = "## ⚔️ Esse é o seu Inventário de Batalha!\nEsse inventário será usado para armazenar os itens que você deseja usar em batalhas. Use **/alocar** para mover itens do inventário padrão para o de batalha.\n\n".to_string();
        }

        let mut is_empty = true;
        for j in 0..ITEMS_PER_PAGE {
            let Some(inventory_item) = inventory.get(i + j) else {
                break;
            };

            is_empty = false;

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

        if is_empty {
            description.push_str("## Inventário Vazio!");
        }

        embed = embed.set_description(description);
        pages.push(embed);
    }

    pages
}
