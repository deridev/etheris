use std::time::Duration;

use anyhow::bail;
use etheris_rpg::{Battle, BattleController, BattleSettings, FighterData};

use crate::prelude::*;

#[command("Use itens para o seu personagem!")]
#[name("usar")]
#[character_required(true)]
pub async fn usecmd(
    mut ctx: CommandContext,
    #[rename("item")]
    #[description("Nome do item que você quer ler")]
    item: String,
    #[rename("usuário")]
    #[description("O usuário que você quer usar o item")]
    user: Option<User>,
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
            user_character.health_xp += 150;
            user_character.knowledge_xp += 80;
            user_character.strength_xp += 150;
            user_character.intelligence_xp += 150;
            ctx.db().characters().save(user_character).await?;

            ctx.reply(
                Response::new_user_reply(
                    &user,
                    "você recebeu um presente! O presente te deu muito XP. Aproveite e agradeça a pessoa que te ofereceu!",
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
                FighterData::new_from_character(0, &character, author, Default::default()),
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
        _ => {
            ctx.reply(
                Response::new_user_reply(
                    &author,
                    "esse item não pode ser utilizado com esse comando!",
                )
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
