use std::time::Duration;

use etheris_common::Color;
use etheris_data::{emojis, items, ShopItem};
use etheris_database::bson::oid::ObjectId;
use etheris_discord::{
    twilight_http::request::channel::reaction::RequestReactionType,
    twilight_model::{
        channel::{
            message::{component::ButtonStyle, ReactionType},
            Message,
        },
        user::User,
    },
    ActionRowBuilder, ButtonBuilder, EmbedAuthor, EmbedBuilder, InteractionExtension,
    UserExtension,
};
use etheris_framework::{util::make_multiple_rows, watcher::WatcherOptions, *};
use tokio_stream::StreamExt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Shop {
    pub shop_name: String,
    pub profit_goes_to: Option<ObjectId>,
    pub items: Vec<ShopItem>,
}

const ITEMS_PER_PAGE: usize = 6;

impl Shop {
    pub fn new(shop_name: String, profit_owner: Option<ObjectId>, items: Vec<ShopItem>) -> Shop {
        Shop {
            shop_name,
            profit_goes_to: profit_owner,
            items,
        }
    }

    pub async fn prompt(mut self, user: User, ctx: &mut CommandContext) -> anyhow::Result<()> {
        let Some(channel_id) = ctx.interaction.channel.as_ref().map(|c| c.id) else {
            ctx.send(
                Response::new_user_reply(
                    &user,
                    "esse comando precisa ser usado em um canal válido!",
                )
                .add_emoji_prefix(emojis::ERROR),
            )
            .await?;
            return Ok(());
        };

        let user_id = user.id;
        if self.items.is_empty() {
            ctx.send(
                Response::new_user_reply(
                    &user,
                    format!(
                        "a loja **{}** não possui nenhum item à venda!",
                        self.shop_name
                    ),
                )
                .add_emoji_prefix(emojis::ERROR),
            )
            .await?;
            return Ok(());
        }

        let mut pages = vec![];
        for i in (0..self.items.len()).step_by(ITEMS_PER_PAGE) {
            let mut items_indexes = vec![];
            let mut page = EmbedBuilder::new_common()
                .set_color(Color::CYAN_GREEN)
                .set_author(EmbedAuthor {
                    name: format!("Loja {}", self.shop_name),
                    icon_url: Some(user.avatar_url()),
                });

            for j in 0..ITEMS_PER_PAGE {
                let Some(shop_item) = self.items.get(i + j) else {
                    break;
                };

                items_indexes.push(i + j);

                let item = items::get_item(&shop_item.identifier).unwrap_or_else(|| {
                    panic!(
                        "items::get_item should never fail at Shop::prompt. Item: {:?}",
                        shop_item
                    )
                });

                page = page.add_inlined_field(
                    format!(
                        "{} {} ({}x)",
                        item.emoji, item.display_name, shop_item.quantity
                    ),
                    format!("{} ◎", shop_item.price),
                )
            }

            pages.push((page, items_indexes));
        }

        let mut page = 0;

        let items = self.get_shop_items(&pages[page].1);
        let message = ctx
            .send(
                Response::from(pages[page].0.clone())
                    .add_string_content(user.mention())
                    .set_components(self.make_rows(&items)),
            )
            .await?;

        let stream = ctx.watcher.create_component_stream(
            message.id,
            move |interaction| interaction.author_id() == Some(user_id),
            WatcherOptions {
                timeout: Duration::from_secs(512),
            },
        );
        tokio::pin!(stream);

        while let Some(Ok(interaction)) = stream.next().await {
            let data = interaction.parse_message_component_data()?;
            let mut ctx = CommandContext::from_with_interaction(ctx, interaction.into());

            let mut update_page = false;
            if data.custom_id == "next" {
                page = (page + 1) % pages.len();
                update_page = true;
            } else if data.custom_id == "previous" {
                page = if page == 0 { pages.len() - 1 } else { page - 1 };
                update_page = true;
            }

            if update_page {
                self.update_page(&mut ctx, page, &pages).await?;
                continue;
            }

            let item_id = data.custom_id.clone();
            let Some((index, shop_item)) = self
                .items
                .iter()
                .enumerate()
                .find(|(_, i)| i.identifier == item_id)
            else {
                ctx.send("Item não encontrado!").await?;
                break;
            };

            let item = items::get_item(&shop_item.identifier)
                .expect("-> This should not fail: items::get_item(&shop_item.identifier)");

            let buy_embed = EmbedBuilder::new_common()
                .set_color(Color::YELLOW)
                .set_author_to_user(&user)
                .set_description(format!(
                    "Escreva a quantia de **{} {}** que você quer comprar por **{} ◎**!",
                    item.emoji, item.display_name, shop_item.price
                ))
                .add_footer_text(format!("Quantia em estoque: {}", shop_item.quantity));

            ctx.update_message(Response::from(buy_embed).remove_all_components())
                .await?;
            let Ok(Some(msg)) = ctx
                .watcher
                .await_single_message(
                    channel_id,
                    move |m| m.author.id == user_id,
                    WatcherOptions {
                        timeout: Duration::from_secs(60),
                    },
                )
                .await
            else {
                self.update_specific_page(&message, &mut ctx, page, &pages)
                    .await?;
                continue;
            };

            let Ok(amount) = msg.content.parse::<i32>() else {
                ctx.client
                    .http
                    .create_reaction(
                        msg.channel_id,
                        msg.id,
                        &RequestReactionType::Unicode { name: "❓" },
                    )
                    .await
                    .ok();
                self.update_specific_page(&message, &mut ctx, page, &pages)
                    .await?;
                continue;
            };

            let amount = amount.clamp(1, i32::MAX) as i64;
            if amount > shop_item.quantity as i64 {
                ctx.send(
                    Response::new_user_reply(
                        &user,
                        "a loja não tem essa quantia do item em estoque!",
                    )
                    .add_emoji_prefix(emojis::ERROR),
                )
                .await?;
                self.update_specific_page(&message, &mut ctx, page, &pages)
                    .await?;
                continue;
            }

            let price = amount * shop_item.price;
            let confirmation = ctx
                .helper()
                .create_confirmation(
                    user.id,
                    true,
                    Response::new_user_reply(
                        &user,
                        format!(
                            "você tem certeza que quer comprar **{}x {}**? Vai custar **{} ◎**.",
                            amount, item.display_name, price
                        ),
                    )
                    .add_emoji_prefix(item.emoji),
                )
                .await?;
            if !confirmation {
                self.update_specific_page(&message, &mut ctx, page, &pages)
                    .await?;
                continue;
            }

            let Some(mut character) = ctx
                .db()
                .characters()
                .get_by_user(&user_id.to_string())
                .await?
            else {
                ctx.send("Você não tem um personagem!").await?;
                break;
            };

            if character.orbs < price {
                ctx.send(
                    Response::new_user_reply(&user, "você não tem orbs suficientes!")
                        .add_emoji_prefix(emojis::ERROR),
                )
                .await?;
                self.update_specific_page(&message, &mut ctx, page, &pages)
                    .await?;
                continue;
            }

            character.remove_orbs(price);
            character.add_item(item, amount as usize, None);
            ctx.db().characters().save(character).await?;

            self.items[index].quantity -= amount as i32;

            ctx.send(
                Response::new_user_reply(
                    &user,
                    format!(
                        "você comprou **{}x {}** com sucesso!",
                        amount, item.display_name
                    ),
                )
                .add_emoji_prefix(item.emoji),
            )
            .await?;
            self.update_specific_page(&message, &mut ctx, page, &pages)
                .await?;
        }

        Ok(())
    }

