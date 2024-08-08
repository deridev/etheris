use std::time::Duration;

use anyhow::bail;
use etheris_rpg::{
    pacts::list::get_boxed_pact_from_kind, Battle, BattleController, BattleSettings, Fighter,
    FighterData, FighterIndex,
};
use rand::{rngs::StdRng, SeedableRng};
use watcher::WatcherOptions;

use crate::prelude::*;

#[command("Use itens para o seu personagem!")]
#[name("usar")]
#[character_required(true)]
pub async fn usecmd(
    mut ctx: CommandContext,
    #[rename("item")]
    #[description("Nome do item que você quer usar")]
    item: String,
    #[rename("usuário")]
    #[description("O usuário que você quer usar o item")]
    user: Option<User>,
) -> anyhow::Result<()> {
    let author = ctx.author().await?;
    let character = parse_user_character!(ctx, author);
    let Some(inventory_item) = character.get_inventory_item_by_name(&item) else {
        ctx.reply(
            Response::new_user_reply(&author, "você não possui esse item ou esse item não existe! Utilize **/inventário** para ver os seus itens.")
            .add_emoji_prefix(emojis::ERROR),
        ).await?;
        return Ok(());
    };

    if let Some(user) = &user {
        let user_character = parse_user_character!(ctx, user);
        if user_character.region != character.region {
            ctx.reply(
                Response::new_user_reply(
                    user,
                    "você precisa estar na mesma região para usar esse item nesse usuário!",
                )
                .add_emoji_prefix(emojis::ERROR)
                .set_ephemeral(),
            )
            .await?;
            return Ok(());
        }
    }

    let Some(item) = items::get_item(&inventory_item.identifier) else {
        bail!("Item not found: {}", inventory_item.identifier);
    };

    match item.identifier {
        "gift" => {
            let Some(user) = user else {
                ctx.reply(
                    Response::new_user_reply(
                        &author,
                        "você precisa especificar o usuário que você quer dar o presente!",
                    )
                    .add_emoji_prefix(emojis::ERROR)
                    .set_ephemeral(),
                )
                .await?;
                return Ok(());
            };

            if user.id == author.id {
                ctx.send(
                    Response::new_user_reply(&author, "você não pode dar um presente a si mesmo!")
                        .add_emoji_prefix(emojis::ERROR)
                        .set_ephemeral(),
                )
                .await?;
                return Ok(());
            }

            let confirmation = ctx
                .helper()
                .create_confirmation(
                    user.id,
                    false,
                    Response::new_user_reply(
                        &user,
                        format!(
                            "**{}** está te oferecendo um presente! Você aceita?",
                            author.display_name(),
                        ),
                    )
                    .add_emoji_prefix("❓"),
                )
                .await?;
            if !confirmation {
                return Ok(());
            }

            let mut user_character = parse_user_character!(ctx, user);
            user_character.health_xp += 180;
            user_character.knowledge_xp += 80;
            user_character.strength_xp += 180;
            user_character.intelligence_xp += 180;
            user_character.add_orbs(75);
            ctx.db().characters().save(user_character).await?;

            ctx.reply(
                Response::new_user_reply(
                    &user,
                    "você recebeu um presente! O presente te deu muito XP e alguns orbs. Aproveite e agradeça a pessoa que te ofereceu!",
                )
                .add_emoji_prefix(items::special::GIFT.emoji)
            ).await?;
        }
        "trap" => {
            let Some(user) = user else {
                ctx.reply(
                    Response::new_user_reply(
                        &author,
                        "você precisa especificar o usuário que você quer prender em uma armadilha de batalha mortal!",
                    )
                    .add_emoji_prefix(emojis::ERROR)
                    .set_ephemeral(),
                )
                .await?;
                return Ok(());
            };

            if user.id == author.id {
                ctx.send(
                    Response::new_user_reply(
                        &author,
                        "você não pode colocar a si mesmo em uma armadilha de batalha mortal!",
                    )
                    .add_emoji_prefix(emojis::ERROR)
                    .set_ephemeral(),
                )
                .await?;
                return Ok(());
            }

            let user_character = parse_user_character!(ctx, user);
            if user_character.region != character.region {
                ctx.send(
                    Response::new_user_reply(
                        &author,
                        "você precisa estar na mesma região para colocar alguém em uma armadilha de batalha mortal!",
                    )
                    .add_emoji_prefix(emojis::ERROR)
                    .set_ephemeral(),
                )
                .await?;
                return Ok(());
            }

            let confirmation = ctx
                .helper()
                .create_confirmation(
                    user.id,
                    false,
                    Response::new_user_reply(
                        &user,
                        format!(
                            "**{}** está te oferecendo um presente! Você aceita?",
                            author.display_name(),
                        ),
                    )
                    .add_emoji_prefix("❓"),
                )
                .await?;
            if !confirmation {
                return Ok(());
            }

            let user_character = parse_user_character!(ctx, user);

            ctx.send_in_channel(
                Response::new_user_reply(&user,
                    "não era um presente. ERA UMA ARMADILHA! Em alguns segundos vocês estarão em uma batalha mortal. Boa sorte!")
                    .add_emoji_prefix(items::special::TRAP.emoji)
                ).await?;

            tokio::time::sleep(Duration::from_secs(2)).await;

            let fighters = vec![
                FighterData::new_from_character(0, &character, author.clone(), Default::default()),
                FighterData::new_from_character(1, &user_character, user, Default::default()),
            ];

            let battle = Battle::new(
                character.region,
                BattleSettings {
                    is_risking_life_allowed: true,
                    has_consequences: true,
                    casual: false,
                    max_intruders: 1,
                },
                fighters,
            )?;

            let mut battle_controller = BattleController::new(battle, ctx.clone());
            battle_controller.run().await?;
        }
        "internal_key" => {
            let mut character = parse_user_character!(ctx, author);

            let all_valid_pacts = PactKind::list()
                .iter()
                .cloned()
                .filter(|p| !character.pacts.contains(p))
                .collect::<Vec<_>>();

            use rand::prelude::SliceRandom;
            let Ok(pact) = all_valid_pacts
                .choose_weighted(&mut StdRng::from_entropy(), |p| p.rarity().weight())
            else {
                return Ok(());
            };

            let pact = pact.clone();

            let fighter = Fighter::new(
                0,
                FighterIndex(0),
                FighterIndex(0),
                FighterData::new_from_character(0, &character, author.clone(), Default::default()),
            );

            let dyn_pact = get_boxed_pact_from_kind(pact.clone());
            let pact_data = dyn_pact.data(&fighter);
            let auto_equipped = character.pacts.len() < character.max_pacts as usize;

            let pact_description = format!(
                "## 📜 {}\n**{}**\n{}",
                pact_data.name,
                pact.rarity().name(),
                pact_data.description
            );

            let current_pacts_description = {
                let mut parts = vec![];
                for pact in character.pacts.iter() {
                    let dyn_pact = get_boxed_pact_from_kind(pact.clone());
                    let data = dyn_pact.data(&fighter);
                    parts.push(format!("**{}**: {}", data.name, data.description));
                }

                parts.join("\n")
            };

            let optional_substitution_string = if auto_equipped {
                String::new()
            } else {
                format!("\n\n## Escolha um pacto para substituir:\n{current_pacts_description}")
            };

            let embed = EmbedBuilder::new_common()
                .set_author(EmbedAuthor {
                    name: format!("{} descobriu um pacto!", author.display_name()),
                    icon_url: Some(author.avatar_url()),
                })
                .set_color(pact.rarity().color())
                .set_description(format!("{pact_description}{optional_substitution_string}",));

            if auto_equipped {
                character.pacts.insert(pact.clone());
                ctx.db().characters().save(character).await?;

                ctx.reply(embed).await?;
            } else {
                let mut buttons = vec![];
                let pacts = {
                    let mut p = vec![];
                    for pact in character.pacts.iter() {
                        let dyn_pact = get_boxed_pact_from_kind(pact.clone());
                        let data = dyn_pact.data(&fighter);
                        p.push((data, pact.clone()));
                    }

                    p
                };

                for (data, _) in pacts.iter() {
                    buttons.push(
                        ButtonBuilder::new()
                            .set_label(data.name.to_string())
                            .set_custom_id(data.identifier),
                    );
                }

                let action_row = ActionRowBuilder::new().add_buttons(buttons);

                ctx.reply(Response::from(embed).set_components(vec![action_row]))
                    .await?;

                let message = ctx.fetch_interaction_reply().await?;

                let Ok(Some(interaction)) = ctx
                    .watcher
                    .await_single_component(
                        message.id,
                        move |interaction| interaction.author_id() == Some(author.id),
                        WatcherOptions {
                            timeout: Duration::from_secs(250),
                        },
                    )
                    .await
                else {
                    return Ok(());
                };

                let data = interaction.parse_message_component_data()?;
                let choosen_pact = pacts
                    .iter()
                    .find(|p| p.0.identifier == data.custom_id)
                    .context("expected a valid pact")?
                    .1
                    .clone();

                let mut character = parse_user_character!(ctx, author);
                if !character.pacts.remove(&choosen_pact) {
                    return Ok(());
                }
                character.pacts.insert(pact);
                ctx.db().characters().save(character).await?;

                let mut interaction_ctx =
                    CommandContext::from_with_interaction(&ctx, Box::new(interaction.clone()));
                interaction_ctx
                    .reply(Response::new_user_reply(
                        &author,
                        format!("você formou o pacto **{}**!", pact_data.name),
                    ))
                    .await?;
            }
        }
        _ => {
            ctx.reply(
                Response::new_user_reply(
                    &author,
                    "esse item não pode ser utilizado com esse comando! Tente **/ler** ou **/consumir**.",
                )
                .add_emoji_prefix(emojis::ERROR),
            )
            .await?;
            return Ok(());
        }
    }

    let mut character = parse_user_character!(ctx, author);
    character.remove_item(item, 1);
    ctx.db().characters().save(character).await?;

    Ok(())
}