    pub async fn update_page(
        &self,
        ctx: &mut CommandContext,
        page: usize,
        pages: &[(EmbedBuilder, Vec<usize>)],
    ) -> anyhow::Result<()> {
        let page = pages.get(page).expect("pages.get(page) should not fail");
        let items = self.get_shop_items(&page.1);
        ctx.update_message(Response::from(page.0.clone()).set_components(self.make_rows(&items)))
            .await?;
        Ok(())
    }

    pub async fn update_specific_page(
        &self,
        message: &Message,
        ctx: &mut CommandContext,
        page: usize,
        pages: &[(EmbedBuilder, Vec<usize>)],
    ) -> anyhow::Result<()> {
        let page = pages.get(page).expect("pages.get(page) should not fail");
        let items = self.get_shop_items(&page.1);
        ctx.update_specific_message(
            message,
            Response::from(page.0.clone()).set_components(self.make_rows(&items)),
        )
        .await?;
        Ok(())
    }

    pub fn get_shop_items(&self, indexes: &[usize]) -> Vec<ShopItem> {
        self.items
            .iter()
            .enumerate()
            .filter(|(index, _)| indexes.contains(index))
            .map(|(_, item)| item.clone())
            .collect()
    }

    pub fn make_rows(&self, items: &[ShopItem]) -> Vec<ActionRowBuilder> {
        let mut buttons = vec![
            ButtonBuilder::new()
                .set_custom_id("previous")
                .set_emoji(ReactionType::Unicode {
                    name: "◀️".into()
                })
                .set_style(ButtonStyle::Primary),
            ButtonBuilder::new()
                .set_custom_id("next")
                .set_emoji(ReactionType::Unicode {
                    name: "▶️".into()
                })
                .set_style(ButtonStyle::Primary),
        ];

        for shop_item in items {
            let item = items::get_item(&shop_item.identifier).unwrap_or_else(|| {
                panic!(
                    "items::get_item should never fail at Shop::make_rows. Item: {:?}",
                    shop_item
                )
            });

            buttons.push(
                ButtonBuilder::new()
                    .set_custom_id(item.identifier)
                    .set_emoji(item.emoji)
                    .set_label(item.display_name),
            )
        }

        make_multiple_rows(buttons)
    }
}
